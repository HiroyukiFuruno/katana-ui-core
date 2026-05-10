use crate::theme::Theme;
use crate::theme::color::Color;

const CORNER_RADIUS: f32 = 6.0;
const DEFAULT_OFFSET: f32 = 4.0;
const SHADOW_ALPHA: u8 = 40;

pub(super) fn popover_bg(theme: &Theme) -> Color {
    theme.color.surface
}

pub(super) fn popover_border(theme: &Theme) -> Color {
    theme.color.border
}

pub(super) fn shadow_color(theme: &Theme) -> Color {
    Color {
        r: theme.color.bg.r,
        g: theme.color.bg.g,
        b: theme.color.bg.b,
        a: SHADOW_ALPHA,
    }
}

pub(super) fn corner_radius() -> f32 {
    CORNER_RADIUS
}

pub(super) fn default_offset() -> f32 {
    DEFAULT_OFFSET
}
