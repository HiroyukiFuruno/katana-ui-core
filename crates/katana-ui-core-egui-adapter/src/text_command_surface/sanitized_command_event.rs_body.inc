use super::sanitized_command_projection::{
    CommandCapability, SanitizedCommandCapabilityRejection, SanitizedCommandDropdownItem,
    SanitizedCommandItem, SanitizedCommandProjection,
};
use katana_ui_core::molecule::command_chrome::{
    CommandChromeToolbarEvent, FloatingCommandToolbarEvent,
};
use sha2::{Digest, Sha256};

const SHA256_BYTES: usize = 32;

pub(super) struct SanitizedCommandActivationTransport {
    target: SanitizedCommandRoutedTarget,
    revision: u64,
    correlation: String,
}

struct SanitizedCommandRoutedTarget {
    signature: [u8; SHA256_BYTES],
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
    let Some((group, item)) = projection
        .groups()
        .iter()
        .filter(|group| group.visible())
        .find_map(|group| {
            group
                .items()
                .iter()
                .filter(|item| item.visible())
                .find(|item| target_id("kuc-command", item) == action_id)
                .map(|item| (group, item))
        })
    else {
        return Err(SanitizedCommandCapabilityRejection::Missing);
    };
    if !command_and_group_enabled(group.enabled(), item.enabled()) {
        return Ok(None);
    }
    let Some(dropdown) = item
        .dropdown_items()
        .iter()
        .filter(|item| item.visible())
        .find(|dropdown| target_id("kuc-dropdown", dropdown) == item_id)
    else {
        return Err(SanitizedCommandCapabilityRejection::Missing);
    };
    if !dropdown.enabled() {
        return Ok(None);
    }
    activation(dropdown.target(), revision, correlation).map(Some)
}

