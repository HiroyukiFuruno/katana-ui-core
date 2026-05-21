use crate::layout::{Alignment, Length};
use crate::render_model::{UiInteractionState, UiNode, UiStateId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SplitPaneAxis {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SplitPaneResizeMode {
    PointerOnly,
    KeyboardOnly,
    PointerAndKeyboard,
    Disabled,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SplitPane {
    pub(in crate::layout) state_id: UiStateId,
    pub(in crate::layout) children: Vec<UiNode>,
    pub(super) gap: Length,
    pub(super) alignment: Alignment,
    pub(in crate::layout) interaction: UiInteractionState,
    pub(super) axis: SplitPaneAxis,
    pub(in crate::layout) ratio_percent: u8,
    pub(super) min_percent: u8,
    pub(super) max_percent: u8,
    pub(super) handle_width_px: u8,
    pub(in crate::layout) reset_percent: u8,
    pub(super) resize_mode: SplitPaneResizeMode,
}
