use crate::theme::Theme;
use crate::theme::typography::TextStyle;

use super::types::{TextProps, TextRole};

pub(super) fn resolve_style(role: TextRole, theme: &Theme) -> TextStyle {
    match role {
        TextRole::Body => theme.typography.body.clone(),
        TextRole::BodyStrong => theme.typography.body_strong.clone(),
        TextRole::Caption => theme.typography.caption.clone(),
        TextRole::Code => theme.typography.code.clone(),
        TextRole::Heading1 => theme.typography.heading_1.clone(),
        TextRole::Heading2 => theme.typography.heading_2.clone(),
        TextRole::Heading3 => theme.typography.heading_3.clone(),
    }
}

pub(super) fn resolve_color(props: &TextProps, theme: &Theme) -> (u8, u8, u8, u8) {
    let c = props.color_override.as_ref().unwrap_or(&theme.color.text);
    (c.r, c.g, c.b, c.a)
}
