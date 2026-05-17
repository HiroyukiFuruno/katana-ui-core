use crate::floem_view::FloemColor;
use crate::theme::Theme;
use floem::peniko::Color;

use super::types::MenuButtonVariant;

pub(super) const MENU_RADIUS: f32 = crate::floem_view::CORNER_RADIUS_SM;
pub(super) const MENU_GAP: f32 = crate::floem_view::GAP_XS;
pub(super) const MENU_PADDING: f32 = crate::floem_view::GAP_SM;
pub(super) const MENU_OFFSET_Y: f32 = 4.0;

pub(super) fn menu_style(variant: MenuButtonVariant, theme: &Theme) -> (Color, Color, Color, f32) {
    match variant {
        MenuButtonVariant::Framed => (
            FloemColor::from_token(theme.color.text),
            FloemColor::from_token(theme.color.surface),
            FloemColor::from_token(theme.color.border),
            1.0,
        ),
        MenuButtonVariant::Unframed => (
            FloemColor::from_token(theme.color.accent),
            FloemColor::from_token(theme.color.bg),
            FloemColor::from_token(theme.color.bg),
            0.0,
        ),
    }
}
