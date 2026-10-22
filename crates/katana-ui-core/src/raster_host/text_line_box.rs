use super::text_runtime::{RichTextDrawRequest, ui_span};
use super::{Canvas, RichTextBidiContext, RichTextLineSpan, RichTextStyle, TextRenderer};
use katana_ui_core::render_model::UiTextSpan;
use katana_ui_core::theme::FontToken;

struct RichTextLineBox {
    top: f32,
    height: f32,
    baseline_from_top: f32,
}

impl TextRenderer {
    pub(crate) fn rich_line_raster_baseline(
        &self,
        spans: &[RichTextLineSpan],
        line_box_height: f32,
        scale_factor: f32,
    ) -> f32 {
        let font = self.font_with_size(
            spans
                .first()
                .map(|span| span.style.size)
                .unwrap_or(self.font.size),
        );
        let spans = spans
            .iter()
            .map(|span| ui_span(&span.text, span.style))
            .collect::<Vec<_>>();
        self.raster_baseline(&spans, font, line_box_height, scale_factor)
    }

    pub(crate) fn draw_signed_styled_in_line_box(
        &self,
        canvas: &mut Canvas,
        text: &str,
        x: isize,
        line_box_top: f32,
        line_box_height: f32,
        baseline_from_line_box_top: f32,
        style: RichTextStyle,
    ) {
        self.draw_layout_in_line_box(
            canvas,
            vec![ui_span(text, style)],
            x,
            line_box_top,
            line_box_height,
            baseline_from_line_box_top,
            style.raster_vertical_scale,
            self.font_with_size(style.size),
        );
    }

    #[cfg(test)]
    pub(crate) fn draw_rich_line_signed_in_line_box(
        &self,
        canvas: &mut Canvas,
        spans: &[RichTextLineSpan],
        x: isize,
        line_box_top: f32,
        line_box_height: f32,
        baseline_from_line_box_top: f32,
    ) -> f32 {
        self.draw_rich_line_signed_in_line_box_with_context(
            canvas,
            spans,
            None,
            x,
            RichTextLineBox {
                top: line_box_top,
                height: line_box_height,
                baseline_from_top: baseline_from_line_box_top,
            },
            None,
        )
    }

    pub(crate) fn draw_rich_line_signed_in_line_box_with_resolved_direction(
        &self,
        canvas: &mut Canvas,
        spans: &[RichTextLineSpan],
        x: isize,
        line_box_top: f32,
        line_box_height: f32,
        baseline_from_line_box_top: f32,
        resolved_rtl: Option<bool>,
    ) -> f32 {
        self.draw_rich_line_signed_in_line_box_with_context(
            canvas,
            spans,
            None,
            x,
            RichTextLineBox {
                top: line_box_top,
                height: line_box_height,
                baseline_from_top: baseline_from_line_box_top,
            },
            resolved_rtl,
        )
    }

    pub(crate) fn draw_rich_line_signed_in_line_box_with_bidi_context_and_resolved_direction(
        &self,
        canvas: &mut Canvas,
        spans: &[RichTextLineSpan],
        bidi_context: &RichTextBidiContext,
        x: isize,
        line_box_top: f32,
        line_box_height: f32,
        baseline_from_line_box_top: f32,
        resolved_rtl: Option<bool>,
    ) -> f32 {
        self.draw_rich_line_signed_in_line_box_with_context(
            canvas,
            spans,
            Some(bidi_context),
            x,
            RichTextLineBox {
                top: line_box_top,
                height: line_box_height,
                baseline_from_top: baseline_from_line_box_top,
            },
            resolved_rtl,
        )
    }

    fn draw_rich_line_signed_in_line_box_with_context(
        &self,
        canvas: &mut Canvas,
        spans: &[RichTextLineSpan],
        bidi_context: Option<&RichTextBidiContext>,
        x: isize,
        line_box: RichTextLineBox,
        resolved_rtl: Option<bool>,
    ) -> f32 {
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
        let raster_spans = Self::rich_raster_spans(spans, bidi_context);
        let scale = canvas.scale_factor();
        let raster_baseline =
            self.raster_baseline(&raster_spans, font.clone(), line_box.height, scale);
        let origin_y = draw_origin_for_target_baseline(
            line_box.top,
            line_box.baseline_from_top,
            raster_baseline,
        );
        self.draw_rich_request(
            canvas,
            spans,
            bidi_context,
            RichTextDrawRequest {
                x,
                paint_origin_y: origin_y,
                text_run_origin_y: line_box.top,
                scale_factor: scale,
                raster_vertical_scale,
                font,
                line_height_px: line_box.height,
                resolved_rtl,
            },
        );
        raster_baseline
    }

    #[allow(clippy::too_many_arguments)]
    fn draw_layout_in_line_box(
        &self,
        canvas: &mut Canvas,
        spans: Vec<UiTextSpan>,
        x: isize,
        line_box_top: f32,
        line_box_height: f32,
        baseline_from_line_box_top: f32,
        raster_vertical_scale: f32,
        font: FontToken,
    ) -> f32 {
        let scale = canvas.scale_factor();
        let raster_baseline = self.raster_baseline(&spans, font.clone(), line_box_height, scale);
        let origin_y = draw_origin_for_target_baseline(
            line_box_top,
            baseline_from_line_box_top,
            raster_baseline,
        );
        self.draw_request(
            canvas,
            spans,
            x,
            origin_y,
            line_box_top,
            scale,
            raster_vertical_scale,
            font,
            line_box_height,
        );
        raster_baseline
    }
}

pub(super) fn draw_origin_for_target_baseline(
    line_box_top: f32,
    baseline_from_line_box_top: f32,
    baseline_from_raster_origin: f32,
) -> f32 {
    line_box_top + baseline_from_line_box_top - baseline_from_raster_origin
}
