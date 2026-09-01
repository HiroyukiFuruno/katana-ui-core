use super::adapter::EguiSourceAddressStripAdapter;
use super::types::{
    EguiSourceAddressStripError, EguiSourceAddressStripOutput, SourceAddressPaintPlan,
    SourceAddressRasterEvidenceReceipt,
};
use crate::egui::text_surface::{EguiTextSurfaceAdapter, SharedTextMetrics};
use crate::egui::texture_cache::{DEFAULT_TEXTURE_CACHE_CAPACITY, RgbaTextureCache};
use crate::molecule::structured::source_address_strip::{SourceAddressAction, SourceAddressStrip};
use crate::text_raster::{
    PlatformTextRasterConfig, PlatformTextRasterResources, PlatformTextRasterizer,
};
use crate::text_surface::TextSurfaceEvent;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

impl EguiSourceAddressStripAdapter {
    pub fn new(id_source: impl egui::AsId) -> Result<Self, EguiSourceAddressStripError> {
        let config = PlatformTextRasterConfig::default();
        let catalog = Arc::new(crate::text_raster::PlatformFontCatalog::new(
            config.catalog_policy().clone(),
        ));
        let metrics = Rc::new(RefCell::new(
            crate::text_raster::PlatformTextMetricsFrame::new(),
        ));
        Self::with_catalog_and_metrics(id_source, catalog, config, metrics)
    }

    pub fn with_catalog_and_metrics(
        id_source: impl egui::AsId,
        catalog: Arc<crate::text_raster::PlatformFontCatalog>,
        config: PlatformTextRasterConfig,
        metrics: SharedTextMetrics,
    ) -> Result<Self, EguiSourceAddressStripError> {
        Ok(Self {
            field_id: egui::Id::new(id_source),
            text_surface_adapter: EguiTextSurfaceAdapter::with_catalog_and_metrics(
                Arc::clone(&catalog),
                config.clone(),
                Rc::clone(&metrics),
            )?,
            text_rasterizer: PlatformTextRasterizer::with_catalog(catalog, config)?,
            metrics,
            textures: RgbaTextureCache::new(DEFAULT_TEXTURE_CACHE_CAPACITY),
            surface: None,
            last_input_artifact: None,
            last_label_rasters: Vec::new(),
            last_paint_plan: None,
        })
    }

    pub(crate) fn with_resources_and_metrics(
        id_source: impl egui::AsId,
        resources: &PlatformTextRasterResources,
        metrics: SharedTextMetrics,
    ) -> Self {
        Self {
            field_id: egui::Id::new(id_source),
            text_surface_adapter: EguiTextSurfaceAdapter::with_resources_and_metrics(
                resources,
                Rc::clone(&metrics),
            ),
            text_rasterizer: resources.rasterizer(),
            metrics,
            textures: RgbaTextureCache::new(DEFAULT_TEXTURE_CACHE_CAPACITY),
            surface: None,
            last_input_artifact: None,
            last_label_rasters: Vec::new(),
            last_paint_plan: None,
        }
    }

    #[cfg(test)]
    pub(crate) fn catalog(&self) -> Arc<crate::text_raster::PlatformFontCatalog> {
        self.text_rasterizer.catalog()
    }

    #[must_use]
    pub fn raster_evidence(&self) -> Option<SourceAddressRasterEvidenceReceipt> {
        self.last_input_artifact
            .as_ref()
            .map(|artifact| SourceAddressRasterEvidenceReceipt {
                input_paint_plan_hash: artifact.paint_plan_hash.clone(),
                input_has_text_texture: artifact
                    .record
                    .layers
                    .contains(&crate::egui::text_surface::EguiTextSurfaceDrawLayer::TextTexture),
                label_rasters: self.last_label_rasters.clone(),
            })
    }

    #[must_use]
    pub fn artifact_paint_plan(&self) -> Option<&SourceAddressPaintPlan> {
        self.last_paint_plan.as_ref()
    }

    pub(crate) fn required_paint_plan(
        &self,
    ) -> Result<SourceAddressPaintPlan, EguiSourceAddressStripError> {
        self.artifact_paint_plan()
            .cloned()
            .ok_or(EguiSourceAddressStripError::PaintPlanNotProduced)
    }

    pub(super) fn apply_text_surface_events(
        &self,
        output: &mut EguiSourceAddressStripOutput,
        strip: &mut SourceAddressStrip,
        events: &[TextSurfaceEvent],
    ) {
        for event in events {
            match event {
                TextSurfaceEvent::TextArea(crate::atom::TextAreaEvent::Change(value)) => {
                    if let Some(event) =
                        strip.apply_action(SourceAddressAction::SetDraft(value.clone()))
                    {
                        output.record(event);
                    }
                }
                TextSurfaceEvent::FocusChanged(focused) => {
                    if let Some(event) =
                        strip.apply_action(SourceAddressAction::SetFocused(*focused))
                    {
                        output.record(event);
                    }
                }
                TextSurfaceEvent::TextArea(crate::atom::TextAreaEvent::Submit(_)) => {
                    if let Some(event) = strip.apply_action(SourceAddressAction::Submit) {
                        output.record(event);
                    }
                }
                _ => {}
            }
        }
    }
}
