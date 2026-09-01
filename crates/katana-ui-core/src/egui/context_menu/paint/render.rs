use super::super::types::{ContextMenuPaintOperationKind, ContextMenuPaintPlan};
use crate::egui::texture_cache::RgbaTextureCache;
use crate::render_model::{RGBA_CHANNEL_COUNT, UiRect};

pub(super) fn paint_plan(ui: &egui::Ui, cache: &mut RgbaTextureCache, plan: &ContextMenuPaintPlan) {
    for operation in &plan.operations {
        let painter = ui
            .painter()
            .with_clip_rect(egui_rect(operation.clip_bounds));
        match &operation.kind {
            ContextMenuPaintOperationKind::Fill { bounds, color_rgba } => {
                painter.rect_filled(egui_rect(*bounds), 0.0, color(*color_rgba));
            }
            ContextMenuPaintOperationKind::Texture { bounds, texture } => {
                let handle = cache.texture_for_rgba(
                    ui.ctx(),
                    &texture.identity,
                    texture.width as usize,
                    texture.height as usize,
                    &texture.rgba_pixels,
                );
                painter.image(
                    handle.id(),
                    egui_rect(*bounds),
                    egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
                    egui::Color32::WHITE,
                );
            }
        }
    }
}

fn color(rgba: [u8; RGBA_CHANNEL_COUNT]) -> egui::Color32 {
    let [red, green, blue, alpha] = rgba;
    egui::Color32::from_rgba_unmultiplied(red, green, blue, alpha)
}

fn egui_rect(value: UiRect) -> egui::Rect {
    egui::Rect::from_min_size(
        egui::pos2(value.x as f32, value.y as f32),
        egui::vec2(value.width as f32, value.height as f32),
    )
}
