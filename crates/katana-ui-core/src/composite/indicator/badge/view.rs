use super::types::{BadgeSize, BadgeTone, BadgeVariant};
use crate::theme::Theme;
use crate::theme::color::Color;

const FONT_SM: f32 = 10.0;
const FONT_MD: f32 = 11.0;
const PAD_V_SM: f32 = 1.0;
const PAD_V_MD: f32 = 2.0;
const PAD_H_SM: f32 = 4.0;
const PAD_H_MD: f32 = 6.0;
const SUBTLE_ALPHA: u8 = 40;

pub(super) fn font_size(size: BadgeSize) -> f32 {
    match size {
        BadgeSize::Sm => FONT_SM,
        BadgeSize::Md => FONT_MD,
    }
}

pub(super) fn padding(size: BadgeSize) -> (f32, f32) {
    match size {
        BadgeSize::Sm => (PAD_V_SM, PAD_H_SM),
        BadgeSize::Md => (PAD_V_MD, PAD_H_MD),
    }
}

fn tone_base(tone: BadgeTone, theme: &Theme) -> Color {
    match tone {
        BadgeTone::Neutral => theme.color.text_muted,
        BadgeTone::Accent => theme.color.accent,
        BadgeTone::Danger => theme.color.danger,
        BadgeTone::Warning => theme.color.warning,
        BadgeTone::Success => theme.color.success,
        BadgeTone::Info => theme.color.accent,
    }
}

pub(super) fn bg_color(tone: BadgeTone, variant: BadgeVariant, theme: &Theme) -> Option<Color> {
    match variant {
        BadgeVariant::Solid => Some(tone_base(tone, theme)),
        BadgeVariant::Subtle => {
            let base = tone_base(tone, theme);
            Some(Color {
                r: base.r,
                g: base.g,
                b: base.b,
                a: SUBTLE_ALPHA,
            })
        }
        BadgeVariant::Outline => None,
    }
}

pub(super) fn text_color(tone: BadgeTone, variant: BadgeVariant, theme: &Theme) -> Color {
    match variant {
        BadgeVariant::Solid => theme.color.bg,
        BadgeVariant::Subtle | BadgeVariant::Outline => tone_base(tone, theme),
    }
}

pub(super) fn border_color(tone: BadgeTone, variant: BadgeVariant, theme: &Theme) -> Option<Color> {
    match variant {
        BadgeVariant::Outline => Some(tone_base(tone, theme)),
        _ => None,
    }
}
