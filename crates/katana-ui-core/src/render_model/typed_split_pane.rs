use serde::{Deserialize, Serialize};

const DEFAULT_SPLIT_PANE_HANDLE_WIDTH_PX: u8 = 6;
const DEFAULT_SPLIT_PANE_RATIO_PERCENT: u8 = 50;
const DEFAULT_SPLIT_PANE_MIN_PERCENT: u8 = 10;
const DEFAULT_SPLIT_PANE_MAX_PERCENT: u8 = 90;
const DEFAULT_SPLIT_PANE_RESET_PERCENT: u8 = 50;
const DEFAULT_SPLIT_PANE_HIT_TARGET_PX: u8 = 24;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiSplitPaneAxis {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiSplitPaneResizeMode {
    PointerOnly,
    KeyboardOnly,
    PointerAndKeyboard,
    Disabled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiSplitPaneProps {
    pub axis: UiSplitPaneAxis,
    pub ratio_percent: u8,
    pub min_percent: u8,
    pub max_percent: u8,
    pub reset_percent: u8,
    pub handle_width_px: u8,
    pub handle: UiSplitPaneHandleProps,
    pub resize_mode: UiSplitPaneResizeMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiSplitPaneHandleProps {
    pub width_px: u8,
    pub focusable: bool,
    pub hit_target_px: u8,
}

impl Default for UiSplitPaneProps {
    fn default() -> Self {
        let handle_width_px = DEFAULT_SPLIT_PANE_HANDLE_WIDTH_PX;
        Self {
            axis: UiSplitPaneAxis::Horizontal,
            ratio_percent: DEFAULT_SPLIT_PANE_RATIO_PERCENT,
            min_percent: DEFAULT_SPLIT_PANE_MIN_PERCENT,
            max_percent: DEFAULT_SPLIT_PANE_MAX_PERCENT,
            reset_percent: DEFAULT_SPLIT_PANE_RESET_PERCENT,
            handle_width_px,
            handle: UiSplitPaneHandleProps {
                width_px: handle_width_px,
                focusable: true,
                hit_target_px: DEFAULT_SPLIT_PANE_HIT_TARGET_PX,
            },
            resize_mode: UiSplitPaneResizeMode::PointerAndKeyboard,
        }
    }
}
