use crate::molecule::disclosure_foundation::DisclosureTriggerArea;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct DisclosureTypedModel {
    pub placement: String,
    pub offset: (i16, i16),
    pub outside_click_dismiss: bool,
    pub escape_dismiss: bool,
    pub anchor_summary: String,
    pub backdrop: String,
    pub focus_return: String,
    pub dismiss_policy: String,
    pub controlled: bool,
    pub multiple: bool,
    pub indicator_position: String,
    pub trigger_area: DisclosureTriggerArea,
    pub toggle_icon: String,
    pub tree_mode: bool,
    pub minimum: i32,
    pub maximum: i32,
    pub step: i32,
    pub binding: String,
}

impl Default for DisclosureTypedModel {
    fn default() -> Self {
        Self {
            placement: String::new(),
            offset: (0, 0),
            outside_click_dismiss: false,
            escape_dismiss: false,
            anchor_summary: String::new(),
            backdrop: String::new(),
            focus_return: String::new(),
            dismiss_policy: String::new(),
            controlled: false,
            multiple: false,
            indicator_position: String::new(),
            trigger_area: DisclosureTriggerArea::default(),
            toggle_icon: String::new(),
            tree_mode: false,
            minimum: 0,
            maximum: 100,
            step: 1,
            binding: String::new(),
        }
    }
}
