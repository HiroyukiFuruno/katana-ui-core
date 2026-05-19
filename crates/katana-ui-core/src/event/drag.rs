use crate::interaction::drag_and_drop::{DndPoint, DragData, DropAcceptance, DropEffect};
use crate::render_model::UiNodeId;
use serde::{Deserialize, Serialize};

pub const DRAG_CANCEL_REASON_KEYBOARD_ESCAPE: &str = "keyboard_escape";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DragEvent {
    DragStart {
        source: UiNodeId,
        data: DragData,
    },
    DragMove {
        source: UiNodeId,
        position: DndPoint,
    },
    DragEnter {
        target: UiNodeId,
        data: DragData,
    },
    DragLeave {
        target: UiNodeId,
    },
    DragOver {
        target: UiNodeId,
        position: DndPoint,
        acceptance: DropAcceptance,
    },
    Drop {
        target: UiNodeId,
        data: DragData,
        effect: DropEffect,
    },
    DragCancel {
        source: UiNodeId,
        reason: String,
    },
    DragEnd {
        source: UiNodeId,
        committed: bool,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DragEventRouteNode {
    pub node_id: UiNodeId,
    pub disabled: bool,
}

impl DragEventRouteNode {
    #[must_use]
    pub fn new(node_id: UiNodeId, disabled: bool) -> Self {
        Self { node_id, disabled }
    }

    #[must_use]
    pub fn enabled(node_id: UiNodeId) -> Self {
        Self::new(node_id, false)
    }

    #[must_use]
    pub fn disabled(node_id: UiNodeId) -> Self {
        Self::new(node_id, true)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DragEventRouting;

impl DragEventRouting {
    #[must_use]
    pub fn bubble_route(target: UiNodeId, ancestors: Vec<DragEventRouteNode>) -> Vec<UiNodeId> {
        let mut route = vec![target];
        route.extend(enabled_node_ids(ancestors));
        route
    }

    #[must_use]
    pub fn capture_route(nodes: Vec<DragEventRouteNode>) -> Vec<UiNodeId> {
        enabled_node_ids(nodes).collect()
    }
}

fn enabled_node_ids(nodes: Vec<DragEventRouteNode>) -> impl Iterator<Item = UiNodeId> {
    nodes
        .into_iter()
        .filter(|node| !node.disabled)
        .map(|node| node.node_id)
}
