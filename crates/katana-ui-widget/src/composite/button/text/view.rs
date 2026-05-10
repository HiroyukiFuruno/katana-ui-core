use crate::theme::Theme;
use crate::theme::color::Color;

use super::types::{Size, Tone, Variant};

const OPACITY_LOADING: u8 = 128;

/// Resolved background color at rest.
pub(super) fn bg_color(variant: Variant, tone: Tone, theme: &Theme) -> Option<Color> {
    match variant {
        Variant::Primary => Some(fill_color(tone, theme)),
        Variant::Secondary | Variant::Ghost | Variant::Link => None,
    }
}

/// Resolved hover background color.
pub(super) fn hover_bg_color(variant: Variant, tone: Tone, theme: &Theme) -> Color {
    match variant {
        Variant::Primary => {
            let c = fill_color(tone, theme);
            darken(c)
        }
        Variant::Secondary | Variant::Ghost => subtle_bg(tone, theme),
        Variant::Link => theme.color.bg,
    }
}

/// Resolved text color.
pub(super) fn text_color(variant: Variant, tone: Tone, theme: &Theme) -> Color {
    match variant {
        Variant::Primary => theme.color.surface,
        Variant::Secondary | Variant::Ghost | Variant::Link => tone_text(tone, theme),
    }
}

/// Resolved font size from size token.
pub(super) fn font_size(size: Size, theme: &Theme) -> f32 {
    match size {
        Size::Sm => theme.typography.caption.font_size,
        Size::Md => theme.typography.body.font_size,
        Size::Lg => theme.typography.heading_3.font_size,
    }
}

/// Resolved padding (vertical, horizontal).
pub(super) fn padding(size: Size, theme: &Theme) -> (f32, f32) {
    match size {
        Size::Sm => (theme.spacing.xs, theme.spacing.sm),
        Size::Md => (theme.spacing.sm, theme.spacing.md),
        Size::Lg => (theme.spacing.md, theme.spacing.lg),
    }
}

/// Alpha for loading label.
pub(super) const fn loading_text_alpha() -> u8 {
    OPACITY_LOADING
}

fn fill_color(tone: Tone, theme: &Theme) -> Color {
    match tone {
        Tone::Neutral => theme.color.border,
        Tone::Accent => theme.color.accent,
        Tone::Danger => theme.color.danger,
        Tone::Success => theme.color.success,
    }
}

const SUBTLE_ALPHA: u8 = 40;

fn subtle_bg(tone: Tone, theme: &Theme) -> Color {
    match tone {
        Tone::Neutral => theme.color.border,
        Tone::Accent => theme.color.accent_muted,
        Tone::Danger => Color {
            r: theme.color.danger.r,
            g: theme.color.danger.g,
            b: theme.color.danger.b,
            a: SUBTLE_ALPHA,
        },
        Tone::Success => Color {
            r: theme.color.success.r,
            g: theme.color.success.g,
            b: theme.color.success.b,
            a: SUBTLE_ALPHA,
        },
    }
}

fn tone_text(tone: Tone, theme: &Theme) -> Color {
    match tone {
        Tone::Neutral => theme.color.text,
        Tone::Accent => theme.color.accent,
        Tone::Danger => theme.color.danger,
        Tone::Success => theme.color.success,
    }
}

const DARKEN: u8 = 20;

fn darken(c: Color) -> Color {
    Color {
        r: c.r.saturating_sub(DARKEN),
        g: c.g.saturating_sub(DARKEN),
        b: c.b.saturating_sub(DARKEN),
        a: c.a,
    }
}
