use super::sanitized_context_projection::{
    ContextMenuCapability, SanitizedContextMenuCapabilityRejection, SanitizedContextMenuItem,
    SanitizedContextMenuProjection,
};
use crate::molecule::selection::ContextMenuEvent;
use sha2::{Digest, Sha256};

const SHA256_SIGNATURE_LENGTH: usize = 32;

pub(super) struct SanitizedContextMenuActivationTransport {
    target: SanitizedContextMenuRoutedTarget,
    revision: u64,
    correlation: String,
}

struct SanitizedContextMenuRoutedTarget {
    signature: [u8; SHA256_SIGNATURE_LENGTH],
    capability: Option<ContextMenuCapability>,
}

impl SanitizedContextMenuActivationTransport {
    pub(super) fn invoke_once(&mut self) -> Result<(), SanitizedContextMenuCapabilityRejection> {
        let slot = self
            .target
            .capability
            .take()
            .ok_or(SanitizedContextMenuCapabilityRejection::Missing)?;
        let callback = slot
            .try_borrow_mut()
            .map_err(|_| SanitizedContextMenuCapabilityRejection::Reentrant)?
            .take()
            .ok_or(SanitizedContextMenuCapabilityRejection::AlreadyConsumed)?;
        callback()
    }

    pub(super) fn fingerprint_into(&self, hasher: &mut Sha256) {
        hasher.update(b"sanitized-context-menu-activation");
        hasher.update(self.target.signature);
        hasher.update(self.revision.to_le_bytes());
        hasher.update(self.correlation.as_bytes());
    }
}

impl std::fmt::Debug for SanitizedContextMenuActivationTransport {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SanitizedContextMenuActivationTransport")
            .field("payload", &"<opaque>")
            .finish()
    }
}

pub(super) fn route_context_menu_events(
    projection: Option<&SanitizedContextMenuProjection>,
    events: &[ContextMenuEvent],
    revision: u64,
    root_identity_fingerprint: &str,
) -> Result<Vec<SanitizedContextMenuActivationTransport>, SanitizedContextMenuCapabilityRejection> {
    let Some(projection) = projection else {
        return Ok(Vec::new());
    };
    let correlation = event_correlation(root_identity_fingerprint, revision);
    let mut routed = Vec::new();
    for event in events {
        let ContextMenuEvent::ItemSelected { path, command } = event else {
            continue;
        };
        if let Some(event) = route_leaf(projection, path, command, revision, &correlation)? {
            routed.push(event);
        }
    }
    Ok(routed)
}

fn route_leaf(
    projection: &SanitizedContextMenuProjection,
    path: &[usize],
    command: &str,
    revision: u64,
    correlation: &str,
) -> Result<Option<SanitizedContextMenuActivationTransport>, SanitizedContextMenuCapabilityRejection>
{
    let mut items = projection.items();
    let mut item = None;
    for index in path {
        let current = items
            .get(*index)
            .ok_or(SanitizedContextMenuCapabilityRejection::Missing)?;
        item = Some(current);
        items = current.submenu();
    }
    let item = item.ok_or(SanitizedContextMenuCapabilityRejection::Missing)?;
    if target_id(item) != command {
        return Err(SanitizedContextMenuCapabilityRejection::Missing);
    }
    if !item.enabled() || !item.submenu().is_empty() {
        return Ok(None);
    }
    let capability = item
        .target()
        .capability()
        .ok_or(SanitizedContextMenuCapabilityRejection::Missing)?;
    Ok(Some(SanitizedContextMenuActivationTransport {
        target: SanitizedContextMenuRoutedTarget {
            signature: Sha256::digest(item.target().opaque()).into(),
            capability: Some(capability),
        },
        revision,
        correlation: correlation.to_owned(),
    }))
}

fn target_id(item: &SanitizedContextMenuItem) -> String {
    let mut digest = Sha256::new();
    digest.update((item.target().opaque().len() as u64).to_le_bytes());
    digest.update(item.target().opaque());
    format!(
        concat!("kuc-context-menu-", "{}"),
        hex::encode(digest.finalize())
    )
}

fn event_correlation(root_identity_fingerprint: &str, revision: u64) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"kuc.sanitized-context-menu-correlation/v1");
    hasher.update(root_identity_fingerprint.as_bytes());
    hasher.update(revision.to_le_bytes());
    hex::encode(hasher.finalize())
}

#[cfg(test)]
#[path = "sanitized_context_event_tests.rs"]
mod tests;
