use super::render::show_entries;
use super::types::{
    EguiSourceAddressStripError, EguiSourceAddressStripOutput, SourceAddressLabelRasterEvidence,
    SourceAddressPaintOperation, SourceAddressPaintOperationKind, SourceAddressPaintPlan,
    SourceAddressRenderStyle,
};
use super::{interaction::Interaction, paint::Paint, raster::Raster};
use crate::text_surface::{
    EguiTextSurfaceAdapter, EguiTextSurfaceInputPolicy, SharedTextMetrics, TextSurfaceArtifactFrame,
};
use crate::texture_cache::RgbaTextureCache;
use katana_ui_core::atom::TextArea;
use katana_ui_core::molecule::structured::source_address_strip::{
    SourceAddressAction, SourceAddressStrip,
};
use katana_ui_core::render_model::UiTextSpan;
use katana_ui_core::text_surface::{
    TextSurface, TextSurfacePresentation, TextSurfaceProps, TextSurfaceViewport,
};
use katana_ui_core_text_raster::PlatformTextRasterizer;

pub struct EguiSourceAddressStripAdapter {
    pub(crate) field_id: egui::Id,
    pub(crate) text_surface_adapter: EguiTextSurfaceAdapter,
    pub(crate) text_rasterizer: PlatformTextRasterizer,
    pub(crate) metrics: SharedTextMetrics,
    pub(crate) textures: RgbaTextureCache,
    pub(crate) surface: Option<TextSurface>,
    pub(crate) last_input_artifact: Option<TextSurfaceArtifactFrame>,
    pub(crate) last_label_rasters: Vec<SourceAddressLabelRasterEvidence>,
    pub(crate) last_paint_plan: Option<SourceAddressPaintPlan>,
}

impl EguiSourceAddressStripAdapter {
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        strip: &mut SourceAddressStrip,
    ) -> Result<EguiSourceAddressStripOutput, EguiSourceAddressStripError> {
        self.show_with_style(ui, strip, &SourceAddressRenderStyle::default())
    }
    pub fn show_with_style(
        &mut self,
        ui: &mut egui::Ui,
        strip: &mut SourceAddressStrip,
        style: &SourceAddressRenderStyle,
    ) -> Result<EguiSourceAddressStripOutput, EguiSourceAddressStripError> {
        let mut output = EguiSourceAddressStripOutput {
            event_classes: Vec::new(),
            submissions: Vec::new(),
        };
        let root_bounds = ui.available_rect_before_wrap();
        self.last_label_rasters.clear();
        self.last_input_artifact = None;
        self.last_paint_plan = None;
        let enabled = strip.enabled();
        let visible = strip.presentation().visible().to_owned();
        let accessibility_label = strip.presentation().accessibility().to_owned();
        let field_id = self.field_id;
        let mut surface = self.surface.take().unwrap_or_else(|| {
            TextSurface::new(
                TextSurfaceProps::new(
                    TextArea::new(accessibility_label.clone())
                        .stable_state_id(format!("source-address:{field_id:?}"))
                        .value(strip.draft())
                        .placeholder(visible.clone())
                        .disabled(!enabled)
                        .min_rows(1)
                        .max_rows(1)
                        .ime_enabled(true),
                    Vec::<UiTextSpan>::new(),
                    TextSurfaceViewport::new(0, 0, style.input_width_px, style.input_height_px),
                )
                .accessibility_label(accessibility_label.clone()),
            )
        });
        let mut presentation = TextSurfacePresentation::from_props(surface.props());
        presentation.disabled = !enabled;
        let _ = surface.synchronize_presentation(presentation);
        surface.synchronize_value(strip.draft());
        let mut paint_plan = SourceAddressPaintPlan {
            surface_bounds: Paint::ui_rect(root_bounds),
            operations: Vec::new(),
        };
        let response = ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            let rendered = self.text_surface_adapter.show_with_input_policy(
                ui,
                &mut surface,
                &style.input_raster,
                &style.input_paint,
                &EguiTextSurfaceInputPolicy::default(),
            )?;
            paint_plan.operations.extend(
                rendered
                    .artifact
                    .paint_plan
                    .operations
                    .iter()
                    .cloned()
                    .map(|operation| SourceAddressPaintOperation {
                        clip_bounds: operation.clip_bounds,
                        kind: SourceAddressPaintOperationKind::Input(Raster::sanitize_input_kind(
                            operation.kind,
                        )),
                    }),
            );
            if !strip.history().is_empty() {
                let label = if strip.history_open() {
                    "履歴を閉じる"
                } else {
                    "履歴を開く"
                };
                let button = Raster::raster_button(
                    self,
                    &mut paint_plan,
                    ui,
                    label,
                    "履歴",
                    enabled,
                    style,
                )?;
                Interaction::publish_button_accessibility(
                    ui,
                    button.id,
                    button.rect,
                    label,
                    enabled,
                );
                if Interaction::activated(ui, &button) {
                    let action = if strip.history_open() {
                        SourceAddressAction::CloseHistory
                    } else {
                        SourceAddressAction::OpenHistory
                    };
                    if let Some(event) = strip.apply_action(action) {
                        output.record(event);
                    }
                }
            }
            if !strip.candidates().is_empty() {
                let label = if strip.candidates_open() {
                    "候補を閉じる"
                } else {
                    "候補を開く"
                };
                let button = Raster::raster_button(
                    self,
                    &mut paint_plan,
                    ui,
                    label,
                    "候補",
                    enabled,
                    style,
                )?;
                Interaction::publish_button_accessibility(
                    ui,
                    button.id,
                    button.rect,
                    label,
                    enabled,
                );
                if Interaction::activated(ui, &button) {
                    let action = if strip.candidates_open() {
                        SourceAddressAction::CloseCandidates
                    } else {
                        SourceAddressAction::OpenCandidates
                    };
                    if let Some(event) = strip.apply_action(action) {
                        output.record(event);
                    }
                }
            }
            let submit = Raster::raster_button(
                self,
                &mut paint_plan,
                ui,
                "開く",
                &accessibility_label,
                enabled,
                style,
            )?;
            Interaction::publish_button_accessibility(ui, submit.id, submit.rect, "開く", enabled);
            if Interaction::activated(ui, &submit)
                && let Some(event) = strip.apply_action(SourceAddressAction::Submit)
            {
                output.record(event);
            }
            Ok::<_, EguiSourceAddressStripError>(rendered)
        });
        let field = response.inner?;
        self.apply_text_surface_events(&mut output, strip, &field.events);
        if strip.history_open() {
            output.record_all(show_entries(self, &mut paint_plan, ui, strip, true, style)?);
        }
        if strip.candidates_open() {
            output.record_all(show_entries(
                self,
                &mut paint_plan,
                ui,
                strip,
                false,
                style,
            )?);
        }
        Paint::paint_button_plan(ui, self, Some(&paint_plan));
        self.last_paint_plan = Some(paint_plan);
        self.last_input_artifact = Some(field.artifact.clone());
        self.surface = Some(surface);
        Ok(output)
    }
}

#[cfg(test)]
#[path = "adapter_tests.rs"]
mod tests;
