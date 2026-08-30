use super::super::sanitized_search_projection::{
    SanitizedSearchCapability, SanitizedSearchCapabilityRejection,
    SanitizedSearchLocalizedPresentation, SanitizedSearchProjectionBuilder, SanitizedSearchTarget,
};
use super::{
    SanitizedSearchEventKind, SanitizedSearchEventTransport, SanitizedSearchOneShotText,
    SanitizedSearchRoutedTarget, SanitizedSearchTextOperation, SanitizedSearchUnitOperation,
    route_search_events,
};
use katana_ui_core::molecule::command_chrome::CommandChromeSearchEvent;
use katana_ui_core::molecule::structured::{ReplaceMode, SearchControlStripEvent};
use std::cell::RefCell;
use std::rc::Rc;

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

fn localized_presentation() -> SanitizedSearchLocalizedPresentation {
    use super::super::sanitized_search_projection::SanitizedSearchResultSummaryPresentation;
    use super::super::sanitized_search_projection::SanitizedSearchTextPresentation;
    use super::super::sanitized_search_projection::SanitizedSearchUnavailablePresentation;
    use super::super::sanitized_search_projection::{
        SanitizedSearchControlPresentation, SanitizedSearchOperationPresentation,
    };

    SanitizedSearchLocalizedPresentation::new(
        SanitizedSearchControlPresentation::new(
            SanitizedSearchTextPresentation::new("検索", "検索", "検索"),
            SanitizedSearchTextPresentation::new("検索語", "検索語", "検索語"),
            SanitizedSearchTextPresentation::new("置換", "置換", "置換"),
            SanitizedSearchTextPresentation::new("大文字小文字", "大文字小文字", "大文字小文字"),
            SanitizedSearchTextPresentation::new("単語", "単語", "単語"),
            SanitizedSearchTextPresentation::new("正規表現", "正規表現", "正規表現"),
        ),
        SanitizedSearchOperationPresentation::new(
            SanitizedSearchTextPresentation::new("前へ", "前へ", "前へ"),
            SanitizedSearchTextPresentation::new("次へ", "次へ", "次へ"),
            SanitizedSearchTextPresentation::new("置換", "置換", "置換"),
            SanitizedSearchTextPresentation::new("すべて置換", "すべて置換", "すべて置換"),
            SanitizedSearchTextPresentation::new("閉じる", "閉じる", "閉じる"),
        ),
        SanitizedSearchResultSummaryPresentation::new(
            "検索待機",
            "一致なし",
            "1件",
            "位置",
            "件数",
        ),
        SanitizedSearchUnavailablePresentation::new("未対応", "未対応", "未対応", "未対応"),
    )
}
