use super::super::super::root::{
    EguiTextCommandSurfaceRootEventForwardingReceipt, EguiTextCommandSurfaceRootEventTransport,
};
use super::super::sanitized_search_projection::SanitizedSearchCapabilityRejection;

/// Opaque callback used to forward one sanitized document root event transport.
pub trait SanitizedDocumentRootEventForwarder {
    type Error;

    fn forward_sanitized_document_root_event(
        &mut self,
        transport: SanitizedDocumentRootEventTransport,
    ) -> Result<(), Self::Error>;
}

/// Sealed, non-clone event transport reserved for the sanitized root boundary.
pub struct SanitizedDocumentRootEventTransport {
    pub(super) root_transport: Option<EguiTextCommandSurfaceRootEventTransport>,
}

/// Failure while the host consumes the opaque root port.
#[derive(Debug, PartialEq, Eq)]
pub enum SanitizedDocumentRootEventDispatchError<DispatcherError> {
    AlreadyConsumed,
    Child(DispatcherError),
    OpaqueHostEffect,
}

/// Receipt returned after one sanitized document root event forwarding operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SanitizedDocumentRootEventForwardingReceipt {
    root_identity: String,
    state_revision: u64,
    correlation_fingerprint: String,
    event_batch_fingerprint: String,
    event_cardinality: usize,
}

impl SanitizedDocumentRootEventForwardingReceipt {
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
    pub const fn event_cardinality(&self) -> usize {
        self.event_cardinality
    }

    #[must_use]
    pub const fn consumed_once(&self) -> bool {
        true
    }
}

/// Typed failure for the one-shot sanitized event forwarding operation.
#[derive(Debug, PartialEq, Eq)]
pub enum SanitizedDocumentRootEventForwardError<ForwarderError> {
    AlreadyConsumed,
    StaleFrame,
    InconsistentTabEventBatch,
    InconsistentSearchEventBatch,
    InconsistentCommandEventBatch,
    InconsistentContextMenuEventBatch,
    Forwarder(ForwarderError),
    SearchCapability(SanitizedSearchCapabilityRejection),
    CommandCapability(
        super::super::sanitized_command_projection::SanitizedCommandCapabilityRejection,
    ),
    ContextMenuCapability(
        super::super::sanitized_context_projection::SanitizedContextMenuCapabilityRejection,
    ),
}

impl SanitizedDocumentRootEventForwardingReceipt {
    pub(super) fn from_root(
        value: EguiTextCommandSurfaceRootEventForwardingReceipt,
        tab_event_fingerprint: &str,
        tab_event_count: usize,
        search_event_fingerprint: &str,
        search_event_count: usize,
        command_event_fingerprint: &str,
        command_event_count: usize,
        context_menu_event_fingerprint: &str,
        context_menu_event_count: usize,
    ) -> Self {
        Self {
            root_identity: value.root_identity().to_owned(),
            state_revision: value.state_revision(),
            correlation_fingerprint: super::compose_correlation_fingerprint(
                &value,
                tab_event_fingerprint,
                tab_event_count,
                search_event_fingerprint,
                search_event_count,
                command_event_fingerprint,
                command_event_count,
                context_menu_event_fingerprint,
                context_menu_event_count,
            ),
            event_batch_fingerprint: super::compose_event_batch_fingerprint(
                &value,
                tab_event_fingerprint,
                tab_event_count,
                search_event_fingerprint,
                search_event_count,
                command_event_fingerprint,
                command_event_count,
                context_menu_event_fingerprint,
                context_menu_event_count,
            ),
            event_cardinality: value.event_cardinality()
                + tab_event_count
                + search_event_count
                + command_event_count
                + context_menu_event_count,
        }
    }
}
