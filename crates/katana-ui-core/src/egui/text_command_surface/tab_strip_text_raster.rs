//! KUC-owned tab-strip label rasterization.
//!
//! The retained tab-strip renderer uses this path instead of egui text widgets
//! so Japanese and color-emoji glyph selection stays under PlatformFontCatalog.

use super::tab_strip_projection_lease::TabStripText;
use crate::render_model::{UiTextSpan, UiTextSpanStyle};
use crate::text_raster::{
    PlatformTextRasterConfig, PlatformTextRasterError, PlatformTextRasterRequest,
    PlatformTextRasterizer,
};
use crate::theme::{FontFamily, FontToken};
use std::sync::Arc;

const RGBA_CHANNEL_COUNT: usize = 4;
const TAB_STRIP_FONT_SIZE_PX: f32 = 14.0;
const TAB_STRIP_FONT_WEIGHT: u16 = 400;
const TAB_STRIP_TEXT_RED: u8 = 220;
const TAB_STRIP_TEXT_GREEN: u8 = 220;
const TAB_STRIP_TEXT_BLUE: u8 = 220;
const OPAQUE_ALPHA: u8 = 255;

/// Generic rasterizer retained by KUC tab-strip presentation.
pub struct TabStripTextRasterizer {
    rasterizer: PlatformTextRasterizer,
    font: FontToken,
    color_rgba: [u8; RGBA_CHANNEL_COUNT],
}

/// Opaque text texture for a KUC-owned tab-strip paint operation.
pub struct TabStripTextRaster {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) rgba_pixels: Vec<u8>,
}

impl TabStripTextRasterizer {
    pub fn new() -> Result<Self, PlatformTextRasterError> {
        let config = PlatformTextRasterConfig::default();
        let catalog = Arc::new(crate::text_raster::PlatformFontCatalog::new(
            config.catalog_policy().clone(),
        ));
        Self::with_catalog(catalog, config)
    }

    pub fn with_catalog(
        catalog: Arc<crate::text_raster::PlatformFontCatalog>,
        config: PlatformTextRasterConfig,
    ) -> Result<Self, PlatformTextRasterError> {
        Ok(Self {
            rasterizer: PlatformTextRasterizer::with_catalog(catalog, config)?,
            font: FontToken {
                name: "system-ui".to_string(),
                family: FontFamily::Proportional,
                size: TAB_STRIP_FONT_SIZE_PX,
                weight: TAB_STRIP_FONT_WEIGHT,
            },
            color_rgba: [
                TAB_STRIP_TEXT_RED,
                TAB_STRIP_TEXT_GREEN,
                TAB_STRIP_TEXT_BLUE,
                OPAQUE_ALPHA,
            ],
        })
    }

    pub fn rasterize(
        &mut self,
        text: &TabStripText,
        scale_factor: f32,
    ) -> Result<TabStripTextRaster, PlatformTextRasterError> {
        let raster = self.rasterizer.rasterize(&PlatformTextRasterRequest {
            spans: UiTextSpan::emoji_marked_spans(
                &text.value,
                UiTextSpanStyle {
                    color_rgba: self.color_rgba,
                    ..UiTextSpanStyle::default()
                },
            ),
            font: self.font.clone(),
            fallback_color_rgba: self.color_rgba,
            line_height_px: self.font.size,
            max_width_px: None,
            scale_factor,
        })?;
        Ok(TabStripTextRaster {
            width: u32::try_from(raster.width).unwrap_or(u32::MAX),
            height: u32::try_from(raster.height).unwrap_or(u32::MAX),
            rgba_pixels: raster.rgba_pixels.iter().flatten().copied().collect(),
        })
    }
}

impl std::fmt::Debug for TabStripTextRaster {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = (self.width, self.height, self.rgba_pixels.len());
        formatter.write_str("TabStripTextRaster(..)")
    }
}

#[cfg(test)]
#[path = "tab_strip_text_raster_tests.rs"]
mod tests;
