use crate::visual::canvas::Canvas;
use crate::visual::markdown_font_loader::font_system_with_markdown_fonts;
use cosmic_text::SwashCache;
use katana_ui_core::facade::UiCoreFacade;
use katana_ui_core::theme::{FontFamily, FontToken};
use std::cell::RefCell;
use unicode_segmentation::UnicodeSegmentation;

#[cfg(test)]
pub(crate) use super::text_raster::TextCacheStats;
use super::text_raster::{RichTextRasterSpan, TextRasterCache, TextStyle};
use super::text_raster_request::TextRasterDrawRequest;
#[path = "text_types.rs"]
mod text_types;
pub use text_types::TextRenderer;
pub(crate) use text_types::{
    RichTextLineSpan, RichTextStyle, TextBox, TextHorizontalAlign, TextOrigin, TextVerticalAlign,
    TextVerticalBox,
};

const LINE_HEIGHT_RATIO: f32 = 1.45;
const REGULAR_WEIGHT: u16 = 400;
const BOLD_WEIGHT: u16 = 700;
const FALLBACK_FONT_SIZE: f32 = 14.0;
const FALLBACK_FONT_NAME: &str = "body";
const CODE_FONT_ROLE: &str = "code";
const SHORTCUT_FONT_ROLE: &str = "shortcut";

