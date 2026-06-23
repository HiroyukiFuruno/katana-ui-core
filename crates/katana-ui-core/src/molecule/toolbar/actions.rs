use super::identifiers::{ToolbarActionId, ToolbarGroupId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolbarInteractionAction {
    Press { action_id: ToolbarActionId },
    Activate { action_id: ToolbarActionId },
    OpenOverflow,
    OpenSplitDropdown { action_id: ToolbarActionId },
    ToggleGroupCollapse { group_id: ToolbarGroupId },
}

impl ToolbarInteractionAction {
    #[must_use]
    pub fn press(action_id: impl Into<ToolbarActionId>) -> Self {
        Self::Press {
            action_id: action_id.into(),
        }
    }

    #[must_use]
    pub fn activate(action_id: impl Into<ToolbarActionId>) -> Self {
        Self::Activate {
            action_id: action_id.into(),
        }
    }

    #[must_use]
    pub fn open_split_dropdown(action_id: impl Into<ToolbarActionId>) -> Self {
        Self::OpenSplitDropdown {
            action_id: action_id.into(),
        }
    }

    #[must_use]
    pub fn toggle_group_collapse(group_id: impl Into<ToolbarGroupId>) -> Self {
        Self::ToggleGroupCollapse {
            group_id: group_id.into(),
        }
    }
}
