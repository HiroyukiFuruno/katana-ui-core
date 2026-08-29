use std::cell::RefCell;

use crate::text_command_surface::sanitized_document_root::{
    sanitized_command_event::SanitizedCommandActivationTransport,
    sanitized_context_event::SanitizedContextMenuActivationTransport,
    sanitized_search_event::SanitizedSearchEventTransport,
    sanitized_tab_projection::adapter::SanitizedTabProjectionClosedEvent,
};

pub(crate) type ForwardedTabEventBatch = Vec<SanitizedTabProjectionClosedEvent>;
pub(crate) type ForwardedSearchEventBatch = Vec<SanitizedSearchEventTransport>;
pub(crate) type ForwardedCommandEventBatch = Vec<SanitizedCommandActivationTransport>;
pub(crate) type ForwardedContextMenuEventBatch = Vec<SanitizedContextMenuActivationTransport>;

pub(crate) struct RootEventForwarderBridge<'a, Forwarder> {
    pub(super) forwarder: &'a mut Forwarder,
    pub(super) tab_closed_events: &'a RefCell<Option<ForwardedTabEventBatch>>,
    pub(super) search_events: &'a RefCell<Option<ForwardedSearchEventBatch>>,
    pub(super) command_events: &'a RefCell<Option<ForwardedCommandEventBatch>>,
    pub(super) context_menu_events: &'a RefCell<Option<ForwardedContextMenuEventBatch>>,
    pub(super) tab_event_count: usize,
    pub(super) tab_event_fingerprint: String,
    pub(super) search_event_count: usize,
    pub(super) search_event_fingerprint: String,
    pub(super) command_event_count: usize,
    pub(super) command_event_fingerprint: String,
    pub(super) context_menu_event_count: usize,
    pub(super) context_menu_event_fingerprint: String,
}

#[derive(Debug)]
pub(crate) enum RootEventForwarderBridgeError<ForwarderError> {
    InconsistentTabEventBatch,
    InconsistentSearchEventBatch,
    InconsistentCommandEventBatch,
    InconsistentContextMenuEventBatch,
    Forwarder(ForwarderError),
}
