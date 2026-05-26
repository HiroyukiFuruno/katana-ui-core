use crate::visual::canvas::Canvas;
use cosmic_text::{FontSystem, SwashCache};
use katana_ui_core::facade::UiCoreFacade;
use katana_ui_core::theme::{FontFamily, FontToken};
use std::cell::RefCell;

#[cfg(test)]
pub(crate) use super::text_raster::TextCacheStats;
use super::text_raster::{TextRasterCache, TextStyle};

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
            text,
            style,
            &self.font,
            &mut self.font_system.borrow_mut(),
            &mut self.swash_cache.borrow_mut(),
            (x as f64 * f64::from(normalized_scale_factor)).round() as usize,
            (y as f64 * f64::from(normalized_scale_factor)).round() as usize,
            normalized_scale_factor,
        );
    }
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
