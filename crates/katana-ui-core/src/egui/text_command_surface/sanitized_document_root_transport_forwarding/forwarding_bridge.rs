use std::cell::RefCell;

use super::forwarding_types::{
    ForwardedCommandEventBatch, ForwardedContextMenuEventBatch, ForwardedSearchEventBatch,
    ForwardedTabEventBatch, RootEventForwarderBridge, RootEventForwarderBridgeError,
};
use crate::egui::text_command_surface::root::{
    EguiTextCommandSurfaceRootEventBatchForwardError, EguiTextCommandSurfaceRootOutput,
};
use crate::egui::text_command_surface::sanitized_document_root::sanitized_document_root_transport::{
    SanitizedDocumentRootEventForwardError, SanitizedDocumentRootEventForwarder,
    SanitizedDocumentRootEventForwardingReceipt,
};

pub(crate) struct RootEventForwarding;

impl RootEventForwarding {
    pub(crate) fn forward_root_events_once<Forwarder>(
        output: &EguiTextCommandSurfaceRootOutput,
        tab_closed_events: &RefCell<Option<ForwardedTabEventBatch>>,
        search_events: &RefCell<Option<ForwardedSearchEventBatch>>,
        command_events: &RefCell<Option<ForwardedCommandEventBatch>>,
        context_menu_events: &RefCell<Option<ForwardedContextMenuEventBatch>>,
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
            .map_err(map_forward_error)?;
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
}

fn map_forward_error<ForwarderError>(
    error: EguiTextCommandSurfaceRootEventBatchForwardError<
        RootEventForwarderBridgeError<ForwarderError>,
    >,
) -> SanitizedDocumentRootEventForwardError<ForwarderError> {
    match error {
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_bridge_failure_maps_to_the_public_forwarding_error() {
        let errors = [
            RootEventForwarderBridgeError::InconsistentTabEventBatch,
            RootEventForwarderBridgeError::InconsistentSearchEventBatch,
            RootEventForwarderBridgeError::InconsistentCommandEventBatch,
            RootEventForwarderBridgeError::InconsistentContextMenuEventBatch,
        ];
        for error in errors {
            assert!(!matches!(
                map_forward_error::<()>(
                    EguiTextCommandSurfaceRootEventBatchForwardError::Forwarder(error)
                ),
                SanitizedDocumentRootEventForwardError::Forwarder(_)
            ));
        }
        assert!(matches!(
            map_forward_error(EguiTextCommandSurfaceRootEventBatchForwardError::Forwarder(
                RootEventForwarderBridgeError::Forwarder("forwarder")
            )),
            SanitizedDocumentRootEventForwardError::Forwarder("forwarder")
        ));
    }
}
