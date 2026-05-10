mod types;
mod view;

pub use types::{AccordionProps, IndicatorPosition};

use crate::theme::Theme;
use crate::theme::color::Color;
use view::{
    animation_ms, border_color, chevron_symbol, header_bg, header_font_size, header_padding,
    header_text,
};

/// Resolved visual properties for `Accordion`.
#[derive(Debug, Clone)]
pub struct ResolvedAccordion {
    pub header: String,
    pub chevron: Option<&'static str>,
    pub indicator: IndicatorPosition,
    pub expanded: bool,
    pub disabled: bool,
    pub header_font_size: f32,
    pub header_pad_v: f32,
    pub header_pad_h: f32,
    pub header_bg: Color,
    pub header_text: Color,
    pub border_color: Color,
    pub animation_ms: u32,
}

/// Builder for the Accordion layout widget.
#[derive(Debug, Clone)]
pub struct Accordion {
    props: AccordionProps,
}

impl Accordion {
    #[must_use]
    pub fn new(header: impl Into<String>) -> Self {
        Self {
            props: AccordionProps {
                header: header.into(),
                expanded: false,
                disabled: false,
                indicator: IndicatorPosition::default(),
            },
        }
    }

    #[must_use]
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.props.expanded = expanded;
        self
    }

    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.props.disabled = disabled;
        self
    }

    #[must_use]
    pub fn indicator(mut self, indicator: IndicatorPosition) -> Self {
        self.props.indicator = indicator;
        self
    }

    #[must_use]
    pub fn resolve(&self, theme: &Theme) -> ResolvedAccordion {
        let (pv, ph) = header_padding();
        ResolvedAccordion {
            header: self.props.header.clone(),
            chevron: chevron_symbol(self.props.expanded, self.props.indicator),
            indicator: self.props.indicator,
            expanded: self.props.expanded,
            disabled: self.props.disabled,
            header_font_size: header_font_size(),
            header_pad_v: pv,
            header_pad_h: ph,
            header_bg: header_bg(self.props.disabled, theme),
            header_text: header_text(self.props.disabled, theme),
            border_color: border_color(theme),
            animation_ms: animation_ms(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::Theme;

    #[test]
    fn collapsed_shows_down_chevron() {
        let theme = Theme::default_light();
        let r = Accordion::new("Section").resolve(&theme);
        assert!(!r.expanded);
        assert_eq!(r.chevron, Some("▼"));
    }

    #[test]
    fn expanded_shows_up_chevron() {
        let theme = Theme::default_light();
        let r = Accordion::new("Section").expanded(true).resolve(&theme);
        assert!(r.expanded);
        assert_eq!(r.chevron, Some("▲"));
    }

    #[test]
    fn indicator_none_hides_chevron() {
        let theme = Theme::default_light();
        let r = Accordion::new("Section")
            .indicator(IndicatorPosition::None)
            .resolve(&theme);
        assert!(r.chevron.is_none());
    }

    #[test]
    fn disabled_uses_muted_text() {
        let theme = Theme::default_light();
        let r = Accordion::new("Section").disabled(true).resolve(&theme);
        assert_eq!(r.header_text, theme.color.text_disabled);
    }
}
