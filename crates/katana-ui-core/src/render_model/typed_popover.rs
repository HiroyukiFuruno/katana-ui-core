use serde::{Deserialize, Serialize};

const DEFAULT_POPOVER_OFFSET_X: i16 = 0;
const DEFAULT_POPOVER_OFFSET_Y: i16 = 0;
const DEFAULT_POPOVER_ARROW_SIZE_PX: u16 = 8;
const DEFAULT_POPOVER_ACTION_COUNT: usize = 0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiPopoverPlacement {
    Top,
    TopStart,
    TopEnd,
    Right,
    RightStart,
    RightEnd,
    Bottom,
    BottomStart,
    BottomEnd,
    Left,
    LeftStart,
    LeftEnd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiPopoverFocusManagement {
    None,
    FirstInteractive,
    NodeId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiPopoverProps {
    pub anchor: String,
    pub placement: UiPopoverPlacement,
    pub offset_x: i16,
    pub offset_y: i16,
    pub width: String,
    pub focus_handling: String,
    pub dismiss_on_outside_click: bool,
    pub dismiss_on_escape: bool,
    pub arrow_visible: bool,
    pub arrow_size_px: u16,
    pub arrow_tone: String,
    pub heading: String,
    pub body: String,
    pub footer: String,
    pub action_count: usize,
    pub focus_management: UiPopoverFocusManagement,
    pub auto_flip_priority: Vec<UiPopoverPlacement>,
}

impl Default for UiPopoverProps {
    fn default() -> Self {
        Self {
            anchor: String::new(),
            placement: UiPopoverPlacement::BottomStart,
            offset_x: DEFAULT_POPOVER_OFFSET_X,
            offset_y: DEFAULT_POPOVER_OFFSET_Y,
            width: String::new(),
            focus_handling: String::new(),
            dismiss_on_outside_click: false,
            dismiss_on_escape: false,
            arrow_visible: false,
            arrow_size_px: DEFAULT_POPOVER_ARROW_SIZE_PX,
            arrow_tone: String::new(),
            heading: String::new(),
            body: String::new(),
            footer: String::new(),
            action_count: DEFAULT_POPOVER_ACTION_COUNT,
            focus_management: UiPopoverFocusManagement::None,
            auto_flip_priority: Vec::new(),
        }
    }
}
