use super::super::artifact_model::{EguiTextSurfaceError, EguiTextSurfaceOutput};
use super::super::model::{
    EguiTextSurfaceAdapter, EguiTextSurfaceInputPolicy, TextSurfacePaintStyle,
    TextSurfaceRasterStyle,
};
use crate::texture_cache::{DEFAULT_TEXTURE_CACHE_CAPACITY, RgbaTextureCache};
use katana_ui_core::text_surface::TextSurface;
use katana_ui_core_svg_raster::{UiSvgRasterConfig, UiSvgRasterizer};
use katana_ui_core_text_raster::{
    PlatformTextRasterConfig, PlatformTextRasterError, PlatformTextRasterizer,
};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

impl EguiTextSurfaceAdapter {
    #[cfg(test)]
    pub(crate) fn catalog(&self) -> Arc<katana_ui_core_text_raster::PlatformFontCatalog> {
        self.rasterizer.catalog()
    }

    #[must_use]
    pub fn new(config: PlatformTextRasterConfig) -> Self {
        Self {
            rasterizer: PlatformTextRasterizer::new(config),
            svg_rasterizer: UiSvgRasterizer::new(UiSvgRasterConfig::default()),
            textures: RgbaTextureCache::new(DEFAULT_TEXTURE_CACHE_CAPACITY),
            pending_focus_request: None,
            metrics: Rc::new(RefCell::new(
                katana_ui_core_text_raster::PlatformTextMetricsFrame::new(),
            )),
        }
    }

    pub fn with_catalog_and_metrics(
        catalog: Arc<katana_ui_core_text_raster::PlatformFontCatalog>,
        config: PlatformTextRasterConfig,
        metrics: super::super::model::SharedTextMetrics,
    ) -> Result<Self, PlatformTextRasterError> {
        Ok(Self {
            rasterizer: PlatformTextRasterizer::with_catalog(catalog, config)?,
            svg_rasterizer: UiSvgRasterizer::new(UiSvgRasterConfig::default()),
            textures: RgbaTextureCache::new(DEFAULT_TEXTURE_CACHE_CAPACITY),
            pending_focus_request: None,
            metrics,
        })
    }

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        surface: &mut TextSurface,
        raster_style: &TextSurfaceRasterStyle,
        paint_style: &TextSurfacePaintStyle,
    ) -> Result<EguiTextSurfaceOutput, EguiTextSurfaceError> {
        self.show_with_input_policy(
            ui,
            surface,
            raster_style,
            paint_style,
            &EguiTextSurfaceInputPolicy::default(),
        )
    }
}

impl Default for EguiTextSurfaceAdapter {
    fn default() -> Self {
        Self::new(PlatformTextRasterConfig::default())
    }
}
