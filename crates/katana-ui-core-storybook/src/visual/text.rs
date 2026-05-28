use crate::visual::canvas::Canvas;
use cosmic_text::{FontSystem, SwashCache};
use katana_ui_core::facade::UiCoreFacade;
use katana_ui_core::theme::{FontFamily, FontToken};
use std::cell::RefCell;

#[cfg(test)]
pub(crate) use super::text_raster::TextCacheStats;
use super::text_raster::{TextRasterCache, TextStyle};
use super::text_raster_request::TextRasterDrawRequest;

const LINE_HEIGHT_RATIO: f32 = 1.45;
const REGULAR_WEIGHT: u16 = 400;
const FALLBACK_FONT_SIZE: f32 = 14.0;
const FALLBACK_FONT_NAME: &str = "body";
const CODE_FONT_ROLE: &str = "code";
const SHORTCUT_FONT_ROLE: &str = "shortcut";

pub(crate) struct TextRenderer {
    font_system: RefCell<FontSystem>,
    swash_cache: RefCell<SwashCache>,
    raster_cache: RefCell<TextRasterCache>,
    font: FontToken,
}

impl TextRenderer {
    pub(crate) fn load(facade: &UiCoreFacade, role: &str) -> Self {
        let font = resolve_font(facade, role);
        Self {
            font_system: RefCell::new(FontSystem::new()),
            swash_cache: RefCell::new(SwashCache::new()),
            raster_cache: RefCell::new(TextRasterCache::default()),
            font,
        }
    }

    pub(crate) fn draw(
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
            x,
            y,
            TextStyle::new(size, size * LINE_HEIGHT_RATIO, color),
            canvas.scale_factor(),
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
            x,
            vertical_box.y,
            TextStyle::new(size, vertical_box.height, color),
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
            origin.x,
            origin.y,
            TextStyle::new(size, text_box.line_height(), color),
            canvas.scale_factor(),
        );
    }

    pub(crate) fn measure_width(&self, text: &str, size: f32) -> usize {
        self.raster_cache.borrow_mut().measure_width(
            text,
            TextStyle::new(size, size * LINE_HEIGHT_RATIO, 0),
            &self.font,
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
        x: usize,
        y: usize,
        style: TextStyle,
        scale_factor: f32,
    ) {
        let normalized_scale_factor = if scale_factor.is_finite() && scale_factor >= 1.0 {
            scale_factor
        } else {
            1.0
        };
        self.raster_cache.borrow_mut().draw(
            canvas,
            TextRasterDrawRequest {
                text,
                style,
                font: &self.font,
                origin_x: (x as f64 * f64::from(normalized_scale_factor)).round() as usize,
                origin_y: (y as f64 * f64::from(normalized_scale_factor)).round() as usize,
                scale_factor: normalized_scale_factor,
            },
            &mut self.font_system.borrow_mut(),
            &mut self.swash_cache.borrow_mut(),
        );
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TextOrigin {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TextBox {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    horizontal_align: TextHorizontalAlign,
    vertical_align: TextVerticalAlign,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TextHorizontalAlign {
    Start,
    Center,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TextVerticalAlign {
    Top,
    Center,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct TextVerticalBox {
    y: usize,
    height: f32,
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
