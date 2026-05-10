mod types;
mod view;

pub use types::{SvgButtonProps, Tone, Variant};

use crate::primitive::icon::{IconSize, IconSource};
use crate::theme::Theme;
use crate::theme::color::Color;
use view::{bg_color, disabled_icon_color, hover_bg_color, icon_color};

/// Resolved visual properties for `SvgButton`.
#[derive(Debug, Clone)]
pub struct ResolvedSvgButton {
    pub icon_source: IconSource,
    pub size_px: f32,
    pub icon_color: Color,
    pub bg_color: Option<Color>,
    pub hover_bg_color: Color,
    pub disabled: bool,
    pub loading: bool,
    pub a11y_label: String,
}

/// Builder for the SvgButton composite widget.
#[derive(Debug, Clone)]
pub struct SvgButton {
    props: SvgButtonProps,
}

impl SvgButton {
    /// Create a new `SvgButton`.
    ///
    /// `a11y_label` is required — it must be a non-empty string describing the button action.
    #[must_use]
    pub fn new(icon: IconSource, a11y_label: impl Into<String>) -> Self {
        Self {
            props: SvgButtonProps {
                icon,
                size: IconSize::Lg,
                variant: Variant::default(),
                tone: Tone::default(),
                disabled: false,
                loading: false,
                a11y_label: a11y_label.into(),
            },
        }
    }

    #[must_use]
    pub fn size(mut self, size: IconSize) -> Self {
        self.props.size = size;
        self
    }

    #[must_use]
    pub fn variant(mut self, variant: Variant) -> Self {
        self.props.variant = variant;
        self
    }

    #[must_use]
    pub fn tone(mut self, tone: Tone) -> Self {
        self.props.tone = tone;
        self
    }

    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.props.disabled = disabled;
        self
    }

    #[must_use]
    pub fn loading(mut self, loading: bool) -> Self {
        self.props.loading = loading;
        self
    }

    /// Resolve visual properties against the given theme.
    #[must_use]
    pub fn resolve(&self, theme: &Theme) -> ResolvedSvgButton {
        let size_px = self.props.size.resolve_px(&theme.spacing);
        let ic = if self.props.disabled {
            disabled_icon_color(theme)
        } else {
            icon_color(self.props.variant, self.props.tone, theme)
        };
        ResolvedSvgButton {
            icon_source: self.props.icon.clone(),
            size_px,
            icon_color: ic,
            bg_color: if self.props.disabled {
                None
            } else {
                bg_color(self.props.variant, self.props.tone, theme)
            },
            hover_bg_color: if self.props.disabled {
                theme.color.bg
            } else {
                hover_bg_color(self.props.variant, self.props.tone, theme)
            },
            disabled: self.props.disabled,
            loading: self.props.loading,
            a11y_label: self.props.a11y_label.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitive::icon::IconSource;
    use crate::theme::Theme;

    const ICON: &[u8] = b"<svg/>";

    #[test]
    fn all_variant_tone_combos_resolve_without_panic() {
        let theme = Theme::default_light();
        let variants = [Variant::Plain, Variant::Subtle, Variant::Filled];
        let tones = [Tone::Neutral, Tone::Accent, Tone::Danger];
        for v in variants {
            for t in tones {
                let _ = SvgButton::new(IconSource::SvgBytes(ICON), "label")
                    .variant(v)
                    .tone(t)
                    .resolve(&theme);
            }
        }
    }

    #[test]
    fn disabled_button_has_no_bg_and_muted_icon() {
        let theme = Theme::default_light();
        let resolved = SvgButton::new(IconSource::SvgBytes(ICON), "close")
            .disabled(true)
            .resolve(&theme);
        assert!(resolved.bg_color.is_none());
        assert_eq!(resolved.icon_color, theme.color.text_disabled);
    }
}
