mod types;
mod view;

pub use types::SearchBoxProps;

use crate::composite::input::text::{InputSize, TextInput, TrailingSlot};
use crate::primitive::icon::IconSource;
use crate::theme::Theme;

const SEARCH_ICON: &[u8] = b"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 16 16\"><circle cx=\"6\" cy=\"6\" r=\"4\" stroke=\"currentColor\" stroke-width=\"1.5\" fill=\"none\"/><line x1=\"9.5\" y1=\"9.5\" x2=\"13\" y2=\"13\" stroke=\"currentColor\" stroke-width=\"1.5\"/></svg>";

/// Resolved visual properties for `SearchBox`.
pub type ResolvedSearchBox = crate::composite::input::text::ResolvedTextInput;

/// Builder for the SearchBox composite widget.
#[derive(Debug, Clone)]
pub struct SearchBox {
    props: SearchBoxProps,
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
                a11y_label: a11y_label.into(),
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
    pub fn resolve(&self, theme: &Theme) -> ResolvedSearchBox {
        let trailing = if view::show_clear(&self.props.value, self.props.disabled) {
            TrailingSlot::ClearButton
        } else {
            TrailingSlot::None
        };

        let mut builder = TextInput::new(self.props.a11y_label.clone())
            .value(self.props.value.clone())
            .leading_icon(IconSource::SvgBytes(SEARCH_ICON))
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
        let r = SearchBox::new("Search").value("query").resolve(&theme);
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
}
