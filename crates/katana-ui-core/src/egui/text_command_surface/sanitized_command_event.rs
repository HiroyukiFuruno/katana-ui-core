use super::sanitized_command_projection::{
    CommandCapability, SanitizedCommandCapabilityRejection, SanitizedCommandDropdownItem,
    SanitizedCommandItem, SanitizedCommandProjection,
};
use crate::molecule::command_chrome::{CommandChromeToolbarEvent, FloatingCommandToolbarEvent};
use sha2::{Digest, Sha256};

const COMMAND_TARGET_SIGNATURE_BYTES: usize = 32;

pub(super) struct SanitizedCommandActivationTransport {
    target: SanitizedCommandRoutedTarget,
    revision: u64,
    correlation: String,
}

struct SanitizedCommandRoutedTarget {
    signature: [u8; COMMAND_TARGET_SIGNATURE_BYTES],
    capability: Option<CommandCapability>,
}

impl SanitizedCommandActivationTransport {
    pub(super) fn invoke_once(&mut self) -> Result<(), SanitizedCommandCapabilityRejection> {
        let slot = self
            .target
            .capability
            .take()
            .ok_or(SanitizedCommandCapabilityRejection::Missing)?;
        let callback = slot
            .try_borrow_mut()
            .map_err(|_| SanitizedCommandCapabilityRejection::Reentrant)?
            .take()
            .ok_or(SanitizedCommandCapabilityRejection::AlreadyConsumed)?;
        callback()
    }

    pub(super) fn fingerprint_into(&self, hasher: &mut Sha256) {
        hasher.update(b"sanitized-command-activation");
        hasher.update(self.target.signature);
        hasher.update(self.revision.to_le_bytes());
        hasher.update(self.correlation.as_bytes());
    }
}

impl std::fmt::Debug for SanitizedCommandActivationTransport {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SanitizedCommandActivationTransport")
            .field("payload", &"<opaque>")
            .finish()
    }
}

pub(super) fn route_command_events(
    top_projection: Option<&SanitizedCommandProjection>,
    floating_projection: Option<&SanitizedCommandProjection>,
    events: &[CommandChromeToolbarEvent],
    floating_events: &[FloatingCommandToolbarEvent],
    revision: u64,
    root_identity_fingerprint: &str,
) -> Result<Vec<SanitizedCommandActivationTransport>, SanitizedCommandCapabilityRejection> {
    let correlation = event_correlation(root_identity_fingerprint, revision);
    let mut routed = Vec::new();
    if let Some(projection) = top_projection {
        for event in events {
            if let Some(transport) = route_toolbar_event(projection, event, revision, &correlation)?
            {
                routed.push(transport);
            }
        }
    }
    if let Some(projection) = floating_projection {
        for event in floating_events {
            if let FloatingCommandToolbarEvent::Toolbar { event } = event
                && let Some(transport) =
                    route_toolbar_event(projection, event, revision, &correlation)?
            {
                routed.push(transport);
            }
        }
    }
    Ok(routed)
}

fn route_toolbar_event(
    projection: &SanitizedCommandProjection,
    event: &CommandChromeToolbarEvent,
    revision: u64,
    correlation: &str,
) -> Result<Option<SanitizedCommandActivationTransport>, SanitizedCommandCapabilityRejection> {
    match event {
        CommandChromeToolbarEvent::CommandActivated { action_id }
        | CommandChromeToolbarEvent::AcceleratorTriggered { action_id, .. } => {
            route_direct(projection, action_id.as_str(), revision, correlation)
        }
        CommandChromeToolbarEvent::DropdownItemActivated { action_id, item_id } => route_dropdown(
            projection,
            action_id.as_str(),
            item_id.as_str(),
            revision,
            correlation,
        ),
        _ => Ok(None),
    }
}

fn route_direct(
    projection: &SanitizedCommandProjection,
    action_id: &str,
    revision: u64,
    correlation: &str,
) -> Result<Option<SanitizedCommandActivationTransport>, SanitizedCommandCapabilityRejection> {
    for group in projection.groups().iter().filter(|group| group.visible()) {
        for item in group.items().iter().filter(|item| item.visible()) {
            if target_id("kuc-command", item) == action_id {
                if !group.enabled() || !item.enabled() {
                    return Ok(None);
                }
                return activation(item.target(), revision, correlation).map(Some);
            }
        }
    }
    Err(SanitizedCommandCapabilityRejection::Missing)
}

fn route_dropdown(
    projection: &SanitizedCommandProjection,
    action_id: &str,
    item_id: &str,
    revision: u64,
    correlation: &str,
) -> Result<Option<SanitizedCommandActivationTransport>, SanitizedCommandCapabilityRejection> {
    for group in projection.groups().iter().filter(|group| group.visible()) {
        for item in group.items().iter().filter(|item| item.visible()) {
            if target_id("kuc-command", item) != action_id {
                continue;
            }
            if !group.enabled() || !item.enabled() {
                return Ok(None);
            }
            let Some(dropdown) = item
                .dropdown_items()
                .iter()
                .filter(|item| item.visible())
                .find(|dropdown| target_id("kuc-dropdown", *dropdown) == item_id)
            else {
                return Err(SanitizedCommandCapabilityRejection::Missing);
            };
            if !dropdown.enabled() {
                return Ok(None);
            }
            return activation(dropdown.target(), revision, correlation).map(Some);
        }
    }
    Err(SanitizedCommandCapabilityRejection::Missing)
}

fn activation(
    target: &super::sanitized_command_projection::SanitizedCommandTarget,
    revision: u64,
    correlation: &str,
) -> Result<SanitizedCommandActivationTransport, SanitizedCommandCapabilityRejection> {
    Ok(SanitizedCommandActivationTransport {
        target: SanitizedCommandRoutedTarget {
            signature: Sha256::digest(target.stable_fingerprint().as_bytes()).into(),
            capability: Some(
                target
                    .capability()
                    .ok_or(SanitizedCommandCapabilityRejection::Missing)?,
            ),
        },
        revision,
        correlation: correlation.to_owned(),
    })
}

fn target_id<T>(namespace: &str, target: &T) -> String
where
    T: TargetFingerprint,
{
    format!("{namespace}-{}", target.target_fingerprint())
}

trait TargetFingerprint {
    fn target_fingerprint(&self) -> String;
}

impl TargetFingerprint for SanitizedCommandItem {
    fn target_fingerprint(&self) -> String {
        self.target().stable_fingerprint()
    }
}

impl TargetFingerprint for SanitizedCommandDropdownItem {
    fn target_fingerprint(&self) -> String {
        self.target().stable_fingerprint()
    }
}

fn event_correlation(root_identity_fingerprint: &str, revision: u64) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"kuc.sanitized-command-correlation/v1");
    hasher.update(root_identity_fingerprint.as_bytes());
    hasher.update(revision.to_le_bytes());
    hex::encode(hasher.finalize())
}

#[cfg(test)]
#[path = "sanitized_command_event_tests.rs"]
mod tests;
