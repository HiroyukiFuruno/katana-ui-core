use crate::event::UiEvent;
use crate::render_model::{RenderContext, UiTree};
use serde::{Deserialize, Serialize};

pub trait WidgetAdapter {
    type Output;

    fn render_tree(&self, tree: &UiTree, context: &RenderContext) -> Self::Output;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventSink {
    events: Vec<UiEvent>,
}

impl EventSink {
    #[must_use]
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn emit(&mut self, event: UiEvent) {
        self.events.push(event);
    }

    #[must_use]
    pub fn events(&self) -> &[UiEvent] {
        &self.events
    }
}

impl Default for EventSink {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostHandle {
    id: String,
}

impl HostHandle {
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }

    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdapterExtension {
    PlatformMenu(PlatformMenuRequest),
    Ime(ImeRequest),
    DragDrop(DragDropRequest),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformMenuRequest {
    pub menu_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImeRequest {
    pub composition: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DragDropRequest {
    pub payload: String,
}
