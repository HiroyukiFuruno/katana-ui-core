use std::cell::RefCell;
use std::collections::HashSet;

use crate::text_command_surface::accesskit_evidence::AccessKitEvidence;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KucInteractionActionClass {
    TextSurfaceContextTarget,
    TextInput,
    Toolbar,
    FloatingToolbar,
    DropdownTrigger,
    DropdownItem,
    SearchControl,
    ContextMenuItem,
    StatusBarSegment,
    DiagnosticsScope,
    DiagnosticsSeverityFilter,
    DiagnosticsItem,
    DiagnosticsFix,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KucInteractionSelector {
    pub(super) action_identity: String,
    pub(super) action_class: KucInteractionActionClass,
}

impl KucInteractionSelector {
    #[must_use]
    pub fn new(
        action_identity: impl Into<String>,
        action_class: KucInteractionActionClass,
    ) -> Self {
        Self {
            action_identity: action_identity.into(),
            action_class,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KucInteractionLocatorError {
    Missing,
    Disabled,
    Hidden,
    Ambiguous,
    Duplicate,
}

impl std::fmt::Display for KucInteractionLocatorError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::Missing => "interaction action is missing",
            Self::Disabled => "interaction action is disabled",
            Self::Hidden => "interaction action is hidden",
            Self::Ambiguous => "interaction action is ambiguous",
            Self::Duplicate => "interaction action is duplicated",
        })
    }
}

impl std::error::Error for KucInteractionLocatorError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KucInteractionRequestError {
    RootMismatch,
    Stale,
    Duplicate,
    AlreadyQueued,
}

impl std::fmt::Display for KucInteractionRequestError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::RootMismatch => "interaction request belongs to another root",
            Self::Stale => "interaction request is stale",
            Self::Duplicate => "interaction request is duplicated",
            Self::AlreadyQueued => "interaction request is already queued",
        })
    }
}

impl std::error::Error for KucInteractionRequestError {}

/// Errors while advancing a KUC-owned physical text-selection continuation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KucTextSelectionContinuationError {
    Unavailable,
    RootMismatch,
    FrameDiscontinuity,
    NotApplied,
    AlreadyApplied,
    SelectionNotEstablished,
    FloatingNotVisible,
}

impl std::fmt::Display for KucTextSelectionContinuationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::Unavailable => "current root frame has no selectable text area",
            Self::RootMismatch => "text-selection continuation belongs to another root",
            Self::FrameDiscontinuity => "text-selection continuation requires the next root frame",
            Self::NotApplied => "text-selection continuation was not applied",
            Self::AlreadyApplied => "text-selection continuation was already applied",
            Self::SelectionNotEstablished => {
                "text-selection continuation did not establish selection"
            }
            Self::FloatingNotVisible => "text-selection continuation did not open floating output",
        })
    }
}

impl std::error::Error for KucTextSelectionContinuationError {}

/// Errors while advancing a KUC-owned generic search interaction trace.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KucSearchTraceContinuationError {
    Unavailable,
    RootMismatch,
    FrameDiscontinuity,
    NotApplied,
    AlreadyApplied,
    FocusNotEstablished,
    CloseNotApplied,
    Interaction(KucInteractionLocatorError),
    Request(KucInteractionRequestError),
}

impl std::fmt::Display for KucSearchTraceContinuationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unavailable => formatter.write_str("search trace is unavailable"),
            Self::RootMismatch => formatter.write_str("search trace belongs to another root"),
            Self::FrameDiscontinuity => {
                formatter.write_str("search trace requires the next root frame")
            }
            Self::NotApplied => formatter.write_str("search trace step was not applied"),
            Self::AlreadyApplied => formatter.write_str("search trace step was already applied"),
            Self::FocusNotEstablished => {
                formatter.write_str("search query focus was not established")
            }
            Self::CloseNotApplied => {
                formatter.write_str("search close did not hide the retained strip")
            }
            Self::Interaction(error) => write!(formatter, "search target failed: {error}"),
            Self::Request(error) => write!(formatter, "search request failed: {error}"),
        }
    }
}

impl std::error::Error for KucSearchTraceContinuationError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KucOpaqueClickContinuationError {
    RootMismatch,
    FrameDiscontinuity,
    NotApplied,
    AlreadyApplied,
    Interaction(KucInteractionLocatorError),
}

