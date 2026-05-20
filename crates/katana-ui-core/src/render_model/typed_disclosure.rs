use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiDisclosureIndicatorPosition {
    #[default]
    Trailing,
    Leading,
    None,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiDisclosureTriggerArea {
    IconOnly,
    IconAndText,
    #[default]
    WholeElement,
    TextOnly,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiDisclosureProps {
    pub controlled: bool,
    pub multiple: bool,
    pub indicator_position: UiDisclosureIndicatorPosition,
    pub trigger_area: UiDisclosureTriggerArea,
    pub toggle_icon: String,
    pub tree_mode: bool,
    pub reduced_motion: bool,
    pub body_border: bool,
    pub selected: bool,
    pub depth: u8,
    pub show_lines: bool,
}
