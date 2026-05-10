mod types;
mod view;

pub use types::{Size, TextButtonProps, Tone, Variant};

use crate::theme::Theme;
use crate::theme::color::Color;
use view::{bg_color, font_size, hover_bg_color, loading_text_alpha, padding, text_color};

/// Resolved visual properties for `TextButton`.
#[derive(Debug, Clone)]
pub struct ResolvedTextButton {
    pub label: String,
    pub font_size: f32,
    pub text_color: Color,
    pub bg_color: Option<Color>,
    pub hover_bg_color: Color,
    pub pad_v: f32,
    pub pad_h: f32,
    pub disabled: bool,
    pub loading: bool,
    pub text_alpha: u8,
}

/// Builder for the TextButton composite widget.
#[derive(Debug, Clone)]
pub struct TextButton {
    props: TextButtonProps,
}

impl TextButton {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            props: TextButtonProps {
                label: label.into(),
                variant: Variant::default(),
                tone: Tone::default(),
                size: Size::default(),
                disabled: false,
                loading: false,
            },
        }
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
    pub fn size(mut self, size: Size) -> Self {
        self.props.size = size;
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

    #[must_use]
    pub fn resolve(&self, theme: &Theme) -> ResolvedTextButton {
        let tc = if self.props.disabled {
            theme.color.text_disabled
        } else {
            text_color(self.props.variant, self.props.tone, theme)
        };
        let text_alpha = if self.props.loading {
            loading_text_alpha()
        } else {
            u8::MAX
        };
        let (pad_v, pad_h) = padding(self.props.size, theme);
        ResolvedTextButton {
            label: self.props.label.clone(),
            font_size: font_size(self.props.size, theme),
            text_color: tc,
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
            pad_v,
            pad_h,
            disabled: self.props.disabled,
            loading: self.props.loading,
            text_alpha,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::Theme;

    #[test]
    fn all_variant_tone_size_combos_resolve_without_panic() {
        let theme = Theme::default_light();
        let variants = [
            Variant::Primary,
            Variant::Secondary,
            Variant::Ghost,
            Variant::Link,
        ];
        let tones = [Tone::Neutral, Tone::Accent, Tone::Danger, Tone::Success];
        let sizes = [Size::Sm, Size::Md, Size::Lg];
        for v in variants {
            for t in tones {
                for s in sizes {
                    let _ = TextButton::new("OK")
                        .variant(v)
                        .tone(t)
                        .size(s)
                        .resolve(&theme);
                }
            }
        }
    }

    #[test]
    fn disabled_button_has_no_bg() {
        let theme = Theme::default_light();
        let r = TextButton::new("OK").disabled(true).resolve(&theme);
        assert!(r.bg_color.is_none());
        assert_eq!(r.text_color, theme.color.text_disabled);
    }

    #[test]
    fn loading_button_reduces_text_alpha() {
        let theme = Theme::default_light();
        let r = TextButton::new("Save").loading(true).resolve(&theme);
        assert_eq!(r.text_alpha, 128);
    }
}
