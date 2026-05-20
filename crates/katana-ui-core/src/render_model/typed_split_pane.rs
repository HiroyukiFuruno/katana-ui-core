use serde::{Deserialize, Serialize};

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
        let handle_width_px = 6;
        Self {
            axis: UiSplitPaneAxis::Horizontal,
            ratio_percent: 50,
            min_percent: 10,
            max_percent: 90,
            reset_percent: 50,
            handle_width_px,
            handle: UiSplitPaneHandleProps {
                width_px: handle_width_px,
                focusable: true,
                hit_target_px: 24,
            },
            resize_mode: UiSplitPaneResizeMode::PointerAndKeyboard,
        }
    }
}
