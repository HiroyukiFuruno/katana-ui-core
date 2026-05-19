use serde::{Deserialize, Serialize};

const DEFAULT_CONTEXT_MENU_MIN_WIDTH: u32 = 180;
const DEFAULT_CONTEXT_MENU_MAX_HEIGHT: u32 = 320;
const DEFAULT_SUBMENU_OPEN_DELAY_MS: u16 = 180;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiContextMenuRect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl UiContextMenuRect {
    #[must_use]
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiContextMenuAnchor {
    Pointer { x: i32, y: i32 },
    VirtualRect(UiContextMenuRect),
    NodeId(String),
}

impl Default for UiContextMenuAnchor {
    fn default() -> Self {
        Self::Pointer { x: 0, y: 0 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiContextMenuPlacement {
    BelowStart,
    BelowEnd,
    AboveStart,
    AboveEnd,
    RightStart,
    LeftStart,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiContextMenuItemKind {
    Action,
    Toggle,
    Radio,
    Submenu,
    Section,
    Divider,
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
    pub children: Vec<UiContextMenuItem>,
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
            children: Vec::new(),
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
    pub fn child(mut self, value: UiContextMenuItem) -> Self {
        self.children.push(value);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiContextMenuProps {
    pub anchor: UiContextMenuAnchor,
    pub placement_priority: Vec<UiContextMenuPlacement>,
    pub placement_used: UiContextMenuPlacement,
    pub min_width: u32,
    pub max_height: u32,
    pub submenu_open_delay_ms: u16,
    pub highlighted_path: Vec<usize>,
    pub focus_return_target: String,
    pub items: Vec<UiContextMenuItem>,
}

impl Default for UiContextMenuProps {
    fn default() -> Self {
        Self {
            anchor: UiContextMenuAnchor::default(),
            placement_priority: vec![
                UiContextMenuPlacement::BelowStart,
                UiContextMenuPlacement::BelowEnd,
                UiContextMenuPlacement::AboveStart,
                UiContextMenuPlacement::AboveEnd,
                UiContextMenuPlacement::RightStart,
                UiContextMenuPlacement::LeftStart,
            ],
            placement_used: UiContextMenuPlacement::BelowStart,
            min_width: DEFAULT_CONTEXT_MENU_MIN_WIDTH,
            max_height: DEFAULT_CONTEXT_MENU_MAX_HEIGHT,
            submenu_open_delay_ms: DEFAULT_SUBMENU_OPEN_DELAY_MS,
            highlighted_path: Vec::new(),
            focus_return_target: String::new(),
            items: Vec::new(),
        }
    }
}
