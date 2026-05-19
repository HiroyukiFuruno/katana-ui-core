use super::accelerator::{KeyCombo, ToolbarKeyInput, matching_action_id};
use super::action_model::ToolbarAction;
use super::actions::ToolbarInteractionAction;
use super::events::{ToolbarEvent, ToolbarPlacementRequest};
use super::identifiers::ToolbarActionId;
use super::options::ToolbarDisplayMode;
use super::overflow::MeasuredToolbarAction;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolbarState {
    active_action: Option<ToolbarActionId>,
    overflow_visible: bool,
    split_open: Option<ToolbarActionId>,
    measured_widths: Vec<MeasuredToolbarAction>,
    display_mode: ToolbarDisplayMode,
}

impl ToolbarState {
    #[must_use]
    pub fn new(display_mode: ToolbarDisplayMode) -> Self {
        Self {
            active_action: None,
            overflow_visible: false,
            split_open: None,
            measured_widths: Vec::new(),
            display_mode,
        }
    }

    #[must_use]
    pub fn with_measured_width(mut self, value: MeasuredToolbarAction) -> Self {
        self.measured_widths.push(value);
        self
    }

    #[must_use]
    pub fn measured_widths(&self) -> &[MeasuredToolbarAction] {
        &self.measured_widths
    }

    #[must_use]
    pub const fn display_mode(&self) -> ToolbarDisplayMode {
        self.display_mode
    }

    #[must_use]
    pub const fn split_open(&self) -> Option<&ToolbarActionId> {
        self.split_open.as_ref()
    }

    #[must_use]
    pub fn set_display_mode(&mut self, value: ToolbarDisplayMode) -> bool {
        if self.display_mode == value {
            return false;
        }
        self.display_mode = value;
        self.measured_widths.clear();
        true
    }

    #[must_use]
    pub fn apply_action(
        &mut self,
        action: &ToolbarInteractionAction,
        actions: &[ToolbarAction],
    ) -> Vec<ToolbarEvent> {
        match action {
            ToolbarInteractionAction::Press { action_id }
            | ToolbarInteractionAction::Activate { action_id } => self.command(action_id, actions),
            ToolbarInteractionAction::OpenOverflow => self.open_overflow(),
            ToolbarInteractionAction::OpenSplitDropdown { action_id } => {
                self.open_split_dropdown(action_id, actions)
            }
            ToolbarInteractionAction::ToggleGroupCollapse { group_id } => {
                vec![ToolbarEvent::GroupCollapseToggled {
                    group_id: group_id.clone(),
                }]
            }
        }
    }

    #[must_use]
    pub fn trigger_accelerator(
        &self,
        actions: &[ToolbarAction],
        input: &ToolbarKeyInput,
        focus: ToolbarFocusState,
    ) -> ToolbarAcceleratorResult {
        let events = matching_action_id(actions, input)
            .map_or_else(Vec::new, |(id, combo)| accelerator_events(id, combo));
        ToolbarAcceleratorResult::new(focus.clone(), focus, events)
    }

    fn command(
        &mut self,
        action_id: &ToolbarActionId,
        actions: &[ToolbarAction],
    ) -> Vec<ToolbarEvent> {
        let Some(action) = actions.iter().find(|it| it.id() == action_id) else {
            return Vec::new();
        };
        if action.split_state().primary_disabled() {
            return Vec::new();
        }
        self.active_action = Some(action_id.clone());
        vec![ToolbarEvent::Command {
            action_id: action_id.clone(),
        }]
    }

    fn open_overflow(&mut self) -> Vec<ToolbarEvent> {
        self.overflow_visible = true;
        vec![ToolbarEvent::OverflowOpened]
    }

    fn open_split_dropdown(
        &mut self,
        action_id: &ToolbarActionId,
        actions: &[ToolbarAction],
    ) -> Vec<ToolbarEvent> {
        let Some(action) = actions.iter().find(|it| it.id() == action_id) else {
            return Vec::new();
        };
        if action.split_state().secondary_disabled() {
            return Vec::new();
        }
        self.split_open = Some(action_id.clone());
        vec![ToolbarEvent::SplitDropdownOpened {
            action_id: action_id.clone(),
            placement: ToolbarPlacementRequest::Menu,
        }]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolbarFocusState(String);

impl ToolbarFocusState {
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolbarAcceleratorResult {
    focus_before: ToolbarFocusState,
    focus_after: ToolbarFocusState,
    events: Vec<ToolbarEvent>,
}

impl ToolbarAcceleratorResult {
    #[must_use]
    pub fn new(
        focus_before: ToolbarFocusState,
        focus_after: ToolbarFocusState,
        events: Vec<ToolbarEvent>,
    ) -> Self {
        Self {
            focus_before,
            focus_after,
            events,
        }
    }

    #[must_use]
    pub const fn focus_before(&self) -> &ToolbarFocusState {
        &self.focus_before
    }

    #[must_use]
    pub const fn focus_after(&self) -> &ToolbarFocusState {
        &self.focus_after
    }

    #[must_use]
    pub fn events(&self) -> &[ToolbarEvent] {
        &self.events
    }
}

fn accelerator_events(action_id: ToolbarActionId, combo: KeyCombo) -> Vec<ToolbarEvent> {
    vec![
        ToolbarEvent::AcceleratorTriggered {
            action_id: action_id.clone(),
            combo,
        },
        ToolbarEvent::Command { action_id },
    ]
}
