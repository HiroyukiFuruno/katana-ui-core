use crate::render_model::UiNodeId;
use serde::{Deserialize, Serialize};

const DEFAULT_ARROW_SIZE_PX: u16 = 8;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PopoverArrowSpec {
    pub visible: bool,
    pub size_px: u16,
    pub tone: String,
}

impl PopoverArrowSpec {
    #[must_use]
    pub fn new(visible: bool, size_px: u16, tone: impl Into<String>) -> Self {
        Self {
            visible,
            size_px,
            tone: tone.into(),
        }
    }
}

impl Default for PopoverArrowSpec {
    fn default() -> Self {
        Self {
            visible: false,
            size_px: DEFAULT_ARROW_SIZE_PX,
            tone: String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PopoverActionSlot {
    pub node_id: UiNodeId,
    pub label: String,
}

impl PopoverActionSlot {
    #[must_use]
    pub fn new(node_id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            node_id: UiNodeId::new(node_id),
            label: label.into(),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PopoverSlots {
    pub heading: String,
    pub body: String,
    pub footer: String,
    pub actions: Vec<PopoverActionSlot>,
}

impl PopoverSlots {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn heading(mut self, value: impl Into<String>) -> Self {
        self.heading = value.into();
        self
    }

    #[must_use]
    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body = value.into();
        self
    }

    #[must_use]
    pub fn footer(mut self, value: impl Into<String>) -> Self {
        self.footer = value.into();
        self
    }

    #[must_use]
    pub fn action(mut self, value: PopoverActionSlot) -> Self {
        self.actions.push(value);
        self
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PopoverFocusManagement {
    #[default]
    None,
    FirstInteractive,
    NodeId(UiNodeId),
}
