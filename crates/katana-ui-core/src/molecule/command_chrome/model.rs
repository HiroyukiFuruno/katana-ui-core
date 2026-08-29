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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interaction::placement::{Rect, Size};
    use crate::molecule::command_chrome::CommandChromeIcon;
    use crate::molecule::command_chrome::{
        CommandChromeDisplayMode, CommandChromeOpenDropdown, CommandChromeToolbarAction,
        CommandChromeToolbarPresentation,
    };
    use crate::molecule::toolbar::{ToolbarDensity, ToolbarKeyboardInput, ToolbarStrategy};

    #[test]
    fn new_toolbar_defaults_are_stable() {
        let toolbar = CommandChromeToolbar::new();
        assert_eq!(toolbar, CommandChromeToolbar::default());
        assert_eq!(
            CommandChromeDisplayMode::IconLeading,
            toolbar.display_mode_model()
        );
        assert_eq!(
            CommandChromeFamilyId::default(),
            *toolbar.command_family_id()
        );
        assert_eq!(0, toolbar.actions.len());
        assert!(toolbar.focused_index.is_none());
        assert!(toolbar.open_dropdown.is_none());
        assert!(toolbar.groups.is_empty());
        assert_eq!(toolbar.overflow_strategy, ToolbarStrategy::default());
        assert_eq!(toolbar.density, ToolbarDensity::default());
    }

    #[test]
    fn command_family_id_is_mutable_and_display_mode_resets_focus() {
        let icon = CommandChromeIcon::EmphasisStrong.icon_props();
        let mut toolbar = CommandChromeToolbar::new()
            .action(CommandChromeAction::new("one", "One").icon(icon.clone()))
            .action(CommandChromeAction::new("two", "Two").icon(icon));
        let _ = toolbar.apply_action(CommandChromeToolbarAction::Keyboard {
            input: ToolbarKeyboardInput::Home,
        });

        assert_eq!(
            Some("one"),
            toolbar
                .focused_action_id_model()
                .as_ref()
                .map(|value| value.as_str())
        );

        let toolbar = toolbar
            .command_family(CommandChromeFamilyId::new("workspace"))
            .display_mode(CommandChromeDisplayMode::IconOnly);

        assert_eq!("workspace", toolbar.command_family_id().as_str());
        assert!(toolbar.focused_action_id_model().is_none());
    }

    #[test]
    fn synchronize_presentation_reports_changed_and_unchanged_states() {
        let mut toolbar =
            CommandChromeToolbar::new().action(CommandChromeAction::new("one", "One"));
        let same = CommandChromeToolbarPresentation {
            actions: toolbar.actions.clone(),
            groups: toolbar.groups.clone(),
            display_mode: toolbar.display_mode,
            density: toolbar.density,
            overflow_strategy: toolbar.overflow_strategy,
        };
        assert!(!toolbar.synchronize_presentation(same));

        let updated = CommandChromeToolbarPresentation {
            actions: vec![CommandChromeAction::new("one", "Uno")],
            groups: Vec::new(),
            display_mode: CommandChromeDisplayMode::IconLeading,
            density: toolbar.density,
            overflow_strategy: toolbar.overflow_strategy,
        };
        assert!(toolbar.synchronize_presentation(updated));
        assert_eq!("Uno", toolbar.actions[0].label_model());

        toolbar.open_dropdown = Some(CommandChromeOpenDropdown::new(
            "one".into(),
            CommandChromeDropdownLayout::new(
                Rect::new(0, 0, 10, 10),
                Rect::new(0, 0, 100, 100),
                Size::new(50, 40),
            ),
            None,
        ));
        assert!(
            toolbar.synchronize_presentation(CommandChromeToolbarPresentation {
                actions: vec![CommandChromeAction::new("two", "Two")],
                groups: vec![ToolbarGroup::new("editing")],
                display_mode: CommandChromeDisplayMode::IconOnly,
                density: ToolbarDensity::Compact,
                overflow_strategy: ToolbarStrategy::Custom,
            })
        );
        assert!(toolbar.open_dropdown_model().is_none());
        assert!(toolbar.focused_action_id_model().is_none());
        assert_eq!(
            toolbar.display_mode_model(),
            CommandChromeDisplayMode::IconOnly
        );
    }

    #[test]
    fn builder_retains_group_density_and_overflow_strategy() {
        let toolbar = CommandChromeToolbar::new()
            .group(ToolbarGroup::new("editing"))
            .density(ToolbarDensity::Spacious)
            .overflow_strategy(ToolbarStrategy::Custom);
        assert_eq!(toolbar.groups[0].id().as_str(), "editing");
        assert_eq!(toolbar.density, ToolbarDensity::Spacious);
        assert_eq!(toolbar.overflow_strategy, ToolbarStrategy::Custom);
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
