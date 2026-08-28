use super::artifact_model::EguiTextSurfaceError;
use super::model::{
    EguiTextSurfaceAdapter, EguiTextSurfaceInputPolicy, TextSurfacePaintStyle,
    TextSurfaceRasterStyle,
};
use super::paint::paint_surface;
use crate::texture_cache::{DEFAULT_TEXTURE_CACHE_CAPACITY, RgbaTextureCache};
use katana_ui_core::text_surface::TextSurface;
use katana_ui_core_svg_raster::UiSvgRasterConfig;
use katana_ui_core_text_raster::{PlatformTextRasterConfig, PlatformTextRasterizer};
use std::sync::Arc;

use super::artifact_model::EguiTextSurfaceOutput;

impl EguiTextSurfaceAdapter {
    #[cfg(test)]
    pub(crate) fn catalog(&self) -> Arc<katana_ui_core_text_raster::PlatformFontCatalog> {
        self.rasterizer.catalog()
    }

    pub(crate) const fn request_focus(&mut self, focused: bool) {
        self.pending_focus_request = Some(focused);
    }

    #[must_use]
    pub fn new(config: PlatformTextRasterConfig) -> Self {
        Self {
            rasterizer: PlatformTextRasterizer::new(config),
            svg_rasterizer: katana_ui_core_svg_raster::UiSvgRasterizer::new(
                UiSvgRasterConfig::default(),
            ),
            textures: RgbaTextureCache::new(DEFAULT_TEXTURE_CACHE_CAPACITY),
            pending_focus_request: None,
        }
    }

    pub(crate) fn with_catalog(
        catalog: Arc<katana_ui_core_text_raster::PlatformFontCatalog>,
        config: PlatformTextRasterConfig,
    ) -> Self {
        Self {
            rasterizer: PlatformTextRasterizer::with_catalog_cache_capacity(
                catalog,
                config.cache_capacity,
            ),
            svg_rasterizer: katana_ui_core_svg_raster::UiSvgRasterizer::new(
                UiSvgRasterConfig::default(),
            ),
            textures: RgbaTextureCache::new(DEFAULT_TEXTURE_CACHE_CAPACITY),
            pending_focus_request: None,
        }
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

    pub fn show_with_input_policy(
        &mut self,
        ui: &mut egui::Ui,
        surface: &mut TextSurface,
        raster_style: &TextSurfaceRasterStyle,
        paint_style: &TextSurfacePaintStyle,
        input_policy: &EguiTextSurfaceInputPolicy,
    ) -> Result<EguiTextSurfaceOutput, EguiTextSurfaceError> {
        self.show_with_input_policy_unpainted(ui, surface, raster_style, paint_style, input_policy)
            .inspect(|output| {
                paint_surface(ui, &mut self.textures, &output.artifact.paint_plan);
            })
    }

    pub(crate) fn show_with_input_policy_unpainted(
        &mut self,
        ui: &mut egui::Ui,
        surface: &mut TextSurface,
        raster_style: &TextSurfaceRasterStyle,
        paint_style: &TextSurfacePaintStyle,
        input_policy: &EguiTextSurfaceInputPolicy,
    ) -> Result<EguiTextSurfaceOutput, EguiTextSurfaceError> {
        super::render::show_with_input_policy_unpainted(
            ui,
            self,
            surface,
            raster_style,
            paint_style,
            input_policy,
        )
    }
}

impl Default for EguiTextSurfaceAdapter {
    fn default() -> Self {
        Self::new(PlatformTextRasterConfig::default())
    }
}
