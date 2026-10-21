use super::{
    Canvas, SpanTextRenderers, TextRenderer, UiTextSpan, UiTreeTextLineContext, UiTreeTextMetrics,
    preserves_whitespace, span_line_width, span_part_width,
};
use crate::raster_host::text::RichTextLineSpan;
use std::ops::Range;
use unicode_bidi::BidiInfo;

pub(in crate::raster_host) struct PositionedSpan {
    pub(in crate::raster_host) line_span_index: usize,
    pub(in crate::raster_host) span: UiTextSpan,
    pub(in crate::raster_host) x: isize,
    pub(in crate::raster_host) width: usize,
    source_range: Range<usize>,
}

pub(in crate::raster_host) fn visual_span_positions(
    renderers: SpanTextRenderers<'_>,
    line: &[UiTextSpan],
    metrics: UiTreeTextMetrics,
    preserve_whitespace: bool,
    x: isize,
) -> Option<Vec<PositionedSpan>> {
    if !renderers.has_independent_code_raster_config() {
        return None;
    }
    if !line.iter().any(is_code) || !line.iter().any(|span| !is_code(span)) {
        return None;
    }
    let text = line
        .iter()
        .map(|span| span.text.as_str())
        .collect::<String>();
    let bidi = BidiInfo::new(&text, None);
    let paragraph = bidi.paragraphs.first()?;
    let (levels, visual_ranges) = bidi.visual_runs(paragraph, paragraph.range.clone());
    let reorders_spans = visual_ranges
        .windows(2)
        .any(|ranges| ranges[1].start < ranges[0].start)
        || visual_ranges.iter().any(|range| {
            levels
                .get(range.start)
                .is_some_and(unicode_bidi::Level::is_rtl)
        });
    if !reorders_spans {
        return None;
    }
    let mut positioned = Vec::new();
    let mut cursor = x;
    for visual_range in visual_ranges {
        let level = levels.get(visual_range.start)?;
        let mut fragments = Vec::new();
        let mut source_start = 0;
        for (line_span_index, source_span) in line.iter().enumerate() {
            let source_end = source_start + source_span.text.len();
            let source_range = source_start..source_end;
            if let Some(fragment_range) = intersection(&source_range, &visual_range) {
                let start = fragment_range.start.checked_sub(source_start)?;
                let end = fragment_range.end.checked_sub(source_start)?;
                if !text.is_char_boundary(fragment_range.start)
                    || !text.is_char_boundary(fragment_range.end)
                    || !source_span.text.is_char_boundary(start)
                    || !source_span.text.is_char_boundary(end)
                {
                    return None;
                }
                let mut span = source_span.clone();
                span.text = source_span.text[start..end].to_owned();
                fragments.push(PositionedSpan {
                    line_span_index,
                    span,
                    x,
                    width: 0,
                    source_range: fragment_range,
                });
            }
            source_start = source_end;
        }
        if level.is_rtl() {
            fragments.reverse();
        }
        for fragment in &mut fragments {
            fragment.x = cursor;
            fragment.width =
                span_part_width(renderers, &fragment.span, metrics, preserve_whitespace);
            cursor = cursor.saturating_add(fragment.width as isize);
        }
        positioned.extend(fragments);
    }
    positioned.sort_by_key(|fragment| fragment.source_range.start);
    (!positioned.is_empty()).then_some(positioned)
}

pub(super) struct RasterRun<'a> {
    renderer: &'a TextRenderer,
    spans: Vec<RichTextLineSpan>,
    source_spans: Vec<UiTextSpan>,
    x: isize,
}

impl<'a> RasterRun<'a> {
    pub(super) fn for_line(
        context: UiTreeTextLineContext<'a>,
        line: &[UiTextSpan],
        rich: &'a [RichTextLineSpan],
        x: isize,
    ) -> Vec<Self> {
        if !context.code_renderer.has_independent_raster_config() {
            return vec![Self {
                renderer: context.renderer,
                spans: rich.to_vec(),
                source_spans: line.to_vec(),
                x,
            }];
        }
        let renderers = SpanTextRenderers::new(context.renderer, context.code_renderer);
        let mut source_runs = source_runs(line, rich, renderers, x);
        let Some(positioned_spans) = visual_span_positions(
            renderers,
            line,
            context.metrics,
            preserves_whitespace(context.node),
            x,
        ) else {
            assign_source_positions(
                &mut source_runs,
                renderers,
                context.metrics,
                preserves_whitespace(context.node),
                x,
            );
            return source_runs;
        };
        let mut runs = Vec::with_capacity(positioned_spans.len());
        for positioned in positioned_spans {
            let Some(rich_span) = rich.get(positioned.line_span_index) else {
                assign_source_positions(
                    &mut source_runs,
                    renderers,
                    context.metrics,
                    preserves_whitespace(context.node),
                    x,
                );
                return source_runs;
            };
            runs.push(Self {
                renderer: renderers.for_span(&positioned.span),
                spans: vec![rich_span.with_text(positioned.span.text.clone())],
                source_spans: vec![positioned.span],
                x: positioned.x,
            });
        }
        runs
    }

    pub(super) fn draw(&self, canvas: &mut Canvas, metrics: UiTreeTextMetrics, top: f32) {
        if let Some(baseline) = metrics.baseline_from_line_box_top {
            self.renderer.draw_rich_line_signed_in_line_box(
                canvas,
                &self.spans,
                self.x,
                top,
                metrics.line_box_height,
                baseline,
            );
        } else {
            let integer_top = top.max(0.0).floor() as usize;
            canvas.with_fractional_y_origin(top, integer_top, |canvas| {
                self.renderer
                    .draw_rich_line_signed(canvas, &self.spans, self.x, integer_top);
            });
        }
    }
}

fn source_runs<'a>(
    line: &[UiTextSpan],
    rich: &'a [RichTextLineSpan],
    renderers: SpanTextRenderers<'a>,
    x: isize,
) -> Vec<RasterRun<'a>> {
    let mut runs = Vec::new();
    let mut start = 0;
    while start < line.len() {
        let code = is_code(&line[start]);
        let end = line[start..]
            .iter()
            .position(|span| is_code(span) != code)
            .map_or(line.len(), |offset| start + offset);
        runs.push(RasterRun {
            renderer: renderers.for_span(&line[start]),
            spans: rich[start..end].to_vec(),
            source_spans: line[start..end].to_vec(),
            x,
        });
        start = end;
    }
    runs
}

fn intersection(left: &Range<usize>, right: &Range<usize>) -> Option<Range<usize>> {
    let start = left.start.max(right.start);
    let end = left.end.min(right.end);
    (start < end).then_some(start..end)
}

fn assign_source_positions(
    runs: &mut [RasterRun<'_>],
    renderers: SpanTextRenderers<'_>,
    metrics: UiTreeTextMetrics,
    preserve_whitespace: bool,
    x: isize,
) {
    let mut cursor = x;
    for run in runs {
        run.x = cursor;
        cursor =
            cursor.saturating_add(run_width(run, renderers, metrics, preserve_whitespace) as isize);
    }
}

fn run_width(
    run: &RasterRun<'_>,
    renderers: SpanTextRenderers<'_>,
    metrics: UiTreeTextMetrics,
    preserve_whitespace: bool,
) -> usize {
    span_line_width(renderers, &run.source_spans, metrics, preserve_whitespace)
}

fn is_code(span: &UiTextSpan) -> bool {
    span.style.monospace || span.style.inline_code
}
