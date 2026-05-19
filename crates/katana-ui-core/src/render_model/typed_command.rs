use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiCommandResultProps {
    pub id: String,
    pub secondary_label: String,
    pub icon: String,
    pub shortcut: String,
    pub provider_id: String,
    pub group_id: String,
    pub disabled_reason: String,
    pub aria_pos_in_set: usize,
    pub aria_set_size: usize,
}
