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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interaction::placement::{Rect, Size};
    use crate::molecule::toolbar::{
        KeyCombo, ToolbarActionId, ToolbarContractViolation, ToolbarEvent, ToolbarFocusState,
        ToolbarGroupId, ToolbarKeyInput, ToolbarKeyboardInput, ToolbarPlacementRequest,
    };

    #[test]
    fn every_display_mode_maps_without_fallback_semantics() {
        assert_eq!(
            crate::molecule::toolbar::ToolbarDisplayMode::IconOnly,
            CommandChromeDisplayMode::IconOnly.into()
        );
        assert_eq!(
            crate::molecule::toolbar::ToolbarDisplayMode::IconLeading,
            CommandChromeDisplayMode::IconLeading.into()
        );
        assert_eq!(
            crate::molecule::toolbar::ToolbarDisplayMode::IconTrailing,
            CommandChromeDisplayMode::IconTrailing.into()
        );
        assert_eq!(
            crate::molecule::toolbar::ToolbarDisplayMode::LabelOnly,
            CommandChromeDisplayMode::LabelOnly.into()
        );
    }

    #[test]
    fn maps_toolbar_actions_to_toolbar_interactions() {
        let action_id: ToolbarActionId = "format".into();
        let group_id: ToolbarGroupId = "group-1".into();

        assert_eq!(
            Some(ToolbarInteractionAction::press(action_id.clone())),
            to_toolbar_action(CommandChromeToolbarAction::Press {
                action_id: action_id.clone()
            })
        );
        assert_eq!(
            Some(ToolbarInteractionAction::activate(action_id.clone())),
            to_toolbar_action(CommandChromeToolbarAction::Activate {
                action_id: action_id.clone()
            })
        );
        assert_eq!(
            Some(ToolbarInteractionAction::OpenOverflow),
            to_toolbar_action(CommandChromeToolbarAction::OpenOverflow)
        );
        assert_eq!(
            Some(ToolbarInteractionAction::open_split_dropdown(
                action_id.clone()
            )),
            to_toolbar_action(CommandChromeToolbarAction::OpenSplitDropdown {
                action_id: action_id.clone()
            })
        );
        assert_eq!(
            Some(ToolbarInteractionAction::ToggleGroupCollapse {
                group_id: group_id.clone()
            }),
            to_toolbar_action(CommandChromeToolbarAction::ToggleGroupCollapse { group_id })
        );
        assert!(
            to_toolbar_action(CommandChromeToolbarAction::UpdateDropdownLayout {
                action_id: action_id.clone(),
                layout: super::super::dropdown_model::CommandChromeDropdownLayout::new(
                    Rect::new(0, 0, 10, 10),
                    Rect::new(0, 0, 20, 20),
                    Size::new(10, 20),
                ),
            })
            .is_none()
        );
        assert!(
            to_toolbar_action(CommandChromeToolbarAction::DismissDropdown {
                reason: crate::molecule::command_chrome::CommandChromeDropdownCloseReason::Escape
            })
            .is_none()
        );
        assert!(
            to_toolbar_action(CommandChromeToolbarAction::SelectDropdownItem {
                action_id,
                item_id: "item".into()
            })
            .is_none()
        );
        assert!(
            to_toolbar_action(CommandChromeToolbarAction::DropdownKeyboard {
                input: CommandChromeDropdownKey::Enter
            })
            .is_none()
        );
    }

    #[test]
    fn maps_toolbar_violations() {
        let action_id: ToolbarActionId = "format".into();
        assert_eq!(
            Some(
                CommandChromeContractViolation::MissingIconOnlyAccessibleName {
                    action_id: action_id.clone()
                }
            ),
            map_toolbar_violation(ToolbarContractViolation::MissingIconOnlyAccessibleName {
                action_id: action_id.clone()
            })
        );
    }

    #[test]
    fn maps_toolbar_events_to_command_chrome_events() {
        let action_id: ToolbarActionId = "copy".into();
        let combo = KeyCombo::command_or_control("c");
        let group_id: ToolbarGroupId = "group-1".into();

        assert_eq!(
            Some(CommandChromeToolbarEvent::CommandActivated {
                action_id: action_id.clone()
            }),
            map_toolbar_event(&ToolbarEvent::Command {
                action_id: action_id.clone()
            })
        );
        assert_eq!(
            Some(CommandChromeToolbarEvent::OverflowOpened),
            map_toolbar_event(&ToolbarEvent::OverflowOpened)
        );
        assert_eq!(
            Some(CommandChromeToolbarEvent::SplitDropdownOpened {
                action_id: action_id.clone(),
                placement: ToolbarPlacementRequest::Menu,
            }),
            map_toolbar_event(&ToolbarEvent::SplitDropdownOpened {
                action_id: action_id.clone(),
                placement: ToolbarPlacementRequest::Menu,
            })
        );
        assert_eq!(
            Some(CommandChromeToolbarEvent::AcceleratorTriggered {
                action_id: action_id.clone(),
                combo: combo.clone(),
            }),
            map_toolbar_event(&ToolbarEvent::AcceleratorTriggered {
                action_id: action_id.clone(),
                combo
            })
        );
        assert_eq!(
            Some(CommandChromeToolbarEvent::GroupCollapseToggled {
                group_id: group_id.clone()
            }),
            map_toolbar_event(&ToolbarEvent::GroupCollapseToggled { group_id })
        );
        assert_eq!(
            Some(CommandChromeDropdownKey::ArrowUp),
            dropdown_key(ToolbarKeyboardInput::ArrowUp)
        );
        assert_eq!(
            Some(CommandChromeDropdownKey::ArrowDown),
            dropdown_key(ToolbarKeyboardInput::ArrowDown)
        );
        assert_eq!(
            Some(CommandChromeDropdownKey::Home),
            dropdown_key(ToolbarKeyboardInput::Home)
        );
        assert_eq!(
            Some(CommandChromeDropdownKey::End),
            dropdown_key(ToolbarKeyboardInput::End)
        );
        assert_eq!(
            Some(CommandChromeDropdownKey::Space),
            dropdown_key(ToolbarKeyboardInput::Space)
        );
        assert_eq!(
            Some(CommandChromeDropdownKey::Enter),
            dropdown_key(ToolbarKeyboardInput::Enter)
        );
        assert_eq!(
            Some(CommandChromeDropdownKey::Escape),
            dropdown_key(ToolbarKeyboardInput::Escape)
        );
        assert_eq!(None, dropdown_key(ToolbarKeyboardInput::ArrowLeft));
        assert_eq!(None, dropdown_key(ToolbarKeyboardInput::ArrowRight));
    }

    #[test]
    fn maps_all_toolbar_action_variants_to_expected_targets() {
        let action_id: ToolbarActionId = "format".into();
        assert!(
            to_toolbar_action(CommandChromeToolbarAction::DismissDropdown {
                reason: crate::molecule::command_chrome::CommandChromeDropdownCloseReason::Escape
            })
            .is_none()
        );
        assert!(
            to_toolbar_action(CommandChromeToolbarAction::SelectDropdownItem {
                action_id: action_id.clone(),
                item_id: "item".into()
            })
            .is_none()
        );
        assert!(
            to_toolbar_action(CommandChromeToolbarAction::DropdownKeyboard {
                input: CommandChromeDropdownKey::Enter
            })
            .is_none()
        );
        assert!(
            to_toolbar_action(CommandChromeToolbarAction::TriggerAccelerator {
                input: ToolbarKeyInput::new("c"),
                focus: ToolbarFocusState::new("test")
            })
            .is_none()
        );
        assert!(
            to_toolbar_action(CommandChromeToolbarAction::Keyboard {
                input: ToolbarKeyboardInput::Enter
            })
            .is_none()
        );
    }
}
