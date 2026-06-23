use super::host_action_types::UiHostActionSpec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiContextMenuItemKind {
    Action,
    Toggle,
    Radio,
    Submenu,
    Section,
    Divider,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiContextMenuDividerTone {
    Neutral,
    Emphasis,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiContextMenuItem {
    pub id: String,
    pub label: String,
    pub kind: UiContextMenuItemKind,
    pub leading_icon: String,
    pub disabled: bool,
    pub destructive: bool,
    pub checked: bool,
    pub radio_group: String,
    pub shortcut: String,
    pub accessibility_label: String,
    pub divider_tone: UiContextMenuDividerTone,
    pub children: Vec<UiContextMenuItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_action: Option<UiHostActionSpec>,
}

impl UiContextMenuItem {
    #[must_use]
    pub fn action(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(id, label, UiContextMenuItemKind::Action)
    }

    #[must_use]
    pub fn new(
        id: impl Into<String>,
        label: impl Into<String>,
        kind: UiContextMenuItemKind,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            kind,
            leading_icon: String::new(),
            disabled: false,
            destructive: false,
            checked: false,
            radio_group: String::new(),
            shortcut: String::new(),
            accessibility_label: String::new(),
            divider_tone: UiContextMenuDividerTone::Neutral,
            children: Vec::new(),
            host_action: None,
        }
    }

    #[must_use]
    pub fn leading_icon(mut self, value: impl Into<String>) -> Self {
        self.leading_icon = value.into();
        self
    }

    #[must_use]
    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = value;
        self
    }

    #[must_use]
    pub fn destructive(mut self, value: bool) -> Self {
        self.destructive = value;
        self
    }

    #[must_use]
    pub fn checked(mut self, value: bool) -> Self {
        self.checked = value;
        self
    }

    #[must_use]
    pub fn radio_group(mut self, value: impl Into<String>) -> Self {
        self.radio_group = value.into();
        self
    }

    #[must_use]
    pub fn shortcut(mut self, value: impl Into<String>) -> Self {
        self.shortcut = value.into();
        self
    }

    #[must_use]
    pub fn accessibility_label(mut self, value: impl Into<String>) -> Self {
        self.accessibility_label = value.into();
        self
    }

    #[must_use]
    pub fn divider_tone(mut self, value: UiContextMenuDividerTone) -> Self {
        self.divider_tone = value;
        self
    }

    #[must_use]
    pub fn child(mut self, value: UiContextMenuItem) -> Self {
        self.children.push(value);
        self
    }

    #[must_use]
    pub fn host_action(mut self, value: UiHostActionSpec) -> Self {
        self.host_action = Some(value);
        self
    }
}
