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
    pub resize_mode: UiSplitPaneResizeMode,
}

impl Default for UiSplitPaneProps {
    fn default() -> Self {
        Self {
            axis: UiSplitPaneAxis::Horizontal,
            ratio_percent: 50,
            min_percent: 10,
            max_percent: 90,
            reset_percent: 50,
            handle_width_px: 6,
            resize_mode: UiSplitPaneResizeMode::PointerAndKeyboard,
        }
    }
}
