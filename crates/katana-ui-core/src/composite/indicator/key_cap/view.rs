use super::types::{KeyCapSize, KeyCapTone};
use crate::theme::Theme;
use crate::theme::color::Color;

const FONT_SM: f32 = 10.0;
const FONT_MD: f32 = 11.0;
const PAD_V_SM: f32 = 1.0;
const PAD_V_MD: f32 = 2.0;
const PAD_H_SM: f32 = 4.0;
const PAD_H_MD: f32 = 6.0;

pub(super) fn font_size(size: KeyCapSize) -> f32 {
    match size {
        KeyCapSize::Sm => FONT_SM,
        KeyCapSize::Md => FONT_MD,
    }
}

pub(super) fn padding(size: KeyCapSize) -> (f32, f32) {
    match size {
        KeyCapSize::Sm => (PAD_V_SM, PAD_H_SM),
        KeyCapSize::Md => (PAD_V_MD, PAD_H_MD),
    }
}

pub(super) fn bg_color(tone: KeyCapTone, theme: &Theme) -> Color {
    match tone {
        KeyCapTone::Neutral => theme.color.surface,
        KeyCapTone::Subtle => theme.color.bg,
    }
}

pub(super) fn text_color(theme: &Theme) -> Color {
    theme.color.text_muted
}

pub(super) fn border_color(theme: &Theme) -> Color {
    theme.color.border
}