impl TextRenderer {
    pub fn load(facade: &UiCoreFacade, role: &str) -> Self {
        let font = resolve_font(facade, role);
        Self {
            font_system: RefCell::new(font_system_with_markdown_fonts()),
            swash_cache: RefCell::new(SwashCache::new()),
            raster_cache: RefCell::new(TextRasterCache::default()),
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
            TextStyle::new(size, size * LINE_HEIGHT_RATIO, color),
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
        self.draw_layout_with_font(
            canvas,
            text,
            x,
            y,
            TextStyle::new(style.size, style.size * LINE_HEIGHT_RATIO, style.color)
                .italic(style.italic)
                .raster_vertical_scale(style.raster_vertical_scale),
            style.emoji,
            canvas.scale_factor(),
            &self.font_for_weight(style.bold),
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
            TextStyle::new(size, size * LINE_HEIGHT_RATIO, color),
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
        self.record_rich_text_run(canvas, spans, x, y);
        let normalized_scale_factor = normalized_scale_factor(canvas.scale_factor());
        let raster_spans = spans
            .iter()
            .map(RichTextLineSpan::to_raster_span)
            .collect::<Vec<_>>();
        self.raster_cache.borrow_mut().draw_rich_line(
            canvas,
            &raster_spans,
            scaled_signed_coordinate(x, normalized_scale_factor),
            scaled_unsigned_coordinate(y, normalized_scale_factor),
            normalized_scale_factor,
            &mut self.font_system.borrow_mut(),
            &mut self.swash_cache.borrow_mut(),
        );
    }

    fn record_rich_text_run(
        &self,
        canvas: &mut Canvas,
        spans: &[RichTextLineSpan],
        x: isize,
        y: usize,
    ) {
        let Some(origin_x) = usize::try_from(x).ok() else {
            return;
        };
        let text = spans
            .iter()
            .map(|span| span.text.as_str())
            .collect::<String>();
        if text.is_empty() {
            return;
        }
        let mut line_height = 1usize;
        let mut glyph_widths = Vec::new();
        let mut width = 0usize;
        {
            let mut font_system = self.font_system.borrow_mut();
            let mut swash_cache = self.swash_cache.borrow_mut();
            for span in spans {
                for grapheme in span.text.graphemes(true) {
                    let glyph_width = TextRasterCache::measure_width_uncached(
                        grapheme,
                        span.style,
                        &span.font,
                        span.emoji,
                        &mut font_system,
                        &mut swash_cache,
                        1.0,
                    );
                    glyph_widths.push(glyph_width.max(1));
                    width = width.saturating_add(glyph_width.max(1));
                }
                line_height = line_height.max(span.style.line_height.ceil().max(1.0) as usize);
            }
        }
        canvas.record_text_run_with_glyph_widths(
            &text,
            origin_x,
            y,
            width.max(1),
            line_height,
            &glyph_widths,
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
        self.draw_layout(
            canvas,
            text,
            x as isize,
            vertical_box.y,
            TextStyle::new(size, vertical_box.height, color),
            false,
            canvas.scale_factor(),
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
        self.draw_layout(
            canvas,
            text,
            origin.x as isize,
            origin.y,
            TextStyle::new(size, text_box.line_height(), color),
            false,
            canvas.scale_factor(),
        );
    }

    pub fn measure_width(&self, text: &str, size: f32) -> usize {
        self.measure_width_with_emoji(text, size, false)
    }

    pub fn measure_emoji_width(&self, text: &str, size: f32) -> usize {
        self.measure_width_with_emoji(text, size, true)
    }

    pub(crate) fn measure_width_rich(&self, text: &str, style: RichTextStyle) -> usize {
        self.raster_cache.borrow_mut().measure_width(
            text,
            TextStyle::new(style.size, style.size * LINE_HEIGHT_RATIO, 0).italic(style.italic),
            &self.font_for_weight(style.bold),
            style.emoji,
            &mut self.font_system.borrow_mut(),
            &mut self.swash_cache.borrow_mut(),
            1.0,
        )
    }

    fn measure_width_with_emoji(&self, text: &str, size: f32, emoji: bool) -> usize {
        self.raster_cache.borrow_mut().measure_width(
            text,
            TextStyle::new(size, size * LINE_HEIGHT_RATIO, 0),
            &self.font,
            emoji,
            &mut self.font_system.borrow_mut(),
            &mut self.swash_cache.borrow_mut(),
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
        self.raster_cache.borrow().stats()
    }

    fn draw_layout(
        &self,
        canvas: &mut Canvas,
        text: &str,
        x: isize,
        y: usize,
        style: TextStyle,
        emoji: bool,
        scale_factor: f32,
    ) {
        self.draw_layout_with_font(canvas, text, x, y, style, emoji, scale_factor, &self.font);
    }

    fn draw_layout_with_font(
        &self,
        canvas: &mut Canvas,
        text: &str,
        x: isize,
        y: usize,
        style: TextStyle,
        emoji: bool,
        scale_factor: f32,
        font: &FontToken,
    ) {
        let normalized_scale_factor = normalized_scale_factor(scale_factor);
        self.record_measured_text_run(canvas, text, x, y, style, emoji, font);
        self.raster_cache.borrow_mut().draw(
            canvas,
            TextRasterDrawRequest {
                text,
                style,
                font,
                emoji,
                origin_x: scaled_signed_coordinate(x, normalized_scale_factor),
                origin_y: scaled_unsigned_coordinate(y, normalized_scale_factor),
                scale_factor: normalized_scale_factor,
            },
            &mut self.font_system.borrow_mut(),
            &mut self.swash_cache.borrow_mut(),
        );
    }

    fn record_measured_text_run(
        &self,
        canvas: &mut Canvas,
        text: &str,
        x: isize,
        y: usize,
        style: TextStyle,
        emoji: bool,
        font: &FontToken,
    ) {
        let Some(origin_x) = usize::try_from(x).ok() else {
            return;
        };
        let mut font_system = self.font_system.borrow_mut();
        let mut swash_cache = self.swash_cache.borrow_mut();
        let mut glyph_widths = Vec::new();
        let mut width = 0usize;
        for grapheme in text.graphemes(true) {
            let glyph_width = TextRasterCache::measure_width_uncached(
                grapheme,
                style,
                font,
                emoji,
                &mut font_system,
                &mut swash_cache,
                1.0,
            );
            glyph_widths.push(glyph_width.max(1));
            width = width.saturating_add(glyph_width.max(1));
        }
        canvas.record_text_run_with_glyph_widths(
            text,
            origin_x,
            y,
            width.max(1),
            style.line_height.ceil().max(1.0) as usize,
            &glyph_widths,
        );
    }

    fn font_for_weight(&self, bold: bool) -> FontToken {
        let mut font = self.font.clone();
        if bold {
            font.weight = font.weight.max(BOLD_WEIGHT);
        } else {
            font.weight = font.weight.max(REGULAR_WEIGHT);
        }
        font
    }

    pub(crate) fn rich_line_span(
        &self,
        text: impl Into<String>,
        style: RichTextStyle,
    ) -> RichTextLineSpan {
        RichTextLineSpan {
            text: text.into(),
            font: self.font_for_weight(style.bold),
            style: TextStyle::new(style.size, style.size * LINE_HEIGHT_RATIO, style.color)
                .italic(style.italic)
                .raster_vertical_scale(style.raster_vertical_scale),
            emoji: style.emoji,
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

fn scaled_signed_coordinate(value: isize, scale: f32) -> i32 {
    (value as f64 * f64::from(scale)).round() as i32
}

fn scaled_unsigned_coordinate(value: usize, scale: f32) -> i32 {
    (value as f64 * f64::from(scale)).round() as i32
}

fn normalized_scale_factor(scale_factor: f32) -> f32 {
    if scale_factor.is_finite() && scale_factor >= 1.0 {
        scale_factor
    } else {
        1.0
    }
}

impl RichTextLineSpan {
    fn to_raster_span(&self) -> RichTextRasterSpan<'_> {
        RichTextRasterSpan {
            text: &self.text,
            style: self.style,
            font: &self.font,
            emoji: self.emoji,
        }
    }
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

    const fn line_height(self) -> f32 {
        match self.vertical_align {
            TextVerticalAlign::Top => self.height as f32,
            TextVerticalAlign::Center => self.height as f32,
        }
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
