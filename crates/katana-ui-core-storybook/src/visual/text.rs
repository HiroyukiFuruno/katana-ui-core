use crate::visual::canvas::Canvas;
use katana_ui_core::facade::UiCoreFacade;
use katana_ui_core::render_model::{UiTextSpan, UiTextSpanStyle};
use katana_ui_core::theme::{FontFamily, FontToken};
use katana_ui_core_text_raster::{
    PlatformTextRaster, PlatformTextRasterConfig, PlatformTextRasterRequest, PlatformTextRasterizer,
};
use unicode_segmentation::UnicodeSegmentation;

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TextCacheStats {
    pub(crate) entries: usize,
    pub(crate) raster_misses: usize,
    pub(crate) font_database_loads: usize,
}
#[path = "text_types.rs"]
mod text_types;
pub use text_types::TextRenderer;
pub(crate) use text_types::{
    RichTextLineSpan, RichTextStyle, TextBox, TextHorizontalAlign, TextOrigin, TextVerticalAlign,
    TextVerticalBox,
};

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
const RGBA_ALPHA_COMPONENT_INDEX: usize = 3;
const OPAQUE_ALPHA: u8 = u8::MAX;
const TRANSPARENT_RGBA: [u8; RGBA_COMPONENT_COUNT] = [0; RGBA_COMPONENT_COUNT];
const VERTICAL_SCALE_COVERAGE_ROWS_PER_UNIT: f32 = 6.0;

