use super::UiNodeId;
use serde::{Deserialize, Serialize};

pub const UI_LINK_OPEN_ACTION_ID: &str = "ui.link.open";
pub const UI_DISCLOSURE_TOGGLE_ACTION_ID: &str = "ui.disclosure.toggle";
pub const UI_IMAGE_HIGHLIGHT_ACTION_ID: &str = "ui.image.highlight";
pub const UI_CODE_COPY_ACTION_ID: &str = "ui.code.copy";
pub const UI_TASK_TOGGLE_ACTION_ID: &str = "ui.task.toggle";
pub const UI_TASK_SET_STATE_ACTION_ID: &str = "ui.task.set_state";
pub const UI_TASK_STATE_ID_PREFIX: &str = "ui-task-state:";
pub const UI_SETTINGS_FIELD_ACTIVATE_ACTION_ID: &str = "ui.settings.field.activate";
pub const UI_SETTINGS_SECTION_TOGGLE_ACTION_ID: &str = "ui.settings.section.toggle";
pub const UI_TREE_ROW_ACTION_ID: &str = "ui.tree.row";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiHostActionKind {
    Command,
    Navigation,
    Disclosure,
    SurfaceControl,
    Custom,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiHostActionSpec {
    pub action_id: String,
    pub label: String,
    pub kind: UiHostActionKind,
    pub enabled: bool,
    pub payload: String,
    #[serde(default)]
    pub typed_payload: UiHostActionPayload,
}

impl UiHostActionSpec {
    #[must_use]
    pub fn new(
        kind: UiHostActionKind,
        action_id: impl Into<String>,
        label: impl Into<String>,
    ) -> Self {
        Self {
            action_id: action_id.into(),
            label: label.into(),
            kind,
            enabled: true,
            payload: String::new(),
            typed_payload: UiHostActionPayload::None,
        }
    }

    #[must_use]
    pub fn command(action_id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(UiHostActionKind::Command, action_id, label)
    }

    #[must_use]
    pub fn surface_control(action_id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(UiHostActionKind::SurfaceControl, action_id, label)
    }

    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }

    #[must_use]
    pub fn payload(mut self, value: impl Into<String>) -> Self {
        self.payload = value.into();
        self
    }

    #[must_use]
    pub fn typed_payload(mut self, value: UiHostActionPayload) -> Self {
        self.typed_payload = value;
        self
    }

    #[must_use]
    pub fn task_control(
        label: impl Into<String>,
        node_id: impl Into<String>,
        row_index: usize,
    ) -> Self {
        Self::command(UI_TASK_TOGGLE_ACTION_ID, label).typed_payload(
            UiHostActionPayload::TaskControl(UiTaskControlActionPayload::new(node_id, row_index)),
        )
    }

    #[must_use]
    pub fn task_control_state(
        label: impl Into<String>,
        node_id: impl Into<String>,
        row_index: usize,
        marker: impl Into<String>,
    ) -> Self {
        Self::command(UI_TASK_SET_STATE_ACTION_ID, label).typed_payload(
            UiHostActionPayload::TaskControlState(UiTaskControlStateActionPayload::new(
                node_id, row_index, marker,
            )),
        )
    }

    #[must_use]
    pub fn settings_field_control(label: impl Into<String>, field_id: impl Into<String>) -> Self {
        Self::new(
            UiHostActionKind::Custom,
            UI_SETTINGS_FIELD_ACTIVATE_ACTION_ID,
            label,
        )
        .typed_payload(UiHostActionPayload::SettingsFieldControl(
            UiSettingsFieldControlActionPayload::new(field_id),
        ))
    }

    #[must_use]
    pub fn settings_section_toggle(
        label: impl Into<String>,
        section_id: impl Into<String>,
    ) -> Self {
        Self::new(
            UiHostActionKind::Disclosure,
            UI_SETTINGS_SECTION_TOGGLE_ACTION_ID,
            label,
        )
        .typed_payload(UiHostActionPayload::SettingsSectionToggle(
            UiSettingsSectionToggleActionPayload::new(section_id),
        ))
    }

    #[must_use]
    pub fn tree_row(
        label: impl Into<String>,
        node_id: impl Into<String>,
        action_kind: UiTreeRowActionKind,
    ) -> Self {
        Self::new(UiHostActionKind::Custom, UI_TREE_ROW_ACTION_ID, label).typed_payload(
            UiHostActionPayload::TreeRow(UiTreeRowActionPayload::new(node_id, action_kind)),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum UiHostActionPayload {
    #[default]
    None,
    SurfaceControl(UiSurfaceControlActionPayload),
    TaskControl(UiTaskControlActionPayload),
    TaskControlState(UiTaskControlStateActionPayload),
    SettingsFieldControl(UiSettingsFieldControlActionPayload),
    SettingsSectionToggle(UiSettingsSectionToggleActionPayload),
    TreeRow(UiTreeRowActionPayload),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiTreeRowActionKind {
    Select,
    Toggle,
    Focus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiTreeRowActionPayload {
    pub node_id: String,
    pub action_kind: UiTreeRowActionKind,
}

impl UiTreeRowActionPayload {
    #[must_use]
    pub fn new(node_id: impl Into<String>, action_kind: UiTreeRowActionKind) -> Self {
        Self {
            node_id: node_id.into(),
            action_kind,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiSurfaceControlActionPayload {
    pub node_id: String,
}

impl UiSurfaceControlActionPayload {
    #[must_use]
    pub fn new(node_id: impl Into<String>) -> Self {
        Self {
            node_id: node_id.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiTaskControlActionPayload {
    pub node_id: String,
    pub row_index: usize,
    pub state_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiTaskControlStateActionPayload {
    pub node_id: String,
    pub row_index: usize,
    pub state_id: String,
    pub marker: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiSettingsFieldControlActionPayload {
    pub field_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiSettingsSectionToggleActionPayload {
    pub section_id: String,
}

impl UiSettingsSectionToggleActionPayload {
    #[must_use]
    pub fn new(section_id: impl Into<String>) -> Self {
        Self {
            section_id: section_id.into(),
        }
    }
}

impl UiSettingsFieldControlActionPayload {
    #[must_use]
    pub fn new(field_id: impl Into<String>) -> Self {
        Self {
            field_id: field_id.into(),
        }
    }
}

impl UiTaskControlActionPayload {
    #[must_use]
    pub fn new(node_id: impl Into<String>, row_index: usize) -> Self {
        let node_id = node_id.into();
        Self {
            state_id: format!("{UI_TASK_STATE_ID_PREFIX}{node_id}:{row_index}"),
            node_id,
            row_index,
        }
    }
}

impl UiTaskControlStateActionPayload {
    #[must_use]
    pub fn new(node_id: impl Into<String>, row_index: usize, marker: impl Into<String>) -> Self {
        let node_id = node_id.into();
        Self {
            state_id: format!("{UI_TASK_STATE_ID_PREFIX}{node_id}:{row_index}"),
            node_id,
            row_index,
            marker: marker.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiHostActionPlan {
    pub target: UiNodeId,
    pub action_id: String,
    pub label: String,
    pub kind: UiHostActionKind,
    pub enabled: bool,
    pub payload: String,
    #[serde(default)]
    pub typed_payload: UiHostActionPayload,
}

impl UiHostActionPlan {
    #[must_use]
    pub fn new(target: UiNodeId, spec: UiHostActionSpec) -> Self {
        Self {
            target,
            action_id: spec.action_id,
            label: spec.label,
            kind: spec.kind,
            enabled: spec.enabled,
            payload: spec.payload,
            typed_payload: spec.typed_payload,
        }
    }
}
