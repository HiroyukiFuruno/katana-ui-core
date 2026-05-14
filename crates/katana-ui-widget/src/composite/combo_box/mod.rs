mod ops;
mod overlay;
mod rows;
mod types;
mod view;

use self::ops::toggle_open;
pub use crate::layout::popover::Placement as ComboBoxPlacement;
pub use types::{ComboBoxOption, ComboBoxProps, ResolvedComboBox, ResolvedComboBoxOption};

use std::rc::Rc;

/// Builder for the ComboBox widget.
#[derive(Clone)]
pub struct ComboBox<K> {
    props: ComboBoxProps<K>,
}

impl<K: Clone + PartialEq + 'static> ComboBox<K> {
    #[must_use]
    pub fn new(options: Vec<ComboBoxOption<K>>, a11y_label: impl Into<String>) -> Self {
        Self {
            props: ComboBoxProps {
                options,
                value: None,
                placeholder: "Search…".into(),
                strict: false,
                disabled: false,
                a11y_label: a11y_label.into(),
                on_select: Rc::new(|_| {}),
                on_input_change: Rc::new(|_| {}),
                is_open: false,
                placement: ComboBoxPlacement::Bottom,
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
    pub fn strict(mut self, strict: bool) -> Self {
        self.props.strict = strict;
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
    pub fn placement(mut self, placement: ComboBoxPlacement) -> Self {
        self.props.placement = placement;
        self
    }

    #[must_use]
    pub fn on_select(mut self, on_select: impl Fn(K) + 'static) -> Self {
        self.props.on_select = Rc::new(on_select);
        self
    }

    #[must_use]
    pub fn on_input_change(mut self, on_input_change: impl Fn(String) + 'static) -> Self {
        self.props.on_input_change = Rc::new(on_input_change);
        self
    }

    #[must_use]
    pub fn resolve(&self) -> ResolvedComboBox<K> {
        self.props.resolve()
    }

    /// Returns whether this component can start opened.
    #[must_use]
    pub fn next_open(&self) -> bool {
        toggle_open(self.props.is_open, self.props.disabled)
    }

    /// Clears selection and returns whether user selection updated.
    pub fn select(&self, value: K) -> Option<(K, bool)> {
        if self.props.disabled {
            return None;
        }

        if self
            .props
            .options
            .iter()
            .any(|option| option.value == value)
        {
            (self.props.on_select)(value.clone());
            Some((value, true))
        } else {
            None
        }
    }
}

impl<K: Clone + PartialEq + 'static> Default for ComboBox<K> {
    fn default() -> Self {
        Self {
            props: ComboBoxProps::noop(),
        }
    }
}
