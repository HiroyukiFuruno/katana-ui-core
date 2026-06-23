use super::{DndPoint, DndRect, DragSource, DropTarget};
use crate::render_model::UiNodeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyboardDragKey {
    Space,
    Enter,
    Escape,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
}

impl KeyboardDragKey {
    pub(super) const fn starts_or_drops(self) -> bool {
        matches!(self, Self::Space | Self::Enter)
    }

    pub(super) const fn moves_focus(self) -> bool {
        matches!(
            self,
            Self::ArrowUp | Self::ArrowDown | Self::ArrowLeft | Self::ArrowRight
        )
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyboardDropTargetFocus {
    pub target: DropTarget,
    pub rect: DndRect,
    pub position: DndPoint,
}

impl KeyboardDropTargetFocus {
    #[must_use]
    pub fn new(target: DropTarget, rect: DndRect, position: DndPoint) -> Self {
        Self {
            target,
            rect,
            position,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyboardDragContext {
    pub focused_node: UiNodeId,
    pub source: Option<DragSource>,
    pub target: Option<KeyboardDropTargetFocus>,
}

impl KeyboardDragContext {
    #[must_use]
    pub fn empty(focused_node: UiNodeId) -> Self {
        Self {
            focused_node,
            source: None,
            target: None,
        }
    }

    #[must_use]
    pub fn focused_source(source: DragSource) -> Self {
        Self {
            focused_node: source.node_id.clone(),
            source: Some(source),
            target: None,
        }
    }

    #[must_use]
    pub fn focused_target(target: KeyboardDropTargetFocus) -> Self {
        Self {
            focused_node: target.target.node_id.clone(),
            source: None,
            target: Some(target),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyboardDragPhase {
    Idle,
    Dragging,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DragAnnouncement {
    pub message: String,
}
