mod types;
mod view;

pub use types::{
    ResolvedSearchBox, SearchBox, SearchBoxIconConfig, SearchBoxIconMode, SearchBoxIconPreset,
    SearchBoxIconSlot, SearchBoxProps,
};

use crate::composite::input::text::{InputSize, TextInput, TrailingSlot};
use crate::primitive::icon::{Icon, IconSource};
use crate::theme::Theme;
use floem::reactive::{SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{Decorators, button, container, empty, h_stack, text_input as floem_text_input};
use floem::{IntoView, View};
use std::rc::Rc;

const SEARCH_ICON: &[u8] = b"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 16 16\"><circle cx=\"6\" cy=\"6\" r=\"4\" stroke=\"currentColor\" stroke-width=\"1.5\" fill=\"none\"/><line x1=\"9.5\" y1=\"9.5\" x2=\"13\" y2=\"13\" stroke=\"currentColor\" stroke-width=\"1.5\"/></svg>";
const CLEAR_ICON: &[u8] = b"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 16 16\"><path d=\"M4 4l8 8M12 4l-8 8\" stroke=\"currentColor\" stroke-width=\"1.7\" stroke-linecap=\"round\"/></svg>";
const SUBMIT_ICON: &[u8] = b"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 16 16\"><path d=\"M3 8h9M8.5 4L12.5 8l-4 4\" stroke=\"currentColor\" stroke-width=\"1.7\" stroke-linecap=\"round\" stroke-linejoin=\"round\" fill=\"none\"/></svg>";
const SEARCH_GAP: f32 = crate::floem_view::GAP_XS;
const ICON_RESERVED_SIZE: f32 = 16.0;

fn preset_source(preset: SearchBoxIconPreset) -> IconSource {
    match preset {
        SearchBoxIconPreset::Search => IconSource::SvgBytes(SEARCH_ICON),
        SearchBoxIconPreset::Clear => IconSource::SvgBytes(CLEAR_ICON),
        SearchBoxIconPreset::Submit => IconSource::SvgBytes(SUBMIT_ICON),
    }
}

fn icon_config(preset: SearchBoxIconPreset) -> SearchBoxIconConfig {
    SearchBoxIconConfig {
        mode: SearchBoxIconMode::Hidden,
        source: preset_source(preset),
    }
}

impl SearchBox {
    #[must_use]
    pub fn new(a11y_label: impl Into<String>) -> Self {
        Self {
            props: SearchBoxProps {
                value: String::new(),
                placeholder: Some("Search…".into()),
                size: InputSize::default(),
                disabled: false,
                leading_icon: icon_config(SearchBoxIconPreset::Search),
                clear_icon: icon_config(SearchBoxIconPreset::Clear),
                submit_icon: icon_config(SearchBoxIconPreset::Submit),
                a11y_label: a11y_label.into(),
                on_submit: Rc::new(|_| {}),
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
    pub fn icon_mode(mut self, slot: SearchBoxIconSlot, mode: SearchBoxIconMode) -> Self {
        self.icon_config_mut(slot).mode = mode;
        self
    }

    #[must_use]
    pub fn icon_source(mut self, slot: SearchBoxIconSlot, source: IconSource) -> Self {
        self.icon_config_mut(slot).source = source;
        self
    }

    #[must_use]
    pub fn icon_preset(mut self, slot: SearchBoxIconSlot, preset: SearchBoxIconPreset) -> Self {
        self.icon_config_mut(slot).source = preset_source(preset);
        self
    }

    #[must_use]
    pub fn search_icon(mut self, mode: SearchBoxIconMode) -> Self {
        self.props.leading_icon.mode = mode;
        self
    }

    #[must_use]
    pub fn clear_icon(mut self, mode: SearchBoxIconMode) -> Self {
        self.props.clear_icon.mode = mode;
        self
    }

    #[must_use]
    pub fn submit_icon(mut self, mode: SearchBoxIconMode) -> Self {
        self.props.submit_icon.mode = mode;
        self
    }

    #[must_use]
    pub fn on_submit(mut self, on_submit: impl Fn(String) + 'static) -> Self {
        self.props.on_submit = Rc::new(on_submit);
        self
    }

    fn icon_config_mut(&mut self, slot: SearchBoxIconSlot) -> &mut SearchBoxIconConfig {
        match slot {
            SearchBoxIconSlot::Leading => &mut self.props.leading_icon,
            SearchBoxIconSlot::Clear => &mut self.props.clear_icon,
            SearchBoxIconSlot::Submit => &mut self.props.submit_icon,
        }
    }

    pub fn submit(&self) -> Option<String> {
        if self.props.disabled {
            return None;
        }

        let value = self.props.value.clone();
        (self.props.on_submit)(value.clone());
        Some(value)
    }

    #[must_use]
    pub fn resolve(&self, theme: &Theme) -> ResolvedSearchBox {
        let trailing = if self.props.clear_icon.mode == SearchBoxIconMode::Visible
            && view::show_clear(&self.props.value, self.props.disabled)
        {
            TrailingSlot::ClearButton
        } else {
            TrailingSlot::None
        };

        let mut builder = TextInput::new(self.props.a11y_label.clone())
            .value(self.props.value.clone())
            .trailing(trailing)
            .size(self.props.size)
            .disabled(self.props.disabled);

        if let Some(ph) = &self.props.placeholder {
            builder = builder.placeholder(ph.clone());
        }

        builder.resolve(theme)
    }

    /// Returns new value after Esc key press.
    #[must_use]
    pub fn on_esc(value: &str) -> String {
        view::on_esc(value)
    }

    #[must_use]
    pub fn view(self, theme: Theme) -> impl IntoView {
        let value = create_rw_signal(self.props.value.clone());
        let disabled = self.props.disabled;
        let on_submit = Rc::clone(&self.props.on_submit);
        let placeholder = self.props.placeholder.clone().unwrap_or_default();
        let resolved = self.resolve(&theme);
        let leading_icon = self.props.leading_icon.clone();
        let clear_icon = self.props.clear_icon.clone();
        let submit_icon = self.props.submit_icon.clone();
        let bg = crate::floem_view::FloemColor::from_token(resolved.bg_color);
        let text = crate::floem_view::FloemColor::from_token(resolved.text_color);
        let border = crate::floem_view::FloemColor::from_token(resolved.border_color);

        h_stack((
            icon_slot(leading_icon, &theme),
            floem_text_input(value)
                .placeholder(placeholder)
                .disabled(move || disabled)
                .style(move |style| style.font_size(resolved.font_size).color(text)),
            action_icon_slot(clear_icon, &theme, move || {
                if !disabled {
                    value.set(String::new());
                }
            }),
            action_icon_slot(submit_icon, &theme, move || {
                if !disabled {
                    on_submit(value.try_get_untracked().unwrap_or_default());
                }
            }),
        ))
        .style(move |style| {
            style
                .gap(SEARCH_GAP)
                .items_center()
                .background(bg)
                .border(1.0)
                .border_color(border)
                .padding_vert(resolved.pad_v)
                .padding_horiz(resolved.pad_h)
        })
    }
}

fn icon_slot(config: SearchBoxIconConfig, theme: &Theme) -> Box<dyn View> {
    match config.mode {
        SearchBoxIconMode::Hidden => empty_icon_space(0.0),
        SearchBoxIconMode::Reserved => empty_icon_space(ICON_RESERVED_SIZE),
        SearchBoxIconMode::Visible => Icon::new(config.source).view(theme.clone()).into_any(),
    }
}

fn action_icon_slot(
    config: SearchBoxIconConfig,
    theme: &Theme,
    action: impl Fn() + 'static,
) -> Box<dyn View> {
    match config.mode {
        SearchBoxIconMode::Hidden => empty_icon_space(0.0),
        SearchBoxIconMode::Reserved => empty_icon_space(ICON_RESERVED_SIZE),
        SearchBoxIconMode::Visible => button(Icon::new(config.source).view(theme.clone()))
            .action(action)
            .into_any(),
    }
}

fn empty_icon_space(width: f32) -> Box<dyn View> {
    container(empty())
        .style(move |style| style.width(width).height(ICON_RESERVED_SIZE))
        .into_any()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::Theme;

    #[test]
    fn esc_clears_value() {
        assert_eq!(SearchBox::on_esc("hello"), "");
        assert_eq!(SearchBox::on_esc(""), "");
    }

    #[test]
    fn clear_shown_when_non_empty_and_enabled() {
        let theme = Theme::default_light();
        let r = SearchBox::new("Search")
            .value("query")
            .clear_icon(SearchBoxIconMode::Visible)
            .resolve(&theme);
        assert!(matches!(r.trailing, TrailingSlot::ClearButton));
    }

    #[test]
    fn clear_hidden_when_empty() {
        let theme = Theme::default_light();
        let r = SearchBox::new("Search").resolve(&theme);
        assert!(matches!(r.trailing, TrailingSlot::None));
    }

    #[test]
    fn clear_hidden_when_disabled() {
        let theme = Theme::default_light();
        let r = SearchBox::new("Search")
            .value("query")
            .disabled(true)
            .resolve(&theme);
        assert!(matches!(r.trailing, TrailingSlot::None));
    }

    #[test]
    fn submit_calls_on_submit() {
        let called = std::rc::Rc::new(std::cell::RefCell::new(None));
        let called_ref = std::rc::Rc::clone(&called);
        let search = SearchBox::new("Search")
            .value("query")
            .on_submit(move |value| {
                *called_ref.borrow_mut() = Some(value);
            });

        assert_eq!(search.submit(), Some("query".to_string()));
        assert_eq!(*called.borrow(), Some("query".to_string()));
    }

    #[test]
    fn disabled_submit_does_not_call_on_submit() {
        let called = std::rc::Rc::new(std::cell::RefCell::new(false));
        let called_ref = std::rc::Rc::clone(&called);
        let search = SearchBox::new("Search")
            .value("query")
            .disabled(true)
            .on_submit(move |_| {
                *called_ref.borrow_mut() = true;
            });

        assert_eq!(search.submit(), None);
        assert!(!*called.borrow());
    }
}
