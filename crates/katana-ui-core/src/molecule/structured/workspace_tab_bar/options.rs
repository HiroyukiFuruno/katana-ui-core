use super::identifiers::{WorkspaceTabGroupId, WorkspaceTabId};
use crate::render_model::UiIconProps;
use serde::{Deserialize, Serialize};

const DEFAULT_OVERFLOW_TRIGGER_WIDTH: u16 = 44;
const DEFAULT_COLLAPSED_GROUP_AUTO_EXPAND_MS: u16 = 500;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkspaceTabTone {
    #[default]
    Default,
    Accent,
    Warning,
    Danger,
    Muted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceTabClosePresentation {
    pub visible_label: String,
    pub tooltip: String,
    pub accessibility_label: String,
}

impl WorkspaceTabClosePresentation {
    #[must_use]
    pub fn new(
        visible_label: impl Into<String>,
        tooltip: impl Into<String>,
        accessibility_label: impl Into<String>,
    ) -> Self {
        Self {
            visible_label: visible_label.into(),
            tooltip: tooltip.into(),
            accessibility_label: accessibility_label.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceTab {
    pub id: WorkspaceTabId,
    pub title: String,
    pub icon: Option<UiIconProps>,
    pub dirty: bool,
    pub pinned: bool,
    pub closeable: bool,
    pub groupable: bool,
    pub tone: WorkspaceTabTone,
    pub tooltip: Option<String>,
    pub group_id: Option<WorkspaceTabGroupId>,
    pub accessibility_label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub close_presentation: Option<Box<WorkspaceTabClosePresentation>>,
}

impl WorkspaceTab {
    #[must_use]
    pub fn new(id: impl Into<WorkspaceTabId>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            icon: None,
            dirty: false,
            pinned: false,
            closeable: true,
            groupable: true,
            tone: WorkspaceTabTone::Default,
            tooltip: None,
            group_id: None,
            accessibility_label: None,
            close_presentation: None,
        }
    }

    #[must_use]
    pub fn icon(mut self, value: impl Into<String>) -> Self {
        self.icon = Some(UiIconProps::new(value));
        self
    }

    #[must_use]
    pub fn svg_icon(mut self, value: UiIconProps) -> Self {
        self.icon = Some(value);
        self
    }

    #[must_use]
    pub fn dirty(mut self, value: bool) -> Self {
        self.dirty = value;
        self
    }

    #[must_use]
    pub fn pinned(mut self, value: bool) -> Self {
        self.pinned = value;
        self
    }

    #[must_use]
    pub fn closeable(mut self, value: bool) -> Self {
        self.closeable = value;
        self
    }

    #[must_use]
    pub fn groupable(mut self, value: bool) -> Self {
        self.groupable = value;
        self
    }

    #[must_use]
    pub fn tone(mut self, value: WorkspaceTabTone) -> Self {
        self.tone = value;
        self
    }

    #[must_use]
    pub fn tooltip(mut self, value: impl Into<String>) -> Self {
        self.tooltip = Some(value.into());
        self
    }

    #[must_use]
    pub fn group_id(mut self, value: impl Into<WorkspaceTabGroupId>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    #[must_use]
    pub fn accessibility_label(mut self, value: impl Into<String>) -> Self {
        self.accessibility_label = Some(value.into());
        self
    }

    #[must_use]
    pub fn close_presentation(mut self, value: WorkspaceTabClosePresentation) -> Self {
        self.close_presentation = Some(Box::new(value));
        self
    }

    #[must_use]
    pub fn accessibility_text(&self) -> String {
        self.accessibility_label.clone().unwrap_or_else(|| {
            let dirty = if self.dirty { " dirty" } else { "" };
            let pinned = if self.pinned { " pinned" } else { "" };
            format!("{}{}{}", self.title, dirty, pinned)
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceTabGroup {
    pub id: WorkspaceTabGroupId,
    pub label: String,
    pub color: String,
    #[serde(default)]
    pub parent_group_id: Option<WorkspaceTabGroupId>,
    pub collapsed: bool,
}

impl WorkspaceTabGroup {
    #[must_use]
    pub fn new(id: impl Into<WorkspaceTabGroupId>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            color: String::new(),
            parent_group_id: None,
            collapsed: false,
        }
    }

    #[must_use]
    pub fn color(mut self, value: impl Into<String>) -> Self {
        self.color = value.into();
        self
    }

    #[must_use]
    pub fn parent_group(mut self, parent_group_id: impl Into<WorkspaceTabGroupId>) -> Self {
        self.parent_group_id = Some(parent_group_id.into());
        self
    }

    #[must_use]
    pub fn collapsed(mut self, value: bool) -> Self {
        self.collapsed = value;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceTabBarOptions {
    pub tabs: Vec<WorkspaceTab>,
    pub groups: Vec<WorkspaceTabGroup>,
    pub overflow_trigger_width: u16,
    pub collapsed_group_auto_expand_ms: u16,
}

impl Default for WorkspaceTabBarOptions {
    fn default() -> Self {
        Self {
            tabs: Vec::new(),
            groups: Vec::new(),
            overflow_trigger_width: DEFAULT_OVERFLOW_TRIGGER_WIDTH,
            collapsed_group_auto_expand_ms: DEFAULT_COLLAPSED_GROUP_AUTO_EXPAND_MS,
        }
    }
}
