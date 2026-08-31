#[test]
fn stale_route_ids_are_ignored_after_projection_replace() {
    let projection = selectable_projection();
    let mut adapter = SanitizedTabProjectionAdapter::from_projection(Some(&projection));
    let stale_event = CloseableTabStripEvent::TabSelected {
        tab_id: CloseableTabId::new("sanitized-tab-0-1"),
    };

    assert!(adapter.routes.route_event(&stale_event).is_some());
    adapter.replace_projection(Some(&empty_projection()));
    assert!(adapter.routes.route_event(&stale_event).is_none());
}

#[test]
fn disabled_controls_do_not_emit_close_requests() {
    let projection = selectable_projection();
    let context = egui::Context::default();
    let mut adapter = SanitizedTabProjectionAdapter::from_projection(Some(&projection));
    let frame = run_frame(&context, &mut adapter, Vec::new());
    let target = frame
        .boundary_facts()
        .tab_rects
        .iter()
        .find(|(id, rect)| id == "sanitized-tab-0-1" && rect.width() > 0.0 && rect.height() > 0.0)
        .map(|(_, rect)| rect.center())
        .expect("tab geometry exists");

    assert!(frame.boundary_facts().close_rects.is_empty());
    let pressed = run_frame(&context, &mut adapter, vec![pointer_button(target, true)]);
    let released = run_frame(&context, &mut adapter, vec![pointer_button(target, false)]);
    assert!(pressed.boundary_facts().events.is_empty());
    assert!(released.boundary_facts().events.iter().all(|event| {
        !matches!(
            event,
            CloseableTabStripEvent::TabCloseRequested { .. }
                | CloseableTabStripEvent::TabClosed { .. }
        )
    }));
}

#[test]
fn unknown_and_one_shot_events_do_not_route() {
    let projection = selectable_projection();
    let (_, routes) = projection_to_state(&projection);

    assert!(routes
        .route_event(&CloseableTabStripEvent::TabAdded {
            tab_id: CloseableTabId::new("sanitized-tab-unknown"),
        })
        .is_none());
    assert!(routes
        .route_event(&CloseableTabStripEvent::TabClosed {
            tab_id: CloseableTabId::new("sanitized-tab-0-1"),
        })
        .is_none());
}

#[test]
fn group_collapse_route_event_is_translated_and_fingerprinted_for_transport() {
    let projection = selectable_projection();
    let (_, routes) = projection_to_state(&projection);
    let event = routes
        .route_event(&CloseableTabStripEvent::GroupCollapseChanged {
            group_id: CloseableTabGroupId::new("sanitized-group-0"),
            collapsed: true,
        })
        .expect("recognized structural group id produces a closed event");

    let debug = format!("{event:?}");
    assert_eq!(debug, "SanitizedTabProjectionClosedEvent(..)");
    event.read_for_transport();
    let collapsed_fingerprint =
        super::super::super::sanitized_document_root_transport::tab_event_fingerprint(
            std::slice::from_ref(&event),
        );
    let expanded_event = routes
        .route_event(&CloseableTabStripEvent::GroupCollapseChanged {
            group_id: CloseableTabGroupId::new("sanitized-group-0"),
            collapsed: false,
        })
        .expect("the same group also routes its expanded state");
    let expanded_fingerprint =
        super::super::super::sanitized_document_root_transport::tab_event_fingerprint(
            std::slice::from_ref(&expanded_event),
        );
    assert_eq!(collapsed_fingerprint.len(), FINGERPRINT_HEX_LENGTH);
    assert_eq!(expanded_fingerprint.len(), FINGERPRINT_HEX_LENGTH);
    assert_ne!(collapsed_fingerprint, expanded_fingerprint);

    assert!(matches!(
        event,
        SanitizedTabProjectionClosedEvent::GroupCollapseChanged { target, collapsed }
            if format!("{target:?}") == "SanitizedRoutedGroupTarget(..)" && collapsed
    ));
}

#[test]
fn physical_close_emits_opaque_intent_without_selection_or_mutation() {
    let projection = closeable_projection();
    let context = egui::Context::default();
    let mut adapter = SanitizedTabProjectionAdapter::from_projection(Some(&projection));
    let first = run_frame(&context, &mut adapter, Vec::new());
    let close_target = first
        .boundary_facts()
        .close_rects
        .iter()
        .find(|(id, rect)| id == "sanitized-tab-0-1" && rect.width() > 0.0)
        .map(|(_, rect)| rect.center())
        .expect("close response exists");

    let _ = run_frame(
        &context,
        &mut adapter,
        vec![pointer_button(close_target, true)],
    );
    let released = run_frame(
        &context,
        &mut adapter,
        vec![pointer_button(close_target, false)],
    );
    assert!(released
        .boundary_facts()
        .events
        .iter()
        .all(|event| { matches!(event, CloseableTabStripEvent::TabCloseRequested { .. }) }));
    assert!(matches!(
        released.into_closed_events().as_slice(),
        [SanitizedTabProjectionClosedEvent::TabCloseRequested(_)]
    ));
    assert_eq!(adapter.active_tab_id(), Some("sanitized-tab-0-0"));
    assert_eq!(adapter.strip.options().tabs.len(), 2);
}

#[test]
fn close_capability_without_presentation_has_no_close_affordance() {
    let projection = selectable_projection();
    let context = egui::Context::default();
    let mut adapter = SanitizedTabProjectionAdapter::from_projection(Some(&projection));
    let frame = run_frame(&context, &mut adapter, Vec::new());
    assert!(frame.boundary_facts().close_rects.is_empty());
}
