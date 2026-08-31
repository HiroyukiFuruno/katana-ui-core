#[test]
fn wrong_capability_kind_does_not_call_callback_and_consumes_transport_once() {
    let calls = Rc::new(RefCell::new(Vec::new()));
    let target = SanitizedSearchTarget::from_opaque_bytes([0x73, 0x65, 0x63, 0x72, 0x65, 0x74])
        .with_unit_capability({
            let calls = calls.clone();
            move |operation| {
                calls.borrow_mut().push(operation);
                Ok::<(), ()>(())
            }
        });
    let mut transport = SanitizedSearchEventTransport {
        target: SanitizedSearchRoutedTarget::from_target(&target),
        kind: SanitizedSearchEventKind::Query,
        text: Some(SanitizedSearchOneShotText::new("日本語 ⭐️👩‍💻".to_owned())),
        unit_value: None,
        revision: 1,
        correlation: "correlation".to_owned(),
    };

    assert_eq!(
        transport.invoke_once(),
        Err(SanitizedSearchCapabilityRejection::WrongOperation)
    );
    assert!(calls.borrow().is_empty());
    assert_eq!(
        transport.invoke_once(),
        Err(SanitizedSearchCapabilityRejection::AlreadyConsumed)
    );
    assert!(matches!(
        target.capability.as_ref(),
        Some(SanitizedSearchCapability::Unit(_))
    ));
    let debug = format!("{transport:?}");
    for forbidden in ["日本語", "⭐️", "👩‍💻"] {
        assert!(!debug.contains(forbidden));
    }
}

#[test]
fn text_capability_invokes_text_operation_once() {
    let calls = Rc::new(RefCell::new(Vec::<(String, String)>::new()));
    let target = SanitizedSearchTarget::from_opaque_bytes([0x70, 0x75, 0x72, 0x65])
        .with_text_capability({
            let calls = calls.clone();
            move |operation, value| {
                calls.borrow_mut().push((format!("{operation:?}"), value));
                Ok::<(), ()>(())
            }
        });
    let mut transport = SanitizedSearchEventTransport {
        target: SanitizedSearchRoutedTarget::from_target(&target),
        kind: SanitizedSearchEventKind::Query,
        text: Some(SanitizedSearchOneShotText::new("hello".to_owned())),
        unit_value: None,
        revision: 42,
        correlation: "correlation".to_owned(),
    };

    assert_eq!(transport.invoke_once(), Ok(()));
    assert_eq!(
        transport.invoke_once(),
        Err(SanitizedSearchCapabilityRejection::AlreadyConsumed)
    );
    assert_eq!(
        *calls.borrow(),
        vec![(
            format!("{:?}", SanitizedSearchTextOperation::Query),
            "hello".to_owned()
        )]
    );
}

#[test]
fn unit_capability_invokes_unit_operation_once() {
    let calls = Rc::new(RefCell::new(Vec::<String>::new()));
    let target = SanitizedSearchTarget::from_opaque_bytes([0x6d, 0x61, 0x74, 0x63, 0x68])
        .with_unit_capability({
            let calls = calls.clone();
            move |operation| {
                calls.borrow_mut().push(format!("{operation:?}"));
                Ok::<(), ()>(())
            }
        });
    let mut transport = SanitizedSearchEventTransport {
        target: SanitizedSearchRoutedTarget::from_target(&target),
        kind: SanitizedSearchEventKind::MatchCase,
        text: None,
        unit_value: Some(true),
        revision: 7,
        correlation: "correlation".to_owned(),
    };

    assert_eq!(transport.invoke_once(), Ok(()));
    assert_eq!(
        transport.invoke_once(),
        Err(SanitizedSearchCapabilityRejection::AlreadyConsumed)
    );
    assert_eq!(
        *calls.borrow(),
        vec![format!(
            "{:?}",
            SanitizedSearchUnitOperation::MatchCase(true)
        )]
    );
}

#[test]
fn wrong_text_payload_shape_is_rejected() {
    let called = Rc::new(RefCell::new(false));
    let target = SanitizedSearchTarget::from_opaque_bytes([0x74, 0x65, 0x78, 0x74])
        .with_text_capability({
            let called = called.clone();
            move |_, _| {
                *called.borrow_mut() = true;
                Ok::<(), ()>(())
            }
        });
    let mut transport = SanitizedSearchEventTransport {
        target: SanitizedSearchRoutedTarget::from_target(&target),
        kind: SanitizedSearchEventKind::MatchCase,
        text: None,
        unit_value: Some(true),
        revision: 1,
        correlation: "correlation".to_owned(),
    };

    assert_eq!(
        transport.invoke_once(),
        Err(SanitizedSearchCapabilityRejection::WrongOperation)
    );
    assert!(!*called.borrow());
}

#[test]
fn unit_payload_requires_value() {
    let called = Rc::new(RefCell::new(false));
    let target = SanitizedSearchTarget::from_opaque_bytes([0x75, 0x6e, 0x69, 0x74])
        .with_unit_capability({
            let called = called.clone();
            move |_| {
                *called.borrow_mut() = true;
                Ok::<(), ()>(())
            }
        });
    let mut transport = SanitizedSearchEventTransport {
        target: SanitizedSearchRoutedTarget::from_target(&target),
        kind: SanitizedSearchEventKind::MatchCase,
        text: None,
        unit_value: None,
        revision: 1,
        correlation: "correlation".to_owned(),
    };

    assert_eq!(
        transport.invoke_once(),
        Err(SanitizedSearchCapabilityRejection::WrongOperation)
    );
    assert!(!*called.borrow());
}

#[test]
fn routing_with_no_search_projection_is_closed() {
    let events = route_search_events(
        None,
        &[CommandChromeSearchEvent::Strip {
            event: SearchControlStripEvent::SearchQueryChanged("query".to_owned()),
        }],
        1,
        "root-id",
    )
    .expect("no projection means no routing");

    assert!(events.is_empty());
}

#[test]
fn routing_ignores_non_routed_search_event_variants() {
    let projection = SanitizedSearchProjectionBuilder::new()
        .localized_presentation(localized_presentation())
        .query_target(
            SanitizedSearchTarget::from_opaque_bytes(b"search-proxy")
                .with_text_capability(|_, _| Ok::<(), ()>(())),
        )
        .build()
        .expect("projection valid");

    let events = route_search_events(
        Some(&projection),
        &[
            CommandChromeSearchEvent::Strip {
                event: SearchControlStripEvent::ReplaceModeChanged(ReplaceMode::Visible),
            },
            CommandChromeSearchEvent::Strip {
                event: SearchControlStripEvent::SearchResultPositionChanged {
                    result_count: 2,
                    active_index: Some(1),
                },
            },
        ],
        2,
        "root-id",
    )
    .expect("replace/position events are intentionally ignored");

    assert!(events.is_empty());
}

#[test]
fn routing_discards_disabled_search_operation() {
    let projection = SanitizedSearchProjectionBuilder::new()
        .localized_presentation(localized_presentation())
        .build()
        .expect("no enabled targets is valid with default controls");

    let transports = route_search_events(
        Some(&projection),
        &[CommandChromeSearchEvent::CloseRequested],
        3,
        "root-id",
    )
    .expect("disabled operations are fail-closed to no route");

    assert!(transports.is_empty());
}
