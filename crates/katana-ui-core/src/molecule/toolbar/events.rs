use super::accelerator::KeyCombo;
use super::identifiers::{ToolbarActionId, ToolbarGroupId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolbarPlacementRequest {
    Menu,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolbarEvent {
    Command {
        action_id: ToolbarActionId,
    },
    OverflowOpened,
    SplitDropdownOpened {
        action_id: ToolbarActionId,
        placement: ToolbarPlacementRequest,
    },
    AcceleratorTriggered {
        action_id: ToolbarActionId,
        combo: KeyCombo,
    },
    GroupCollapseToggled {
        group_id: ToolbarGroupId,
    },
}
