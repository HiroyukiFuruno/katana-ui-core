use super::types::IndicatorPosition;
use crate::theme::Theme;
use crate::theme::color::Color;

const HEADER_FONT_SIZE: f32 = 13.0;
const HEADER_PAD_V: f32 = 8.0;
const HEADER_PAD_H: f32 = 12.0;
const ANIMATION_MS: u32 = 180;

pub(super) fn header_font_size() -> f32 {
    HEADER_FONT_SIZE
}

pub(super) fn header_padding() -> (f32, f32) {
    (HEADER_PAD_V, HEADER_PAD_H)
}

pub(super) fn animation_ms() -> u32 {
    ANIMATION_MS
}

pub(super) fn chevron_symbol(expanded: bool, position: IndicatorPosition) -> Option<&'static str> {
    match position {
        IndicatorPosition::None => None,
        _ => Some(if expanded { "▲" } else { "▼" }),
    }
}

pub(super) fn header_bg(disabled: bool, theme: &Theme) -> Color {
    if disabled {
        theme.color.surface
    } else {
        theme.color.bg
    }
}

pub(super) fn header_text(disabled: bool, theme: &Theme) -> Color {
    if disabled {
        theme.color.text_disabled
    } else {
        theme.color.text
    }
}

pub(super) fn border_color(theme: &Theme) -> Color {
    theme.color.border
}
