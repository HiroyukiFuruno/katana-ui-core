use crate::layout::types::{Alignment, Length};
use crate::render_model::{UiInteractionState, UiNode, UiNodeKind, UiRect, UiStateId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScrollAxis {
    Vertical,
    Horizontal,
    Both,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScrollbarVisibility {
    Auto,
    Always,
    Hidden,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScrollbarPlacement {
    Reserved,
    Overlay,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScrollAreaAction {
    ScrollTo { x: u32, y: u32 },
    ScrollBy { dx: i32, dy: i32 },
    ScrollIntoView { target_rect: UiRect },
    SetScrollbarVisibility(ScrollbarVisibility),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScrollAreaEvent {
    Scrolled {
        target: UiStateId,
        x: u32,
        y: u32,
    },
    ScrollEdgeReached {
        target: UiStateId,
        edge: ScrollEdge,
    },
    ScrollCommandRejected {
        target: UiStateId,
        reason: ScrollRejectionReason,
    },
}

impl ScrollAreaEvent {
    #[must_use]
    pub fn target(&self) -> &UiStateId {
        match self {
            Self::Scrolled { target, .. }
            | Self::ScrollEdgeReached { target, .. }
            | Self::ScrollCommandRejected { target, .. } => target,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScrollEdge {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScrollRejectionReason {
    AxisMismatch,
    InvalidExtent,
    NoOverflow,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScrollArea {
    pub(super) state_id: UiStateId,
    pub(super) children: Vec<UiNode>,
    pub(super) interaction: UiInteractionState,
    pub(super) axis: ScrollAxis,
    pub(super) offset_x: u32,
    pub(super) offset_y: u32,
    pub(super) viewport_width: u32,
    pub(super) viewport_height: u32,
    pub(super) content_width: u32,
    pub(super) content_height: u32,
    pub(super) scrollbar_visibility: ScrollbarVisibility,
    pub(super) scrollbar_placement: ScrollbarPlacement,
    pub(super) edge_threshold: u32,
    pub(super) gap: Length,
    pub(super) alignment: Alignment,
}

impl ScrollArea {
    #[must_use]
    pub fn new() -> Self {
        Self {
            state_id: UiStateId::next_for(UiNodeKind::ScrollArea),
            children: Vec::new(),
            interaction: UiInteractionState::default(),
            axis: ScrollAxis::Vertical,
            offset_x: 0,
            offset_y: 0,
            viewport_width: 0,
            viewport_height: 0,
            content_width: 0,
            content_height: 0,
            scrollbar_visibility: ScrollbarVisibility::Auto,
            scrollbar_placement: ScrollbarPlacement::Reserved,
            edge_threshold: 0,
            gap: Length::Px(0.0),
            alignment: Alignment::Start,
        }
    }
}
