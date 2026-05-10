use super::types::SegmentedSize;
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

pub(super) fn font_size(size: SegmentedSize) -> f32 {
    match size {
        SegmentedSize::Sm => FONT_SM,
        SegmentedSize::Md => FONT_MD,
        SegmentedSize::Lg => FONT_LG,
    }
}

pub(super) fn padding(size: SegmentedSize) -> (f32, f32) {
    match size {
        SegmentedSize::Sm => (PAD_V_SM, PAD_H_SM),
        SegmentedSize::Md => (PAD_V_MD, PAD_H_MD),
        SegmentedSize::Lg => (PAD_V_LG, PAD_H_LG),
    }
}

pub(super) fn selected_bg(disabled: bool, theme: &Theme) -> Color {
    if disabled {
        theme.color.border
    } else {
        theme.color.accent
    }
}

pub(super) fn unselected_bg(theme: &Theme) -> Color {
    theme.color.surface
}

pub(super) fn selected_text(disabled: bool, theme: &Theme) -> Color {
    if disabled {
        theme.color.text_disabled
    } else {
        theme.color.bg
    }
}

pub(super) fn unselected_text(disabled: bool, theme: &Theme) -> Color {
    if disabled {
        theme.color.text_disabled
    } else {
        theme.color.text_muted
    }
}
