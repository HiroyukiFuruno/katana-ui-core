use crate::event::UiEvent;
use crate::event::drag::{DRAG_CANCEL_REASON_KEYBOARD_ESCAPE, DragEvent};
use crate::interaction::drag_and_drop::{
    DragData, DropEffect, OS_FILE_LIST_TAG, OS_TEXT_TAG, OS_URL_TAG,
};
use crate::render_model::{RenderContext, UiNodeId, UiTree};
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
    pub target: UiNodeId,
    pub input_kind: ImeInputKind,
    pub phase: ImeRequestPhase,
    pub preedit: String,
    pub commit_text: String,
    pub caret: usize,
}

impl ImeRequest {
    #[must_use]
    pub fn multiline(
        target: UiNodeId,
        phase: ImeRequestPhase,
        preedit: impl Into<String>,
        caret: usize,
    ) -> Self {
        Self {
            target,
            input_kind: ImeInputKind::Multiline,
            phase,
            preedit: preedit.into(),
            commit_text: String::new(),
            caret,
        }
    }

    #[must_use]
    pub fn multiline_commit(
        target: UiNodeId,
        commit_text: impl Into<String>,
        caret: usize,
    ) -> Self {
        Self {
            target,
            input_kind: ImeInputKind::Multiline,
            phase: ImeRequestPhase::Commit,
            preedit: String::new(),
            commit_text: commit_text.into(),
            caret,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImeInputKind {
    SingleLine,
    Multiline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImeRequestPhase {
    Start,
    Update,
    Commit,
    Cancel,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DragDropRequest {
    pub payload: String,
}

pub const NATIVE_DND_ESCAPE_HATCH_TAGS: [&str; 3] = [OS_FILE_LIST_TAG, OS_URL_TAG, OS_TEXT_TAG];

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeDragDropBridge;

impl NativeDragDropBridge {
    #[must_use]
    pub fn is_native_tag(tag: &str) -> bool {
        NATIVE_DND_ESCAPE_HATCH_TAGS.contains(&tag)
    }

    #[must_use]
    pub fn drag_start(source: UiNodeId, data: DragData) -> UiEvent {
        UiEvent::Drag(DragEvent::DragStart { source, data })
    }

    #[must_use]
    pub fn drop(target: UiNodeId, data: DragData, effect: DropEffect) -> UiEvent {
        UiEvent::Drag(DragEvent::Drop {
            target,
            data,
            effect,
        })
    }

    #[must_use]
    pub fn cancel(source: UiNodeId) -> Vec<UiEvent> {
        vec![
            UiEvent::Drag(DragEvent::DragCancel {
                source: source.clone(),
                reason: DRAG_CANCEL_REASON_KEYBOARD_ESCAPE.to_string(),
            }),
            UiEvent::Drag(DragEvent::DragEnd {
                source,
                committed: false,
            }),
        ]
    }
}