impl TextRenderer {
    pub fn load(facade: &UiCoreFacade, role: &str) -> Self {
        let font = resolve_font(facade, role);
        Self {
            rasterizer: std::cell::RefCell::new(PlatformTextRasterizer::new(
                PlatformTextRasterConfig::default(),
            )),
            font,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas, text: &str, x: usize, y: usize, size: f32, color: u32) {
        self.draw_signed(canvas, text, x as isize, y, size, color);
    }

    pub(crate) fn draw_signed(
        &self,
        canvas: &mut Canvas,
        text: &str,
        x: isize,
        y: usize,
        size: f32,
        color: u32,
    ) {
        self.draw_layout(
            canvas,
            text,
            x,
            y,
            RichTextStyle::new(size, color),
            false,
            canvas.scale_factor(),
        );
    }

    pub(crate) fn draw_signed_styled(
        &self,
        canvas: &mut Canvas,
        text: &str,
        x: isize,
        y: usize,
        style: RichTextStyle,
    ) {
        self.draw_layout_with_line_height(
            canvas,
            text,
            x,
            y,
            style,
            canvas.scale_factor(),
            default_line_height(style.size),
        );
    }

    pub fn draw_emoji(
        &self,
        canvas: &mut Canvas,
        text: &str,
        x: usize,
        y: usize,
        size: f32,
        color: u32,
    ) {
        self.draw_layout(
            canvas,
            text,
            x as isize,
            y,
            RichTextStyle::new(size, color),
            true,
            canvas.scale_factor(),
        );
    }

    pub(crate) fn draw_rich_line_signed(
        &self,
        canvas: &mut Canvas,
        spans: &[RichTextLineSpan],
        x: isize,
        y: usize,
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
        let spans = spans
            .iter()
            .map(|span| ui_span(&span.text, span.style))
            .collect();
        self.draw_request(
            canvas,
            spans,
            x,
            y,
            canvas.scale_factor(),
            raster_vertical_scale,
            font,
            line_height_px,
        );
    }

    pub(crate) fn draw_centered(
        &self,
        canvas: &mut Canvas,
        text: &str,
        x: usize,
        vertical_box: TextVerticalBox,
        size: f32,
        color: u32,
    ) {
        self.draw_layout_with_line_height(
            canvas,
            text,
            x as isize,
            vertical_box.y,
            RichTextStyle::new(size, color),
            canvas.scale_factor(),
            vertical_box.height,
        );
    }

    pub(crate) fn draw_in_box(
        &self,
        canvas: &mut Canvas,
        text: &str,
        text_box: TextBox,
        size: f32,
        color: u32,
    ) {
        let origin = self.origin_in_box(text, text_box, size);
        self.draw_layout_with_line_height(
            canvas,
            text,
            origin.x as isize,
            origin.y,
            RichTextStyle::new(size, color),
            canvas.scale_factor(),
            text_box.height as f32,
        );
    }

    pub fn measure_width(&self, text: &str, size: f32) -> usize {
        self.measure_width_with_emoji(text, size, false)
    }

    pub fn measure_emoji_width(&self, text: &str, size: f32) -> usize {
        self.measure_width_with_emoji(text, size, true)
    }

    pub(crate) fn measure_width_rich(&self, text: &str, style: RichTextStyle) -> usize {
        self.measure_request(vec![ui_span(text, style)], style.size, 1.0)
    }

    fn measure_width_with_emoji(&self, text: &str, size: f32, emoji: bool) -> usize {
        self.measure_request(
            vec![ui_span(text, RichTextStyle::new(size, 0).emoji(emoji))],
            size,
            1.0,
        )
    }

    #[cfg(test)]
    pub(crate) fn origin_in_box_for_test(
        &self,
        text: &str,
        text_box: TextBox,
        size: f32,
    ) -> TextOrigin {
        self.origin_in_box(text, text_box, size)
    }

    #[cfg(test)]
    #[must_use]
    pub(crate) fn font_family(&self) -> FontFamily {
        self.font.family
    }

    #[cfg(test)]
    #[must_use]
    pub(crate) fn cache_stats(&self) -> TextCacheStats {
        let stats = self.rasterizer.borrow().stats();
        TextCacheStats {
            entries: stats.cache_entries,
            raster_misses: stats.cache_misses,
            font_database_loads: stats.font_database_loads,
        }
    }

    fn draw_layout(
        &self,
        canvas: &mut Canvas,
        text: &str,
        x: isize,
        y: usize,
        style: RichTextStyle,
        emoji: bool,
        scale_factor: f32,
    ) {
        self.draw_layout_with_line_height(
            canvas,
            text,
            x,
            y,
            style.emoji(emoji),
            scale_factor,
            default_line_height(style.size),
        );
    }

    fn draw_layout_with_line_height(
        &self,
        canvas: &mut Canvas,
        text: &str,
        x: isize,
        y: usize,
        style: RichTextStyle,
        scale_factor: f32,
        line_height_px: f32,
    ) {
        self.draw_request(
            canvas,
            vec![ui_span(text, style)],
            x,
            y,
            scale_factor,
            style.raster_vertical_scale,
            self.font_with_size(style.size),
            line_height_px,
        );
    }

    fn draw_request(
        &self,
        canvas: &mut Canvas,
        spans: Vec<UiTextSpan>,
        x: isize,
        y: usize,
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
        draw_raster(canvas, &raster, x, y, scale_factor, raster_vertical_scale);
        record_runtime_text_run(canvas, &text, &raster, x, y);
    }

    fn measure_request(&self, spans: Vec<UiTextSpan>, size: f32, scale_factor: f32) -> usize {
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

    fn rasterize(
        &self,
        spans: Vec<UiTextSpan>,
        font: FontToken,
        scale_factor: f32,
        line_height_px: f32,
    ) -> Option<PlatformTextRaster> {
        let mut request = PlatformTextRasterRequest::from_text(" ", font, TRANSPARENT_RGBA);
        request.spans = spans;
        request.line_height_px = line_height_px;
        request.scale_factor = normalized_scale_factor(scale_factor);
        self.rasterizer.borrow_mut().rasterize(&request).ok()
    }

    fn font_with_size(&self, size: f32) -> FontToken {
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

    fn origin_in_box(&self, text: &str, text_box: TextBox, size: f32) -> TextOrigin {
        let width = self.measure_width(text, size);
        let x = match text_box.horizontal_align {
            TextHorizontalAlign::Start => text_box.x,
            TextHorizontalAlign::Center => text_box.x + text_box.width.saturating_sub(width) / 2,
        };
        let y = match text_box.vertical_align {
            TextVerticalAlign::Top => text_box.y,
            TextVerticalAlign::Center => text_box.y,
        };
        TextOrigin { x, y }
    }
}

fn normalized_scale_factor(scale_factor: f32) -> f32 {
    if scale_factor.is_finite() && scale_factor >= 1.0 {
        scale_factor
    } else {
        1.0
    }
}

fn default_line_height(size: f32) -> f32 {
    size.max(1.0) * LINE_HEIGHT_RATIO
}

fn ui_span(text: &str, style: RichTextStyle) -> UiTextSpan {
    UiTextSpan {
        text: text.to_string(),
        style: UiTextSpanStyle {
            bold: style.bold,
            italic: style.italic,
            monospace: false,
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

fn draw_raster(
    canvas: &mut Canvas,
    raster: &PlatformTextRaster,
    origin_x: isize,
    origin_y: usize,
    scale_factor: f32,
    raster_vertical_scale: f32,
) {
    let scale = normalized_scale_factor(scale_factor);
    let origin_x = (origin_x as f64 * f64::from(scale)).round() as isize;
    let origin_y = (origin_y as f64 * f64::from(scale)).round() as isize;
    for (index, pixel) in raster.rgba_pixels.iter().enumerate() {
        let [red, green, blue, alpha] = *pixel;
        if alpha == 0 {
            continue;
        }
        let x = origin_x + (index % raster.width) as isize;
        let y = origin_y + (index / raster.width) as isize;
        for extra_y in 0..=extra_vertical_coverage_rows(raster_vertical_scale) {
            let y = y + extra_y;
            if x >= 0 && y >= 0 {
                canvas.blend_physical(x as usize, y as usize, packed_rgb(red, green, blue), alpha);
            }
        }
    }
}

fn record_runtime_text_run(
    canvas: &mut Canvas,
    text: &str,
    raster: &PlatformTextRaster,
    x: isize,
    y: usize,
) {
    let Some(origin_x) = usize::try_from(x).ok() else {
        return;
    };
    let (glyph_widths, selection_width) = selection_glyph_widths(text, raster);
    canvas.record_text_run_with_glyph_widths(
        text,
        origin_x,
        y,
        selection_width,
        raster
            .grapheme_bounds
            .iter()
            .map(|bounds| bounds.height.ceil().max(1.0) as usize)
            .max()
            .unwrap_or(1),
        &glyph_widths,
    );
}

fn selection_glyph_widths(text: &str, raster: &PlatformTextRaster) -> (Vec<usize>, usize) {
    let mut right = 0usize;
    let widths = text
        .grapheme_indices(true)
        .map(|(byte_start, grapheme)| {
            let next_right = raster
                .grapheme_bounds
                .iter()
                .find(|bounds| {
                    bounds.byte_start == byte_start
                        && bounds.byte_end == byte_start + grapheme.len()
                })
                .map(|bounds| logical_position(bounds.x + bounds.width))
                .unwrap_or_else(|| right.saturating_add(1));
            let next_right = next_right.max(right.saturating_add(1));
            let width = next_right.saturating_sub(right);
            right = next_right;
            width
        })
        .collect::<Vec<_>>();
    (widths, right.max(1))
}

fn logical_extent(extent: usize, scale_factor: f32) -> usize {
    (extent as f64 / f64::from(normalized_scale_factor(scale_factor)))
        .ceil()
        .max(1.0) as usize
}

fn logical_position(position: f32) -> usize {
    position.ceil().max(1.0) as usize
}

fn visible_raster_width(raster: &PlatformTextRaster, scale_factor: f32) -> usize {
    raster
        .rgba_pixels
        .iter()
        .enumerate()
        .filter(|(_, pixel)| pixel[RGBA_ALPHA_COMPONENT_INDEX] != 0)
        .map(|(index, _)| index % raster.width + 1)
        .max()
        .map(|extent| logical_extent(extent, scale_factor))
        .unwrap_or(1)
}

fn packed_rgb(red: u8, green: u8, blue: u8) -> u32 {
    (u32::from(red) << RGBA_RED_BIT_SHIFT)
        | (u32::from(green) << RGBA_GREEN_BIT_SHIFT)
        | u32::from(blue)
}

fn extra_vertical_coverage_rows(scale: f32) -> isize {
    ((scale - 1.0).max(0.0) * VERTICAL_SCALE_COVERAGE_ROWS_PER_UNIT).ceil() as isize
}

impl RichTextStyle {
    pub(crate) const fn new(size: f32, color: u32) -> Self {
        Self {
            size,
            color,
            bold: false,
            italic: false,
            emoji: false,
            raster_vertical_scale: 1.0,
        }
    }

    pub(crate) const fn bold(mut self, value: bool) -> Self {
        self.bold = value;
        self
    }

    pub(crate) const fn italic(mut self, value: bool) -> Self {
        self.italic = value;
        self
    }

    pub(crate) const fn emoji(mut self, value: bool) -> Self {
        self.emoji = value;
        self
    }

    pub(crate) const fn raster_vertical_scale(mut self, value: f32) -> Self {
        self.raster_vertical_scale = value;
        self
    }
}

impl TextBox {
    pub(crate) const fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            x,
            y,
            width,
            height,
            horizontal_align: TextHorizontalAlign::Start,
            vertical_align: TextVerticalAlign::Top,
        }
    }

    pub(crate) const fn centered(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self::new(x, y, width, height)
            .justify_content(TextHorizontalAlign::Center)
            .align_items(TextVerticalAlign::Center)
    }

    pub(crate) const fn justify_content(mut self, align: TextHorizontalAlign) -> Self {
        self.horizontal_align = align;
        self
    }

    pub(crate) const fn align_items(mut self, align: TextVerticalAlign) -> Self {
        self.vertical_align = align;
        self
    }
}

impl TextVerticalBox {
    pub(crate) const fn new(y: usize, height: f32) -> Self {
        Self { y, height }
    }
}

fn resolve_font(facade: &UiCoreFacade, role: &str) -> FontToken {
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
    use katana_ui_core::theme::ThemeSnapshot;

    #[test]
    fn scale_and_font_resolution_cover_invalid_and_empty_theme_fallbacks() {
        assert_eq!(1.0, normalized_scale_factor(f32::NAN));
        assert_eq!(1.0, normalized_scale_factor(0.5));
        assert_eq!(2.0, normalized_scale_factor(2.0));

        let mut theme = ThemeSnapshot::dark();
        theme.fonts.clear();
        let font = resolve_font(&UiCoreFacade::new(theme), "missing");
        assert_eq!(FALLBACK_FONT_NAME, font.name);
        assert_eq!(FontFamily::Proportional, font.family);
        assert_eq!(FALLBACK_FONT_SIZE, font.size);
        assert_eq!(REGULAR_WEIGHT, font.weight);
    }
}
