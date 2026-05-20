use super::CommandItem;
use super::command_launcher_results::CommandResultRow;
use crate::interaction::VirtualizationConfig;
use crate::molecule::disclosure_foundation::DisclosureTriggerArea;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TreeLineStyle {
    #[default]
    Solid,
    Dotted,
    Dashed,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct StructuredTypedModel {
    pub active_id: String,
    pub line_display: bool,
    pub line_style: TreeLineStyle,
    pub line_width: u8,
    pub icons_visible: bool,
    pub directory_icon: String,
    pub file_icon: String,
    pub font_role: String,
    pub theme_id: String,
    pub empty_area_context_menu: bool,
    pub default_open: bool,
    pub toggle_icon: String,
    pub toggle_trigger_area: DisclosureTriggerArea,
    pub query: String,
    pub filtered_actions: Vec<CommandItem>,
    pub keyboard_action: String,
    pub command_result_rows: Vec<CommandResultRow>,
    pub command_highlighted_index: Option<usize>,
    pub command_virtualization: Option<VirtualizationConfig>,
    pub virtualization: Option<VirtualizationConfig>,
    pub add_action: String,
    pub delete_action: String,
    pub reorder_action: String,
    pub edit_action: String,
    pub empty_state: String,
}
