mod forwarding_bridge;
mod forwarding_fingerprints;
mod forwarding_types;

use crate::text_command_surface::root::{
    EguiTextCommandSurfaceRootEventTransport, KucOpaqueHostEffectBatch, KucOpaqueHostEffectError,
    KucRootEventBatchForwarder,
};
use crate::text_command_surface::sanitized_document_root::{
    sanitized_document_root_transport::SanitizedDocumentRootEventForwarder,
    sanitized_document_root_transport::SanitizedDocumentRootEventTransport,
};

pub(crate) use forwarding_bridge::RootEventForwarding;
pub(crate) use forwarding_fingerprints::SanitizedEventFingerprints;
use forwarding_types::{RootEventForwarderBridge, RootEventForwarderBridgeError};

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
        self.tab_event_fingerprint =
            SanitizedEventFingerprints::tab_event_fingerprint(&tab_closed_events);
        self.search_event_count = search_events.len();
        self.search_event_fingerprint =
            SanitizedEventFingerprints::search_event_fingerprint(&search_events);
        self.command_event_count = command_events.len();
        self.command_event_fingerprint =
            SanitizedEventFingerprints::command_event_fingerprint(&command_events);
        self.context_menu_event_count = context_menu_events.len();
        self.context_menu_event_fingerprint =
            SanitizedEventFingerprints::context_menu_event_fingerprint(&context_menu_events);
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
