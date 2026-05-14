use std::rc::Rc;

use crate::layout::popover::Placement;

/// Label and value for one selectable item.
#[derive(Debug, Clone)]
pub struct ComboBoxOption<K> {
    pub label: String,
    pub value: K,
}

impl<K> ComboBoxOption<K> {
    #[must_use]
    pub fn new(label: impl Into<String>, value: K) -> Self {
        Self {
            label: label.into(),
            value,
        }
    }
}

fn noop_select<K: 'static>() -> Rc<dyn Fn(K)> {
    Rc::new(|_| {})
}

fn noop_input_change() -> Rc<dyn Fn(String)> {
    Rc::new(|_| {})
}

/// Base properties for `ComboBox`.
#[derive(Clone)]
pub struct ComboBoxProps<K> {
    pub options: Vec<ComboBoxOption<K>>,
    pub value: Option<K>,
    pub placeholder: String,
    pub strict: bool,
    pub disabled: bool,
    pub a11y_label: String,
    pub on_select: Rc<dyn Fn(K)>,
    pub on_input_change: Rc<dyn Fn(String)>,
    pub is_open: bool,
    pub placement: Placement,
}

#[derive(Debug, Clone)]
pub struct ResolvedComboBoxOption<K> {
    pub label: String,
    pub value: K,
    pub selected: bool,
}

#[derive(Clone)]
pub struct ResolvedComboBox<K> {
    pub input_value: String,
    pub placeholder: String,
    pub strict: bool,
    pub disabled: bool,
    pub is_open: bool,
    pub options: Vec<ResolvedComboBoxOption<K>>,
    pub a11y_label: String,
    pub on_select: Rc<dyn Fn(K)>,
    pub on_input_change: Rc<dyn Fn(String)>,
    pub placement: Placement,
}

impl<K: Clone + PartialEq + 'static> ComboBoxProps<K> {
    #[must_use]
    pub(crate) fn noop() -> Self {
        Self {
            options: Vec::new(),
            value: None,
            placeholder: "Select…".into(),
            strict: false,
            disabled: false,
            a11y_label: String::new(),
            on_select: noop_select(),
            on_input_change: noop_input_change(),
            is_open: false,
            placement: Placement::Bottom,
        }
    }

    pub(crate) fn label_for_value(&self, value: &Option<K>) -> Option<String> {
        match value {
            None => None,
            Some(target) => self
                .options
                .iter()
                .find(|option| option.value == *target)
                .map(|option| option.label.clone()),
        }
    }

    #[must_use]
    pub(crate) fn resolve(&self) -> ResolvedComboBox<K> {
        let input_value = self.label_for_value(&self.value).unwrap_or_default();
        let options = self
            .options
            .iter()
            .map(|option| {
                let selected = self.value.as_ref() == Some(&option.value);
                ResolvedComboBoxOption {
                    label: option.label.clone(),
                    value: option.value.clone(),
                    selected,
                }
            })
            .collect();

        ResolvedComboBox {
            input_value,
            placeholder: self.placeholder.clone(),
            strict: self.strict,
            disabled: self.disabled,
            is_open: self.is_open,
            options,
            a11y_label: self.a11y_label.clone(),
            on_select: Rc::clone(&self.on_select),
            on_input_change: Rc::clone(&self.on_input_change),
            placement: self.placement,
        }
    }
}
