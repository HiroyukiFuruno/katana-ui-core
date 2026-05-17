use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChoiceItem {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

impl ChoiceItem {
    #[must_use]
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }

    #[must_use]
    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = value;
        self
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct SelectionTypedModel {
    pub input_value: String,
    pub filter_results: Vec<ChoiceItem>,
    pub free_input: bool,
    pub selected_option: Option<ChoiceItem>,
    pub keyboard_navigation_summary: String,
    pub framed: bool,
    pub trigger_summary: String,
    pub select_action: String,
    pub crumb_action: String,
    pub icon_action: String,
    pub hover_expansion: bool,
    pub section: String,
    pub marker: String,
    pub more_row: bool,
}
