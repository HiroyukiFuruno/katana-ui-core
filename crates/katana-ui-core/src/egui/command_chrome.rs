#[path = "command_chrome_artifact.rs"]
mod command_chrome_artifact;
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
#[path = "command_chrome_types.rs"]
mod command_chrome_types;
#[path = "command_chrome/toolbar.rs"]
mod toolbar;

use crate::interaction::placement::Size;
use crate::molecule::command_chrome::CommandChromeToolbar;
use crate::render_model::UiRect;
use crate::svg_raster::{UiSvgRasterConfig, UiSvgRasterizer};
use crate::text_raster::{
    PlatformTextRasterConfig, PlatformTextRasterResources, PlatformTextRasterizer,
};
use command_chrome_paint::paint_command_chrome;
use command_chrome_presentation::toolbar_size;
use std::cell::RefCell;
use std::rc::Rc;
#[cfg(test)]
use std::sync::Arc;

pub use command_chrome_artifact::{
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
    pub(crate) fn catalog(&self) -> Arc<crate::text_raster::PlatformFontCatalog> {
        self.text_rasterizer.catalog()
    }

    #[cfg(test)]
    pub(crate) fn with_catalog_and_metrics(
        catalog: Arc<crate::text_raster::PlatformFontCatalog>,
        text: PlatformTextRasterConfig,
        svg: UiSvgRasterConfig,
        metrics: crate::egui::text_surface::SharedTextMetrics,
    ) -> Result<Self, EguiCommandChromeError> {
        Ok(Self {
            text_surface_adapter:
                crate::egui::text_surface::EguiTextSurfaceAdapter::with_catalog_and_metrics(
                    Arc::clone(&catalog),
                    text.clone(),
                    Rc::clone(&metrics),
                )?,
            text_rasterizer: PlatformTextRasterizer::with_catalog(catalog, text)?,
            svg_rasterizer: UiSvgRasterizer::new(svg),
            textures: crate::egui::texture_cache::RgbaTextureCache::new(
                crate::egui::texture_cache::DEFAULT_TEXTURE_CACHE_CAPACITY,
            ),
            search_surfaces: None,
            metrics,
            dropdown_primary_press: None,
            floating_pointer_exclusions: Vec::new(),
        })
    }

    pub(crate) fn with_resources_and_metrics(
        resources: &PlatformTextRasterResources,
        svg: UiSvgRasterConfig,
        metrics: crate::egui::text_surface::SharedTextMetrics,
    ) -> Self {
        Self {
            text_surface_adapter:
                crate::egui::text_surface::EguiTextSurfaceAdapter::with_resources_and_metrics(
                    resources,
                    Rc::clone(&metrics),
                ),
            text_rasterizer: resources.rasterizer(),
            svg_rasterizer: UiSvgRasterizer::new(svg),
            textures: crate::egui::texture_cache::RgbaTextureCache::new(
                crate::egui::texture_cache::DEFAULT_TEXTURE_CACHE_CAPACITY,
            ),
            search_surfaces: None,
            metrics,
            dropdown_primary_press: None,
            floating_pointer_exclusions: Vec::new(),
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
            text_surface_adapter: crate::egui::text_surface::EguiTextSurfaceAdapter::new(
                text.clone(),
            ),
            text_rasterizer: PlatformTextRasterizer::new(text),
            svg_rasterizer: UiSvgRasterizer::new(svg),
            textures: crate::egui::texture_cache::RgbaTextureCache::new(
                crate::egui::texture_cache::DEFAULT_TEXTURE_CACHE_CAPACITY,
            ),
            search_surfaces: None,
            metrics: Rc::new(RefCell::new(
                crate::text_raster::PlatformTextMetricsFrame::new(),
            )),
            dropdown_primary_press: None,
            floating_pointer_exclusions: Vec::new(),
        }
    }

    pub(crate) fn floating_pointer_exclusions(&self) -> &[UiRect] {
        &self.floating_pointer_exclusions
    }

    pub fn show_toolbar(
        &mut self,
        ui: &mut egui::Ui,
        toolbar: &mut CommandChromeToolbar,
        raster_style: &CommandChromeRasterStyle,
        paint_style: &CommandChromePaintStyle,
    ) -> Result<EguiCommandChromeOutput, EguiCommandChromeError> {
        let output = self.show_toolbar_unpainted(
            ui,
            toolbar,
            raster_style,
            paint_style,
            crate::egui::text_command_surface::accesskit_evidence::AccessKitTargetClass::Toolbar,
        )?;
        paint_command_chrome(ui, &mut self.textures, &output.artifact.paint_plan);
        Ok(output)
    }
}

fn dropdown_focus_return_target(
    events: &[crate::molecule::command_chrome::CommandChromeToolbarEvent],
) -> Option<&str> {
    events.iter().rev().find_map(|event| {
        let crate::molecule::command_chrome::CommandChromeToolbarEvent::DropdownClosed {
            action_id,
            reason,
        } = event
        else {
            return None;
        };
        matches!(
            reason,
            crate::molecule::command_chrome::CommandChromeDropdownCloseReason::Escape
                | crate::molecule::command_chrome::CommandChromeDropdownCloseReason::OutsideClick
                | crate::molecule::command_chrome::CommandChromeDropdownCloseReason::ItemActivated
        )
        .then_some(action_id.as_str())
    })
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
    UiRect::new(
        rect.min.x.round() as i32,
        rect.min.y.round() as i32,
        rect.width().round().max(0.0) as u32,
        rect.height().round().max(0.0) as u32,
    )
}

#[cfg(test)]
#[path = "command_chrome_tests.rs"]
mod tests;
