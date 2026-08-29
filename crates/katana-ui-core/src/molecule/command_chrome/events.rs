use super::{
    CommandChromeDropdownCloseReason, CommandChromeDropdownItemId, CommandChromeDropdownKey,
    CommandChromeDropdownLayout, FloatingCommandToolbarCloseReason, FloatingCommandToolbarLayout,
};
use crate::interaction::placement::PlacementResult;
use crate::molecule::toolbar::{
    KeyCombo, ToolbarActionId, ToolbarFocusState, ToolbarGroupId, ToolbarKeyInput,
    ToolbarKeyboardInput, ToolbarPlacementRequest,
};
use crate::render_model::UiNodeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandChromeToolbarAction {
    Press {
        action_id: ToolbarActionId,
    },
    Activate {
        action_id: ToolbarActionId,
    },
    OpenOverflow,
    OpenSplitDropdown {
        action_id: ToolbarActionId,
    },
    UpdateDropdownLayout {
        action_id: ToolbarActionId,
        layout: CommandChromeDropdownLayout,
    },
    DismissDropdown {
        reason: CommandChromeDropdownCloseReason,
    },
    SelectDropdownItem {
        action_id: ToolbarActionId,
        item_id: CommandChromeDropdownItemId,
    },
    DropdownKeyboard {
        input: CommandChromeDropdownKey,
    },
    ToggleGroupCollapse {
        group_id: ToolbarGroupId,
    },
    TriggerAccelerator {
        input: ToolbarKeyInput,
        focus: ToolbarFocusState,
    },
    Keyboard {
        input: ToolbarKeyboardInput,
    },
}

impl CommandChromeToolbarAction {
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
    pub fn update_dropdown_layout(
        action_id: impl Into<ToolbarActionId>,
        layout: CommandChromeDropdownLayout,
    ) -> Self {
        Self::UpdateDropdownLayout {
            action_id: action_id.into(),
            layout,
        }
    }

    #[must_use]
    pub fn select_dropdown_item(
        action_id: impl Into<ToolbarActionId>,
        item_id: impl Into<CommandChromeDropdownItemId>,
    ) -> Self {
        Self::SelectDropdownItem {
            action_id: action_id.into(),
            item_id: item_id.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CommandChromeToolbarAction;
    use crate::molecule::toolbar::ToolbarActionId;

    #[test]
    fn press_constructor_preserves_the_action_identity() {
        assert_eq!(
            CommandChromeToolbarAction::press("bold"),
            CommandChromeToolbarAction::Press {
                action_id: ToolbarActionId::new("bold"),
            }
        );
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandChromeToolbarEvent {
    CommandActivated {
        action_id: ToolbarActionId,
    },
    OverflowOpened,
    SplitDropdownOpened {
        action_id: ToolbarActionId,
        placement: ToolbarPlacementRequest,
    },
    DropdownOpened {
        action_id: ToolbarActionId,
        placement: PlacementResult,
    },
    DropdownClosed {
        action_id: ToolbarActionId,
        reason: CommandChromeDropdownCloseReason,
    },
    DropdownFocusChanged {
        action_id: ToolbarActionId,
        item_id: CommandChromeDropdownItemId,
    },
    DropdownItemActivated {
        action_id: ToolbarActionId,
        item_id: CommandChromeDropdownItemId,
    },
    AcceleratorTriggered {
        action_id: ToolbarActionId,
        combo: KeyCombo,
    },
    GroupCollapseToggled {
        group_id: ToolbarGroupId,
    },
    FocusChanged {
        action_id: ToolbarActionId,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FloatingCommandToolbarAction {
    Open,
    UpdateLayout {
        layout: FloatingCommandToolbarLayout,
    },
    Dismiss {
        reason: FloatingCommandToolbarCloseReason,
    },
    Toolbar {
        action: CommandChromeToolbarAction,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FloatingCommandToolbarEvent {
    Opened {
        placement: PlacementResult,
    },
    Repositioned {
        placement: PlacementResult,
    },
    Closed {
        reason: FloatingCommandToolbarCloseReason,
    },
    FocusRetained,
    FocusReturnRequested {
        target: UiNodeId,
    },
    Toolbar {
        event: CommandChromeToolbarEvent,
    },
}
