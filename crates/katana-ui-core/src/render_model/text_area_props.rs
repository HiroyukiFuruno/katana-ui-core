use serde::{Deserialize, Serialize};

const DEFAULT_TEXT_AREA_MIN_ROWS: u16 = 2;
const DEFAULT_TEXT_AREA_MAX_ROWS: u16 = 6;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiTextAreaWrapPolicy {
    #[default]
    Soft,
    Hard,
    None,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiTextAreaSubmitKey {
    #[default]
    Enter,
    ModEnter,
    Disabled,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiTextAreaNewlineKey {
    Enter,
    #[default]
    ShiftEnter,
    Disabled,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiTextAreaTabBehavior {
    InsertTab,
    #[default]
    MoveFocus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiTextAreaProps {
    pub min_rows: u16,
    pub max_rows: u16,
    pub auto_grow: bool,
    pub wrap_policy: UiTextAreaWrapPolicy,
    pub submit_key: UiTextAreaSubmitKey,
    pub newline_key: UiTextAreaNewlineKey,
    pub tab_behavior: UiTextAreaTabBehavior,
    pub ime_enabled: bool,
    pub measured_rows: u16,
    pub internal_scroll: bool,
}

impl Default for UiTextAreaProps {
    fn default() -> Self {
        Self {
            min_rows: DEFAULT_TEXT_AREA_MIN_ROWS,
            max_rows: DEFAULT_TEXT_AREA_MAX_ROWS,
            auto_grow: true,
            wrap_policy: UiTextAreaWrapPolicy::Soft,
            submit_key: UiTextAreaSubmitKey::Enter,
            newline_key: UiTextAreaNewlineKey::ShiftEnter,
            tab_behavior: UiTextAreaTabBehavior::MoveFocus,
            ime_enabled: true,
            measured_rows: DEFAULT_TEXT_AREA_MIN_ROWS,
            internal_scroll: false,
        }
    }
}
