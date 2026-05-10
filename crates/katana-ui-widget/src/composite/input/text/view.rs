use super::types::InputSize;
use crate::theme::Theme;
use crate::theme::color::Color;

const FONT_SM: f32 = 11.0;
const FONT_MD: f32 = 13.0;
const FONT_LG: f32 = 15.0;
const PAD_V_SM: f32 = 4.0;
const PAD_V_MD: f32 = 6.0;
const PAD_V_LG: f32 = 8.0;
const PAD_H_SM: f32 = 8.0;
const PAD_H_MD: f32 = 10.0;
const PAD_H_LG: f32 = 12.0;

pub(super) fn font_size(size: InputSize) -> f32 {
    match size {
        InputSize::Sm => FONT_SM,
        InputSize::Md => FONT_MD,
        InputSize::Lg => FONT_LG,
    }
}

pub(super) fn padding(size: InputSize) -> (f32, f32) {
    match size {
        InputSize::Sm => (PAD_V_SM, PAD_H_SM),
        InputSize::Md => (PAD_V_MD, PAD_H_MD),
        InputSize::Lg => (PAD_V_LG, PAD_H_LG),
    }
}

pub(super) fn bg_color(disabled: bool, theme: &Theme) -> Color {
    if disabled {
        theme.color.surface
    } else {
        theme.color.bg
    }
}

pub(super) fn text_color(disabled: bool, has_value: bool, theme: &Theme) -> Color {
    if disabled {
        theme.color.text_disabled
    } else if has_value {
        theme.color.text
    } else {
        theme.color.text_muted
    }
}

pub(super) fn border_color(invalid: bool, disabled: bool, theme: &Theme) -> Color {
    if disabled {
        theme.color.border
    } else if invalid {
        theme.color.danger
    } else {
        theme.color.border
    }
}

pub(super) fn focus_ring_color(invalid: bool, theme: &Theme) -> Color {
    if invalid {
        theme.color.danger
    } else {
        theme.color.accent
    }
}
