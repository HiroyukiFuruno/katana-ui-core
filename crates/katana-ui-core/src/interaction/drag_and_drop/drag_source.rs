use super::{DragData, DropEffect};
use crate::render_model::UiNodeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DragSource {
    pub node_id: UiNodeId,
    pub payload: DragData,
    pub allowed_effects: Vec<DropEffect>,
    pub keyboard_draggable: bool,
}

impl DragSource {
    #[must_use]
    pub fn new(node_id: UiNodeId, payload: DragData) -> Self {
        Self {
            node_id,
            payload,
            allowed_effects: vec![DropEffect::Move],
            keyboard_draggable: false,
        }
    }

    #[must_use]
    pub fn allowed_effect(mut self, value: DropEffect) -> Self {
        if !self.allowed_effects.contains(&value) {
            self.allowed_effects.push(value);
        }
        self
    }

    #[must_use]
    pub fn keyboard_draggable(mut self, value: bool) -> Self {
        self.keyboard_draggable = value;
        self
    }

    #[must_use]
    pub fn allows_effect(&self, value: DropEffect) -> bool {
        self.allowed_effects.contains(&value)
    }
}
