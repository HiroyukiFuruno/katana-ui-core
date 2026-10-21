use super::{
    Canvas, SpanTextRenderers, TextRenderer, UiTextSpan, UiTreeTextLineContext, UiTreeTextMetrics,
    preserves_whitespace, span_line_width,
};
use crate::raster_host::text::RichTextLineSpan;
use std::ops::Range;
use unicode_bidi::BidiInfo;

pub(super) struct RasterRun<'a> {
    renderer: &'a TextRenderer,
    spans: Vec<RichTextLineSpan>,
    source_spans: Vec<UiTextSpan>,
    source_range: Range<usize>,
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
                source_range: 0..line.iter().map(|span| span.text.len()).sum(),
                x,
            }];
        }
        let renderers = SpanTextRenderers::new(context.renderer, context.code_renderer);
        let mut source_runs = Vec::new();
        let mut start = 0;
        let mut byte_start = 0;
        while start < line.len() {
            let code = is_code(&line[start]);
            let end = line[start..]
                .iter()
                .position(|span| is_code(span) != code)
                .map_or(line.len(), |offset| start + offset);
            let byte_end = byte_start
                + line[start..end]
                    .iter()
                    .map(|span| span.text.len())
                    .sum::<usize>();
            source_runs.push(Self {
                renderer: renderers.for_span(&line[start]),
                spans: rich[start..end].to_vec(),
                source_spans: line[start..end].to_vec(),
                source_range: byte_start..byte_end,
                x,
            });
            byte_start = byte_end;
            start = end;
        }

        let text = line
            .iter()
            .map(|span| span.text.as_str())
            .collect::<String>();
        let bidi = BidiInfo::new(&text, None);
        let Some(paragraph) = bidi.paragraphs.first() else {
            assign_source_positions(
                &mut source_runs,
                renderers,
                context.metrics,
                preserves_whitespace(context.node),
                x,
            );
            return source_runs;
        };
        let (levels, visual_ranges) = bidi.visual_runs(paragraph, paragraph.range.clone());
        let Some(mut runs) = split_at_visual_ranges(&source_runs, &visual_ranges, &text) else {
            assign_source_positions(
                &mut source_runs,
                renderers,
                context.metrics,
                preserves_whitespace(context.node),
                x,
            );
            return source_runs;
        };
        if !assign_visual_positions(
            &mut runs,
            &visual_ranges,
            &levels,
            renderers,
            context.metrics,
            preserves_whitespace(context.node),
            x,
        ) {
            assign_source_positions(
                &mut source_runs,
                renderers,
                context.metrics,
                preserves_whitespace(context.node),
                x,
            );
            return source_runs;
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

fn split_at_visual_ranges<'a>(
    source_runs: &[RasterRun<'a>],
    visual_ranges: &[Range<usize>],
    text: &str,
) -> Option<Vec<RasterRun<'a>>> {
    let mut fragments = Vec::new();
    for source_run in source_runs {
        for visual_range in visual_ranges {
            let Some(range) = intersection(&source_run.source_range, visual_range) else {
                continue;
            };
            fragments.push(source_run.fragment(range, text)?);
        }
    }
    fragments.sort_by_key(|run| run.source_range.start);
    (!fragments.is_empty()).then_some(fragments)
}

fn intersection(left: &Range<usize>, right: &Range<usize>) -> Option<Range<usize>> {
    let start = left.start.max(right.start);
    let end = left.end.min(right.end);
    (start < end).then_some(start..end)
}

impl<'a> RasterRun<'a> {
    fn fragment(&self, fragment_range: Range<usize>, text: &str) -> Option<Self> {
        if !text.is_char_boundary(fragment_range.start)
            || !text.is_char_boundary(fragment_range.end)
        {
            return None;
        }
        let mut spans = Vec::new();
        let mut source_spans = Vec::new();
        let mut cursor = self.source_range.start;
        for (source_span, rich_span) in self.source_spans.iter().zip(&self.spans) {
            let span_end = cursor + source_span.text.len();
            let span_range = cursor..span_end;
            if let Some(range) = intersection(&span_range, &fragment_range) {
                let start = range.start.checked_sub(cursor)?;
                let end = range.end.checked_sub(cursor)?;
                if !source_span.text.is_char_boundary(start)
                    || !source_span.text.is_char_boundary(end)
                {
                    return None;
                }
                let fragment_text = source_span.text[start..end].to_owned();
                let mut source_fragment = source_span.clone();
                source_fragment.text = fragment_text.clone();
                let rich_fragment = rich_span.with_text(fragment_text);
                source_spans.push(source_fragment);
                spans.push(rich_fragment);
            }
            cursor = span_end;
        }
        (!spans.is_empty()).then_some(Self {
            renderer: self.renderer,
            spans,
            source_spans,
            source_range: fragment_range,
            x: self.x,
        })
    }
}

fn assign_visual_positions(
    runs: &mut [RasterRun<'_>],
    visual_ranges: &[Range<usize>],
    levels: &[unicode_bidi::Level],
    renderers: SpanTextRenderers<'_>,
    metrics: UiTreeTextMetrics,
    preserve_whitespace: bool,
    x: isize,
) -> bool {
    let mut cursor = x;
    let mut positioned = vec![false; runs.len()];
    for visual_range in visual_ranges {
        let Some(level) = levels.get(visual_range.start) else {
            return false;
        };
        let mut indexes = runs
            .iter()
            .enumerate()
            .filter_map(|(index, run)| {
                (visual_range.start <= run.source_range.start
                    && run.source_range.end <= visual_range.end)
                    .then_some(index)
            })
            .collect::<Vec<_>>();
        if level.is_rtl() {
            indexes.reverse();
        }
        for index in indexes {
            if positioned[index] {
                return false;
            }
            runs[index].x = cursor;
            cursor = cursor.saturating_add(run_width(
                &runs[index],
                renderers,
                metrics,
                preserve_whitespace,
            ) as isize);
            positioned[index] = true;
        }
    }
    positioned.into_iter().all(|positioned| positioned)
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
