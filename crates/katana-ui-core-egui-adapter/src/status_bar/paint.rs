use katana_ui_core::render_model::{RGBA_CHANNEL_COUNT, UiRect, UiTone};

const COLOR_CHANNEL_MAX: u8 = 255;
const ACCENT_COLOR: [u8; RGBA_CHANNEL_COUNT] = [100, 175, COLOR_CHANNEL_MAX, COLOR_CHANNEL_MAX];
const SUCCESS_COLOR: [u8; RGBA_CHANNEL_COUNT] = [100, 210, 145, COLOR_CHANNEL_MAX];
const WARNING_COLOR: [u8; RGBA_CHANNEL_COUNT] = [240, 190, 75, COLOR_CHANNEL_MAX];
const DANGER_COLOR: [u8; RGBA_CHANNEL_COUNT] = [240, 105, 105, COLOR_CHANNEL_MAX];
const RED_CHANNEL_INDEX: usize = 0;
const GREEN_CHANNEL_INDEX: usize = 1;
const BLUE_CHANNEL_INDEX: usize = 2;
const ALPHA_CHANNEL_INDEX: usize = 3;

pub(crate) struct StatusBarPaint;

impl StatusBarPaint {
    pub(crate) fn tone_color(
        tone: UiTone,
        neutral: [u8; RGBA_CHANNEL_COUNT],
    ) -> [u8; RGBA_CHANNEL_COUNT] {
        match tone {
            UiTone::Neutral => neutral,
            UiTone::Accent => ACCENT_COLOR,
            UiTone::Success => SUCCESS_COLOR,
            UiTone::Warning => WARNING_COLOR,
            UiTone::Danger => DANGER_COLOR,
        }
    }
    pub(crate) fn color(rgba: [u8; RGBA_CHANNEL_COUNT]) -> egui::Color32 {
        egui::Color32::from_rgba_unmultiplied(
            rgba[RED_CHANNEL_INDEX],
            rgba[GREEN_CHANNEL_INDEX],
            rgba[BLUE_CHANNEL_INDEX],
            rgba[ALPHA_CHANNEL_INDEX],
        )
    }
    pub(crate) fn egui_rect(rect: UiRect) -> egui::Rect {
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
}
