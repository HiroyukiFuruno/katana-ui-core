#[path = "sanitized_document_root_transport/fingerprint.rs"]
mod fingerprint;
#[path = "sanitized_document_root_transport/types.rs"]
mod types;

use super::super::root::{
    EguiTextCommandSurfaceRootEventBatchDispatchError,
    EguiTextCommandSurfaceRootEventBatchForwardError,
    EguiTextCommandSurfaceRootEventDispatchReceipt, EguiTextCommandSurfaceRootEventTransport,
    EguiTextCommandSurfaceRootOutput, KucOpaqueHostEffectBatch, KucOpaqueHostEffectError,
    KucRootEventBatchDispatcher, KucRootEventBatchForwarder,
};
use super::sanitized_command_event::SanitizedCommandActivationTransport;
use super::sanitized_context_event::SanitizedContextMenuActivationTransport;
use super::sanitized_search_event::SanitizedSearchEventTransport;
use super::sanitized_tab_projection::adapter::SanitizedTabProjectionClosedEvent;
use std::cell::RefCell;

pub use types::{
    SanitizedDocumentRootEventDispatchError, SanitizedDocumentRootEventForwardError,
    SanitizedDocumentRootEventForwarder, SanitizedDocumentRootEventForwardingReceipt,
    SanitizedDocumentRootEventTransport,
};

impl SanitizedDocumentRootEventTransport {
    /// Relays the opaque root event to the actual host dispatcher exactly once.
    pub fn dispatch_root_once<Dispatcher>(
        &mut self,
        dispatcher: &mut Dispatcher,
    ) -> Result<
        EguiTextCommandSurfaceRootEventDispatchReceipt,
        SanitizedDocumentRootEventDispatchError<Dispatcher::Error>,
    >
    where
        Dispatcher: KucRootEventBatchDispatcher,
    {
        let transport = self
            .root_transport
            .take()
            .ok_or(SanitizedDocumentRootEventDispatchError::AlreadyConsumed)?;
        match transport.dispatch_once(dispatcher) {
            Ok(receipt) => Ok(receipt),
            Err(EguiTextCommandSurfaceRootEventBatchDispatchError::Dispatcher(error)) => {
                Err(SanitizedDocumentRootEventDispatchError::Child(error))
            }
            Err(
                EguiTextCommandSurfaceRootEventBatchDispatchError::AlreadyConsumed
                | EguiTextCommandSurfaceRootEventBatchDispatchError::OpaqueHostEffect
                | EguiTextCommandSurfaceRootEventBatchDispatchError::SourceAddressPort(_),
            ) => Err(SanitizedDocumentRootEventDispatchError::OpaqueHostEffect),
        }
    }
}

impl std::fmt::Debug for SanitizedDocumentRootEventTransport {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SanitizedDocumentRootEventTransport")
            .field("payload", &"<opaque>")
            .finish()
    }
}
pub(super) use fingerprint::tab_event_fingerprint;
use fingerprint::{
    command_event_fingerprint, compose_correlation_fingerprint, compose_event_batch_fingerprint,
    context_menu_event_fingerprint, search_event_fingerprint,
};

struct RootEventForwarderBridge<'a, Forwarder> {
    forwarder: &'a mut Forwarder,
    tab_closed_events: &'a RefCell<Option<Vec<SanitizedTabProjectionClosedEvent>>>,
    search_events: &'a RefCell<Option<Vec<SanitizedSearchEventTransport>>>,
    command_events: &'a RefCell<Option<Vec<SanitizedCommandActivationTransport>>>,
    context_menu_events: &'a RefCell<Option<Vec<SanitizedContextMenuActivationTransport>>>,
    tab_event_count: usize,
    tab_event_fingerprint: String,
    search_event_count: usize,
    search_event_fingerprint: String,
    command_event_count: usize,
    command_event_fingerprint: String,
    context_menu_event_count: usize,
    context_menu_event_fingerprint: String,
}

enum RootEventForwarderBridgeError<ForwarderError> {
    InconsistentTabEventBatch,
    InconsistentSearchEventBatch,
    InconsistentCommandEventBatch,
    InconsistentContextMenuEventBatch,
    Forwarder(ForwarderError),
}

pub(super) fn forward_root_events_once<Forwarder>(
    output: &EguiTextCommandSurfaceRootOutput,
    tab_closed_events: &RefCell<Option<Vec<SanitizedTabProjectionClosedEvent>>>,
    search_events: &RefCell<Option<Vec<SanitizedSearchEventTransport>>>,
    command_events: &RefCell<Option<Vec<SanitizedCommandActivationTransport>>>,
    context_menu_events: &RefCell<Option<Vec<SanitizedContextMenuActivationTransport>>>,
    forwarder: &mut Forwarder,
) -> Result<
    SanitizedDocumentRootEventForwardingReceipt,
    SanitizedDocumentRootEventForwardError<Forwarder::Error>,
