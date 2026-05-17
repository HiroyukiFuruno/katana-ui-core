use crate::theme::Theme;
use crate::theme::color::Color;

use super::types::{Tone, Variant};

const DANGER_SUBTLE_ALPHA: u8 = 40;
const DARKEN_AMOUNT: u8 = 20;

/// Resolved background color for the button at rest.
pub(super) fn bg_color(variant: Variant, tone: Tone, theme: &Theme) -> Option<Color> {
    match variant {
        Variant::Plain | Variant::Subtle => None,
        Variant::Filled => Some(fill_color(tone, theme)),
    }
}

/// Resolved hover background color.
pub(super) fn hover_bg_color(variant: Variant, tone: Tone, theme: &Theme) -> Color {
    match variant {
        Variant::Plain | Variant::Subtle => subtle_color(tone, theme),
        Variant::Filled => darken(fill_color(tone, theme)),
    }
}

/// Resolved icon color.
pub(super) fn icon_color(variant: Variant, tone: Tone, theme: &Theme) -> Color {
    match variant {
        Variant::Plain | Variant::Subtle => tone_text_color(tone, theme),
        Variant::Filled => theme.color.surface,
    }
}

pub(super) fn disabled_icon_color(theme: &Theme) -> Color {
    theme.color.text_disabled
}

fn fill_color(tone: Tone, theme: &Theme) -> Color {
    match tone {
        Tone::Neutral => theme.color.border,
        Tone::Accent => theme.color.accent,
        Tone::Danger => theme.color.danger,
    }
}

fn subtle_color(tone: Tone, theme: &Theme) -> Color {
    match tone {
        Tone::Neutral => theme.color.border,
        Tone::Accent => theme.color.accent_muted,
        Tone::Danger => Color {
            r: theme.color.danger.r,
            g: theme.color.danger.g,
            b: theme.color.danger.b,
            a: DANGER_SUBTLE_ALPHA,
        },
    }
}

fn tone_text_color(tone: Tone, theme: &Theme) -> Color {
    match tone {
        Tone::Neutral => theme.color.text,
        Tone::Accent => theme.color.accent,
        Tone::Danger => theme.color.danger,
    }
}

fn darken(c: Color) -> Color {
    Color {
        r: c.r.saturating_sub(DARKEN_AMOUNT),
        g: c.g.saturating_sub(DARKEN_AMOUNT),
        b: c.b.saturating_sub(DARKEN_AMOUNT),
        a: c.a,
    }
}
