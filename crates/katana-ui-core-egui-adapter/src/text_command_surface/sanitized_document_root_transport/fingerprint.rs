use super::super::super::root::EguiTextCommandSurfaceRootEventForwardingReceipt;
use super::super::sanitized_command_event::SanitizedCommandActivationTransport;
use super::super::sanitized_context_event::SanitizedContextMenuActivationTransport;
use super::super::sanitized_search_event::SanitizedSearchEventTransport;
use super::super::sanitized_tab_projection::adapter::SanitizedTabProjectionClosedEvent;
use sha2::{Digest, Sha256};

pub(super) fn compose_event_batch_fingerprint(
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
    hasher.update(if value.event_cardinality() == 0 {
        &b"root-no-events\0"[..]
    } else {
        &b"root-events\0"[..]
    });
    hasher.update(value.event_cardinality().to_le_bytes());
    hasher.update(value.event_batch_fingerprint().as_bytes());
    hasher.update(if tab_event_count == 0 {
        &b"tab-no-events\0"[..]
    } else {
        &b"tab-events\0"[..]
    });
    hasher.update(tab_event_count.to_le_bytes());
    hasher.update(tab_event_fingerprint.as_bytes());
    hasher.update(if search_event_count == 0 {
        &b"search-no-events\0"[..]
    } else {
        &b"search-events\0"[..]
    });
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
    format!("{:x}", hasher.finalize())
}

pub(super) fn compose_correlation_fingerprint(
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
    hasher.update(if tab_event_count == 0 {
        &b"tab-no-events\0"[..]
    } else {
        &b"tab-events\0"[..]
    });
    hasher.update(tab_event_count.to_le_bytes());
    hasher.update(tab_event_fingerprint.as_bytes());
    hasher.update(if search_event_count == 0 {
        &b"search-no-events\0"[..]
    } else {
        &b"search-events\0"[..]
    });
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
    format!("{:x}", hasher.finalize())
}

pub(super) fn tab_event_fingerprint(events: &[SanitizedTabProjectionClosedEvent]) -> String {
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
    format!("{:x}", hasher.finalize())
}

pub(super) fn search_event_fingerprint(events: &[SanitizedSearchEventTransport]) -> String {
    fingerprint_events(b"kuc.sanitized-search-events/v1\0", events)
}

pub(super) fn command_event_fingerprint(events: &[SanitizedCommandActivationTransport]) -> String {
    fingerprint_events(b"kuc.sanitized-command-events/v1\0", events)
}

pub(super) fn context_menu_event_fingerprint(
    events: &[SanitizedContextMenuActivationTransport],
) -> String {
    fingerprint_events(b"kuc.sanitized-context-menu-events/v1\0", events)
}

fn fingerprint_events<T>(domain: &[u8], events: &[T]) -> String
where
    T: FingerprintEvent,
{
    let mut hasher = Sha256::new();
    hasher.update(domain);
    hasher.update(events.len().to_le_bytes());
    for event in events {
        event.fingerprint_into(&mut hasher);
    }
    format!("{:x}", hasher.finalize())
}

trait FingerprintEvent {
    fn fingerprint_into(&self, hasher: &mut Sha256);
}

impl FingerprintEvent for SanitizedSearchEventTransport {
    fn fingerprint_into(&self, hasher: &mut Sha256) {
        SanitizedSearchEventTransport::fingerprint_into(self, hasher);
    }
}

impl FingerprintEvent for SanitizedCommandActivationTransport {
    fn fingerprint_into(&self, hasher: &mut Sha256) {
        SanitizedCommandActivationTransport::fingerprint_into(self, hasher);
    }
}

impl FingerprintEvent for SanitizedContextMenuActivationTransport {
    fn fingerprint_into(&self, hasher: &mut Sha256) {
        SanitizedContextMenuActivationTransport::fingerprint_into(self, hasher);
    }
}
