#[cfg(test)]
mod ops;
mod types;
mod view;

pub use types::{SelectBoxProps, SelectSize};

use crate::theme::Theme;
use crate::theme::color::Color;
use view::{border_color, font_size, option_bg, option_text, padding, trigger_bg, trigger_text};

/// Resolved visual properties for a single option row.
#[derive(Debug, Clone)]
pub struct ResolvedOption {
    pub label: String,
    pub bg_color: Color,
    pub text_color: Color,
    pub selected: bool,
}

/// Resolved visual properties for `SelectBox`.
#[derive(Debug, Clone)]
pub struct ResolvedSelectBox {
    pub trigger_label: String,
    pub trigger_bg: Color,
    pub trigger_text: Color,
    pub border_color: Color,
    pub font_size: f32,
    pub pad_v: f32,
    pub pad_h: f32,
    pub is_open: bool,
    pub disabled: bool,
    pub options: Vec<ResolvedOption>,
    pub a11y_label: String,
}

/// Builder for the SelectBox composite widget.
#[derive(Debug, Clone)]
pub struct SelectBox<K> {
    props: SelectBoxProps<K>,
}

impl<K: PartialEq + Clone> SelectBox<K> {
    #[must_use]
    pub fn new(options: Vec<(K, String)>, a11y_label: impl Into<String>) -> Self {
        Self {
            props: SelectBoxProps {
                value: None,
                options,
                placeholder: "Select…".into(),
                size: SelectSize::default(),
                disabled: false,
                is_open: false,
                a11y_label: a11y_label.into(),
            },
        }
    }

    #[must_use]
    pub fn value(mut self, value: K) -> Self {
        self.props.value = Some(value);
        self
    }

    #[must_use]
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.props.placeholder = placeholder.into();
        self
    }

    #[must_use]
    pub fn size(mut self, size: SelectSize) -> Self {
        self.props.size = size;
        self
    }

    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.props.disabled = disabled;
        self
    }

    #[must_use]
    pub fn open(mut self, is_open: bool) -> Self {
        self.props.is_open = is_open;
        self
    }

    #[must_use]
    pub fn resolve(&self, theme: &Theme) -> ResolvedSelectBox {
        let has_value = self.props.value.is_some();
        let trigger_label = self
            .props
            .value
            .as_ref()
            .and_then(|v| {
                self.props
                    .options
                    .iter()
                    .find(|(k, _)| k == v)
                    .map(|(_, lbl)| lbl.clone())
            })
            .unwrap_or_else(|| self.props.placeholder.clone());

        let options = self
            .props
            .options
            .iter()
            .map(|(key, lbl)| {
                let selected = self.props.value.as_ref() == Some(key);
                ResolvedOption {
                    label: lbl.clone(),
                    bg_color: option_bg(selected, theme),
                    text_color: option_text(selected, theme),
                    selected,
                }
            })
            .collect();

        ResolvedSelectBox {
            trigger_label,
            trigger_bg: trigger_bg(self.props.disabled, theme),
            trigger_text: trigger_text(self.props.disabled, has_value, theme),
            border_color: border_color(self.props.is_open, self.props.disabled, theme),
            font_size: font_size(self.props.size),
            pad_v: padding(self.props.size).0,
            pad_h: padding(self.props.size).1,
            is_open: self.props.is_open,
            disabled: self.props.disabled,
            options,
            a11y_label: self.props.a11y_label.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::Theme;

    fn opts() -> Vec<(u8, String)> {
        vec![(1, "One".into()), (2, "Two".into()), (3, "Three".into())]
    }

    #[test]
    fn toggle_open_flips_state() {
        assert!(ops::toggle_open(false, false));
        assert!(!ops::toggle_open(true, false));
    }

    #[test]
    fn toggle_open_disabled_stays_closed() {
        assert!(!ops::toggle_open(false, true));
        assert!(!ops::toggle_open(true, true));
    }

    #[test]
    fn close_on_select_returns_false() {
        assert!(!ops::close_on_select());
    }

    #[test]
    fn placeholder_shown_when_no_value() {
        let theme = Theme::default_light();
        let r = SelectBox::new(opts(), "Choose")
            .placeholder("Pick one")
            .resolve(&theme);
        assert_eq!(r.trigger_label, "Pick one");
    }

    #[test]
    fn selected_label_shown_when_value_set() {
        let theme = Theme::default_light();
        let r = SelectBox::new(opts(), "Choose").value(2u8).resolve(&theme);
        assert_eq!(r.trigger_label, "Two");
    }

    #[test]
    fn open_border_is_accent() {
        let theme = Theme::default_light();
        let r = SelectBox::new(opts(), "Choose").open(true).resolve(&theme);
        assert_eq!(r.border_color, theme.color.accent);
    }

    #[test]
    fn disabled_trigger_text_is_muted() {
        let theme = Theme::default_light();
        let r = SelectBox::new(opts(), "Choose")
            .disabled(true)
            .resolve(&theme);
        assert_eq!(r.trigger_text, theme.color.text_disabled);
    }
}
