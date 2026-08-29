use super::{
    CommandChromeContractViolation, CommandChromeDisplayMode, CommandChromeDropdownKey,
    CommandChromeToolbarAction, CommandChromeToolbarEvent,
};
use crate::molecule::toolbar::{
    ToolbarContractViolation, ToolbarEvent, ToolbarInteractionAction, ToolbarKeyboardInput,
};

impl From<CommandChromeDisplayMode> for crate::molecule::toolbar::ToolbarDisplayMode {
    fn from(value: CommandChromeDisplayMode) -> Self {
        match value {
            CommandChromeDisplayMode::IconOnly => Self::IconOnly,
            CommandChromeDisplayMode::IconLeading => Self::IconLeading,
            CommandChromeDisplayMode::IconTrailing => Self::IconTrailing,
            CommandChromeDisplayMode::LabelOnly => Self::LabelOnly,
        }
    }
}

pub(super) fn to_toolbar_action(
    action: CommandChromeToolbarAction,
) -> Option<ToolbarInteractionAction> {
    match action {
        CommandChromeToolbarAction::Press { action_id } => {
            Some(ToolbarInteractionAction::press(action_id))
        }
        CommandChromeToolbarAction::Activate { action_id } => {
            Some(ToolbarInteractionAction::activate(action_id))
        }
        CommandChromeToolbarAction::OpenOverflow => Some(ToolbarInteractionAction::OpenOverflow),
        CommandChromeToolbarAction::OpenSplitDropdown { action_id } => {
            Some(ToolbarInteractionAction::open_split_dropdown(action_id))
        }
        CommandChromeToolbarAction::ToggleGroupCollapse { group_id } => {
            Some(ToolbarInteractionAction::ToggleGroupCollapse { group_id })
        }
        CommandChromeToolbarAction::UpdateDropdownLayout { .. }
        | CommandChromeToolbarAction::DismissDropdown { .. }
        | CommandChromeToolbarAction::SelectDropdownItem { .. }
        | CommandChromeToolbarAction::DropdownKeyboard { .. }
        | CommandChromeToolbarAction::TriggerAccelerator { .. }
        | CommandChromeToolbarAction::Keyboard { .. } => None,
    }
}

pub(super) fn map_toolbar_violation(
    value: ToolbarContractViolation,
) -> Option<CommandChromeContractViolation> {
    match value {
        ToolbarContractViolation::MissingIconOnlyAccessibleName { action_id } => {
            Some(CommandChromeContractViolation::MissingIconOnlyAccessibleName { action_id })
        }
    }
}

pub(super) fn map_toolbar_event(value: &ToolbarEvent) -> Option<CommandChromeToolbarEvent> {
    match value {
        ToolbarEvent::Command { action_id } => Some(CommandChromeToolbarEvent::CommandActivated {
            action_id: action_id.clone(),
        }),
        ToolbarEvent::OverflowOpened => Some(CommandChromeToolbarEvent::OverflowOpened),
        ToolbarEvent::SplitDropdownOpened {
            action_id,
            placement,
        } => Some(CommandChromeToolbarEvent::SplitDropdownOpened {
            action_id: action_id.clone(),
            placement: *placement,
        }),
        ToolbarEvent::AcceleratorTriggered { action_id, combo } => {
            Some(CommandChromeToolbarEvent::AcceleratorTriggered {
                action_id: action_id.clone(),
                combo: combo.clone(),
            })
        }
        ToolbarEvent::GroupCollapseToggled { group_id } => {
            Some(CommandChromeToolbarEvent::GroupCollapseToggled {
                group_id: group_id.clone(),
            })
        }
    }
}

pub(super) fn dropdown_key(input: ToolbarKeyboardInput) -> Option<CommandChromeDropdownKey> {
    match input {
        ToolbarKeyboardInput::ArrowUp => Some(CommandChromeDropdownKey::ArrowUp),
        ToolbarKeyboardInput::ArrowDown => Some(CommandChromeDropdownKey::ArrowDown),
        ToolbarKeyboardInput::Home => Some(CommandChromeDropdownKey::Home),
        ToolbarKeyboardInput::End => Some(CommandChromeDropdownKey::End),
        ToolbarKeyboardInput::Enter => Some(CommandChromeDropdownKey::Enter),
        ToolbarKeyboardInput::Space => Some(CommandChromeDropdownKey::Space),
        ToolbarKeyboardInput::Escape => Some(CommandChromeDropdownKey::Escape),
        ToolbarKeyboardInput::ArrowLeft | ToolbarKeyboardInput::ArrowRight => None,
    }
}
