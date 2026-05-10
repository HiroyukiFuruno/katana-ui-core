mod types;
mod view;

pub use types::{TextAlign, TextProps, TextRole};

use crate::theme::Theme;
use crate::theme::color::Color;
use floem::IntoView;
use floem::views::{Decorators, label};
use view::{resolve_color, resolve_style};

/// Builder for the Text primitive.
///
/// ```ignore
/// Text::new("Hello").role(TextRole::Heading1).build()
/// ```
#[derive(Debug, Clone)]
pub struct Text {
    content: String,
    props: TextProps,
}

impl Text {
    #[must_use]
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            props: TextProps::default(),
        }
    }

    #[must_use]
    pub fn role(mut self, role: TextRole) -> Self {
        self.props.role = role;
        self
    }

    #[must_use]
    pub fn color_override(mut self, color: Color) -> Self {
        self.props.color_override = Some(color);
        self
    }

    #[must_use]
    pub fn max_lines(mut self, n: usize) -> Self {
        self.props.max_lines = Some(n);
        self
    }

    #[must_use]
    pub fn align(mut self, align: TextAlign) -> Self {
        self.props.align = align;
        self
    }

    /// Resolve the styled text metadata using the given theme.
    #[must_use]
    pub fn resolve(&self, theme: &Theme) -> ResolvedText {
        let style = resolve_style(self.props.role, theme);
        let (r, g, b, a) = resolve_color(&self.props, theme);
        ResolvedText {
            content: self.content.clone(),
            font_size: style.font_size,
            color_r: r,
            color_g: g,
            color_b: b,
            color_a: a,
            max_lines: self.props.max_lines,
            align: self.props.align,
        }
    }

    #[must_use]
    pub fn view(self, theme: Theme) -> impl IntoView {
        let resolved = self.resolve(&theme);
        let text_color = floem::peniko::Color::rgba8(
            resolved.color_r,
            resolved.color_g,
            resolved.color_b,
            resolved.color_a,
        );
        label(move || resolved.content.clone())
            .style(move |style| style.font_size(resolved.font_size).color(text_color))
    }
}

/// Resolved, ready-to-render text properties.
#[derive(Debug, Clone)]
pub struct ResolvedText {
    pub content: String,
    pub font_size: f32,
    pub color_r: u8,
    pub color_g: u8,
    pub color_b: u8,
    pub color_a: u8,
    pub max_lines: Option<usize>,
    pub align: TextAlign,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::Theme;
    use crate::theme::color::Color;

    #[test]
    fn each_role_resolves_correct_font_size() {
        let theme = Theme::default_light();
        let cases = [
            (TextRole::Body, theme.typography.body.font_size),
            (TextRole::BodyStrong, theme.typography.body_strong.font_size),
            (TextRole::Caption, theme.typography.caption.font_size),
            (TextRole::Code, theme.typography.code.font_size),
            (TextRole::Heading1, theme.typography.heading_1.font_size),
            (TextRole::Heading2, theme.typography.heading_2.font_size),
            (TextRole::Heading3, theme.typography.heading_3.font_size),
        ];
        for (role, expected_size) in cases {
            let resolved = Text::new("sample").role(role).resolve(&theme);
            assert_eq!(
                resolved.font_size, expected_size,
                "font_size mismatch for {role:?}"
            );
        }
    }

    #[test]
    fn color_override_replaces_theme_default() {
        let theme = Theme::default_light();
        let red = Color {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        };
        let resolved = Text::new("hi").color_override(red).resolve(&theme);
        assert_eq!(resolved.color_r, 255);
        assert_eq!(resolved.color_g, 0);
        assert_eq!(resolved.color_b, 0);
    }
}
