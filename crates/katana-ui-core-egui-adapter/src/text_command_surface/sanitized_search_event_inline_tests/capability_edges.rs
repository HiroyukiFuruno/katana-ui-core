#[test]
fn routing_reports_wrong_operation_for_mismatched_capability_kind() {
    let projection = SanitizedSearchProjectionBuilder::new()
        .localized_presentation(localized_presentation())
        .query_target(
            SanitizedSearchTarget::from_opaque_bytes(b"query-unit-only")
                .with_unit_capability(|_| Ok::<(), ()>(())),
        )
        .build()
        .expect("projection valid");

    let err = route_search_events(
        Some(&projection),
        &[CommandChromeSearchEvent::Strip {
            event: SearchControlStripEvent::SearchQueryChanged("query".to_owned()),
        }],
        4,
        "root-id",
    )
    .expect_err("query operation requires text capability");

    assert_eq!(err, SanitizedSearchCapabilityRejection::WrongOperation);
}

#[test]
fn routed_query_invokes_the_real_text_capability_once() {
    let calls = Rc::new(RefCell::new(Vec::new()));
    let projection = SanitizedSearchProjectionBuilder::new()
        .localized_presentation(localized_presentation())
        .query_target(
            SanitizedSearchTarget::from_opaque_bytes(b"routed-query").with_text_capability({
                let calls = Rc::clone(&calls);
                move |operation, value| {
                    calls.borrow_mut().push((operation, value));
                    Ok::<(), ()>(())
                }
            }),
        )
        .build()
        .expect("projection valid");
    let mut transports = route_search_events(
        Some(&projection),
        &[CommandChromeSearchEvent::Strip {
            event: SearchControlStripEvent::SearchQueryChanged("日本語 ⭐️".to_owned()),
        }],
        5,
        "root-id",
    )
    .expect("the query is routed through its declared text capability");

    assert_eq!(transports.len(), 1);
    assert_eq!(transports[0].invoke_once(), Ok(()));
    assert_eq!(
        *calls.borrow(),
        vec![(SanitizedSearchTextOperation::Query, "日本語 ⭐️".to_owned())]
    );
    assert_eq!(
        transports[0].invoke_once(),
        Err(SanitizedSearchCapabilityRejection::AlreadyConsumed)
    );
}

#[test]
fn transport_invoke_once_rejects_reentrant_usage_while_capability_is_mut_borrowed() {
    let called = Rc::new(RefCell::new(0_u32));
    let target = SanitizedSearchTarget::from_opaque_bytes([1, 2, 3]).with_unit_capability({
        let called = Rc::clone(&called);
        move |_| {
            *called.borrow_mut() += 1;
            Ok::<(), ()>(())
        }
    });
    let mut transport = SanitizedSearchEventTransport {
        target: SanitizedSearchRoutedTarget::from_target(&target),
        kind: SanitizedSearchEventKind::MatchCase,
        text: None,
        unit_value: Some(true),
        revision: 1,
        correlation: "root-id".to_owned(),
    };
    let borrow = match &target.capability {
        Some(SanitizedSearchCapability::Unit(slot)) => slot.try_borrow_mut(),
        Some(SanitizedSearchCapability::Text(_)) => unreachable!(),
        None => unreachable!(),
    }
    .expect("borrow for reentrant simulation");

    let first = transport.invoke_once();
    assert_eq!(first, Err(SanitizedSearchCapabilityRejection::Reentrant));
    assert_eq!(*called.borrow(), 0);
    drop(borrow);
    assert_eq!(
        transport.invoke_once(),
        Err(SanitizedSearchCapabilityRejection::AlreadyConsumed)
    );
    assert_eq!(*called.borrow(), 0);
}

#[test]
fn transport_invoke_once_propagates_callback_rejection() {
    let target = SanitizedSearchTarget::from_opaque_bytes([9, 9, 9])
        .with_unit_capability(|_| Err::<(), &'static str>("callback rejected"));
    let mut transport = SanitizedSearchEventTransport {
        target: SanitizedSearchRoutedTarget::from_target(&target),
        kind: SanitizedSearchEventKind::MatchCase,
        text: None,
        unit_value: Some(false),
        revision: 10,
        correlation: "root-id".to_owned(),
    };

    assert_eq!(
        transport.invoke_once(),
        Err(SanitizedSearchCapabilityRejection::CallbackRejected)
    );
}

#[test]
fn target_and_payload_fmt_are_opaque() {
    let target =
        SanitizedSearchTarget::from_opaque_bytes([0x70, 0x75, 0x74, 0x2d, 0x6f, 0x70, 0x61, 0x71])
            .with_unit_capability(|_| Ok::<(), ()>(()));
    let target = SanitizedSearchRoutedTarget::from_target(&target);
    let message_target = format!("{target:?}");
    assert_eq!(message_target, "SanitizedSearchRoutedTarget(..)");

    let message_payload = format!(
        "{:?}",
        SanitizedSearchOneShotText::new("private".to_owned())
    );
    assert_eq!(message_payload, "SanitizedSearchOneShotText(..)");
}

#[test]
fn unit_capability_rejects_a_text_operation_without_invoking_the_callback() {
    let calls = Rc::new(RefCell::new(0_u32));
    let target = SanitizedSearchTarget::from_opaque_bytes([0x75]).with_unit_capability({
        let calls = Rc::clone(&calls);
        move |_| {
            *calls.borrow_mut() += 1;
            Ok::<(), ()>(())
        }
    });
    let mut transport = SanitizedSearchEventTransport {
        target: SanitizedSearchRoutedTarget::from_target(&target),
        kind: SanitizedSearchEventKind::Query,
        text: None,
        unit_value: None,
        revision: 1,
        correlation: "correlation".to_owned(),
    };

    assert_eq!(
        transport.invoke_once(),
        Err(SanitizedSearchCapabilityRejection::WrongOperation)
    );
    assert_eq!(*calls.borrow(), 0);
}
