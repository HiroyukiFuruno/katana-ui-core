use super::CommandChromeFamilyId;
use super::{
    CommandChromeAction, CommandChromeDisplayMode, CommandChromeDropdownLayout,
    CommandChromeOpenDropdown,
};
use crate::molecule::toolbar::{
    ToolbarActionId, ToolbarDensity, ToolbarGroup, ToolbarState, ToolbarStrategy,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandChromeToolbar {
    pub(super) family_id: CommandChromeFamilyId,
    pub(super) actions: Vec<CommandChromeAction>,
    pub(super) groups: Vec<ToolbarGroup>,
    pub(super) display_mode: CommandChromeDisplayMode,
    pub(super) density: ToolbarDensity,
    pub(super) overflow_strategy: ToolbarStrategy,
    pub(super) state: ToolbarState,
    pub(super) focused_index: Option<usize>,
    pub(super) dropdown_layouts: Vec<(ToolbarActionId, CommandChromeDropdownLayout)>,
    pub(super) open_dropdown: Option<CommandChromeOpenDropdown>,
}

/// Consumer-provided command presentation. KUC retains interaction state by opaque action id.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandChromeToolbarPresentation {
    pub actions: Vec<CommandChromeAction>,
    pub groups: Vec<ToolbarGroup>,
    pub display_mode: CommandChromeDisplayMode,
    pub density: ToolbarDensity,
    pub overflow_strategy: ToolbarStrategy,
}

impl CommandChromeToolbar {
    #[must_use]
    pub fn new() -> Self {
        let display_mode = CommandChromeDisplayMode::default();
        Self {
            family_id: CommandChromeFamilyId::default(),
            actions: Vec::new(),
            groups: Vec::new(),
            display_mode,
            density: ToolbarDensity::default(),
            overflow_strategy: ToolbarStrategy::default(),
            state: ToolbarState::new(display_mode.into()),
            focused_index: None,
            dropdown_layouts: Vec::new(),
            open_dropdown: None,
        }
    }

    #[must_use]
    pub fn command_family(mut self, value: CommandChromeFamilyId) -> Self {
        self.family_id = value;
        self
    }

    #[must_use]
    pub const fn command_family_id(&self) -> &CommandChromeFamilyId {
        &self.family_id
    }

    #[must_use]
    pub fn action(mut self, value: CommandChromeAction) -> Self {
        self.actions.push(value);
        self
    }

    #[must_use]
    pub fn group(mut self, value: ToolbarGroup) -> Self {
        self.groups.push(value);
        self
    }

    #[must_use]
    pub fn display_mode(mut self, value: CommandChromeDisplayMode) -> Self {
        self.display_mode = value;
        self.state = ToolbarState::new(value.into());
        self.focused_index = None;
        self
    }

    #[must_use]
    pub fn density(mut self, value: ToolbarDensity) -> Self {
        self.density = value;
        self
    }

    #[must_use]
    pub fn overflow_strategy(mut self, value: ToolbarStrategy) -> Self {
        self.overflow_strategy = value;
        self
    }

    #[must_use]
    pub fn actions(&self) -> &[CommandChromeAction] {
        &self.actions
    }

    #[must_use]
    pub const fn display_mode_model(&self) -> CommandChromeDisplayMode {
        self.display_mode
    }

    #[must_use]
    pub fn focused_action_id_model(&self) -> Option<ToolbarActionId> {
        self.focused_index.and_then(|index| {
            self.valid_actions()
                .get(index)
                .map(|action| action.id().clone())
        })
    }

    #[must_use]
    pub const fn open_dropdown_model(&self) -> Option<&CommandChromeOpenDropdown> {
        self.open_dropdown.as_ref()
    }

    /// Applies external presentation changes without synthesizing command interaction events.
    pub fn synchronize_presentation(&mut self, value: CommandChromeToolbarPresentation) -> bool {
        let changed = self.actions != value.actions
            || self.groups != value.groups
            || self.display_mode != value.display_mode
            || self.density != value.density
            || self.overflow_strategy != value.overflow_strategy;
        if !changed {
            return false;
        }
        let focused_action_id = self.focused_action_id_model();
        self.actions = value.actions;
        self.groups = value.groups;
        self.density = value.density;
        self.overflow_strategy = value.overflow_strategy;
        if self.display_mode != value.display_mode {
            self.display_mode = value.display_mode;
            self.state = ToolbarState::new(value.display_mode.into());
            self.focused_index = None;
        }
        let valid_ids = self
            .valid_actions()
            .into_iter()
            .map(|action| action.id().clone())
            .collect::<Vec<_>>();
        self.focused_index = focused_action_id
            .and_then(|focused| valid_ids.iter().position(|candidate| candidate == &focused));
        if self
            .open_dropdown
            .as_ref()
            .is_some_and(|dropdown| !valid_ids.iter().any(|id| id == dropdown.action_id()))
        {
            self.open_dropdown = None;
        }
        true
    }
}

impl Default for CommandChromeToolbar {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandChromeContractViolation {
    MissingIconOnlyIcon { action_id: ToolbarActionId },
    MissingIconOnlyAccessibleName { action_id: ToolbarActionId },
    EmptyDropdownItems { action_id: ToolbarActionId },
}

impl CommandChromeContractViolation {
    #[must_use]
    pub const fn action_id(&self) -> &ToolbarActionId {
        match self {
            Self::MissingIconOnlyIcon { action_id }
            | Self::MissingIconOnlyAccessibleName { action_id }
            | Self::EmptyDropdownItems { action_id } => action_id,
        }
    }
}
