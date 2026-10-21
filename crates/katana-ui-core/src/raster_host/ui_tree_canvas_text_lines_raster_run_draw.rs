use super::raster_runs::{is_code, visual_span_positions};
use super::{
    Canvas, SpanTextRenderers, TextRenderer, UiTextSpan, UiTreeTextLineContext, UiTreeTextMetrics,
    preserves_whitespace, span_line_width,
};
use crate::raster_host::text::{RichTextBidiContext, RichTextLineSpan};

pub(super) struct RasterRun<'a> {
    renderer: &'a TextRenderer,
    spans: Vec<RichTextLineSpan>,
    source_spans: Vec<UiTextSpan>,
    x: isize,
    bidi_context: Option<RichTextBidiContext>,
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
                bidi_context: None,
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
            let rich_span = &rich[positioned.line_span_index];
            runs.push(Self {
                renderer: renderers.for_span(&positioned.span),
                spans: vec![rich_span.with_text(positioned.span.text.clone())],
                source_spans: vec![positioned.span],
                x: positioned.x,
                bidi_context: positioned.bidi_context,
            });
        }
        runs
    }

    pub(super) fn draw(&self, canvas: &mut Canvas, metrics: UiTreeTextMetrics, top: f32) {
        if let Some(baseline) = metrics.baseline_from_line_box_top {
            if let Some(bidi_context) = &self.bidi_context {
                self.renderer
                    .draw_rich_line_signed_in_line_box_with_bidi_context(
                        canvas,
                        &self.spans,
                        bidi_context,
                        self.x,
                        top,
                        metrics.line_box_height,
                        baseline,
                    );
            } else {
                self.renderer.draw_rich_line_signed_in_line_box(
                    canvas,
                    &self.spans,
                    self.x,
                    top,
                    metrics.line_box_height,
                    baseline,
                );
            }
        } else {
            let integer_top = top.max(0.0).floor() as usize;
            canvas.with_fractional_y_origin(top, integer_top, |canvas| {
                if let Some(bidi_context) = &self.bidi_context {
                    self.renderer.draw_rich_line_signed_with_bidi_context(
                        canvas,
                        &self.spans,
                        bidi_context,
                        self.x,
                        integer_top,
                    );
                } else {
                    self.renderer
                        .draw_rich_line_signed(canvas, &self.spans, self.x, integer_top);
                }
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
            bidi_context: None,
        });
        start = end;
    }
    runs
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
