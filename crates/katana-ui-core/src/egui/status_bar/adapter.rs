use super::paint::StatusBarPaint;
use super::types::{
    EguiStatusBarError, EguiStatusBarOutput, StatusBarLabelRasterEvidence, StatusBarPaintOperation,
    StatusBarPaintOperationKind, StatusBarPaintPlan, StatusBarRenderStyle,
};
use crate::egui::texture_cache::{DEFAULT_TEXTURE_CACHE_CAPACITY, RgbaTextureCache};
use crate::molecule::{StatusBar, StatusBarDensity, StatusBarMode};
use crate::text_raster::{
    PlatformTextRasterConfig, PlatformTextRasterResources, PlatformTextRasterizer,
};
use std::collections::BTreeMap;

const COMPACT_HEIGHT_REDUCTION_PX: u32 = 4;

pub struct EguiStatusBarAdapter {
    pub(super) id: egui::Id,
    pub(super) text_rasterizer: PlatformTextRasterizer,
    pub(super) textures: RgbaTextureCache,
    pub(super) last_paint_plan: Option<StatusBarPaintPlan>,
    pub(super) last_label_rasters: Vec<StatusBarLabelRasterEvidence>,
    pub(super) last_tooltip_segment: Option<String>,
    pub(super) segment_bounds: BTreeMap<String, egui::Rect>,
}

impl EguiStatusBarAdapter {
    pub fn new(id_source: impl egui::AsId) -> Result<Self, EguiStatusBarError> {
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
            last_label_rasters: Vec::new(),
            last_tooltip_segment: None,
            segment_bounds: BTreeMap::new(),
        }
    }

    #[must_use]
    pub fn artifact_paint_plan(&self) -> Option<&StatusBarPaintPlan> {
        self.last_paint_plan.as_ref()
    }

    #[must_use]
    pub fn raster_evidence(&self) -> &[StatusBarLabelRasterEvidence] {
        &self.last_label_rasters
    }

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        status: &mut StatusBar,
    ) -> Result<EguiStatusBarOutput, EguiStatusBarError> {
        let style = StatusBarRenderStyle::standard();
        self.show_with_style(ui, status, &style)
    }

    pub fn show_with_style(
        &mut self,
        ui: &mut egui::Ui,
        status: &mut StatusBar,
        style: &StatusBarRenderStyle,
    ) -> Result<EguiStatusBarOutput, EguiStatusBarError> {
        self.last_label_rasters.clear();
        self.segment_bounds.clear();
        self.last_paint_plan = None;
        let width = ui.available_width().max(1.0);
        let height = match status.density_value() {
            StatusBarDensity::Compact => {
                style.height_px.saturating_sub(COMPACT_HEIGHT_REDUCTION_PX)
            }
            StatusBarDensity::Default => style.height_px,
        } as f32;
        let (rect, _) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::hover());
        let bounds = StatusBarPaint::ui_rect(rect);
        let mut paint_plan = StatusBarPaintPlan {
            surface_bounds: bounds,
            operations: vec![StatusBarPaintOperation {
                clip_bounds: bounds,
                kind: StatusBarPaintOperationKind::Fill {
                    bounds,
                    color_rgba: style.background_rgba,
                },
            }],
        };
        let mut out = EguiStatusBarOutput {
            events: Vec::new(),
            paint_plan: StatusBarPaintPlan {
                surface_bounds: bounds,
                operations: Vec::new(),
            },
        };
        let render_result: Result<(), EguiStatusBarError> = (|| {
            if status.mode_value() == StatusBarMode::SingleMessage {
                if let Some(message) = status.single_message().map(str::to_owned) {
                    let snapshot =
                        super::render::SegmentSnapshot::single_message(message, status.label());
                    self.paint_segment(
                        ui,
                        rect,
                        &snapshot,
                        style,
                        &mut paint_plan,
                        &mut out,
                        status,
                    )?;
                }
            } else {
                for alignment in super::render::STATUS_ALIGNMENTS {
                    self.paint_alignment(
                        ui,
                        rect,
                        status,
                        alignment,
                        style,
                        &mut paint_plan,
                        &mut out,
                    )?;
                }
            }
            if ui.input(|input| input.key_pressed(egui::Key::Escape))
                && let Some(id) = status.state().open_popover().cloned()
            {
                self.close_popover(ui, status, &id, &mut out);
            }
            self.paint_open_popover(ui, status, &mut paint_plan)
        })();
        if let Err(error) = render_result {
            self.last_label_rasters.clear();
            self.segment_bounds.clear();
            return Err(error);
        }
        self.paint_plan(ui, &paint_plan);
        self.last_paint_plan = Some(paint_plan.clone());
        out.paint_plan = paint_plan;
        Ok(out)
    }
}
