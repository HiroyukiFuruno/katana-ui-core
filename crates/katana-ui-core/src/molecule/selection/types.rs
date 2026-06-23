use crate::interaction::VirtualizationConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChoiceItem {
    pub value: String,
    pub label: String,
    pub disabled: bool,
    pub pinned: bool,
    pub closeable: bool,
    pub dirty: bool,
    pub group: String,
    pub svg_icon: String,
}

impl ChoiceItem {
    #[must_use]
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
            pinned: false,
            closeable: false,
            dirty: false,
            group: String::new(),
            svg_icon: String::new(),
        }
    }

    #[must_use]
    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = value;
        self
    }

    #[must_use]
    pub fn pinned(mut self, value: bool) -> Self {
        self.pinned = value;
        self
    }

    #[must_use]
    pub fn closeable(mut self, value: bool) -> Self {
        self.closeable = value;
        self
    }

    #[must_use]
    pub fn dirty(mut self, value: bool) -> Self {
        self.dirty = value;
        self
    }

    #[must_use]
    pub fn group(mut self, value: impl Into<String>) -> Self {
        self.group = value.into();
        self
    }

    #[must_use]
    pub fn svg_icon(mut self, value: impl Into<String>) -> Self {
        self.svg_icon = value.into();
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
    pub placement: String,
    pub highlighted_index: usize,
    pub long_list: bool,
    pub outside_click_dismiss: bool,
    pub framed: bool,
    pub trigger_summary: String,
    pub select_action: String,
    pub crumb_action: String,
    pub icon_action: String,
    pub hover_expansion: bool,
    pub section: String,
    pub marker: String,
    pub more_row: bool,
    pub virtualization: Option<VirtualizationConfig>,
}