impl std::fmt::Display for KucOpaqueClickContinuationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::RootMismatch => "click continuation belongs to another root",
            Self::FrameDiscontinuity => "click continuation requires the next root frame",
            Self::NotApplied => "click continuation step was not applied",
            Self::AlreadyApplied => "click continuation step was already applied",
            Self::Interaction(error) => return write!(formatter, "click target failed: {error}"),
        })
    }
}

impl std::error::Error for KucOpaqueClickContinuationError {}

/// Opaque one-shot input generated from one current-frame action.
pub struct KucOpaqueInteractionRequest {
    pub(super) root_identity: String,
    pub(super) state_revision: u64,
    pub(super) correlation_fingerprint: String,
    pub(super) events: Vec<egui::Event>,
    pub(super) queued: bool,
}

pub struct KucOpaqueClickContinuation {
    pub(super) root_identity: String,
    pub(super) frame_serial: u64,
    pub(super) selector: KucInteractionSelector,
    pub(super) event: egui::Event,
    pub(super) phase: OpaqueClickPhase,
    pub(super) applied: bool,
}

/// Opaque one-frame step in a KUC-owned physical text-selection trace.
pub struct KucOpaqueTextSelectionContinuation {
    pub(super) root_identity: String,
    pub(super) frame_serial: u64,
    pub(super) geometry: TextSelectionGeometry,
    pub(super) phase: TextSelectionPhase,
    pub(super) applied: bool,
}

/// Opaque one-frame step in a KUC-owned generic search interaction trace.
pub struct KucOpaqueSearchTraceContinuation {
    pub(super) root_identity: String,
    pub(super) frame_serial: u64,
    pub(super) phase: SearchTracePhase,
    pub(super) applied: bool,
}

impl std::fmt::Debug for KucOpaqueTextSelectionContinuation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("KucOpaqueTextSelectionContinuation(..)")
    }
}

impl std::fmt::Debug for KucOpaqueSearchTraceContinuation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("KucOpaqueSearchTraceContinuation(..)")
    }
}

impl std::fmt::Debug for KucOpaqueInteractionRequest {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("KucOpaqueInteractionRequest(..)")
    }
}

impl std::fmt::Debug for KucOpaqueClickContinuation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("KucOpaqueClickContinuation(..)")
    }
}

/// Current-frame locator. Geometry and egui identity resolution stay inside KUC.
pub struct KucInteractionLocator {
    pub(super) root_identity: String,
    pub(super) state_revision: u64,
    pub(super) frame_serial: u64,
    pub(super) correlation_fingerprint: String,
    pub(super) targets: Vec<LocatorTarget>,
    pub(super) ambiguous_bounds: Vec<katana_ui_core::render_model::UiRect>,
    pub(super) hidden: HashSet<(String, KucInteractionActionClass)>,
    pub(super) requested: RefCell<HashSet<(String, KucInteractionActionClass)>>,
    pub(super) selection_geometry: Option<TextSelectionGeometry>,
    pub(super) selection_established: bool,
    pub(super) floating_visible: bool,
    pub(super) search_visible: bool,
    pub(super) search_query_focused: bool,
}

impl std::fmt::Debug for KucInteractionLocator {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("KucInteractionLocator(..)")
    }
}

pub(super) struct LocatorTarget {
    pub(super) action_identity: String,
    pub(super) action_class: KucInteractionActionClass,
    pub(super) disabled: bool,
    pub(super) evidence: AccessKitEvidence,
}

#[derive(Clone, Copy)]
pub(super) struct TextSelectionGeometry {
    pub(super) start: egui::Pos2,
    pub(super) midpoint: egui::Pos2,
    pub(super) end: egui::Pos2,
}

pub(super) enum SearchTracePhase {
    Focus(KucOpaqueInteractionRequest),
    Preedit,
    Commit,
    Next(KucOpaqueInteractionRequest),
    Previous(KucOpaqueInteractionRequest),
    Close(KucOpaqueInteractionRequest),
    VerifyClosed,
}

#[derive(Clone, Copy)]
pub(super) enum TextSelectionPhase {
    Aim,
    Press,
    MoveToMidpoint,
    MoveToEnd,
    Release,
}

#[derive(Clone, Copy)]
pub(super) enum OpaqueClickPhase {
    Aim,
    Press,
    Release,
}
