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
