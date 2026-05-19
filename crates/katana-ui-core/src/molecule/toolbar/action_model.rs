use super::accelerator::KeyCombo;
use super::identifiers::{ToolbarActionId, ToolbarGroupId, ToolbarPriority};
use super::split_model::{SplitAction, ToolbarSplitState};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolbarAction {
    id: ToolbarActionId,
    label: String,
    disabled: bool,
    priority: ToolbarPriority,
    accelerator: Option<KeyCombo>,
    split: Option<SplitAction>,
    group_id: Option<ToolbarGroupId>,
    tooltip: Option<String>,
    accessibility_label: Option<String>,
}

impl ToolbarAction {
    #[must_use]
    pub fn new(id: impl Into<ToolbarActionId>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            disabled: false,
            priority: ToolbarPriority::default(),
            accelerator: None,
            split: None,
            group_id: None,
            tooltip: None,
            accessibility_label: None,
        }
    }

    #[must_use]
    pub const fn id(&self) -> &ToolbarActionId {
        &self.id
    }

    #[must_use]
    pub const fn accelerator_model(&self) -> Option<&KeyCombo> {
        self.accelerator.as_ref()
    }

    #[must_use]
    pub const fn group_id_model(&self) -> Option<&ToolbarGroupId> {
        self.group_id.as_ref()
    }

    #[must_use]
    pub const fn disabled_model(&self) -> bool {
        self.disabled
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
    pub fn split(mut self, value: SplitAction) -> Self {
        self.split = Some(value);
        self
    }

    #[must_use]
    pub fn group_id(mut self, value: impl Into<ToolbarGroupId>) -> Self {
        self.group_id = Some(value.into());
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
    pub fn has_accessible_name(&self) -> bool {
        non_empty(self.accessibility_label.as_deref()) || non_empty(self.tooltip.as_deref())
    }

    #[must_use]
    pub fn split_state(&self) -> ToolbarSplitState {
        let primary_disabled = self.split.as_ref().map_or(self.disabled, |split| {
            self.disabled || split.primary().is_disabled()
        });
        let secondary_disabled = self
            .split
            .as_ref()
            .is_none_or(|split| self.disabled || split.secondary().is_disabled());
        ToolbarSplitState::new(primary_disabled, secondary_disabled)
    }
}

fn non_empty(value: Option<&str>) -> bool {
    value.is_some_and(|it| !it.trim().is_empty())
}