>
where
    Forwarder: SanitizedDocumentRootEventForwarder,
{
    let mut bridge = RootEventForwarderBridge {
        forwarder,
        tab_closed_events,
        search_events,
        command_events,
        context_menu_events,
        tab_event_count: 0,
        tab_event_fingerprint: String::new(),
        search_event_count: 0,
        search_event_fingerprint: String::new(),
        command_event_count: 0,
        command_event_fingerprint: String::new(),
        context_menu_event_count: 0,
        context_menu_event_fingerprint: String::new(),
    };
    let receipt = output
        .events()
        .forward_once(&mut bridge)
        .map_err(|error| match error {
            EguiTextCommandSurfaceRootEventBatchForwardError::AlreadyConsumed => {
                SanitizedDocumentRootEventForwardError::AlreadyConsumed
            }
            EguiTextCommandSurfaceRootEventBatchForwardError::Forwarder(error) => match error {
                RootEventForwarderBridgeError::InconsistentTabEventBatch => {
                    SanitizedDocumentRootEventForwardError::InconsistentTabEventBatch
                }
                RootEventForwarderBridgeError::InconsistentSearchEventBatch => {
                    SanitizedDocumentRootEventForwardError::InconsistentSearchEventBatch
                }
                RootEventForwarderBridgeError::InconsistentCommandEventBatch => {
                    SanitizedDocumentRootEventForwardError::InconsistentCommandEventBatch
                }
                RootEventForwarderBridgeError::InconsistentContextMenuEventBatch => {
                    SanitizedDocumentRootEventForwardError::InconsistentContextMenuEventBatch
                }
                RootEventForwarderBridgeError::Forwarder(error) => {
                    SanitizedDocumentRootEventForwardError::Forwarder(error)
                }
            },
        })?;
    Ok(SanitizedDocumentRootEventForwardingReceipt::from_root(
        receipt,
        &bridge.tab_event_fingerprint,
        bridge.tab_event_count,
        &bridge.search_event_fingerprint,
        bridge.search_event_count,
        &bridge.command_event_fingerprint,
        bridge.command_event_count,
        &bridge.context_menu_event_fingerprint,
        bridge.context_menu_event_count,
    ))
}

impl<Forwarder> KucRootEventBatchForwarder for RootEventForwarderBridge<'_, Forwarder>
where
    Forwarder: SanitizedDocumentRootEventForwarder,
{
    type Error = RootEventForwarderBridgeError<Forwarder::Error>;

    fn forward_root_event_batch(
        &mut self,
        transport: EguiTextCommandSurfaceRootEventTransport,
    ) -> Result<(), Self::Error> {
        let tab_closed_events = self
            .tab_closed_events
            .borrow_mut()
            .take()
            .ok_or(RootEventForwarderBridgeError::InconsistentTabEventBatch)?;
        let search_events = self
            .search_events
            .borrow_mut()
            .take()
            .ok_or(RootEventForwarderBridgeError::InconsistentSearchEventBatch)?;
        let command_events = self
            .command_events
            .borrow_mut()
            .take()
            .ok_or(RootEventForwarderBridgeError::InconsistentCommandEventBatch)?;
        let context_menu_events = self
            .context_menu_events
            .borrow_mut()
            .take()
            .ok_or(RootEventForwarderBridgeError::InconsistentContextMenuEventBatch)?;
        for event in &tab_closed_events {
            event.read_for_transport();
        }
        for event in &search_events {
            event.read_for_transport();
        }
        self.tab_event_count = tab_closed_events.len();
        self.tab_event_fingerprint = tab_event_fingerprint(&tab_closed_events);
        self.search_event_count = search_events.len();
        self.search_event_fingerprint = search_event_fingerprint(&search_events);
        self.command_event_count = command_events.len();
        self.command_event_fingerprint = command_event_fingerprint(&command_events);
        self.context_menu_event_count = context_menu_events.len();
        self.context_menu_event_fingerprint = context_menu_event_fingerprint(&context_menu_events);
        let effect_batch = KucOpaqueHostEffectBatch::from_handler(move || {
            for mut event in command_events {
                event.invoke_once().map_err(|_| KucOpaqueHostEffectError)?;
            }
            for mut event in context_menu_events {
                event.invoke_once().map_err(|_| KucOpaqueHostEffectError)?;
            }
            for mut event in search_events {
                event.invoke_once().map_err(|_| KucOpaqueHostEffectError)?;
            }
            Ok(())
        });
        let transport = transport.with_opaque_host_effect_batch(effect_batch);
        let transport = SanitizedDocumentRootEventTransport {
            root_transport: Some(transport),
        };
        let _ = &transport.root_transport;
        self.forwarder
            .forward_sanitized_document_root_event(transport)
            .map_err(RootEventForwarderBridgeError::Forwarder)
    }
}

#[cfg(test)]
#[path = "sanitized_document_root_transport_inline_tests.rs"]
mod tests;
