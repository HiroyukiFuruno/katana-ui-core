use crate::theme::Theme;
use crate::theme::color::Color;

const HANDLE_THICKNESS: f32 = 4.0;
const HANDLE_HOVER_ALPHA: u8 = 80;

pub(super) fn handle_thickness() -> f32 {
    HANDLE_THICKNESS
}

pub(super) fn handle_color(theme: &Theme) -> Color {
    theme.color.border
}

pub(super) fn handle_hover_color(theme: &Theme) -> Color {
    Color {
        r: theme.color.accent.r,
        g: theme.color.accent.g,
        b: theme.color.accent.b,
        a: HANDLE_HOVER_ALPHA,
    }
}
