use super::context_menu_item::UiContextMenuItem;
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
    pub render_height: u32,
    pub vertical_scroll_enabled: bool,
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
            render_height: 0,
            vertical_scroll_enabled: false,
            items: Vec::new(),
        }
    }
}
