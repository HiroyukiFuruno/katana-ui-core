mod types;
mod view;

pub use types::{CardPadding, CardProps, CardVariant};

use crate::theme::Theme;
use crate::theme::color::Color;
use floem::IntoView;
use floem::views::{Decorators, container};
use view::{bg_color, border_color, corner_radius, has_shadow, hover_bg, padding_px};

/// Resolved visual properties for `Card`.
#[derive(Debug, Clone)]
pub struct ResolvedCard {
    pub bg_color: Color,
    pub hover_bg_color: Color,
    pub border_color: Option<Color>,
    pub has_shadow: bool,
    pub corner_radius: f32,
    pub padding: f32,
    pub interactive: bool,
}

/// Builder for the Card layout widget.
#[derive(Debug, Clone)]
pub struct Card {
    props: CardProps,
}

impl Card {
    #[must_use]
    pub fn new() -> Self {
        Self {
            props: CardProps {
                variant: CardVariant::default(),
                padding: CardPadding::default(),
                interactive: false,
            },
        }
    }

    #[must_use]
    pub fn variant(mut self, variant: CardVariant) -> Self {
        self.props.variant = variant;
        self
    }

    #[must_use]
    pub fn padding(mut self, padding: CardPadding) -> Self {
        self.props.padding = padding;
        self
    }

    #[must_use]
    pub fn interactive(mut self, interactive: bool) -> Self {
        self.props.interactive = interactive;
        self
    }

    #[must_use]
    pub fn resolve(&self, theme: &Theme) -> ResolvedCard {
        ResolvedCard {
            bg_color: bg_color(self.props.variant, theme),
            hover_bg_color: hover_bg(self.props.variant, theme),
            border_color: border_color(self.props.variant, theme),
            has_shadow: has_shadow(self.props.variant),
            corner_radius: corner_radius(),
            padding: padding_px(self.props.padding, theme),
            interactive: self.props.interactive,
        }
    }

    #[must_use]
    pub fn view(self, theme: Theme, child: impl IntoView + 'static) -> impl IntoView {
        let resolved = self.resolve(&theme);
        let bg = crate::floem_view::FloemColor::from_token(resolved.bg_color);
        let border = resolved
            .border_color
            .map(crate::floem_view::FloemColor::from_token);
        container(child).style(move |style| {
            let style = style
                .background(bg)
                .border_radius(resolved.corner_radius)
                .padding(resolved.padding);
            if let Some(border) = border {
                style.border(1.0).border_color(border)
            } else {
                style
            }
        })
    }
}

impl Default for Card {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::Theme;

    #[test]
    fn plain_has_no_border_no_shadow() {
        let theme = Theme::default_light();
        let r = Card::new().variant(CardVariant::Plain).resolve(&theme);
        assert!(r.border_color.is_none());
        assert!(!r.has_shadow);
    }

    #[test]
    fn elevated_has_shadow_no_border() {
        let theme = Theme::default_light();
        let r = Card::new().variant(CardVariant::Elevated).resolve(&theme);
        assert!(r.has_shadow);
        assert!(r.border_color.is_none());
    }

    #[test]
    fn outlined_has_border_no_shadow() {
        let theme = Theme::default_light();
        let r = Card::new().variant(CardVariant::Outlined).resolve(&theme);
        assert!(r.border_color.is_some());
        assert!(!r.has_shadow);
    }

    #[test]
    fn interactive_flag_preserved() {
        let theme = Theme::default_light();
        let r = Card::new().interactive(true).resolve(&theme);
        assert!(r.interactive);
    }

    #[test]
    fn no_padding_is_zero() {
        let theme = Theme::default_light();
        let r = Card::new().padding(CardPadding::None).resolve(&theme);
        assert_eq!(r.padding, 0.0);
    }
}
