use super::UiHostActionSpec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiTaskMarker {
    Empty,
    Done,
    Progress,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiTaskControlTarget {
    pub node_id: String,
    pub row_index: usize,
    pub state_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiTaskControlAction {
    pub node_id: String,
    pub row_index: usize,
    pub current_marker: UiTaskMarker,
    pub state_id: String,
    pub menu_items: Vec<UiTaskControlMenuItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiTaskControlMenuItem {
    pub item_id: String,
    pub marker: UiTaskMarker,
    pub label: String,
    pub checked: bool,
    pub host_action: Option<UiHostActionSpec>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiTaskControlStateAction {
    pub node_id: String,
    pub row_index: usize,
    pub state_id: String,
    pub marker: UiTaskMarker,
}
