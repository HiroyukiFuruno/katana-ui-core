use super::text_runtime_paint::{
    draw_raster, record_runtime_text_run, record_runtime_text_run_with_raster_prefix,
    visible_raster_width,
};
#[cfg(test)]
use super::text_runtime_paint::{selection_glyph_positions, selection_glyph_widths};
use super::{RichTextBidiContext, RichTextLineSpan, RichTextStyle, TextRenderer};
use crate::raster_host::canvas::Canvas;
use katana_ui_core::facade::UiCoreFacade;
use katana_ui_core::render_model::{UiTextSpan, UiTextSpanStyle};
use katana_ui_core::text_raster::{PlatformTextRaster, PlatformTextRasterRequest};
use katana_ui_core::theme::{FontFamily, FontToken};

const LINE_HEIGHT_RATIO: f32 = 1.45;
const REGULAR_WEIGHT: u16 = 400;
const FALLBACK_FONT_SIZE: f32 = 14.0;
const FALLBACK_FONT_NAME: &str = "body";
const CODE_FONT_ROLE: &str = "code";
const SHORTCUT_FONT_ROLE: &str = "shortcut";
const RGBA_COMPONENT_COUNT: usize = 4;
const RGBA_RED_BIT_SHIFT: u32 = 16;
const RGBA_GREEN_BIT_SHIFT: u32 = 8;
const RGBA_CHANNEL_MASK: u32 = 0xff;
const OPAQUE_ALPHA: u8 = u8::MAX;
const TRANSPARENT_RGBA: [u8; RGBA_COMPONENT_COUNT] = [0; RGBA_COMPONENT_COUNT];

pub(super) struct RichTextDrawRequest {
    pub(super) x: isize,
    pub(super) paint_origin_y: f32,
    pub(super) text_run_origin_y: f32,
    pub(super) scale_factor: f32,
    pub(super) raster_vertical_scale: f32,
    pub(super) font: FontToken,
    pub(super) line_height_px: f32,
    pub(super) resolved_rtl: Option<bool>,
}

impl TextRenderer {
    pub(super) fn draw_rich_request(
        &self,
        canvas: &mut Canvas,
        source_spans: &[RichTextLineSpan],
        bidi_context: Option<&RichTextBidiContext>,
        request: RichTextDrawRequest,
    ) {
        let source_text = source_spans
            .iter()
            .map(|span| span.text.as_str())
            .collect::<String>();
        let spans = Self::rich_raster_spans(source_spans, bidi_context);
        let Some(raster) = self.rasterize(
            spans,
            request.font,
            request.scale_factor,
            request.line_height_px,
        ) else {
            return;
        };
        draw_raster(
            canvas,
            &raster,
            request.x,
            request.paint_origin_y,
            request.scale_factor,
            request.raster_vertical_scale,
        );
        record_runtime_text_run_with_raster_prefix(
            canvas,
            &source_text,
            &raster,
            request.x,
            request.text_run_origin_y,
            bidi_context.map_or(0, |context| context.prefix.len()),
            request.resolved_rtl,
        );
    }

