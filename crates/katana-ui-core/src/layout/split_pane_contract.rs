use crate::render_model::UiStateId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SplitPaneOptions {
    pub axis: super::SplitPaneAxis,
    pub ratio_percent: u8,
    pub min_percent: u8,
    pub max_percent: u8,
    pub reset_percent: u8,
    pub handle_width_px: u8,
    pub resize_mode: super::SplitPaneResizeMode,
    pub overflow: super::OverflowBehavior,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SplitPaneResizeSource {
    Pointer,
    Keyboard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SplitPaneRejectionReason {
    ResizeDisabled,
    SourceNotAllowed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SplitPaneAction {
    SetRatio(u8),
    ResizeBy {
        delta_percent: i8,
        source: SplitPaneResizeSource,
    },
    ResetRatio,
    StartResize,
    EndResize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SplitPaneEvent {
    RatioChanged {
        target: UiStateId,
        ratio_percent: u8,
        clamped: bool,
        source: SplitPaneResizeSource,
    },
    ResizeStarted {
        target: UiStateId,
    },
    ResizeEnded {
        target: UiStateId,
    },
    ResizeRejected {
        target: UiStateId,
        reason: SplitPaneRejectionReason,
    },
}
