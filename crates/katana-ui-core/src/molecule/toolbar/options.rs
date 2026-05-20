use super::action_model::ToolbarAction;
use super::group_model::ToolbarGroup;
use super::identifiers::ToolbarActionId;
use crate::molecule::selection::ContextMenuAnchor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolbarStrategy {
    Hide,
    #[default]
    Menu,
    Custom,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolbarDisplayMode {
    IconOnly,
    #[default]
    IconLeading,
    IconTrailing,
    LabelOnly,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolbarDensity {
    Compact,
    #[default]
    Default,
    Spacious,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolbarOptions {
    display_mode: ToolbarDisplayMode,
    density: ToolbarDensity,
    overflow_strategy: ToolbarStrategy,
    actions: Vec<ToolbarAction>,
    groups: Vec<ToolbarGroup>,
    context_menu_anchor: Option<ContextMenuAnchor>,
}

impl ToolbarOptions {
    #[must_use]
    pub fn new() -> Self {
        Self {
            display_mode: ToolbarDisplayMode::default(),
            density: ToolbarDensity::default(),
            overflow_strategy: ToolbarStrategy::default(),
            actions: Vec::new(),
            groups: Vec::new(),
            context_menu_anchor: None,
        }
    }

    #[must_use]
    pub fn display_mode(mut self, value: ToolbarDisplayMode) -> Self {
        self.display_mode = value;
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
    pub fn action(mut self, value: ToolbarAction) -> Self {
        self.actions.push(value);
        self
    }

    #[must_use]
    pub fn group(mut self, value: ToolbarGroup) -> Self {
        self.groups.push(value);
        self
    }

    #[must_use]
    pub fn context_menu_anchor(mut self, value: ContextMenuAnchor) -> Self {
        self.context_menu_anchor = Some(value);
        self
    }

    #[must_use]
    pub const fn context_menu_anchor_model(&self) -> Option<&ContextMenuAnchor> {
        self.context_menu_anchor.as_ref()
    }

    #[must_use]
    pub fn validate(&self) -> Vec<ToolbarContractViolation> {
        if self.display_mode != ToolbarDisplayMode::IconOnly {
            return Vec::new();
        }
        self.actions
            .iter()
            .filter(|action| !action.has_accessible_name())
            .map(
                |action| ToolbarContractViolation::MissingIconOnlyAccessibleName {
                    action_id: action.id().clone(),
                },
            )
            .collect()
    }
}

impl Default for ToolbarOptions {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolbarContractViolation {
    MissingIconOnlyAccessibleName { action_id: ToolbarActionId },
}
