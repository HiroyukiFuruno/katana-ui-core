use super::focus_request::TextSurfaceFocusRequestAcknowledgement;
use super::props::TextSurfacePoint;
use super::scroll_request_types::{
    TextSurfaceScrollRequestAcknowledgement, TextSurfaceScrollRequestRejection,
    TextSurfaceScrollRequestToken,
};
use super::state::TextSurfaceState;
use crate::atom::{TextAreaAction, TextAreaEvent, TextAreaKeyChord, TextAreaValidationError};
use crate::text_selection::UiTextSelectionRange;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextSurfaceClipboardOperation {
    Copy,
    Cut,
    Paste,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextSurfaceHistoryOperation {
    Undo,
    Redo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextSurfaceLayoutAction {
    PointerPress {
        point: TextSurfacePoint,
        extend_selection: bool,
    },
    PointerDrag {
        point: TextSurfacePoint,
    },
    PointerRelease,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextSurfaceAction {
    TextArea(TextAreaAction),
    Key(TextAreaKeyChord),
    SetFocus(bool),
    ClipboardRequest(TextSurfaceClipboardOperation),
    HistoryRequest(TextSurfaceHistoryOperation),
    ScrollBy {
        delta_x: i32,
        delta_y: i32,
    },
    RequestContextTarget {
        selection: UiTextSelectionRange,
    },
    CancelComposition,
    ActivateGutterRow {
        logical_row: usize,
    },
    ActivateGutterMarker {
        logical_row: usize,
        marker_id: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextSurfaceEvent {
    TextArea(TextAreaEvent),
    KeyValidationFailed {
        key: TextAreaKeyChord,
        error: TextAreaValidationError,
    },
    FocusChanged(bool),
    ClipboardRequested {
        operation: TextSurfaceClipboardOperation,
        selection_start: usize,
        selection_end: usize,
    },
    HistoryRequested(TextSurfaceHistoryOperation),
    SelectionChanged {
        selection_start: usize,
        selection_end: usize,
    },
    Scrolled {
        scroll_x: i32,
        scroll_y: i32,
    },
    ScrollRequestAcknowledged(TextSurfaceScrollRequestAcknowledgement),
    ScrollRequestRejected {
        token: TextSurfaceScrollRequestToken,
        reason: TextSurfaceScrollRequestRejection,
    },
    FocusRequestAcknowledged(TextSurfaceFocusRequestAcknowledgement),
    ContextTargetRequested {
        selection: UiTextSelectionRange,
    },
    CompositionCancelled,
    GutterRowActivated {
        logical_row: usize,
    },
    GutterMarkerActivated {
        logical_row: usize,
        marker_id: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextSurfaceActionOutcome {
    pub handled: bool,
    pub events: Vec<TextSurfaceEvent>,
    pub state: TextSurfaceState,
}
