mod types;
mod view;

pub use types::{BadgeProps, BadgeSize, BadgeTone, BadgeVariant};

use crate::theme::Theme;
use crate::theme::color::Color;
use floem::IntoView;
use floem::views::{Decorators, h_stack, label};
use view::{bg_color, border_color, font_size, padding, text_color};

const BADGE_GAP: f32 = crate::floem_view::GAP_XS;
const BADGE_RADIUS: f32 = crate::floem_view::CORNER_RADIUS_SM;

/// Resolved visual properties for `Badge`.
#[derive(Debug, Clone)]
pub struct ResolvedBadge {
    pub label: String,
    pub font_size: f32,
    pub pad_v: f32,
    pub pad_h: f32,
    pub bg_color: Option<Color>,
    pub text_color: Color,
    pub border_color: Option<Color>,
    pub has_leading_icon: bool,
}

/// Builder for the Badge composite widget.
#[derive(Debug, Clone)]
pub struct Badge {
    props: BadgeProps,
}

impl Badge {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            props: BadgeProps {
                label: label.into(),
                tone: BadgeTone::default(),
                variant: BadgeVariant::default(),
                size: BadgeSize::default(),
                leading_icon: None,
            },
        }
    }

    #[must_use]
    pub fn tone(mut self, tone: BadgeTone) -> Self {
        self.props.tone = tone;
        self
    }

    #[must_use]
    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.props.variant = variant;
        self
    }

    #[must_use]
    pub fn size(mut self, size: BadgeSize) -> Self {
        self.props.size = size;
        self
    }

    #[must_use]
    pub fn leading_icon(mut self, icon: crate::primitive::icon::IconSource) -> Self {
        self.props.leading_icon = Some(icon);
        self
    }

    #[must_use]
    pub fn resolve(&self, theme: &Theme) -> ResolvedBadge {
        ResolvedBadge {
            label: self.props.label.clone(),
            font_size: font_size(self.props.size),
            pad_v: padding(self.props.size).0,
            pad_h: padding(self.props.size).1,
            bg_color: bg_color(self.props.tone, self.props.variant, theme),
            text_color: text_color(self.props.tone, self.props.variant, theme),
            border_color: border_color(self.props.tone, self.props.variant, theme),
            has_leading_icon: self.props.leading_icon.is_some(),
        }
    }

    #[must_use]
    pub fn view(self, theme: Theme) -> impl IntoView {
        let resolved = self.resolve(&theme);
        let text = crate::floem_view::FloemColor::from_token(resolved.text_color);
        let bg = resolved
            .bg_color
            .map(crate::floem_view::FloemColor::from_token);
        let border = resolved
            .border_color
            .map(crate::floem_view::FloemColor::from_token);
        h_stack((
            label(move || {
                if resolved.has_leading_icon {
                    "●".to_string()
                } else {
                    String::new()
                }
            }),
            label(move || resolved.label.clone())
                .style(move |style| style.font_size(resolved.font_size).color(text)),
        ))
        .style(move |style| {
            let style = style
                .gap(BADGE_GAP)
                .items_center()
                .padding_vert(resolved.pad_v)
                .padding_horiz(resolved.pad_h)
                .border_radius(BADGE_RADIUS);
            let style = if let Some(bg) = bg {
                style.background(bg)
            } else {
                style
            };
            if let Some(border) = border {
                style.border(1.0).border_color(border)
            } else {
                style
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::Theme;

    #[test]
    fn solid_has_bg_color() {
        let theme = Theme::default_light();
        let r = Badge::new("New")
            .tone(BadgeTone::Accent)
            .variant(BadgeVariant::Solid)
            .resolve(&theme);
        assert!(r.bg_color.is_some());
        assert!(r.border_color.is_none());
    }

    #[test]
    fn outline_has_border_no_bg() {
        let theme = Theme::default_light();
        let r = Badge::new("Draft")
            .tone(BadgeTone::Neutral)
            .variant(BadgeVariant::Outline)
            .resolve(&theme);
        assert!(r.bg_color.is_none());
        assert!(r.border_color.is_some());
    }

    #[test]
    fn subtle_has_semi_transparent_bg() {
        let theme = Theme::default_light();
        let r = Badge::new("Beta")
            .tone(BadgeTone::Warning)
            .variant(BadgeVariant::Subtle)
            .resolve(&theme);
        assert_eq!(r.bg_color.map(|bg| bg.a < 255), Some(true));
    }

    #[test]
    fn all_tones_resolve_without_panic() {
        let theme = Theme::default_light();
        for tone in [
            BadgeTone::Neutral,
            BadgeTone::Accent,
            BadgeTone::Danger,
            BadgeTone::Warning,
            BadgeTone::Success,
            BadgeTone::Info,
        ] {
            for variant in [
                BadgeVariant::Solid,
                BadgeVariant::Subtle,
                BadgeVariant::Outline,
            ] {
                let _ = Badge::new("x").tone(tone).variant(variant).resolve(&theme);
            }
        }
    }
}
