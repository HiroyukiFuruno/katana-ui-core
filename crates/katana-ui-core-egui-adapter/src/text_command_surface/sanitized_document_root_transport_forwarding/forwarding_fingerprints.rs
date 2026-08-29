use sha2::{Digest, Sha256};

use crate::text_command_surface::root::EguiTextCommandSurfaceRootEventForwardingReceipt;
use crate::text_command_surface::sanitized_document_root::{
    sanitized_command_event::SanitizedCommandActivationTransport,
    sanitized_context_event::SanitizedContextMenuActivationTransport,
    sanitized_search_event::SanitizedSearchEventTransport,
    sanitized_tab_projection::adapter::SanitizedTabProjectionClosedEvent,
};

pub(crate) struct SanitizedEventFingerprints;

impl SanitizedEventFingerprints {
    pub(crate) fn compose_event_batch_fingerprint(
        value: &EguiTextCommandSurfaceRootEventForwardingReceipt,
        tab_event_fingerprint: &str,
        tab_event_count: usize,
        search_event_fingerprint: &str,
        search_event_count: usize,
        command_event_fingerprint: &str,
        command_event_count: usize,
        context_menu_event_fingerprint: &str,
        context_menu_event_count: usize,
    ) -> String {
        let mut hasher = Sha256::new();
        hasher.update(b"kuc.sanitized-root-event-batch/v1\0");
        if value.event_cardinality() == 0 {
            hasher.update(b"root-no-events\0");
        } else {
            hasher.update(b"root-events\0");
        }
        hasher.update(value.event_cardinality().to_le_bytes());
        hasher.update(value.event_batch_fingerprint().as_bytes());
        if tab_event_count == 0 {
            hasher.update(b"tab-no-events\0");
        } else {
            hasher.update(b"tab-events\0");
        }
        hasher.update(tab_event_count.to_le_bytes());
        hasher.update(tab_event_fingerprint.as_bytes());
        if search_event_count == 0 {
            hasher.update(b"search-no-events\0");
        } else {
            hasher.update(b"search-events\0");
        }
        hasher.update(search_event_count.to_le_bytes());
        hasher.update(search_event_fingerprint.as_bytes());
        hasher.update(if command_event_count == 0 {
            &b"command-no-events\0"[..]
        } else {
            &b"command-events\0"[..]
        });
        hasher.update(command_event_count.to_le_bytes());
        hasher.update(command_event_fingerprint.as_bytes());
        hasher.update(if context_menu_event_count == 0 {
            &b"context-menu-no-events\0"[..]
        } else {
            &b"context-menu-events\0"[..]
        });
        hasher.update(context_menu_event_count.to_le_bytes());
        hasher.update(context_menu_event_fingerprint.as_bytes());
        hex::encode(hasher.finalize())
    }

    pub(crate) fn compose_correlation_fingerprint(
        value: &EguiTextCommandSurfaceRootEventForwardingReceipt,
        tab_event_fingerprint: &str,
        tab_event_count: usize,
        search_event_fingerprint: &str,
        search_event_count: usize,
        command_event_fingerprint: &str,
        command_event_count: usize,
        context_menu_event_fingerprint: &str,
        context_menu_event_count: usize,
    ) -> String {
        let mut hasher = Sha256::new();
        hasher.update(b"kuc.sanitized-root-event-correlation/v1\0");
        hasher.update(value.correlation_fingerprint().as_bytes());
        if tab_event_count == 0 {
            hasher.update(b"tab-no-events\0");
        } else {
            hasher.update(b"tab-events\0");
        }
        hasher.update(tab_event_count.to_le_bytes());
        hasher.update(tab_event_fingerprint.as_bytes());
        if search_event_count == 0 {
            hasher.update(b"search-no-events\0");
        } else {
            hasher.update(b"search-events\0");
        }
        hasher.update(search_event_count.to_le_bytes());
        hasher.update(search_event_fingerprint.as_bytes());
        hasher.update(if command_event_count == 0 {
            &b"command-no-events\0"[..]
        } else {
            &b"command-events\0"[..]
        });
        hasher.update(command_event_count.to_le_bytes());
        hasher.update(command_event_fingerprint.as_bytes());
        hasher.update(if context_menu_event_count == 0 {
            &b"context-menu-no-events\0"[..]
        } else {
            &b"context-menu-events\0"[..]
        });
        hasher.update(context_menu_event_count.to_le_bytes());
        hasher.update(context_menu_event_fingerprint.as_bytes());
        hex::encode(hasher.finalize())
    }

    pub(crate) fn tab_event_fingerprint(events: &[SanitizedTabProjectionClosedEvent]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(b"kuc.sanitized-tab-events/v1\0");
        hasher.update(events.len().to_le_bytes());
        for event in events {
            match event {
                SanitizedTabProjectionClosedEvent::TabActivated(_) => {
                    hasher.update(b"tab-activated\0");
                }
                SanitizedTabProjectionClosedEvent::TabCloseRequested(_) => {
                    hasher.update(b"tab-close-requested\0");
                }
                SanitizedTabProjectionClosedEvent::GroupCollapseChanged { collapsed, .. } => {
                    hasher.update(b"group-collapse-changed\0");
                    hasher.update([u8::from(*collapsed)]);
                }
            }
        }
        hex::encode(hasher.finalize())
    }

    pub(crate) fn search_event_fingerprint(events: &[SanitizedSearchEventTransport]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(b"kuc.sanitized-search-events/v1\0");
        hasher.update(events.len().to_le_bytes());
        for event in events {
            event.fingerprint_into(&mut hasher);
        }
        hex::encode(hasher.finalize())
    }

    pub(crate) fn command_event_fingerprint(
        events: &[SanitizedCommandActivationTransport],
    ) -> String {
        let mut hasher = Sha256::new();
        hasher.update(b"kuc.sanitized-command-events/v1\0");
        hasher.update(events.len().to_le_bytes());
        for event in events {
            event.fingerprint_into(&mut hasher);
        }
        hex::encode(hasher.finalize())
    }

    pub(crate) fn context_menu_event_fingerprint(
        events: &[SanitizedContextMenuActivationTransport],
    ) -> String {
        let mut hasher = Sha256::new();
        hasher.update(b"kuc.sanitized-context-menu-events/v1\0");
        hasher.update(events.len().to_le_bytes());
        for event in events {
            event.fingerprint_into(&mut hasher);
        }
        hex::encode(hasher.finalize())
    }
}
