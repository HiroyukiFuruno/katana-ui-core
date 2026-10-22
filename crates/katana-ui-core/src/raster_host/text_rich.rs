use super::text_runtime::{RichTextDrawRequest, default_line_height};
use super::{Canvas, RichTextBidiContext, RichTextLineSpan, RichTextStyle, TextRenderer};

impl TextRenderer {
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
