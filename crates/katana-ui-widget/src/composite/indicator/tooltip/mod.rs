mod types;
mod view;

pub use types::{Placement, TooltipProps};

use crate::theme::Theme;
use crate::theme::color::Color;
use view::{
    bg_color, default_delay_ms, default_max_width, effective_placement, font_size, padding,
    text_color,
};

/// Resolved visual properties for `Tooltip`.
#[derive(Debug, Clone)]
pub struct ResolvedTooltip {
    pub label: String,
    pub placement: Placement,
    pub delay_ms: u32,
    pub max_width: f32,
    pub font_size: f32,
    pub pad_v: f32,
    pub pad_h: f32,
    pub bg_color: Color,
    pub text_color: Color,
}

/// Builder for the Tooltip composite widget.
#[derive(Debug, Clone)]
pub struct Tooltip {
    props: TooltipProps,
}

impl Tooltip {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            props: TooltipProps {
                label: label.into(),
                placement: Placement::default(),
                delay_ms: default_delay_ms(),
                max_width: default_max_width(),
            },
        }
    }

    #[must_use]
    pub fn placement(mut self, placement: Placement) -> Self {
        self.props.placement = placement;
        self
    }

    #[must_use]
    pub fn delay_ms(mut self, ms: u32) -> Self {
        self.props.delay_ms = ms;
        self
    }

    #[must_use]
    pub fn max_width(mut self, width: f32) -> Self {
        self.props.max_width = width;
        self
    }

    #[must_use]
    pub fn resolve(&self, theme: &Theme) -> ResolvedTooltip {
        let (pv, ph) = padding();
        ResolvedTooltip {
            label: self.props.label.clone(),
            placement: effective_placement(self.props.placement),
            delay_ms: self.props.delay_ms,
            max_width: self.props.max_width,
            font_size: font_size(),
            pad_v: pv,
            pad_h: ph,
            bg_color: bg_color(theme),
            text_color: text_color(theme),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::Theme;

    #[test]
    fn default_placement_is_top() {
        let theme = Theme::default_light();
        let r = Tooltip::new("Hello").resolve(&theme);
        assert_eq!(r.placement, Placement::Top);
    }

    #[test]
    fn default_delay_is_400ms() {
        let theme = Theme::default_light();
        let r = Tooltip::new("Hello").resolve(&theme);
        assert_eq!(r.delay_ms, 400);
    }

    #[test]
    fn label_preserved() {
        let theme = Theme::default_light();
        let r = Tooltip::new("Tooltip text").resolve(&theme);
        assert_eq!(r.label, "Tooltip text");
    }

    #[test]
    fn custom_placement_bottom() {
        let theme = Theme::default_light();
        let r = Tooltip::new("Hello")
            .placement(Placement::Bottom)
            .resolve(&theme);
        assert_eq!(r.placement, Placement::Bottom);
    }

    #[test]
    fn bg_is_text_color_for_contrast() {
        let theme = Theme::default_light();
        let r = Tooltip::new("Hello").resolve(&theme);
        assert_eq!(r.bg_color, theme.color.text);
        assert_eq!(r.text_color, theme.color.bg);
    }
}
