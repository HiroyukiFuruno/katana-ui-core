mod types;

pub use types::{IconPosition, IconTextButtonProps};

use crate::composite::button::text::{Size, TextButton, Tone, Variant};
use crate::primitive::icon::{Icon, IconSize, IconSource};
use crate::primitive::spinner::{Spinner, SpinnerSize};
use crate::theme::Theme;
use crate::theme::color::Color;
use floem::IntoView;
use floem::views::{Decorators, button, h_stack, label, svg};

/// Resolved visual properties for `IconTextButton`.
#[derive(Debug, Clone)]
pub struct ResolvedIconTextButton {
    pub icon_svg: String,
    pub icon_size_px: f32,
    pub icon_color: Color,
    pub label: String,
    pub font_size: f32,
    pub text_color: Color,
    pub text_alpha: u8,
    pub bg_color: Option<Color>,
    pub hover_bg_color: Color,
    pub icon_position: IconPosition,
    pub gap: f32,
    pub pad_v: f32,
    pub pad_h: f32,
    pub disabled: bool,
    pub loading: bool,
}

/// Builder for the IconTextButton composite widget.
#[derive(Debug, Clone)]
pub struct IconTextButton {
    props: IconTextButtonProps,
}

impl IconTextButton {
    #[must_use]
    pub fn new(icon: IconSource, label: impl Into<String>) -> Self {
        Self {
            props: IconTextButtonProps {
                icon,
                label: label.into(),
                icon_position: IconPosition::default(),
                icon_size: IconSize::Md,
                variant: Variant::default(),
                tone: Tone::default(),
                size: Size::default(),
                disabled: false,
                loading: false,
            },
        }
    }

    #[must_use]
    pub fn icon_position(mut self, pos: IconPosition) -> Self {
        self.props.icon_position = pos;
        self
    }

    #[must_use]
    pub fn icon_size(mut self, size: IconSize) -> Self {
        self.props.icon_size = size;
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
    pub fn resolve(&self, theme: &Theme) -> ResolvedIconTextButton {
        let text_r = TextButton::new(self.props.label.clone())
            .variant(self.props.variant)
            .tone(self.props.tone)
            .size(self.props.size)
            .disabled(self.props.disabled)
            .loading(self.props.loading)
            .resolve(theme);

        let icon_r = Icon::new(self.props.icon.clone())
            .size(self.props.icon_size)
            .resolve(theme);

        let icon_color = if self.props.disabled {
            theme.color.text_disabled
        } else {
            text_r.text_color
        };

        ResolvedIconTextButton {
            icon_svg: icon_r.svg_content,
            icon_size_px: icon_r.size_px,
            icon_color,
            label: text_r.label,
            font_size: text_r.font_size,
            text_color: text_r.text_color,
            text_alpha: text_r.text_alpha,
            bg_color: text_r.bg_color,
            hover_bg_color: text_r.hover_bg_color,
            icon_position: self.props.icon_position,
            gap: theme.spacing.xs,
            pad_v: text_r.pad_v,
            pad_h: text_r.pad_h,
            disabled: self.props.disabled,
            loading: self.props.loading,
        }
    }

    #[must_use]
    pub fn view(self, theme: Theme, mut on_press: impl FnMut() + 'static) -> impl IntoView {
        let resolved = self.resolve(&theme);
        let icon_color = crate::floem_view::FloemColor::from_token(resolved.icon_color);
        let text_color = crate::floem_view::FloemColor::from_token(Color {
            a: resolved.text_alpha,
            ..resolved.text_color
        });
        let bg_color = resolved
            .bg_color
            .map(crate::floem_view::FloemColor::from_token);
        let disabled = resolved.disabled || resolved.loading;
        let icon_view = if resolved.loading {
            Spinner::new()
                .size(SpinnerSize::Pt(resolved.icon_size_px))
                .color_override(resolved.icon_color)
                .view(theme)
                .into_any()
        } else {
            svg(resolved.icon_svg)
                .style(move |style| {
                    style
                        .width(resolved.icon_size_px)
                        .height(resolved.icon_size_px)
                        .color(icon_color)
                })
                .into_any()
        };
        let text_view = label(move || resolved.label.clone())
            .style(move |style| style.font_size(resolved.font_size).color(text_color));
        let content = match resolved.icon_position {
            IconPosition::Leading => h_stack((icon_view, text_view)).into_any(),
            IconPosition::Trailing => h_stack((text_view, icon_view)).into_any(),
        };

        button(content)
            .action(move || {
                if !disabled {
                    on_press();
                }
            })
            .style(move |style| {
                let style = style
                    .gap(resolved.gap)
                    .padding_horiz(resolved.pad_h)
                    .padding_vert(resolved.pad_v);
                if let Some(bg) = bg_color {
                    style.background(bg)
                } else {
                    style
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composite::button::text::{Size, Tone, Variant};
    use crate::primitive::icon::IconSource;
    use crate::theme::Theme;

    const ICON: &[u8] = b"<svg/>";

    #[test]
    fn leading_and_trailing_both_resolve() {
        let theme = Theme::default_light();
        let leading = IconTextButton::new(IconSource::SvgBytes(ICON), "Save")
            .icon_position(IconPosition::Leading)
            .resolve(&theme);
        let trailing = IconTextButton::new(IconSource::SvgBytes(ICON), "Save")
            .icon_position(IconPosition::Trailing)
            .resolve(&theme);
        assert_eq!(leading.icon_position, IconPosition::Leading);
        assert_eq!(trailing.icon_position, IconPosition::Trailing);
    }

    #[test]
    fn disabled_resolves_muted_colors() {
        let theme = Theme::default_light();
        let r = IconTextButton::new(IconSource::SvgBytes(ICON), "Delete")
            .variant(Variant::Primary)
            .tone(Tone::Danger)
            .size(Size::Md)
            .disabled(true)
            .resolve(&theme);
        assert!(r.bg_color.is_none());
        assert_eq!(r.icon_color, theme.color.text_disabled);
    }

    #[test]
    fn loading_reduces_text_alpha() {
        let theme = Theme::default_light();
        let r = IconTextButton::new(IconSource::SvgBytes(ICON), "Loading")
            .loading(true)
            .resolve(&theme);
        assert_eq!(r.text_alpha, 128);
    }
}
