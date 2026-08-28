#[path = "command_chrome_artifact.rs"]
mod command_chrome_artifact;
#[path = "command_chrome_artifact_types.rs"]
mod command_chrome_artifact_types;
#[path = "command_chrome_dropdown.rs"]
mod command_chrome_dropdown;
#[path = "command_chrome_floating.rs"]
mod command_chrome_floating;
#[path = "command_chrome_floating_paint.rs"]
mod command_chrome_floating_paint;
#[path = "command_chrome_interaction.rs"]
mod command_chrome_interaction;
#[path = "command_chrome_paint.rs"]
mod command_chrome_paint;
#[path = "command_chrome_presentation.rs"]
mod command_chrome_presentation;
#[path = "command_chrome_search.rs"]
mod command_chrome_search;
#[path = "command_chrome_search_controls.rs"]
mod command_chrome_search_controls;
#[path = "command_chrome_search_interaction.rs"]
mod command_chrome_search_interaction;
#[path = "command_chrome_search_paint.rs"]
mod command_chrome_search_paint;
#[path = "command_chrome_search_state.rs"]
mod command_chrome_search_state;
#[path = "command_chrome_toolbar.rs"]
mod command_chrome_toolbar;
#[path = "command_chrome_types.rs"]
mod command_chrome_types;

use crate::text_command_surface::accesskit_evidence::AccessKitTargetClass;
use command_chrome_presentation::toolbar_size;
use katana_ui_core::interaction::placement::Size;
use katana_ui_core::molecule::command_chrome::CommandChromeToolbar;
use katana_ui_core::render_model::UiRect;
use katana_ui_core_svg_raster::{UiSvgRasterConfig, UiSvgRasterizer};
use katana_ui_core_text_raster::{PlatformTextRasterConfig, PlatformTextRasterizer};
use std::sync::Arc;

pub use command_chrome_artifact_types::{
    CommandChromeArtifactFrame, CommandChromePaintOperation, CommandChromePaintOperationKind,
    CommandChromePaintPlan, CommandChromePaintTexture, EguiCommandChromeFloatingArtifactFrame,
    EguiCommandChromeSearchArtifactFrame,
};
pub use command_chrome_types::{
    CommandChromePaintStyle, CommandChromeRasterStyle, EguiCommandChromeActionFrame,
    EguiCommandChromeAdapter, EguiCommandChromeDrawLayer, EguiCommandChromeDropdownFrame,
    EguiCommandChromeDropdownItemFrame, EguiCommandChromeError,
    EguiCommandChromeFloatingFrameRecord, EguiCommandChromeFloatingOutput,
    EguiCommandChromeFrameRecord, EguiCommandChromeOutput, EguiCommandChromeSearchControlFrame,
    EguiCommandChromeSearchFrameRecord, EguiCommandChromeSearchOutput,
    EguiCommandChromeSearchStyle,
};

impl EguiCommandChromeAdapter {
    #[cfg(test)]
    pub(crate) fn catalog(&self) -> Arc<katana_ui_core_text_raster::PlatformFontCatalog> {
        self.text_rasterizer.catalog()
    }

    pub(crate) fn with_catalog(
        catalog: Arc<katana_ui_core_text_raster::PlatformFontCatalog>,
        text: PlatformTextRasterConfig,
        svg: UiSvgRasterConfig,
    ) -> Self {
        Self {
            text_surface_adapter: crate::text_surface::EguiTextSurfaceAdapter::with_catalog(
                Arc::clone(&catalog),
                text.clone(),
            ),
            text_rasterizer: PlatformTextRasterizer::with_catalog_cache_capacity(
                catalog,
                text.cache_capacity,
            ),
            svg_rasterizer: UiSvgRasterizer::new(svg),
            textures: crate::texture_cache::RgbaTextureCache::new(
                crate::texture_cache::DEFAULT_TEXTURE_CACHE_CAPACITY,
            ),
            search_surfaces: None,
        }
    }

    pub(crate) fn measure_toolbar(
        &mut self,
        ui: &egui::Ui,
        toolbar: &CommandChromeToolbar,
        raster_style: &CommandChromeRasterStyle,
    ) -> Result<Size, EguiCommandChromeError> {
        let rendered = toolbar
            .actions()
            .iter()
            .map(|action| {
                self.render_action(ui, action, toolbar.display_mode_model(), raster_style)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let size = toolbar_size(ui, &rendered);
        Ok(Size::new(size.x.ceil() as u32, size.y.ceil() as u32))
    }

    #[must_use]
    pub fn new(text: PlatformTextRasterConfig, svg: UiSvgRasterConfig) -> Self {
        Self {
            text_surface_adapter: crate::text_surface::EguiTextSurfaceAdapter::new(text.clone()),
            text_rasterizer: PlatformTextRasterizer::new(text),
            svg_rasterizer: UiSvgRasterizer::new(svg),
            textures: crate::texture_cache::RgbaTextureCache::new(
                crate::texture_cache::DEFAULT_TEXTURE_CACHE_CAPACITY,
            ),
            search_surfaces: None,
        }
    }

    pub fn show_toolbar(
        &mut self,
        ui: &mut egui::Ui,
        toolbar: &mut CommandChromeToolbar,
        raster_style: &CommandChromeRasterStyle,
        paint_style: &CommandChromePaintStyle,
    ) -> Result<EguiCommandChromeOutput, EguiCommandChromeError> {
        self.show_toolbar_unpainted(
            ui,
            toolbar,
            raster_style,
            paint_style,
            AccessKitTargetClass::Toolbar,
        )
    }

    pub(super) fn show_toolbar_unpainted(
        &mut self,
        ui: &mut egui::Ui,
        toolbar: &mut CommandChromeToolbar,
        raster_style: &CommandChromeRasterStyle,
        paint_style: &CommandChromePaintStyle,
        target_class: AccessKitTargetClass,
    ) -> Result<EguiCommandChromeOutput, EguiCommandChromeError> {
        command_chrome_toolbar::show_toolbar_unpainted(
            self,
            ui,
            toolbar,
            raster_style,
            paint_style,
            target_class,
        )
    }
}

impl Default for EguiCommandChromeAdapter {
    fn default() -> Self {
        Self::new(
            PlatformTextRasterConfig::default(),
            UiSvgRasterConfig::default(),
        )
    }
}

pub(super) fn ui_rect(rect: egui::Rect) -> UiRect {
    command_chrome_toolbar::ui_rect(rect)
}
