use super::adapter::EguiSourceAddressStripAdapter;
use super::types::{SourceAddressPaintOperationKind, SourceAddressPaintPlan};
use katana_ui_core::render_model::UiRect;

const RGBA_CHANNEL_COUNT: usize = 4;

pub(crate) struct Paint;

impl Paint {
    pub(crate) fn paint_button_plan(
        ui: &egui::Ui,
        adapter: &mut EguiSourceAddressStripAdapter,
        plan: Option<&SourceAddressPaintPlan>,
    ) {
        let Some(plan) = plan else { return };
        for operation in &plan.operations {
            let painter = ui
                .painter()
                .with_clip_rect(Self::egui_rect(operation.clip_bounds));
            match &operation.kind {
                SourceAddressPaintOperationKind::Fill { bounds, color_rgba } => {
                    painter.rect_filled(Self::egui_rect(*bounds), 0.0, Self::color(*color_rgba));
                }
                SourceAddressPaintOperationKind::Texture { bounds, texture } => {
                    let handle = adapter.textures.texture_for_rgba(
                        ui.ctx(),
                        &texture.identity,
                        texture.width as usize,
                        texture.height as usize,
                        &texture.rgba_pixels,
                    );
                    painter.image(
                        handle.id(),
                        Self::egui_rect(*bounds),
                        egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
                        egui::Color32::WHITE,
                    );
                }
                SourceAddressPaintOperationKind::Input(_) => {}
            }
        }
    }

    fn egui_rect(rect: UiRect) -> egui::Rect {
        egui::Rect::from_min_size(
            egui::pos2(rect.x as f32, rect.y as f32),
            egui::vec2(rect.width as f32, rect.height as f32),
        )
    }
    pub(crate) fn ui_rect(rect: egui::Rect) -> UiRect {
        UiRect::new(
            rect.min.x.round() as i32,
            rect.min.y.round() as i32,
            rect.width().round().max(0.0) as u32,
            rect.height().round().max(0.0) as u32,
        )
    }
    pub(crate) fn color(rgba: [u8; RGBA_CHANNEL_COUNT]) -> egui::Color32 {
        let [red, green, blue, alpha] = rgba;
        egui::Color32::from_rgba_unmultiplied(red, green, blue, alpha)
    }
}