fn command_and_group_enabled(group_enabled: bool, item_enabled: bool) -> bool {
    group_enabled && item_enabled
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

impl<T> TargetFingerprint for &T
where
    T: TargetFingerprint + ?Sized,
{
    fn target_fingerprint(&self) -> String {
        (**self).target_fingerprint()
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
mod tests {
    use super::*;
    use crate::text_command_surface::{
        SanitizedCommandDropdownItem, SanitizedCommandGroup, SanitizedCommandItem,
        SanitizedCommandProjection, SanitizedCommandTarget,
    };
    use katana_ui_core::molecule::command_chrome::{
        CommandChromeToolbarEvent, FloatingCommandToolbarEvent,
    };
    use sha2::{Digest, Sha256};
    use std::cell::RefCell;

    fn successful_capability() -> Result<(), ()> {
        Ok(())
    }
    use std::rc::Rc;

    #[test]
    fn command_and_group_must_both_be_enabled() {
        assert!(command_and_group_enabled(true, true));
        assert!(!command_and_group_enabled(false, true));
        assert!(!command_and_group_enabled(true, false));
    }

    #[test]
    fn floating_toolbar_activation_routes_only_to_floating_opaque_target_once() {
        let top_calls = Rc::new(RefCell::new(0_u32));
        let floating_calls = Rc::new(RefCell::new(0_u32));
        let top_bytes = b"top-target-secret";
        let floating_bytes = b"floating-target-secret";
        let floating_action_id = format!("kuc-command-{}", hex::encode(Sha256::digest(floating_bytes)));

        let target = |bytes: &[u8], calls: Rc<RefCell<u32>>| {
            SanitizedCommandTarget::from_opaque_bytes(bytes.to_vec()).with_unit_capability(
                move || {
                    *calls.borrow_mut() += 1;
                    Ok::<(), ()>(())
                },
            )
        };
        let top_projection =
            SanitizedCommandProjection::new([SanitizedCommandGroup::new(0, "トップ 日本語 ⭐️")
                .item(SanitizedCommandItem::new(
                    target(top_bytes, top_calls.clone()),
                    0,
                    "トップ操作 日本語 ⭐️",
                ))]);
        let floating_projection = SanitizedCommandProjection::new([SanitizedCommandGroup::new(
            0,
            "フローティング 日本語 ⭐️",
        )
        .item(SanitizedCommandItem::new(
            target(floating_bytes, floating_calls.clone()),
            0,
            "フローティング操作 日本語 ⭐️",
        ))]);

        let mut transports = route_command_events(
            Some(&top_projection),
            Some(&floating_projection),
            &[CommandChromeToolbarEvent::OverflowOpened],
            &[FloatingCommandToolbarEvent::Toolbar {
                event: CommandChromeToolbarEvent::CommandActivated {
                    action_id: floating_action_id.into(),
                },
            }],
            17,
            "root-fingerprint",
        )
        .expect("floating activation routes");

        assert_eq!(transports.len(), 1);
        let transport_debug = format!("{transports:?}");
        for forbidden in [
            "top-target-secret",
            "floating-target-secret",
            "トップ操作 日本語 ⭐️",
            "フローティング操作 日本語 ⭐️",
        ] {
            assert!(
                !transport_debug.contains(forbidden),
                "transport Debug leaked `{forbidden}`: {transport_debug}"
            );
        }

        let transport = transports.first_mut().expect("one floating transport");
        assert_eq!(transport.invoke_once(), Ok(()));
        assert_eq!(*floating_calls.borrow(), 1);
        assert_eq!(*top_calls.borrow(), 0);
        assert_eq!(
            transport.invoke_once(),
            Err(SanitizedCommandCapabilityRejection::Missing)
        );
        assert_eq!(*floating_calls.borrow(), 1);
        assert_eq!(*top_calls.borrow(), 0);
    }

    #[test]
    fn direct_and_dropdown_routing_fail_closed_for_disabled_or_unknown_targets() {
        assert!(successful_capability().is_ok());
        let target = |value: &'static [u8]| {
            SanitizedCommandTarget::from_opaque_bytes(value)
                .with_unit_capability(successful_capability)
        };
        let projection = SanitizedCommandProjection::new([
            SanitizedCommandGroup::new(0, "group")
                .item(SanitizedCommandItem::new(target(b"direct"), 0, "direct"))
                .item(
                    SanitizedCommandItem::new(target(b"disabled"), 1, "disabled")
                        .enabled_state(false),
                )
                .item(
                    SanitizedCommandItem::new(target(b"dropdown"), 2, "dropdown")
                        .dropdown_item(
                            SanitizedCommandDropdownItem::new(
                                target(b"dropdown-disabled"),
                                0,
                                "disabled choice",
                            )
                            .enabled_state(false),
                        ),
                ),
        ]);
        let items = projection.groups()[0].items();
        let direct_id = target_id("kuc-command", &items[0]);
        let disabled_id = target_id("kuc-command", &items[1]);
        let dropdown_id = target_id("kuc-command", &items[2]);
        let disabled_choice_id = target_id("kuc-dropdown", &items[2].dropdown_items()[0]);

        let accelerator = CommandChromeToolbarEvent::AcceleratorTriggered {
            action_id: direct_id.into(),
            combo: katana_ui_core::molecule::toolbar::KeyCombo::command_or_control("d"),
        };
        assert_eq!(
            route_command_events(Some(&projection), None, &[accelerator], &[], 1, "root")
                .expect("enabled accelerator routes")
                .len(),
            1
        );
        assert!(
            route_command_events(
                Some(&projection),
                None,
                &[CommandChromeToolbarEvent::CommandActivated {
                    action_id: disabled_id.into(),
                }],
                &[],
                1,
                "root",
            )
            .expect("disabled command is ignored")
            .is_empty()
        );
        assert!(matches!(
            route_command_events(
                Some(&projection),
                None,
                &[CommandChromeToolbarEvent::CommandActivated {
                    action_id: "unknown".into(),
                }],
                &[],
                1,
                "root",
            ),
            Err(SanitizedCommandCapabilityRejection::Missing)
        ));

        let disabled_group_projection = SanitizedCommandProjection::new([
            SanitizedCommandGroup::new(0, "disabled group")
                .enabled_state(false)
                .item(
                    SanitizedCommandItem::new(target(b"group-dropdown"), 0, "dropdown").dropdown_item(
                        SanitizedCommandDropdownItem::new(target(b"choice"), 0, "choice"),
                    ),
                ),
        ]);
        let disabled_group_item = &disabled_group_projection.groups()[0].items()[0];
        assert!(
            route_dropdown(
                &disabled_group_projection,
                &target_id("kuc-command", disabled_group_item),
                &target_id("kuc-dropdown", &disabled_group_item.dropdown_items()[0]),
                1,
                "root",
            )
            .expect("disabled group is ignored")
            .is_none()
        );
        assert!(
            route_command_events(
                Some(&projection),
                None,
                &[CommandChromeToolbarEvent::DropdownItemActivated {
                    action_id: dropdown_id.clone().into(),
                    item_id: disabled_choice_id.into(),
                }],
                &[],
                1,
                "root",
            )
            .expect("disabled dropdown command is ignored")
            .is_empty()
        );
        assert!(matches!(
            route_command_events(
                Some(&projection),
                None,
                &[CommandChromeToolbarEvent::DropdownItemActivated {
                    action_id: dropdown_id.into(),
                    item_id: "unknown".into(),
                }],
                &[],
                1,
                "root",
            ),
            Err(SanitizedCommandCapabilityRejection::Missing)
        ));
        assert!(matches!(
            route_command_events(
                Some(&projection),
                None,
                &[CommandChromeToolbarEvent::DropdownItemActivated {
                    action_id: "unknown".into(),
                    item_id: "unknown".into(),
                }],
                &[],
                1,
                "root",
            ),
            Err(SanitizedCommandCapabilityRejection::Missing)
        ));
    }
}
