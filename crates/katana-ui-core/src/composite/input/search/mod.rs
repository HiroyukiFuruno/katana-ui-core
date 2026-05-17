mod types;
mod view;

pub use types::{
    ResolvedSearchBox, SearchBox, SearchBoxControl, SearchBoxControlMode, SearchBoxIconConfig,
    SearchBoxIconMode, SearchBoxIconPreset, SearchBoxIconSlot, SearchBoxOptions, SearchBoxProps,
};

use crate::composite::input::text::{InputSize, TextInput, TrailingSlot};
use crate::primitive::icon::IconSource;
use crate::theme::Theme;
use floem::IntoView;
use std::rc::Rc;

const SEARCH_ICON: &[u8] = b"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 16 16\"><circle cx=\"6\" cy=\"6\" r=\"4\" stroke=\"currentColor\" stroke-width=\"1.5\" fill=\"none\"/><line x1=\"9.5\" y1=\"9.5\" x2=\"13\" y2=\"13\" stroke=\"currentColor\" stroke-width=\"1.5\"/></svg>";
const CLEAR_ICON: &[u8] = b"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 16 16\"><path d=\"M4 4l8 8M12 4l-8 8\" stroke=\"currentColor\" stroke-width=\"1.7\" stroke-linecap=\"round\"/></svg>";
const SUBMIT_ICON: &[u8] = b"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 16 16\"><path d=\"M3 8h9M8.5 4L12.5 8l-4 4\" stroke=\"currentColor\" stroke-width=\"1.7\" stroke-linecap=\"round\" stroke-linejoin=\"round\" fill=\"none\"/></svg>";

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
                options: SearchBoxOptions::default(),
                regex_control: SearchBoxControlMode::Hidden,
                whole_word_control: SearchBoxControlMode::Hidden,
                case_sensitive_control: SearchBoxControlMode::Hidden,
                a11y_label: a11y_label.into(),
                on_submit: Rc::new(|_| {}),
                on_options_change: Rc::new(|_| {}),
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
    pub fn regex(mut self, enabled: bool) -> Self {
        self.props.options.regex = enabled;
        self
    }

    #[must_use]
    pub fn whole_word(mut self, enabled: bool) -> Self {
        self.props.options.whole_word = enabled;
        self
    }

    #[must_use]
    pub fn case_sensitive(mut self, enabled: bool) -> Self {
        self.props.options.case_sensitive = enabled;
        self
    }

    #[must_use]
    pub fn control_mode(mut self, control: SearchBoxControl, mode: SearchBoxControlMode) -> Self {
        match control {
            SearchBoxControl::Regex => self.props.regex_control = mode,
            SearchBoxControl::WholeWord => self.props.whole_word_control = mode,
            SearchBoxControl::CaseSensitive => self.props.case_sensitive_control = mode,
        }
        self
    }

    #[must_use]
    pub fn show_all_controls(mut self) -> Self {
        self.props.regex_control = SearchBoxControlMode::Visible;
        self.props.whole_word_control = SearchBoxControlMode::Visible;
        self.props.case_sensitive_control = SearchBoxControlMode::Visible;
        self
    }

    #[must_use]
    pub fn on_submit(mut self, on_submit: impl Fn(String) + 'static) -> Self {
        self.props.on_submit = Rc::new(on_submit);
        self
    }

    #[must_use]
    pub fn on_options_change(
        mut self,
        on_options_change: impl Fn(SearchBoxOptions) + 'static,
    ) -> Self {
        self.props.on_options_change = Rc::new(on_options_change);
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
        view::build_view(self, theme)
    }
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

    #[test]
    fn katana_search_controls_default_hidden_and_can_be_enabled() {
        let search = SearchBox::new("Search").show_all_controls();

        assert_eq!(search.props.regex_control, SearchBoxControlMode::Visible);
        assert_eq!(
            search.props.whole_word_control,
            SearchBoxControlMode::Visible
        );
        assert_eq!(
            search.props.case_sensitive_control,
            SearchBoxControlMode::Visible
        );
    }

    #[test]
    fn icon_slots_support_visible_reserved_and_custom_source() {
        let search = SearchBox::new("Search")
            .search_icon(SearchBoxIconMode::Visible)
            .clear_icon(SearchBoxIconMode::Reserved)
            .submit_icon(SearchBoxIconMode::Visible)
            .icon_source(
                SearchBoxIconSlot::Leading,
                IconSource::SvgString("<svg />".to_string()),
            );

        assert_eq!(search.props.leading_icon.mode, SearchBoxIconMode::Visible);
        assert_eq!(search.props.clear_icon.mode, SearchBoxIconMode::Reserved);
        assert_eq!(search.props.submit_icon.mode, SearchBoxIconMode::Visible);
        assert!(matches!(
            search.props.leading_icon.source,
            IconSource::SvgString(_)
        ));
    }

    #[test]
    fn search_options_callback_exposes_regex_word_and_case() {
        let called = std::rc::Rc::new(std::cell::RefCell::new(None));
        let called_ref = std::rc::Rc::clone(&called);
        let search = SearchBox::new("Search").on_options_change(move |options| {
            *called_ref.borrow_mut() = Some(options);
        });
        let options = SearchBoxOptions {
            regex: true,
            whole_word: true,
            case_sensitive: true,
        };

        (search.props.on_options_change)(options);

        assert_eq!(*called.borrow(), Some(options));
    }
}
