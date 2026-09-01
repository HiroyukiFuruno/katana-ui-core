//! Opaque event-batch projection for the retained root.

mod api;
mod batch;
mod context;
mod dispatcher;
mod receipt;
mod transport;

const ROOT_EVENT_CLASS_COUNT: usize = 7;
const ROOT_EVENT_SEARCH_INDEX: usize = 3;
const ROOT_EVENT_CONTEXT_MENU_INDEX: usize = 4;
const ROOT_EVENT_STATUS_BAR_INDEX: usize = 5;
const ROOT_EVENT_DIAGNOSTICS_INDEX: usize = 6;

pub(super) fn build_event_batch(
    output: &mut super::super::types::EguiTextCommandSurfaceOutput,
    source_address_submission_port: Option<
        super::super::source_address_projection_lease::SourceAddressSubmissionPortHandle,
    >,
) -> Result<EguiTextCommandSurfaceRootEventBatch, String> {
    dispatcher::RootEventDispatcher::build_event_batch(output, source_address_submission_port)
}

use super::super::source_address_projection_lease::{
    SourceAddressSubmissionPortError, SourceAddressSubmissionPortHandle,
};
use crate::molecule::command_chrome::{
    CommandChromeSearchEvent, CommandChromeToolbarEvent, FloatingCommandToolbarEvent,
};
use crate::molecule::structured::source_address_strip::SourceAddressSubmission;
use crate::molecule::{DiagnosticsListEvent, StatusBarEvent};
use crate::text_surface::TextSurfaceEvent;
use std::cell::{Cell, RefCell};

/// Generic KUC callback used to forward one opaque root event transport.
pub trait KucRootEventBatchForwarder {
    type Error;

    fn forward_root_event_batch(
        &mut self,
        transport: EguiTextCommandSurfaceRootEventTransport,
    ) -> Result<(), Self::Error>;
}

/// Host-owned opaque one-shot effect batch.
pub struct KucOpaqueHostEffectBatch {
    effect: Option<Box<dyn FnOnce() -> Result<(), KucOpaqueHostEffectError>>>,
}

/// Generic current-root event information supplied to a host effect router.
/// Event payloads and child models remain private to KUC.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KucRootEventBatchContext {
    root_identity: String,
    state_revision: u64,
    correlation_fingerprint: String,
    class_dispatches: [EguiTextCommandSurfaceRootEventClassDispatch; ROOT_EVENT_CLASS_COUNT],
    text_events: Vec<TextSurfaceEvent>,
    toolbar_events: Vec<CommandChromeToolbarEvent>,
    floating_events: Vec<FloatingCommandToolbarEvent>,
    search_events: Vec<CommandChromeSearchEvent>,
    context_menu_events: Vec<crate::molecule::selection::ContextMenuEvent>,
    source_address_submission_count: usize,
    status_bar_events: Vec<StatusBarEvent>,
    diagnostics_list_events: Vec<DiagnosticsListEvent>,
}

/// Generic KUC router for a non-wire host effect.
pub trait KucRootEffectRouter {
    fn route(
        &mut self,
        context: KucRootEventBatchContext,
    ) -> Result<Option<KucOpaqueHostEffectBatch>, KucOpaqueHostEffectError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KucOpaqueHostEffectError;

struct RootEventPayload {
    text: Vec<TextSurfaceEvent>,
    toolbar: Option<Vec<CommandChromeToolbarEvent>>,
    floating: Option<Vec<FloatingCommandToolbarEvent>>,
    search: Option<Vec<CommandChromeSearchEvent>>,
    context_menu: Option<Vec<crate::molecule::selection::ContextMenuEvent>>,
    status_bar: Option<Vec<StatusBarEvent>>,
    diagnostics_list: Option<Vec<DiagnosticsListEvent>>,
    source_address_submissions: Vec<SourceAddressSubmission>,
}

/// Opaque one-shot transport token. It has no consumer-visible semantic accessors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KucOpaqueHostEffectAttachError {
    AlreadyConsumed,
    AlreadyAttached,
}

pub struct EguiTextCommandSurfaceRootEventTransport {
    payload: RootEventPayload,
    opaque_host_effect_batch: Option<KucOpaqueHostEffectBatch>,
    source_address_submission_port: Option<SourceAddressSubmissionPortHandle>,
}

/// Deterministic receipt returned after a root event transport was forwarded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EguiTextCommandSurfaceRootEventForwardingReceipt {
    root_identity: String,
    state_revision: u64,
    correlation_fingerprint: String,
    event_batch_fingerprint: String,
    consumed_once: bool,
    event_cardinality: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EguiTextCommandSurfaceRootEventDispatchReceipt {
    class_dispatches: [EguiTextCommandSurfaceRootEventClassDispatch; ROOT_EVENT_CLASS_COUNT],
}

#[derive(Debug, PartialEq, Eq)]
pub enum EguiTextCommandSurfaceRootEventBatchDispatchError<DispatcherError> {
    AlreadyConsumed,
    Dispatcher(DispatcherError),
    OpaqueHostEffect,
    SourceAddressPort(SourceAddressSubmissionPortError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EguiTextCommandSurfaceRootEventChildClass {
    Text,
    Toolbar,
    Floating,
    Search,
    ContextMenu,
    StatusBar,
    DiagnosticsList,
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
    transport: std::cell::RefCell<Option<EguiTextCommandSurfaceRootEventTransport>>,
    root_identity: String,
    state_revision: u64,
    correlation_fingerprint: RefCell<String>,
    event_batch_fingerprint: RefCell<String>,
    event_cardinality: Cell<usize>,
    search_detached: Cell<bool>,
    command_detached: Cell<bool>,
    context_menu_detached: Cell<bool>,
}

/// Generic callback used to dispatch one-time root event payload per child class.
pub trait KucRootEventBatchDispatcher {
    type Error;

    fn dispatch_text_events(&mut self, events: Vec<TextSurfaceEvent>) -> Result<(), Self::Error>;
    fn dispatch_toolbar_events(
        &mut self,
        events: Vec<CommandChromeToolbarEvent>,
    ) -> Result<(), Self::Error>;
    fn dispatch_floating_events(
        &mut self,
        events: Vec<FloatingCommandToolbarEvent>,
    ) -> Result<(), Self::Error>;
    fn dispatch_search_events(
        &mut self,
        events: Vec<CommandChromeSearchEvent>,
    ) -> Result<(), Self::Error>;
    fn dispatch_context_menu_events(
        &mut self,
        events: Vec<crate::molecule::selection::ContextMenuEvent>,
    ) -> Result<(), Self::Error>;

    fn dispatch_status_bar_events(
        &mut self,
        _events: Vec<StatusBarEvent>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn dispatch_diagnostics_list_events(
        &mut self,
        _events: Vec<DiagnosticsListEvent>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn consume_opaque_host_effect_batch(
        &mut self,
        effect_batch: KucOpaqueHostEffectBatch,
    ) -> Result<(), KucOpaqueHostEffectError> {
        effect_batch.consume_once()
    }
}

#[cfg(test)]
#[path = "root_event_inline_tests.rs"]
mod tests;