    pub(super) fn rich_raster_spans(
        source_spans: &[RichTextLineSpan],
        bidi_context: Option<&RichTextBidiContext>,
    ) -> Vec<UiTextSpan> {
        let mut spans = source_spans
            .iter()
            .map(|span| ui_span(&span.text, span.style))
            .collect::<Vec<_>>();
        let Some(context) = bidi_context else {
            return spans;
        };
        let Some(first) = spans.first_mut() else {
            return spans;
        };
        first.text = format!("{}{}", context.prefix, first.text);
        if let Some(last) = spans.last_mut() {
            last.text.push_str(&context.suffix);
        }
        spans
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn draw_request(
        &self,
        canvas: &mut Canvas,
        spans: Vec<UiTextSpan>,
        x: isize,
        paint_origin_y: f32,
        text_run_origin_y: f32,
        scale_factor: f32,
        raster_vertical_scale: f32,
        font: FontToken,
        line_height_px: f32,
    ) {
        let text = spans
            .iter()
            .map(|span| span.text.as_str())
            .collect::<String>();
        let Some(raster) = self.rasterize(spans, font, scale_factor, line_height_px) else {
            return;
        };
        draw_raster(
            canvas,
            &raster,
            x,
            paint_origin_y,
            scale_factor,
            raster_vertical_scale,
        );
        record_runtime_text_run(canvas, &text, &raster, x, text_run_origin_y);
    }

    pub(super) fn measure_request(
        &self,
        spans: Vec<UiTextSpan>,
        size: f32,
        scale_factor: f32,
    ) -> usize {
        if spans.iter().all(|span| span.text.is_empty()) {
            return 0;
        }
        match self.rasterize(
            spans,
            self.font_with_size(size),
            scale_factor,
            default_line_height(size),
        ) {
            Some(raster) => visible_raster_width(&raster, scale_factor),
            None => size.ceil().max(1.0) as usize,
        }
    }

    pub(super) fn rasterize(
        &self,
        spans: Vec<UiTextSpan>,
        font: FontToken,
        scale_factor: f32,
        line_height_px: f32,
    ) -> Option<PlatformTextRaster> {
        let mut request = PlatformTextRasterRequest::from_text(" ", font, TRANSPARENT_RGBA);
        request.spans = spans;
        request.line_height_px = line_height_px;
        request.scale_factor = Self::normalized_scale_factor(scale_factor);
        self.rasterizer.borrow_mut().rasterize(&request).ok()
    }

    pub(super) fn raster_baseline(
        &self,
        spans: &[UiTextSpan],
        font: FontToken,
        line_box_height: f32,
        scale_factor: f32,
    ) -> f32 {
        let scale = Self::normalized_scale_factor(scale_factor);
        let mut request = PlatformTextRasterRequest::from_text(" ", font, TRANSPARENT_RGBA);
        request.spans = spans.to_vec();
        request.line_height_px = line_box_height;
        request.scale_factor = scale;
        self.rasterizer
            .borrow_mut()
            .measure_line_metrics(&request)
            .map(|metrics| metrics.baseline_from_raster_origin_px)
            .unwrap_or_default()
    }

    pub(super) fn font_with_size(&self, size: f32) -> FontToken {
        let mut font = self.font.clone();
        font.weight = font.weight.max(REGULAR_WEIGHT);
        font.size = size.max(1.0);
        font
    }

    pub(crate) fn rich_line_span(
        &self,
        text: impl Into<String>,
        style: RichTextStyle,
    ) -> RichTextLineSpan {
        RichTextLineSpan {
            text: text.into(),
            style,
        }
    }

    pub(super) fn normalized_scale_factor(scale_factor: f32) -> f32 {
        if scale_factor.is_finite() && scale_factor >= 1.0 {
            scale_factor
        } else {
            1.0
        }
    }
}

pub(super) fn default_line_height(size: f32) -> f32 {
    size.max(1.0) * LINE_HEIGHT_RATIO
}

pub(super) fn ui_span(text: &str, style: RichTextStyle) -> UiTextSpan {
    UiTextSpan {
        text: text.to_string(),
        style: UiTextSpanStyle {
            bold: style.bold,
            italic: style.italic,
            monospace: style.monospace,
            underline: false,
            strikethrough: false,
            highlight: false,
            current_highlight: false,
            inline_code: false,
            inline_math: false,
            emoji: style.emoji,
            color_rgba: packed_rgba(style.color),
        },
        link_target: String::new(),
    }
}

fn packed_rgba(color: u32) -> [u8; RGBA_COMPONENT_COUNT] {
    [
        ((color >> RGBA_RED_BIT_SHIFT) & RGBA_CHANNEL_MASK) as u8,
        ((color >> RGBA_GREEN_BIT_SHIFT) & RGBA_CHANNEL_MASK) as u8,
        (color & RGBA_CHANNEL_MASK) as u8,
        OPAQUE_ALPHA,
    ]
}

pub(super) fn resolve_font(facade: &UiCoreFacade, role: &str) -> FontToken {
    if let Some(font) = facade.theme().font(role) {
        return font.clone();
    }
    if role == SHORTCUT_FONT_ROLE
        && let Some(font) = facade.theme().font(CODE_FONT_ROLE)
    {
        return font.clone();
    }
    if let Some(font) = facade.font(facade.default_font_role()) {
        return font.clone();
    }
    FontToken {
        name: FALLBACK_FONT_NAME.to_string(),
        family: FontFamily::Proportional,
        size: FALLBACK_FONT_SIZE,
        weight: REGULAR_WEIGHT,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use katana_ui_core::text_raster::{PlatformTextRasterConfig, PlatformTextRasterizer};
    use katana_ui_core::theme::ThemeSnapshot;
    use std::cell::RefCell;
    use std::path::PathBuf;
    use unicode_segmentation::UnicodeSegmentation;

    fn renderer_without_color_emoji() -> TextRenderer {
        let config = PlatformTextRasterConfig {
            emoji_candidates: vec![PathBuf::from("/kuc-test-font-catalog/missing-emoji.ttf")],
            emoji_candidate_sha256: Vec::new(),
            ..PlatformTextRasterConfig::default()
        };
        let facade = UiCoreFacade::new(ThemeSnapshot::dark());
        TextRenderer {
            independent_raster_config: false,
            rasterizer: RefCell::new(PlatformTextRasterizer::new(config)),
            font: resolve_font(&facade, "body"),
        }
    }

    #[test]
    fn scale_and_font_resolution_cover_invalid_and_empty_theme_fallbacks() {
        assert_eq!(1.0, TextRenderer::normalized_scale_factor(f32::NAN));
        assert_eq!(1.0, TextRenderer::normalized_scale_factor(0.5));
        assert_eq!(2.0, TextRenderer::normalized_scale_factor(2.0));

        let mut theme = ThemeSnapshot::dark();
        theme.fonts.clear();
        let font = resolve_font(&UiCoreFacade::new(theme), "missing");
        assert_eq!(FALLBACK_FONT_NAME, font.name);
        assert_eq!(FontFamily::Proportional, font.family);
        assert_eq!(FALLBACK_FONT_SIZE, font.size);
        assert_eq!(REGULAR_WEIGHT, font.weight);
    }

    #[test]
    fn ui_span_preserves_the_requested_monospace_face() {
        let span = ui_span("code", RichTextStyle::new(14.0, 0).monospace(true));

        assert!(span.style.monospace);
        assert!(!span.style.inline_code);
    }

    #[test]
    fn fractional_draw_origin_is_quantized_once_at_the_physical_canvas_boundary() {
        assert_eq!(23, (11.25_f64 * 2.0).round() as isize);
        assert_eq!(11, (11.25_f64 * 1.0).round() as isize);
    }

    #[test]
    fn shortcut_role_uses_code_font_when_shortcut_font_is_missing() {
        let theme = ThemeSnapshot::dark();
        let code_font = theme
            .font(CODE_FONT_ROLE)
            .cloned()
            .expect("dark theme code font");

        assert_eq!(
            code_font,
            resolve_font(&UiCoreFacade::new(theme), SHORTCUT_FONT_ROLE)
        );
    }

    #[test]
    fn selection_widths_remain_progressive_when_raster_bounds_are_missing() {
        let renderer = TextRenderer::load(&UiCoreFacade::new(ThemeSnapshot::light()), "body");
        let mut raster = renderer
            .rasterize(
                vec![ui_span("ab", RichTextStyle::new(14.0, 0xffffff))],
                renderer.font_with_size(14.0),
                1.0,
                default_line_height(14.0),
            )
            .expect("plain text should rasterize");
        raster.grapheme_bounds.clear();

        assert_eq!((vec![1, 1], 2), selection_glyph_widths("ab", &raster));
    }

    #[test]
    fn selection_glyph_positions_preserve_rtl_raster_geometry() {
        let text = "אבג";
        let renderer = TextRenderer::load(&UiCoreFacade::new(ThemeSnapshot::light()), "body");
        let raster = renderer
            .rasterize(
                vec![ui_span(text, RichTextStyle::new(14.0, 0xffffff))],
                renderer.font_with_size(14.0),
                1.0,
                default_line_height(14.0),
            )
            .expect("RTL text should rasterize");

        let (positions, selection_width) = selection_glyph_positions(text, &raster);

        assert_eq!(text.grapheme_indices(true).count(), positions.len());
        assert!(
            positions
                .windows(2)
                .all(|positions| positions[0].0 > positions[1].0),
            "RTL source graphemes must retain their descending visual x positions"
        );
        assert_eq!(
            positions
                .iter()
                .map(|(x, width)| x.saturating_add(*width))
                .max()
                .unwrap_or(1),
            selection_width
        );
    }

    #[test]
    fn measure_request_uses_the_stable_font_size_when_color_emoji_is_unavailable() {
        let renderer = renderer_without_color_emoji();

        let width = renderer.measure_request(
            vec![ui_span(
                "emoji",
                RichTextStyle::new(14.0, 0xffffff).emoji(true),
            )],
            14.0,
            1.0,
        );

        assert_eq!(14, width);
    }

    #[test]
    fn rich_draw_skips_an_unavailable_color_emoji_raster() {
        let renderer = renderer_without_color_emoji();
        let style = RichTextStyle::new(14.0, 0xffffff).emoji(true);
        let spans = [renderer.rich_line_span("🔥", style)];
        let mut canvas = Canvas::new(80, 40, 0);

        renderer.draw_rich_request(
            &mut canvas,
            &spans,
            None,
            RichTextDrawRequest {
                x: 0,
                paint_origin_y: 0.0,
                text_run_origin_y: 0.0,
                scale_factor: 1.0,
                raster_vertical_scale: 1.0,
                font: renderer.font_with_size(style.size),
                line_height_px: default_line_height(style.size),
                resolved_rtl: None,
            },
        );

        assert!(canvas.pixels().iter().all(|pixel| *pixel == 0));
        assert!(canvas.text_runs().is_empty());
    }

    #[test]
    fn empty_rich_spans_stay_empty_when_bidi_context_is_present() {
        let context = RichTextBidiContext {
            prefix: "\u{202e}".to_string(),
            suffix: "\u{202c}".to_string(),
        };

        assert!(TextRenderer::rich_raster_spans(&[], Some(&context)).is_empty());
    }
}
