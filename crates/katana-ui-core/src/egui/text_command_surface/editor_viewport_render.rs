use super::editor_viewport_projection_lease::{
    EditorViewportProjectionLease, clamp_split_ratio_percent,
};
use crate::egui::text_surface::{
    EguiTextSurfaceDrawLayer, TextSurfacePaintOperation, TextSurfacePaintOperationKind,
    TextSurfacePaintPlan, TextSurfacePaintTexture,
};
use crate::render_model::{UiImageSurfaceFit, UiRect};
use sha2::{Digest, Sha256};

const SPLIT_HANDLE_WIDTH: f32 = 6.0;
const RGBA_CHANNEL_COUNT: usize = 4;
const ALPHA_CHANNEL_INDEX: usize = 3;

pub(super) struct EditorViewportLayout {
    pub(super) document: egui::Rect,
    pub(super) preview: egui::Rect,
}

#[derive(Debug)]
pub(crate) struct EditorPreviewRootOutput {
    pub(super) paint_plan: TextSurfacePaintPlan,
}

pub(super) fn layout(
    ui: &mut egui::Ui,
    body: egui::Rect,
    lease: &mut EditorViewportProjectionLease,
) -> EditorViewportLayout {
    let available_width = (body.width() - SPLIT_HANDLE_WIDTH).max(2.0);
    let mut document_width = available_width * f32::from(lease.split_ratio_percent) / 100.0;
    let handle = egui::Rect::from_min_size(
        egui::pos2(body.min.x + document_width, body.min.y),
        egui::vec2(SPLIT_HANDLE_WIDTH, body.height()),
    );
    let response = ui.interact(
        handle,
        ui.make_persistent_id("kuc.generic-editor-split"),
        egui::Sense::click_and_drag(),
    );
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Slider, true, "Editor split"));
    if response.drag_started() || response.clicked() {
        response.request_focus();
    }
    if response.dragged() {
        let percent =
            ((document_width + response.drag_delta().x) / available_width * 100.0).round() as i32;
        lease.split_ratio_percent = clamp_split_ratio_percent(percent);
        document_width = available_width * f32::from(lease.split_ratio_percent) / 100.0;
    }
    if response.has_focus() {
        let delta = ui.input(|input| {
            if input.key_pressed(egui::Key::ArrowLeft) {
                -1
            } else if input.key_pressed(egui::Key::ArrowRight) {
                1
            } else {
                0
            }
        });
        if delta != 0 {
            lease.split_ratio_percent =
                clamp_split_ratio_percent(i32::from(lease.split_ratio_percent) + delta);
            document_width = available_width * f32::from(lease.split_ratio_percent) / 100.0;
        }
    }
    let document =
        egui::Rect::from_min_size(body.min, egui::vec2(document_width.max(1.0), body.height()));
    let preview = egui::Rect::from_min_max(
        egui::pos2(document.max.x + SPLIT_HANDLE_WIDTH, body.min.y),
        body.max,
    );
    ui.painter()
        .rect_filled(handle, 0.0, ui.visuals().widgets.inactive.bg_fill);
    EditorViewportLayout { document, preview }
}

pub(super) fn render_preview(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    lease: &EditorViewportProjectionLease,
    texture_slot: &mut Option<(String, egui::TextureId)>,
    background_rgba: [u8; RGBA_CHANNEL_COUNT],
) -> EditorPreviewRootOutput {
    let preview = &lease.preview;
    let identity = preview_identity(preview.width, preview.height, &preview.rgba);
    if texture_slot
        .as_ref()
        .is_none_or(|(current, _)| current != &identity)
    {
        if let Some((_, expired)) = texture_slot.take() {
            ui.ctx().tex_manager().write().free(expired);
        }
        let image = egui::ColorImage::from_rgba_unmultiplied(
            [preview.width as usize, preview.height as usize],
            &preview.rgba,
        );
        let texture = ui.ctx().tex_manager().write().alloc(
            format!("kuc-editor-preview-{identity}"),
            egui::epaint::ImageData::Color(image.into()),
            egui::TextureOptions::LINEAR,
        );
        *texture_slot = Some((identity.clone(), texture));
    }
    let destination = fitted_rect(rect, preview.width, preview.height, preview.fit);
    ui.painter().rect_filled(
        rect,
        0.0,
        egui::Color32::from_rgba_unmultiplied(
            background_rgba[0],
            background_rgba[1],
            background_rgba[2],
            background_rgba[ALPHA_CHANNEL_INDEX],
        ),
    );
    if let Some((_, texture)) = texture_slot.as_ref() {
        ui.painter().image(
            *texture,
            destination,
            egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
            egui::Color32::WHITE,
        );
    }
    let response = ui.interact(
        rect,
        ui.make_persistent_id("kuc.generic-editor-preview"),
        egui::Sense::hover(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Image, true, &preview.accessibility_label)
    });
    let surface_bounds = ui_rect(rect);
    let texture_bounds = ui_rect(destination);
    EditorPreviewRootOutput {
        paint_plan: TextSurfacePaintPlan {
            surface_bounds,
            viewport_bounds: surface_bounds,
            operations: vec![
                TextSurfacePaintOperation {
                    layer: EguiTextSurfaceDrawLayer::Background,
                    clip_bounds: surface_bounds,
                    kind: TextSurfacePaintOperationKind::Fill {
                        bounds: surface_bounds,
                        color_rgba: background_rgba,
                    },
                },
                TextSurfacePaintOperation {
                    layer: EguiTextSurfaceDrawLayer::TextTexture,
                    clip_bounds: surface_bounds,
                    kind: TextSurfacePaintOperationKind::Texture {
                        bounds: texture_bounds,
                        texture: TextSurfacePaintTexture {
                            identity,
                            width: preview.width,
                            height: preview.height,
                            rgba_pixels: preview.rgba.clone(),
                        },
                    },
                },
            ],
        },
    }
}

fn preview_identity(width: u32, height: u32, rgba: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(width.to_le_bytes());
    hasher.update(height.to_le_bytes());
    hasher.update(rgba);
    hex::encode(hasher.finalize())
}

fn fitted_rect(bounds: egui::Rect, width: u32, height: u32, fit: UiImageSurfaceFit) -> egui::Rect {
    if matches!(fit, UiImageSurfaceFit::Stretch) {
        return bounds;
    }
    let source = egui::vec2(width as f32, height as f32);
    let scale_x = bounds.width() / source.x;
    let scale_y = bounds.height() / source.y;
    let scale = match fit {
        UiImageSurfaceFit::Cover => scale_x.max(scale_y),
        UiImageSurfaceFit::Original => 1.0_f32.min(scale_x.min(scale_y)),
        UiImageSurfaceFit::Contain | UiImageSurfaceFit::Stretch => scale_x.min(scale_y),
    };
    egui::Rect::from_center_size(bounds.center(), source * scale)
}

fn ui_rect(bounds: egui::Rect) -> UiRect {
    UiRect::new(
        bounds.min.x.round() as i32,
        bounds.min.y.round() as i32,
        bounds.width().round().max(0.0) as u32,
        bounds.height().round().max(0.0) as u32,
    )
}

#[cfg(test)]
#[path = "editor_viewport_render_inline_tests.rs"]
mod tests;
