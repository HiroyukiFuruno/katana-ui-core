mod types;
mod view;

pub use types::{IconSlotMode, InputSize, TextInputProps, TrailingSlot};

use crate::theme::Theme;
use crate::theme::color::Color;
use std::rc::Rc;
use view::{bg_color, border_color, focus_ring_color, font_size, padding, text_color};

/// Resolved visual properties for `TextInput`.
#[derive(Clone)]
pub struct ResolvedTextInput {
    pub value: String,
    pub placeholder: Option<String>,
    pub has_leading_icon: bool,
    pub trailing: TrailingSlot,
    pub font_size: f32,
    pub pad_v: f32,
    pub pad_h: f32,
    pub bg_color: Color,
    pub text_color: Color,
    pub border_color: Color,
    pub focus_ring_color: Color,
    pub disabled: bool,
    pub readonly: bool,
    pub invalid: bool,
    pub a11y_label: String,
    pub on_change: Rc<dyn Fn(String)>,
}

impl ResolvedTextInput {
    pub fn input(&self, value: impl Into<String>) -> Option<String> {
        if self.disabled || self.readonly {
            return None;
        }

        let next = value.into();
        (self.on_change)(next.clone());
        Some(next)
    }
}

/// Builder for the TextInput composite widget.
#[derive(Clone)]
pub struct TextInput {
    props: TextInputProps,
}

impl TextInput {
    #[must_use]
    pub fn new(a11y_label: impl Into<String>) -> Self {
        Self {
            props: TextInputProps {
                value: String::new(),
                placeholder: None,
                leading_icon: None,
                leading_icon_mode: IconSlotMode::Hidden,
                trailing: TrailingSlot::None,
                size: InputSize::default(),
                disabled: false,
                readonly: false,
                invalid: false,
                a11y_label: a11y_label.into(),
                on_change: Rc::new(|_| {}),
            },
        }
    }

    #[must_use]
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.props.value = value.into();
        self
    }

    #[must_use]
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.props.placeholder = Some(placeholder.into());
        self
    }

    #[must_use]
    pub fn leading_icon(mut self, icon: crate::primitive::icon::IconSource) -> Self {
        self.props.leading_icon = Some(icon);
        self.props.leading_icon_mode = IconSlotMode::Visible;
        self
    }

    #[must_use]
    pub fn leading_icon_mode(mut self, mode: IconSlotMode) -> Self {
        self.props.leading_icon_mode = mode;
        self
    }

    #[must_use]
    pub fn trailing(mut self, trailing: TrailingSlot) -> Self {
        self.props.trailing = trailing;
        self
    }

    #[must_use]
    pub fn size(mut self, size: InputSize) -> Self {
        self.props.size = size;
        self
    }

    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.props.disabled = disabled;
        self
    }

    #[must_use]
    pub fn readonly(mut self, readonly: bool) -> Self {
        self.props.readonly = readonly;
        self
    }

    #[must_use]
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.props.invalid = invalid;
        self
    }

    #[must_use]
    pub fn on_change(mut self, on_change: impl Fn(String) + 'static) -> Self {
        self.props.on_change = Rc::new(on_change);
        self
    }

    #[must_use]
    pub fn resolve(&self, theme: &Theme) -> ResolvedTextInput {
        let has_value = !self.props.value.is_empty();
        ResolvedTextInput {
            value: self.props.value.clone(),
            placeholder: self.props.placeholder.clone(),
            has_leading_icon: self.props.leading_icon.is_some(),
            trailing: self.props.trailing.clone(),
            font_size: font_size(self.props.size),
            pad_v: padding(self.props.size).0,
            pad_h: padding(self.props.size).1,
            bg_color: bg_color(self.props.disabled, theme),
            text_color: text_color(self.props.disabled, has_value, theme),
            border_color: border_color(self.props.invalid, self.props.disabled, theme),
            focus_ring_color: focus_ring_color(self.props.invalid, theme),
            disabled: self.props.disabled,
            readonly: self.props.readonly,
            invalid: self.props.invalid,
            a11y_label: self.props.a11y_label.clone(),
            on_change: Rc::clone(&self.props.on_change),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::Theme;

    #[test]
    fn empty_value_uses_muted_text() {
        let theme = Theme::default_light();
        let r = TextInput::new("Field").resolve(&theme);
        assert_eq!(r.text_color, theme.color.text_muted);
    }

    #[test]
    fn non_empty_value_uses_normal_text() {
        let theme = Theme::default_light();
        let r = TextInput::new("Field").value("hello").resolve(&theme);
        assert_eq!(r.text_color, theme.color.text);
    }

    #[test]
    fn invalid_border_is_danger() {
        let theme = Theme::default_light();
        let r = TextInput::new("Field").invalid(true).resolve(&theme);
        assert_eq!(r.border_color, theme.color.danger);
        assert_eq!(r.focus_ring_color, theme.color.danger);
    }

    #[test]
    fn disabled_bg_is_surface() {
        let theme = Theme::default_light();
        let r = TextInput::new("Field").disabled(true).resolve(&theme);
        assert_eq!(r.bg_color, theme.color.surface);
        assert_eq!(r.text_color, theme.color.text_disabled);
    }

    #[test]
    fn readonly_flag_preserved() {
        let theme = Theme::default_light();
        let r = TextInput::new("Field").readonly(true).resolve(&theme);
        assert!(r.readonly);
    }

    #[test]
    fn input_calls_on_change_when_editable() {
        let called = std::rc::Rc::new(std::cell::RefCell::new(None));
        let called_ref = std::rc::Rc::clone(&called);
        let theme = Theme::default_light();
        let r = TextInput::new("Field")
            .on_change(move |value| {
                *called_ref.borrow_mut() = Some(value);
            })
            .resolve(&theme);

        assert_eq!(r.input("next"), Some("next".to_string()));
        assert_eq!(*called.borrow(), Some("next".to_string()));
    }

    #[test]
    fn readonly_input_does_not_call_on_change() {
        let called = std::rc::Rc::new(std::cell::RefCell::new(false));
        let called_ref = std::rc::Rc::clone(&called);
        let theme = Theme::default_light();
        let r = TextInput::new("Field")
            .readonly(true)
            .on_change(move |_| {
                *called_ref.borrow_mut() = true;
            })
            .resolve(&theme);

        assert_eq!(r.input("next"), None);
        assert!(!*called.borrow());
    }
}
