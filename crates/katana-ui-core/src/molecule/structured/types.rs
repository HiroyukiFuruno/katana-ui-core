use super::CommandItem;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct StructuredTypedModel {
    pub active_id: String,
    pub line_display: bool,
    pub query: String,
    pub filtered_actions: Vec<CommandItem>,
    pub keyboard_action: String,
    pub add_action: String,
    pub delete_action: String,
    pub reorder_action: String,
    pub edit_action: String,
    pub empty_state: String,
}
