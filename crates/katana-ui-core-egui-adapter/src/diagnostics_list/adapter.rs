//! Retained state and public adapter boundary for diagnostics.

use super::types::{
    DiagnosticsListPaintPlan, DiagnosticsListRasterEvidence, EguiDiagnosticsListError,
};
use crate::texture_cache::{DEFAULT_TEXTURE_CACHE_CAPACITY, RgbaTextureCache};
use katana_ui_core_text_raster::{
    PlatformTextRasterConfig, PlatformTextRasterResources, PlatformTextRasterizer,
};

use super::types::DiagnosticsListStyle;

mod input;
mod items;
mod render;

#[cfg(test)]
#[path = "adapter/interaction_boundary_tests.rs"]
mod interaction_boundary_tests;

#[cfg(test)]
#[path = "adapter/action_boundary_tests.rs"]
mod action_boundary_tests;

#[cfg(test)]
#[path = "adapter/texture_identity_tests.rs"]
mod texture_identity_tests;

pub(super) struct DiagnosticsRenderLayout<'a> {
    pub(super) style: &'a DiagnosticsListStyle,
    pub(super) surface: egui::Rect,
    pub(super) header: egui::Rect,
    pub(super) viewport: egui::Rect,
    pub(super) row_heights: &'a [f32],
    pub(super) scale: f32,
}

pub struct EguiDiagnosticsListAdapter {
    pub(super) id: egui::Id,
    pub(super) text_rasterizer: PlatformTextRasterizer,
    pub(super) textures: RgbaTextureCache,
    pub(super) last_paint_plan: Option<DiagnosticsListPaintPlan>,
    pub(super) raster_evidence: Vec<DiagnosticsListRasterEvidence>,
    pub(super) scroll_y: f32,
    pub(super) focused_item: Option<String>,
    pub(super) focused_scope: Option<String>,
}

impl EguiDiagnosticsListAdapter {
    pub fn new(id_source: impl egui::AsId) -> Result<Self, EguiDiagnosticsListError> {
        let resources = PlatformTextRasterResources::new(PlatformTextRasterConfig::default());
        Ok(Self::with_resources(id_source, &resources))
    }

    pub(crate) fn with_resources(
        id_source: impl egui::AsId,
        resources: &PlatformTextRasterResources,
    ) -> Self {
        Self {
            id: egui::Id::new(id_source),
            text_rasterizer: resources.rasterizer(),
            textures: RgbaTextureCache::new(DEFAULT_TEXTURE_CACHE_CAPACITY),
            last_paint_plan: None,
            raster_evidence: Vec::new(),
            scroll_y: 0.0,
            focused_item: None,
            focused_scope: None,
        }
    }

    #[must_use]
    pub fn artifact_paint_plan(&self) -> Option<&DiagnosticsListPaintPlan> {
        self.last_paint_plan.as_ref()
    }

    #[must_use]
    pub fn raster_evidence(&self) -> &[DiagnosticsListRasterEvidence] {
        &self.raster_evidence
    }

    #[must_use]
    pub fn scroll_y(&self) -> f32 {
        self.scroll_y
    }
}
