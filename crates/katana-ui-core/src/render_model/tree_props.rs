use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiTreeNodeKind {
    #[default]
    File,
    Directory,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiTreeNodeProps {
    pub id: String,
    pub label: String,
    pub depth: usize,
    pub kind: UiTreeNodeKind,
    pub expanded: bool,
    pub selected: bool,
    pub active: bool,
}

impl UiTreeNodeProps {
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        label: impl Into<String>,
        depth: usize,
        kind: UiTreeNodeKind,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            depth,
            kind,
            expanded: false,
            selected: false,
            active: false,
        }
    }

    #[must_use]
    pub fn expanded(mut self, value: bool) -> Self {
        self.expanded = value;
        self
    }

    #[must_use]
    pub fn selected(mut self, value: bool) -> Self {
        self.selected = value;
        self
    }

    #[must_use]
    pub fn active(mut self, value: bool) -> Self {
        self.active = value;
        self
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiTreeLineStyle {
    #[default]
    Solid,
    Dotted,
    Dashed,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiTreeToggleTriggerArea {
    IconOnly,
    IconAndText,
    #[default]
    WholeElement,
    TextOnly,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiTreeProps {
    pub active_id: String,
    pub line_display: bool,
    pub line_style: UiTreeLineStyle,
    pub line_width: u8,
    pub icons_visible: bool,
    pub directory_icon: String,
    pub file_icon: String,
    pub font_role: String,
    pub theme_id: String,
    pub empty_area_context_menu: bool,
    pub default_open: bool,
    pub toggle_icon: String,
    pub toggle_trigger_area: UiTreeToggleTriggerArea,
    pub nodes: Vec<UiTreeNodeProps>,
}
