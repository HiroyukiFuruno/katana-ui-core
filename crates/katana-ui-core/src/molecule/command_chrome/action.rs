use super::CommandChromeDropdown;
use crate::molecule::toolbar::{
    KeyCombo, SplitAction, ToolbarAction, ToolbarActionId, ToolbarGroupId, ToolbarPriority,
};
use crate::render_model::UiIconProps;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandChromeDisplayMode {
    IconOnly,
    #[default]
    IconLeading,
    IconTrailing,
    LabelOnly,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandChromeAction {
    id: ToolbarActionId,
    label: String,
    icon: Option<UiIconProps>,
    tooltip: Option<String>,
    accessibility_label: Option<String>,
    disabled: bool,
    priority: ToolbarPriority,
    accelerator: Option<KeyCombo>,
    group_id: Option<ToolbarGroupId>,
    split: Option<SplitAction>,
    dropdown: Option<CommandChromeDropdown>,
}

impl CommandChromeAction {
    #[must_use]
    pub fn new(id: impl Into<ToolbarActionId>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            tooltip: None,
            accessibility_label: None,
            disabled: false,
            priority: ToolbarPriority::default(),
            accelerator: None,
            group_id: None,
            split: None,
            dropdown: None,
        }
    }

    #[must_use]
    pub const fn id(&self) -> &ToolbarActionId {
        &self.id
    }

    #[must_use]
    pub fn label_model(&self) -> &str {
        self.label.as_str()
    }

    #[must_use]
    pub const fn icon_model(&self) -> Option<&UiIconProps> {
        self.icon.as_ref()
    }

    #[must_use]
    pub const fn tooltip_model(&self) -> Option<&String> {
        self.tooltip.as_ref()
    }

    #[must_use]
    pub const fn accessibility_label_model(&self) -> Option<&String> {
        self.accessibility_label.as_ref()
    }

    #[must_use]
    pub const fn split_model(&self) -> Option<&SplitAction> {
        self.split.as_ref()
    }

    #[must_use]
    pub const fn dropdown_model(&self) -> Option<&CommandChromeDropdown> {
        self.dropdown.as_ref()
    }

    #[must_use]
    pub const fn disabled_model(&self) -> bool {
        self.disabled
    }

    #[must_use]
    pub fn icon(mut self, value: UiIconProps) -> Self {
        self.icon = Some(value);
        self
    }

    #[must_use]
    pub fn tooltip(mut self, value: impl Into<String>) -> Self {
        self.tooltip = Some(value.into());
        self
    }

    #[must_use]
    pub fn accessibility_label(mut self, value: impl Into<String>) -> Self {
        self.accessibility_label = Some(value.into());
        self
    }

    #[must_use]
    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = value;
        self
    }

    #[must_use]
    pub fn priority(mut self, value: ToolbarPriority) -> Self {
        self.priority = value;
        self
    }

    #[must_use]
    pub fn accelerator(mut self, value: KeyCombo) -> Self {
        self.accelerator = Some(value);
        self
    }

    #[must_use]
    pub fn group_id(mut self, value: impl Into<ToolbarGroupId>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    #[must_use]
    pub fn split(mut self, value: SplitAction) -> Self {
        self.split = Some(value);
        self
    }

    #[must_use]
    pub fn dropdown(mut self, value: CommandChromeDropdown) -> Self {
        self.dropdown = Some(value);
        self
    }

    #[must_use]
    pub(crate) fn to_toolbar_action(&self) -> ToolbarAction {
        let mut action = ToolbarAction::new(self.id.clone(), self.label.clone())
            .disabled(self.disabled)
            .priority(self.priority);
        if let Some(value) = &self.accelerator {
            action = action.accelerator(value.clone());
        }
        if let Some(value) = &self.group_id {
            action = action.group_id(value.clone());
        }
        if let Some(value) = &self.split {
            action = action.split(value.clone());
        }
        if let Some(value) = &self.tooltip {
            action = action.tooltip(value.clone());
        }
        if let Some(value) = &self.accessibility_label {
            action = action.accessibility_label(value.clone());
        }
        action
    }

    #[must_use]
    pub(crate) fn has_non_empty_icon(&self) -> bool {
        self.icon
            .as_ref()
            .is_some_and(|icon| !icon.svg_source.trim().is_empty())
    }

    #[must_use]
    pub(crate) fn has_accessible_name(&self) -> bool {
        has_non_empty_text(self.accessibility_label.as_deref())
            || has_non_empty_text(self.tooltip.as_deref())
    }

    #[must_use]
    pub(crate) const fn priority_model(&self) -> ToolbarPriority {
        self.priority
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandChromeMeasuredAction {
    action_id: ToolbarActionId,
    width: u32,
}

impl CommandChromeMeasuredAction {
    #[must_use]
    pub fn new(action_id: impl Into<ToolbarActionId>, width: u32) -> Self {
        Self {
            action_id: action_id.into(),
            width,
        }
    }

    #[must_use]
    pub(crate) const fn action_id(&self) -> &ToolbarActionId {
        &self.action_id
    }

    #[must_use]
    pub(crate) const fn width(&self) -> u32 {
        self.width
    }
}

impl Clone for CommandChromeMeasuredAction {
    fn clone(&self) -> Self {
        Self::new(self.action_id.clone(), self.width)
    }
}

fn has_non_empty_text(value: Option<&str>) -> bool {
    value.is_some_and(|text| !text.trim().is_empty())
}
