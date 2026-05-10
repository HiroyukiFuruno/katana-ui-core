use super::types::ModalSize;
use crate::theme::Theme;
use crate::theme::color::Color;

const OVERLAY_ALPHA: u8 = 160;
const CORNER_RADIUS: f32 = 8.0;
const WIDTH_SM: f32 = 320.0;
const WIDTH_MD: f32 = 480.0;
const WIDTH_LG: f32 = 640.0;
const PADDING: f32 = 24.0;
const TITLE_FONT_SIZE: f32 = 16.0;

pub(super) fn overlay_color(theme: &Theme) -> Color {
    Color {
        r: theme.color.bg.r,
        g: theme.color.bg.g,
        b: theme.color.bg.b,
        a: OVERLAY_ALPHA,
    }
}

pub(super) fn dialog_bg(theme: &Theme) -> Color {
    theme.color.surface
}

pub(super) fn dialog_border(theme: &Theme) -> Color {
    theme.color.border
}

pub(super) fn dialog_width(size: &ModalSize) -> f32 {
    match size {
        ModalSize::Sm => WIDTH_SM,
        ModalSize::Md => WIDTH_MD,
        ModalSize::Lg => WIDTH_LG,
        ModalSize::Custom(w) => *w,
    }
}

pub(super) fn corner_radius() -> f32 {
    CORNER_RADIUS
}

pub(super) fn dialog_padding() -> f32 {
    PADDING
}

pub(super) fn title_font_size() -> f32 {
    TITLE_FONT_SIZE
}

pub(super) fn title_color(theme: &Theme) -> Color {
    theme.color.text
}
