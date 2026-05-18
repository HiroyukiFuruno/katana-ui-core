use super::UiStateId;
use super::{UiIconProps, UiLoadingProps, UiStatusProps, UiTextEntryProps};
use crate::facade::DEFAULT_FONT_ROLE;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiVisualRole {
    Content,
    Icon,
    Shortcut,
    Control,
    Input,
    Status,
    Separator,
    Loading,
    Progress,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiVariant {
    Plain,
    Filled,
    Text,
    Icon,
    IconText,
    Outline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiTone {
    Neutral,
    Accent,
    Success,
    Warning,
    Danger,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiSize {
    Small,
    Medium,
    Large,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiInteractionState {
    pub open: bool,
    pub has_selection: bool,
    pub selected_index: usize,
    pub item_count: usize,
    pub value: String,
}

impl UiInteractionState {
    #[must_use]
    pub fn summary(&self) -> String {
        format!(
            "open={} selected={} index={} count={} value={}",
            self.open, self.has_selection, self.selected_index, self.item_count, self.value
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiProps {
    pub label: String,
    pub state_id: UiStateId,
    pub disabled: bool,
    pub focusable: bool,
    pub accessibility_label: String,
    pub interaction: UiInteractionState,
    pub theme_id: String,
    pub font_role: String,
    pub style_classes: Vec<String>,
    pub visual_role: UiVisualRole,
    pub variant: UiVariant,
    pub tone: UiTone,
    pub size: UiSize,
    pub loading: bool,
    pub readonly: bool,
    pub invalid: bool,
    pub placeholder: String,
    pub checked: bool,
    pub determinate: bool,
    pub progress_percent: u8,
    pub severity: UiTone,
    pub text_entry: UiTextEntryProps,
    pub status: UiStatusProps,
    pub loading_indicator: UiLoadingProps,
    pub icon: UiIconProps,
}

impl UiProps {
    #[must_use]
    pub fn new(label: impl Into<String>, state_id: UiStateId) -> Self {
        Self {
            label: label.into(),
            state_id,
            disabled: false,
            focusable: false,
            accessibility_label: String::new(),
            interaction: UiInteractionState::default(),
            theme_id: String::new(),
            font_role: DEFAULT_FONT_ROLE.to_string(),
            style_classes: Vec::new(),
            visual_role: UiVisualRole::Content,
            variant: UiVariant::Plain,
            tone: UiTone::Neutral,
            size: UiSize::Medium,
            loading: false,
            readonly: false,
            invalid: false,
            placeholder: String::new(),
            checked: false,
            determinate: false,
            progress_percent: 0,
            severity: UiTone::Neutral,
            text_entry: UiTextEntryProps::default(),
            status: UiStatusProps::default(),
            loading_indicator: UiLoadingProps::default(),
            icon: UiIconProps::default(),
        }
    }
}
