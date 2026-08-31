use super::adapter::EguiStatusBarAdapter;
use super::paint::StatusBarPaint;
use super::types::{StatusBarPaintOperationKind, StatusBarPaintPlan};

impl EguiStatusBarAdapter {
    pub(super) fn paint_plan(&mut self, ui: &egui::Ui, plan: &StatusBarPaintPlan) {
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

#[cfg(test)]
mod tests {
    use katana_ui_core::render_model::UiRect;

    use super::*;

    #[test]
    fn paint_plan_is_noop_when_empty() {
        let mut adapter =
            super::super::adapter::EguiStatusBarAdapter::new("status-paint-plan-noop")
                .expect("status bar adapter should construct");
        let plan = super::super::types::StatusBarPaintPlan {
            surface_bounds: UiRect::new(0, 0, 10, 10),
            operations: Vec::new(),
        };
        let context = egui::Context::default();
        let mut output = context.run_ui(egui::RawInput::default(), |ui| {
            adapter.paint_plan(ui, &plan);
        });
        output.textures_delta.clear();
    }

    #[test]
    fn paint_plan_renders_fill_and_texture_operations() {
        let mut adapter =
            super::super::adapter::EguiStatusBarAdapter::new("status-paint-plan-path")
                .expect("status bar adapter should construct");
        let plan = super::super::types::StatusBarPaintPlan {
            surface_bounds: UiRect::new(0, 0, 10, 10),
            operations: vec![
                super::super::types::StatusBarPaintOperation {
                    clip_bounds: UiRect::new(0, 0, 10, 10),
                    kind: StatusBarPaintOperationKind::Fill {
                        bounds: UiRect::new(0, 0, 10, 10),
                        color_rgba: [10, 20, 30, 255],
                    },
                },
                super::super::types::StatusBarPaintOperation {
                    clip_bounds: UiRect::new(0, 0, 10, 10),
                    kind: StatusBarPaintOperationKind::Texture {
                        bounds: UiRect::new(1, 1, 2, 2),
                        texture: super::super::types::StatusBarPaintTexture {
                            identity: "status-bar-cover-test".to_owned(),
                            width: 2,
                            height: 2,
                            rgba_pixels: vec![
                                0, 0, 0, 255, 255, 255, 255, 255, 0, 0, 0, 255, 0, 0, 0, 255,
                            ],
                        },
                    },
                },
            ],
        };
        let context = egui::Context::default();
        let mut output = context.run_ui(egui::RawInput::default(), |ui| {
            adapter.paint_plan(ui, &plan);
        });
        output.textures_delta.clear();
    }

    #[test]
    fn retained_label_texture_is_replaced_when_pixels_change() {
        let mut adapter =
            super::super::adapter::EguiStatusBarAdapter::new("status-paint-plan-retained")
                .expect("status bar adapter should construct");
        let plan = |identity: &str, pixel: [u8; 4]| super::super::types::StatusBarPaintPlan {
            surface_bounds: UiRect::new(0, 0, 10, 10),
            operations: vec![super::super::types::StatusBarPaintOperation {
                clip_bounds: UiRect::new(0, 0, 10, 10),
                kind: StatusBarPaintOperationKind::Texture {
                    bounds: UiRect::new(1, 1, 2, 2),
                    texture: super::super::types::StatusBarPaintTexture {
                        identity: identity.to_owned(),
                        width: 1,
                        height: 1,
                        rgba_pixels: pixel.to_vec(),
                    },
                },
            }],
        };
        let context = egui::Context::default();
        let first = context.run_ui(egui::RawInput::default(), |ui| {
            adapter.paint_plan(
                ui,
                &plan("status-bar-label:retained:old", [220, 220, 220, 255]),
            );
        });
        let first_id = *first
            .textures_delta
            .set
            .keys()
            .next()
            .expect("first texture is uploaded");
        let mut first = first;
        first.textures_delta.clear();
        let second = context.run_ui(egui::RawInput::default(), |ui| {
            adapter.paint_plan(
                ui,
                &plan("status-bar-label:retained:new", [240, 190, 75, 255]),
            );
        });

        assert_eq!(second.textures_delta.set.len(), 1);
        let second_id = *second
            .textures_delta
            .set
            .keys()
            .next()
            .expect("replacement texture is uploaded");
        assert_ne!(first_id, second_id);
        let mut second = second;
        second.textures_delta.clear();
    }
}
