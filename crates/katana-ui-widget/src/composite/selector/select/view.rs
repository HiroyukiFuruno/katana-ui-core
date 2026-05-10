use super::types::SelectSize;
use crate::theme::Theme;
use crate::theme::color::Color;

const FONT_SM: f32 = 11.0;
const FONT_MD: f32 = 13.0;
const FONT_LG: f32 = 15.0;
const PAD_V_SM: f32 = 4.0;
const PAD_V_MD: f32 = 6.0;
const PAD_V_LG: f32 = 8.0;
const PAD_H_SM: f32 = 8.0;
const PAD_H_MD: f32 = 12.0;
const PAD_H_LG: f32 = 16.0;

pub(super) fn font_size(size: SelectSize) -> f32 {
    match size {
        SelectSize::Sm => FONT_SM,
        SelectSize::Md => FONT_MD,
        SelectSize::Lg => FONT_LG,
    }
}

pub(super) fn padding(size: SelectSize) -> (f32, f32) {
    match size {
        SelectSize::Sm => (PAD_V_SM, PAD_H_SM),
        SelectSize::Md => (PAD_V_MD, PAD_H_MD),
        SelectSize::Lg => (PAD_V_LG, PAD_H_LG),
    }
}

pub(super) fn trigger_bg(disabled: bool, theme: &Theme) -> Color {
    if disabled {
        theme.color.surface
    } else {
        theme.color.bg
    }
}

pub(super) fn trigger_text(disabled: bool, has_value: bool, theme: &Theme) -> Color {
    if disabled {
        theme.color.text_disabled
    } else if has_value {
        theme.color.text
    } else {
        theme.color.text_muted
    }
}

pub(super) fn border_color(is_open: bool, disabled: bool, theme: &Theme) -> Color {
    if disabled {
        theme.color.border
    } else if is_open {
        theme.color.accent
    } else {
        theme.color.border
    }
}

pub(super) fn option_bg(selected: bool, theme: &Theme) -> Color {
    if selected {
        theme.color.accent_muted
    } else {
        theme.color.bg
    }
}

pub(super) fn option_text(selected: bool, theme: &Theme) -> Color {
    if selected {
        theme.color.accent
    } else {
        theme.color.text
    }
}
