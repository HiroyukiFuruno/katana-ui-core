use super::{
    Canvas, SpanTextRenderers, TextRenderer, UiTextSpan, UiTreeTextLineContext, UiTreeTextMetrics,
    preserves_whitespace, span_line_width,
};
use crate::raster_host::text::RichTextLineSpan;

pub(super) struct RasterRun<'a> {
    renderer: &'a TextRenderer,
    spans: &'a [RichTextLineSpan],
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
                spans: rich,
                x,
            }];
        }
        let renderers = SpanTextRenderers::new(context.renderer, context.code_renderer);
        let mut runs = Vec::new();
        let mut start = 0;
        let mut cursor = x;
        while start < line.len() {
            let code = is_code(&line[start]);
            let end = line[start..]
                .iter()
                .position(|span| is_code(span) != code)
                .map_or(line.len(), |offset| start + offset);
            runs.push(Self {
                renderer: renderers.for_span(&line[start]),
                spans: &rich[start..end],
                x: cursor,
            });
            cursor = cursor.saturating_add(span_line_width(
                renderers,
                &line[start..end],
                context.metrics,
                preserves_whitespace(context.node),
            ) as isize);
            start = end;
        }
        runs
    }

    pub(super) fn draw(&self, canvas: &mut Canvas, metrics: UiTreeTextMetrics, top: f32) {
        if let Some(baseline) = metrics.baseline_from_line_box_top {
            self.renderer.draw_rich_line_signed_in_line_box(
                canvas,
                self.spans,
                self.x,
                top,
                metrics.line_box_height,
                baseline,
            );
        } else {
            let integer_top = top.max(0.0).floor() as usize;
            canvas.with_fractional_y_origin(top, integer_top, |canvas| {
                self.renderer
                    .draw_rich_line_signed(canvas, self.spans, self.x, integer_top);
            });
        }
    }
}

fn is_code(span: &UiTextSpan) -> bool {
    span.style.monospace || span.style.inline_code
}
