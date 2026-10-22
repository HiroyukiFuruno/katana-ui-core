#[cfg(test)]
use super::RichTextStyle;
use super::text_runtime::{RichTextDrawRequest, default_line_height};
use super::{Canvas, RichTextBidiContext, RichTextLineSpan, TextRenderer};

impl TextRenderer {
    pub(in crate::raster_host) fn measure_rich_span_layout_with_bidi_context(
        &self,
        source_spans: &[RichTextLineSpan],
        bidi_context: Option<&RichTextBidiContext>,
    ) -> (Vec<(usize, usize)>, usize) {
        let source_text = source_spans
            .iter()
            .map(|span| span.text.as_str())
            .collect::<String>();
        let spans = Self::rich_raster_spans(source_spans, bidi_context);
        let font = self.font_with_size(
            source_spans
                .first()
                .map(|span| span.style.size)
                .unwrap_or(self.font.size),
        );
        let line_height_px = source_spans
            .iter()
            .map(|span| default_line_height(span.style.size))
            .fold(default_line_height(font.size), f32::max);
        let Some(raster) = self.rasterize(spans, font, 1.0, line_height_px) else {
            return fallback_rich_span_layout(self, source_spans);
        };
        let prefix_len = bidi_context.map_or(0, |context| context.prefix.len());
        let mut byte_start = 0usize;
        let mut layout = Vec::with_capacity(source_spans.len());
        for span in source_spans {
            let byte_end = byte_start.saturating_add(span.text.len());
            let bounds = raster
                .grapheme_bounds
                .iter()
                .filter(|bounds| {
                    bounds.byte_start >= prefix_len.saturating_add(byte_start)
                        && bounds.byte_end <= prefix_len.saturating_add(byte_end)
                })
                .map(|bounds| {
                    let start = super::text_runtime_paint::logical_origin(bounds.x);
                    let width = super::text_runtime_paint::logical_bound_extent(bounds.width);
                    (start, start.saturating_add(width))
                })
                .fold(None, |range: Option<(usize, usize)>, (start, end)| {
                    Some(match range {
                        Some((left, right)) => (left.min(start), right.max(end)),
                        None => (start, end),
                    })
                });
            layout.push(bounds.map_or((0, 0), |(start, end)| {
                (start, end.saturating_sub(start).max(1))
            }));
            byte_start = byte_end;
        }
        let width =
            super::text_runtime_paint::text_run_selection_width(&source_text, &raster, prefix_len);
        (layout, width)
    }

    #[cfg(test)]
    pub(crate) fn draw_rich_line_signed(
        &self,
        canvas: &mut Canvas,
        spans: &[RichTextLineSpan],
        x: isize,
        y: usize,
    ) {
        self.draw_rich_line_signed_with_optional_bidi_context(canvas, spans, None, x, y, None);
    }

    pub(crate) fn draw_rich_line_signed_with_resolved_direction(
        &self,
        canvas: &mut Canvas,
        spans: &[RichTextLineSpan],
        x: isize,
        y: usize,
        resolved_rtl: Option<bool>,
    ) {
        self.draw_rich_line_signed_with_optional_bidi_context(
            canvas,
            spans,
            None,
            x,
            y,
            resolved_rtl,
        );
    }

    pub(crate) fn draw_rich_line_signed_with_bidi_context_and_resolved_direction(
        &self,
        canvas: &mut Canvas,
        spans: &[RichTextLineSpan],
        bidi_context: &RichTextBidiContext,
        x: isize,
        y: usize,
        resolved_rtl: Option<bool>,
    ) {
        self.draw_rich_line_signed_with_optional_bidi_context(
            canvas,
            spans,
            Some(bidi_context),
            x,
            y,
            resolved_rtl,
        );
    }

    fn draw_rich_line_signed_with_optional_bidi_context(
        &self,
        canvas: &mut Canvas,
        spans: &[RichTextLineSpan],
        bidi_context: Option<&RichTextBidiContext>,
        x: isize,
        y: usize,
        resolved_rtl: Option<bool>,
    ) {
        let raster_vertical_scale = spans
            .iter()
            .map(|span| span.style.raster_vertical_scale)
            .fold(1.0_f32, f32::max);
        let font = self.font_with_size(
            spans
                .first()
                .map(|span| span.style.size)
                .unwrap_or(self.font.size),
        );
        let line_height_px = spans
            .iter()
            .map(|span| default_line_height(span.style.size))
            .fold(default_line_height(font.size), f32::max);
        self.draw_rich_request(
            canvas,
            spans,
            bidi_context,
            RichTextDrawRequest {
                x,
                paint_origin_y: y as f32,
                text_run_origin_y: y as f32,
                scale_factor: canvas.scale_factor(),
                raster_vertical_scale,
                font,
                line_height_px,
                resolved_rtl,
            },
        );
    }

    #[cfg(test)]
    pub(crate) fn measure_width_rich_with_bidi_context(
        &self,
        text: &str,
        style: RichTextStyle,
        bidi_context: &RichTextBidiContext,
    ) -> usize {
        let span = self.rich_line_span(text, style);
        let spans = Self::rich_raster_spans(&[span], Some(bidi_context));
        self.rasterize(
            spans,
            self.font_with_size(style.size),
            1.0,
            default_line_height(style.size),
        )
        .map_or_else(
            || self.measure_width_rich(text, style),
            |raster| {
                super::text_runtime_paint::text_run_selection_width(
                    text,
                    &raster,
                    bidi_context.prefix.len(),
                )
            },
        )
    }
}

fn fallback_rich_span_layout(
    renderer: &TextRenderer,
    source_spans: &[RichTextLineSpan],
) -> (Vec<(usize, usize)>, usize) {
    let mut cursor = 0usize;
    let layout = source_spans
        .iter()
        .map(|span| {
            let width = renderer.measure_width_rich(&span.text, span.style).max(1);
            let position = (cursor, width);
            cursor = cursor.saturating_add(width);
            position
        })
        .collect();
    (layout, cursor.max(1))
}

#[cfg(test)]
mod tests {
    use super::*;
    use katana_ui_core::facade::UiCoreFacade;
    use katana_ui_core::text_raster::{PlatformTextFaceSelection, PlatformTextRasterConfig};
    use katana_ui_core::theme::ThemeSnapshot;
    use std::path::PathBuf;

    fn renderer_without_color_emoji() -> TextRenderer {
        let config = PlatformTextRasterConfig {
            emoji_candidates: vec![PathBuf::from("/kuc-test-font-catalog/missing-emoji.ttf")],
            emoji_candidate_sha256: Vec::new(),
            ..PlatformTextRasterConfig::default()
        };
        let facade = UiCoreFacade::new(ThemeSnapshot::dark());
        TextRenderer::load_with_text_raster_config(
            &facade,
            "body",
            config,
            PlatformTextFaceSelection::System,
        )
    }

    #[test]
    fn bidi_measurement_falls_back_when_color_emoji_rasterization_is_unavailable() {
        let renderer = renderer_without_color_emoji();
        let style = RichTextStyle::new(14.0, 0xffffff).emoji(true);
        let context = RichTextBidiContext::default();

        assert_eq!(
            renderer.measure_width_rich("🔥", style),
            renderer.measure_width_rich_with_bidi_context("🔥", style, &context),
        );
    }
}
