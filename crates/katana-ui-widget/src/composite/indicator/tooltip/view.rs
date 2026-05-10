use super::types::Placement;
use crate::theme::Theme;
use crate::theme::color::Color;

const DEFAULT_DELAY_MS: u32 = 400;
const DEFAULT_MAX_WIDTH: f32 = 240.0;
const FONT_SIZE: f32 = 11.0;
const PAD_V: f32 = 4.0;
const PAD_H: f32 = 8.0;

pub(super) fn default_delay_ms() -> u32 {
    DEFAULT_DELAY_MS
}

pub(super) fn default_max_width() -> f32 {
    DEFAULT_MAX_WIDTH
}

pub(super) fn font_size() -> f32 {
    FONT_SIZE
}

pub(super) fn padding() -> (f32, f32) {
    (PAD_V, PAD_H)
}

pub(super) fn bg_color(theme: &Theme) -> Color {
    theme.color.text
}

pub(super) fn text_color(theme: &Theme) -> Color {
    theme.color.bg
}

/// Returns the effective placement after edge-flip logic.
/// In the resolved model, the caller supplies the available space.
/// For simplicity, this returns the original placement as-is (runtime
/// flip is handled by the rendering layer).
pub(super) fn effective_placement(placement: Placement) -> Placement {
    placement
}

pub(super) fn hover_visible(elapsed_ms: u32, delay_ms: u32) -> bool {
    elapsed_ms >= delay_ms
}

pub(super) fn focus_visible() -> bool {
    true
}

pub(super) fn flip_placement(
    placement: Placement,
    preferred_fits: bool,
    opposite_fits: bool,
) -> Placement {
    if preferred_fits || !opposite_fits {
        return placement;
    }

    match placement {
        Placement::Top => Placement::Bottom,
        Placement::Bottom => Placement::Top,
        Placement::Start => Placement::End,
        Placement::End => Placement::Start,
    }
}
