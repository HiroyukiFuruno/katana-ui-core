use super::types::{CardPadding, CardVariant};
use crate::theme::Theme;
use crate::theme::color::Color;

const CORNER_RADIUS: f32 = 6.0;
const HOVER_DARKEN: u8 = 8;

pub(super) fn bg_color(variant: CardVariant, theme: &Theme) -> Color {
    match variant {
        CardVariant::Plain => theme.color.bg,
        CardVariant::Elevated | CardVariant::Outlined => theme.color.surface,
    }
}

pub(super) fn border_color(variant: CardVariant, theme: &Theme) -> Option<Color> {
    match variant {
        CardVariant::Outlined => Some(theme.color.border),
        _ => None,
    }
}

pub(super) fn has_shadow(variant: CardVariant) -> bool {
    matches!(variant, CardVariant::Elevated)
}

pub(super) fn corner_radius() -> f32 {
    CORNER_RADIUS
}

pub(super) fn padding_px(padding: CardPadding, theme: &Theme) -> f32 {
    match padding {
        CardPadding::None => 0.0,
        CardPadding::Sm => theme.spacing.xs,
        CardPadding::Md => theme.spacing.sm,
        CardPadding::Lg => theme.spacing.md,
    }
}

pub(super) fn hover_bg(variant: CardVariant, theme: &Theme) -> Color {
    let base = bg_color(variant, theme);
    Color {
        r: base.r.saturating_sub(HOVER_DARKEN),
        g: base.g.saturating_sub(HOVER_DARKEN),
        b: base.b.saturating_sub(HOVER_DARKEN),
        a: base.a,
    }
}
