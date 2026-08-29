use super::adapter::EguiStatusBarAdapter;
use super::paint::StatusBarPaint;
use super::types::StatusBarPaintOperationKind;

impl EguiStatusBarAdapter {
    pub(super) fn paint_plan(&mut self, ui: &egui::Ui) {
        let Some(plan) = self.last_paint_plan.as_ref() else {
            return;
        };
        for operation in &plan.operations {
            let painter = ui
                .painter()
                .with_clip_rect(StatusBarPaint::egui_rect(operation.clip_bounds));
            match &operation.kind {
                StatusBarPaintOperationKind::Fill { bounds, color_rgba } => {
                    painter.rect_filled(
                        StatusBarPaint::egui_rect(*bounds),
                        0.0,
                        StatusBarPaint::color(*color_rgba),
                    );
                }
                StatusBarPaintOperationKind::Texture { bounds, texture } => {
                    let handle = self.textures.texture_for_rgba(
                        ui.ctx(),
                        &texture.identity,
                        texture.width as usize,
                        texture.height as usize,
                        &texture.rgba_pixels,
                    );
                    painter.image(
                        handle.id(),
                        StatusBarPaint::egui_rect(*bounds),
                        egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
                        egui::Color32::WHITE,
                    );
                }
            }
        }
    }
}
