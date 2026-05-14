mod types;
mod view;

pub use types::{ResolvedToolbar, ToolbarAlignment};

use crate::theme::Theme;
use crate::theme::color::Color;
use floem::IntoView;

/// Builder for a toolbar row with leading / trailing slots.
pub struct Toolbar {
    props: types::ToolbarProps,
}

impl Toolbar {
    const DEFAULT_GAP: f32 = crate::floem_view::GAP_SM;
    const DEFAULT_PADDING: f32 = crate::floem_view::GAP_SM;

    #[must_use]
    pub fn new() -> Self {
        Self {
            props: types::ToolbarProps {
                leading: None,
                trailing: None,
                gap: None,
                alignment: ToolbarAlignment::Center,
                height: None,
                padding: None,
                background: None,
                show_border: false,
            },
        }
    }

    #[must_use]
    pub fn leading<V>(mut self, leading: V) -> Self
    where
        V: IntoView + 'static,
    {
        self.props.leading = Some(leading.into_any());
        self
    }

    #[must_use]
    pub fn trailing<V>(mut self, trailing: V) -> Self
    where
        V: IntoView + 'static,
    {
        self.props.trailing = Some(trailing.into_any());
        self
    }

    #[must_use]
    pub fn gap(mut self, gap: f32) -> Self {
        self.props.gap = Some(gap);
        self
    }

    #[must_use]
    pub fn alignment(mut self, alignment: ToolbarAlignment) -> Self {
        self.props.alignment = alignment;
        self
    }

    #[must_use]
    pub fn height(mut self, height: f32) -> Self {
        self.props.height = Some(height);
        self
    }

    #[must_use]
    pub fn padding(mut self, padding: f32) -> Self {
        self.props.padding = Some(padding);
        self
    }

    #[must_use]
    pub fn background(mut self, background: Color) -> Self {
        self.props.background = Some(background);
        self
    }

    #[must_use]
    pub fn show_border(mut self, show_border: bool) -> Self {
        self.props.show_border = show_border;
        self
    }

    /// Resolves style properties against current theme tokens.
    #[must_use]
    pub fn resolve(&self, theme: &Theme) -> ResolvedToolbar {
        ResolvedToolbar {
            gap: self.props.gap.unwrap_or(Self::DEFAULT_GAP),
            alignment: self.props.alignment,
            height: self.props.height,
            padding: self.props.padding.unwrap_or(Self::DEFAULT_PADDING),
            background: self.props.background,
            show_border: self.props.show_border,
            border_color: theme.color.border,
        }
    }
}

impl Default for Toolbar {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::Theme;
    use floem::views::label;

    #[test]
    fn defaults_resolve_to_center() {
        let theme = Theme::default_light();
        let resolved = Toolbar::new().resolve(&theme);
        assert_eq!(resolved.alignment, ToolbarAlignment::Center);
        assert_eq!(resolved.gap, Theme::default_light().spacing.sm);
        assert_eq!(resolved.padding, Theme::default_light().spacing.sm);
        assert!(!resolved.show_border);
        assert!(resolved.background.is_none());
    }

    #[test]
    fn can_configure_layout_options() {
        let theme = Theme::default_light();
        let color = theme.color.surface;
        let resolved = Toolbar::new()
            .alignment(ToolbarAlignment::Top)
            .gap(12.0)
            .height(42.0)
            .padding(6.0)
            .background(color)
            .show_border(true)
            .resolve(&theme);
        assert_eq!(resolved.alignment, ToolbarAlignment::Top);
        assert_eq!(resolved.gap, 12.0);
        assert_eq!(resolved.height, Some(42.0));
        assert_eq!(resolved.padding, 6.0);
        assert_eq!(resolved.background, Some(color));
        assert!(resolved.show_border);
    }

    #[test]
    fn accepts_optional_slots_as_any_node() {
        let theme = Theme::default_light();
        let _toolbar = Toolbar::new()
            .leading(label(|| "leading"))
            .trailing(label(|| "trailing"))
            .view(theme.clone());
    }
}
