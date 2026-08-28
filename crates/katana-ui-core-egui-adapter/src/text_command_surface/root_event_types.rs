use super::EguiTextCommandSurfaceRootEventTransport;
use katana_ui_core::molecule::command_chrome::{
    CommandChromeSearchEvent, CommandChromeToolbarEvent, FloatingCommandToolbarEvent,
};
use katana_ui_core::text_surface::TextSurfaceEvent;
use std::cell::{Cell, RefCell};

pub(crate) const ROOT_EVENT_CHILD_CLASS_COUNT: usize = 5;
pub(crate) const TEXT_CLASS_INDEX: usize = 0;
pub(crate) const TOOLBAR_CLASS_INDEX: usize = 1;
pub(crate) const FLOATING_CLASS_INDEX: usize = 2;
pub(crate) const SEARCH_CLASS_INDEX: usize = 3;
pub(crate) const CONTEXT_MENU_CLASS_INDEX: usize = 4;

/// Generic current-root event information supplied to a host effect router.
/// Event payloads and child models remain private to KUC.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KucRootEventBatchContext {
    pub(crate) root_identity: String,
    pub(crate) state_revision: u64,
    pub(crate) correlation_fingerprint: String,
    pub(crate) class_dispatches:
        [EguiTextCommandSurfaceRootEventClassDispatch; ROOT_EVENT_CHILD_CLASS_COUNT],
    pub(crate) text_events: Vec<TextSurfaceEvent>,
    pub(crate) toolbar_events: Vec<CommandChromeToolbarEvent>,
    pub(crate) floating_events: Vec<FloatingCommandToolbarEvent>,
    pub(crate) search_events: Vec<CommandChromeSearchEvent>,
    pub(crate) context_menu_events: Vec<katana_ui_core::molecule::selection::ContextMenuEvent>,
}

impl KucRootEventBatchContext {
    #[must_use]
    pub fn root_identity(&self) -> &str {
        &self.root_identity
    }

    #[must_use]
    pub const fn state_revision(&self) -> u64 {
        self.state_revision
    }

    #[must_use]
    pub fn correlation_fingerprint(&self) -> &str {
        &self.correlation_fingerprint
    }

    #[must_use]
    pub const fn class_dispatches(
        &self,
    ) -> &[EguiTextCommandSurfaceRootEventClassDispatch; ROOT_EVENT_CHILD_CLASS_COUNT] {
        &self.class_dispatches
    }

    #[must_use]
    pub fn text_events(&self) -> &[TextSurfaceEvent] {
        &self.text_events
    }

    #[must_use]
    pub fn toolbar_events(&self) -> &[CommandChromeToolbarEvent] {
        &self.toolbar_events
    }

    #[must_use]
    pub fn floating_events(&self) -> &[FloatingCommandToolbarEvent] {
        &self.floating_events
    }

    #[must_use]
    pub fn search_events(&self) -> &[CommandChromeSearchEvent] {
        &self.search_events
    }

    #[must_use]
    pub fn context_menu_events(&self) -> &[katana_ui_core::molecule::selection::ContextMenuEvent] {
        &self.context_menu_events
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum KucOpaqueHostEffectAttachError {
    AlreadyConsumed,
    AlreadyAttached,
}

/// Deterministic receipt returned after a root event transport was forwarded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EguiTextCommandSurfaceRootEventForwardingReceipt {
    pub(crate) root_identity: String,
    pub(crate) state_revision: u64,
    pub(crate) correlation_fingerprint: String,
    pub(crate) event_batch_fingerprint: String,
    pub(crate) consumed_once: bool,
    pub(crate) event_cardinality: usize,
}

impl EguiTextCommandSurfaceRootEventForwardingReceipt {
    #[must_use]
    pub fn root_identity(&self) -> &str {
        &self.root_identity
    }

    #[must_use]
    pub const fn state_revision(&self) -> u64 {
        self.state_revision
    }

    #[must_use]
    pub fn correlation_fingerprint(&self) -> &str {
        &self.correlation_fingerprint
    }

    #[must_use]
    pub fn event_batch_fingerprint(&self) -> &str {
        &self.event_batch_fingerprint
    }

    #[must_use]
    pub const fn consumed_once(&self) -> bool {
        self.consumed_once
    }

    #[must_use]
    pub const fn event_cardinality(&self) -> usize {
        self.event_cardinality
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EguiTextCommandSurfaceRootEventDispatchReceipt {
    pub(crate) class_dispatches:
        [EguiTextCommandSurfaceRootEventClassDispatch; ROOT_EVENT_CHILD_CLASS_COUNT],
}

impl EguiTextCommandSurfaceRootEventDispatchReceipt {
    #[must_use]
    pub fn class_dispatches(&self) -> &[EguiTextCommandSurfaceRootEventClassDispatch] {
        &self.class_dispatches
    }

    #[must_use]
    pub const fn text_count(self) -> usize {
        self.class_dispatches[TEXT_CLASS_INDEX].event_count
    }

    #[must_use]
    pub const fn toolbar_count(self) -> usize {
        self.class_dispatches[TOOLBAR_CLASS_INDEX].event_count
    }

    #[must_use]
    pub const fn floating_count(self) -> usize {
        self.class_dispatches[FLOATING_CLASS_INDEX].event_count
    }

    #[must_use]
    pub const fn search_count(self) -> usize {
        self.class_dispatches[SEARCH_CLASS_INDEX].event_count
    }

    #[must_use]
    pub const fn context_menu_count(self) -> usize {
        self.class_dispatches[CONTEXT_MENU_CLASS_INDEX].event_count
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum EguiTextCommandSurfaceRootEventBatchDispatchError<DispatcherError> {
    AlreadyConsumed,
    Dispatcher(DispatcherError),
    OpaqueHostEffect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EguiTextCommandSurfaceRootEventChildClass {
    Text,
    Toolbar,
    Floating,
    Search,
    ContextMenu,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EguiTextCommandSurfaceRootEventClassDispatch {
    pub child_class: EguiTextCommandSurfaceRootEventChildClass,
    pub event_count: usize,
}

/// Typed failure for one-shot root event forwarding.
#[derive(Debug)]
pub enum EguiTextCommandSurfaceRootEventBatchForwardError<ForwarderError> {
    AlreadyConsumed,
    Forwarder(ForwarderError),
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum EguiTextCommandSurfaceRootEventSearchDetachError {
    AlreadyConsumed,
    AlreadyDetached,
    Serialization,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum EguiTextCommandSurfaceRootEventCommandDetachError {
    AlreadyConsumed,
    AlreadyDetached,
    Serialization,
}

/// Sealed root event batch. Its payload can leave KUC only through `forward_once`.
pub struct EguiTextCommandSurfaceRootEventBatch {
    pub(crate) transport: RefCell<Option<EguiTextCommandSurfaceRootEventTransport>>,
    pub(crate) root_identity: String,
    pub(crate) state_revision: u64,
    pub(crate) correlation_fingerprint: RefCell<String>,
    pub(crate) event_batch_fingerprint: RefCell<String>,
    pub(crate) event_cardinality: Cell<usize>,
    pub(crate) search_detached: Cell<bool>,
    pub(crate) command_detached: Cell<bool>,
    pub(crate) context_menu_detached: Cell<bool>,
}
