use katana_ui_core::render_model::{RGBA_CHANNEL_COUNT, UiRect};

pub(super) fn color(rgba: [u8; RGBA_CHANNEL_COUNT]) -> egui::Color32 {
    let [red, green, blue, alpha] = rgba;
    egui::Color32::from_rgba_unmultiplied(red, green, blue, alpha)
}

pub(super) fn egui_rect(value: UiRect) -> egui::Rect {
    egui::Rect::from_min_size(
        egui::pos2(value.x as f32, value.y as f32),
        egui::vec2(value.width as f32, value.height as f32),
    )
}
