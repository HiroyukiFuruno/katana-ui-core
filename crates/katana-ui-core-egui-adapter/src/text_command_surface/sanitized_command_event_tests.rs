use super::{SanitizedCommandCapabilityRejection, route_command_events};
use crate::text_command_surface::{
    SanitizedCommandDropdownItem, SanitizedCommandGroup, SanitizedCommandItem,
    SanitizedCommandProjection, SanitizedCommandTarget,
};
use katana_ui_core::molecule::command_chrome::{
    CommandChromeToolbarEvent, FloatingCommandToolbarEvent,
};
use katana_ui_core::molecule::toolbar::KeyCombo;
use sha2::{Digest, Sha256};
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn floating_toolbar_activation_routes_only_to_floating_opaque_target_once() {
    let top_calls = Rc::new(RefCell::new(0_u32));
    let floating_calls = Rc::new(RefCell::new(0_u32));
    let top_bytes = b"top-target-secret";
    let floating_bytes = b"floating-target-secret";
    let floating_action_id = format!(
        "kuc-command-{}",
        hex::encode(Sha256::digest(floating_bytes))
    );

    let target = |bytes: &[u8], calls: Rc<RefCell<u32>>| {
        SanitizedCommandTarget::from_opaque_bytes(bytes.to_vec()).with_unit_capability(move || {
            *calls.borrow_mut() += 1;
            Ok::<(), ()>(())
        })
    };
    let top_projection =
        SanitizedCommandProjection::new([SanitizedCommandGroup::new(0, "トップ 日本語 ⭐️").item(
            SanitizedCommandItem::new(
                target(top_bytes, top_calls.clone()),
                0,
                "トップ操作 日本語 ⭐️",
            ),
        )]);
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
fn top_toolbar_activation_routes_opaque_target_and_filters_disabled_items() {
    let disabled_calls = Rc::new(RefCell::new(0_u32));
    let ready_calls = Rc::new(RefCell::new(0_u32));
    let target = |bytes: &[u8], calls: &Rc<RefCell<u32>>| {
        SanitizedCommandTarget::from_opaque_bytes(bytes.to_vec()).with_unit_capability({
            let calls = Rc::clone(calls);
            move || {
                *calls.borrow_mut() += 1;
                Ok::<(), ()>(())
            }
        })
    };
    let top_projection = SanitizedCommandProjection::new([SanitizedCommandGroup::new(1, "top")
        .item(
            SanitizedCommandItem::new(target(b"disabled", &disabled_calls), 0, "disabled")
                .enabled_state(false),
        )
        .item(SanitizedCommandItem::new(
            target(b"ready", &ready_calls),
            1,
            "ready",
        ))]);

    let id_missing = format!("kuc-command-{}", hex::encode(Sha256::digest(b"disabled")));
    let transports = route_command_events(
        Some(&top_projection),
        None,
        &[CommandChromeToolbarEvent::CommandActivated {
            action_id: id_missing.into(),
        }],
        &[],
        2,
        "root-fingerprint",
    )
    .expect("missing due disabled state");
    assert!(transports.is_empty());

    let ready_id = format!("kuc-command-{}", hex::encode(Sha256::digest(b"ready")));
    let mut transports = route_command_events(
        Some(&top_projection),
        None,
        &[CommandChromeToolbarEvent::CommandActivated {
            action_id: ready_id.into(),
        }],
        &[],
        2,
        "root-fingerprint",
    )
    .expect("top path should be available");
    assert_eq!(transports.len(), 1);
    assert_eq!(*ready_calls.borrow(), 0);
    assert_eq!(transports[0].invoke_once(), Ok(()));
    assert_eq!(*ready_calls.borrow(), 1);
    assert_eq!(
        transports[0].invoke_once(),
        Err(SanitizedCommandCapabilityRejection::Missing)
    );
    assert_eq!(*ready_calls.borrow(), 1);
    assert_eq!(*disabled_calls.borrow(), 0);
}

#[test]
fn dropdown_routes_missing_item_as_error_and_accepts_matching_submenu_item() {
    let dropdown_calls = Rc::new(RefCell::new(0_u32));
    let dropdown_target = SanitizedCommandTarget::from_opaque_bytes(b"sub").with_unit_capability({
        let called = Rc::clone(&dropdown_calls);
        move || {
            *called.borrow_mut() += 1;
            Ok::<(), ()>(())
        }
    });
    let dropdown_item = SanitizedCommandDropdownItem::new(dropdown_target, 0, "leaf");
    let action = SanitizedCommandItem::new(
        SanitizedCommandTarget::from_opaque_bytes(b"root"),
        0,
        "root",
    )
    .dropdown_item(dropdown_item);
    let projection =
        SanitizedCommandProjection::new([SanitizedCommandGroup::new(1, "top").item(action)]);
    let action_id = format!(
        concat!("kuc-command-", "{}"),
        hex::encode(Sha256::digest(b"root"))
    );
    let wrong_item_id = format!("kuc-dropdown-{}", hex::encode(Sha256::digest(b"other")));

    assert!(matches!(
        route_command_events(
            Some(&projection),
            None,
            &[CommandChromeToolbarEvent::DropdownItemActivated {
                action_id: action_id.clone().into(),
                item_id: wrong_item_id.into(),
            }],
            &[],
            2,
            "root-fingerprint",
        ),
        Err(SanitizedCommandCapabilityRejection::Missing)
    ));

    let leaf_id = format!("kuc-dropdown-{}", hex::encode(Sha256::digest(b"sub")));
    let mut transports = route_command_events(
        Some(&projection),
        None,
        &[CommandChromeToolbarEvent::DropdownItemActivated {
            action_id: action_id.into(),
            item_id: leaf_id.into(),
        }],
        &[],
        2,
        "root-fingerprint",
    )
    .expect("dropdown item exists");
    assert_eq!(transports.len(), 1);
    assert_eq!(*dropdown_calls.borrow(), 0);
    assert_eq!(transports[0].invoke_once(), Ok(()));
    assert_eq!(*dropdown_calls.borrow(), 1);
}

#[test]
fn top_toolbar_returns_missing_for_non_matching_action_id() {
    let target = SanitizedCommandTarget::from_opaque_bytes(b"exists").with_unit_capability({
        let called = Rc::new(RefCell::new(0_u32));
        move || {
            *called.borrow_mut() += 1;
            Ok::<(), ()>(())
        }
    });
    let top_projection =
        SanitizedCommandProjection::new([SanitizedCommandGroup::new(1, "top")
            .item(SanitizedCommandItem::new(target, 1, "exists"))]);
    let action_id = format!("kuc-command-{}", hex::encode(Sha256::digest(b"other")));

    assert!(matches!(
        route_command_events(
            Some(&top_projection),
            None,
            &[CommandChromeToolbarEvent::CommandActivated {
                action_id: action_id.into(),
            }],
            &[],
            3,
            "root-fingerprint",
        ),
        Err(SanitizedCommandCapabilityRejection::Missing)
    ));
}

#[test]
fn accelerator_event_routes_like_command_activation() {
    let calls = Rc::new(RefCell::new(0_u32));
    let projection = SanitizedCommandProjection::new([SanitizedCommandGroup::new(1, "top").item(
        SanitizedCommandItem::new(
            SanitizedCommandTarget::from_opaque_bytes(b"accel").with_unit_capability({
                let calls = Rc::clone(&calls);
                move || {
                    *calls.borrow_mut() += 1;
                    Ok::<(), ()>(())
                }
            }),
            1,
            "accelerator",
        ),
    )]);
    let action_id = format!("kuc-command-{}", hex::encode(Sha256::digest(b"accel")));
    let mut transports = route_command_events(
        Some(&projection),
        None,
        &[CommandChromeToolbarEvent::AcceleratorTriggered {
            action_id: action_id.into(),
            combo: KeyCombo::command_or_control("a"),
        }],
        &[],
        3,
        "root-fingerprint",
    )
    .expect("accelerator path should route to same dispatch");

    assert_eq!(transports.len(), 1);
    assert_eq!(transports[0].invoke_once(), Ok(()));
    assert_eq!(*calls.borrow(), 1);
}

#[test]
fn dropdown_disabled_item_results_no_transport() {
    let disabled_dropdown = SanitizedCommandDropdownItem::new(
        SanitizedCommandTarget::from_opaque_bytes(b"disabled"),
        0,
        "leaf",
    )
    .enabled_state(false);
    let action_id = format!("kuc-command-{}", hex::encode(Sha256::digest(b"root")));
    let leaf_id = format!("kuc-dropdown-{}", hex::encode(Sha256::digest(b"disabled")));
    let projection = SanitizedCommandProjection::new([SanitizedCommandGroup::new(1, "top").item(
        SanitizedCommandItem::new(
            SanitizedCommandTarget::from_opaque_bytes(b"root"),
            1,
            "root",
        )
        .dropdown_item(disabled_dropdown),
    )]);

    let transports = route_command_events(
        Some(&projection),
        None,
        &[CommandChromeToolbarEvent::DropdownItemActivated {
            action_id: action_id.into(),
            item_id: leaf_id.into(),
        }],
        &[],
        3,
        "root-fingerprint",
    )
    .expect("disabled dropdown item is intentionally ignored");

    assert!(transports.is_empty());
}

#[test]
fn top_toolbar_action_with_callback_rejected_bubbles_up() {
    let calls = Rc::new(RefCell::new(0_u32));
    let projection = SanitizedCommandProjection::new([SanitizedCommandGroup::new(1, "top").item(
        SanitizedCommandItem::new(
            SanitizedCommandTarget::from_opaque_bytes(b"reject").with_unit_capability({
                let calls = Rc::clone(&calls);
                move || {
                    *calls.borrow_mut() += 1;
                    Err::<(), &'static str>("rejected")
                }
            }),
            1,
            "reject",
        ),
    )]);
    let action_id = format!("kuc-command-{}", hex::encode(Sha256::digest(b"reject")));
    let mut transports = route_command_events(
        Some(&projection),
        None,
        &[CommandChromeToolbarEvent::CommandActivated {
            action_id: action_id.into(),
        }],
        &[],
        3,
        "root-fingerprint",
    )
    .expect("routing still produces transport");

    assert_eq!(transports.len(), 1);
    assert_eq!(*calls.borrow(), 0);
    assert_eq!(
        transports[0].invoke_once(),
        Err(SanitizedCommandCapabilityRejection::CallbackRejected)
    );
    assert_eq!(*calls.borrow(), 1);
}

#[test]
fn dropdown_routing_skips_non_matching_visible_items_before_reporting_missing() {
    let projection = SanitizedCommandProjection::new([SanitizedCommandGroup::new(1, "top").item(
        SanitizedCommandItem::new(
            SanitizedCommandTarget::from_opaque_bytes(b"other"),
            1,
            "other",
        ),
    )]);

    let error = route_command_events(
        Some(&projection),
        None,
        &[CommandChromeToolbarEvent::DropdownItemActivated {
            action_id: "kuc-command-missing".into(),
            item_id: "kuc-dropdown-missing".into(),
        }],
        &[],
        1,
        "root-fingerprint",
    )
    .expect_err("a missing dropdown action must fail closed");

    assert_eq!(error, SanitizedCommandCapabilityRejection::Missing);
}

#[test]
fn dropdown_routing_ignores_a_disabled_matching_action() {
    let projection = SanitizedCommandProjection::new([SanitizedCommandGroup::new(1, "top").item(
        SanitizedCommandItem::new(
            SanitizedCommandTarget::from_opaque_bytes(b"root"),
            1,
            "root",
        )
        .enabled_state(false),
    )]);
    let action_id = format!("kuc-command-{}", hex::encode(Sha256::digest(b"root")));

    let transports = route_command_events(
        Some(&projection),
        None,
        &[CommandChromeToolbarEvent::DropdownItemActivated {
            action_id: action_id.into(),
            item_id: "kuc-dropdown-any".into(),
        }],
        &[],
        1,
        "root-fingerprint",
    )
    .expect("disabled command actions are ignored");

    assert!(transports.is_empty());
}
