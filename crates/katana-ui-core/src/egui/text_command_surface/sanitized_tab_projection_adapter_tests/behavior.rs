#[test]
fn nested_groups_map_to_generic_groups_with_parent_relationship_and_order() {
    let child_a = SanitizedTabGroup::new(
        SanitizedTabGroupTarget::from_opaque_bytes([3]),
        30,
        "Child A",
    )
    .tab(SanitizedTab::new(
        SanitizedTabTarget::from_opaque_bytes([3]),
        2,
        "child-tab-a",
    ));
    let child_b = SanitizedTabGroup::new(
        SanitizedTabGroupTarget::from_opaque_bytes([4]),
        10,
        "Child B",
    )
    .tab(SanitizedTab::new(
        SanitizedTabTarget::from_opaque_bytes([4]),
        1,
        "child-tab-b",
    ));
    let parent =
        SanitizedTabGroup::new(SanitizedTabGroupTarget::from_opaque_bytes([2]), 1, "Parent")
            .tab(SanitizedTab::new(
                SanitizedTabTarget::from_opaque_bytes([2]),
                1,
                "parent-tab",
            ))
            .group(child_a)
            .group(child_b);

    let root = SanitizedTabProjection::new(vec![
        SanitizedTabGroup::new(SanitizedTabGroupTarget::from_opaque_bytes([1]), 2, "Root").tab(
            SanitizedTab::new(SanitizedTabTarget::from_opaque_bytes([1]), 1, "root-tab"),
        ),
        parent,
    ]);

    let strip = projection_to_strip(&root);
    let groups = &strip.options().groups;

    let parent_id = structural_id("group", &[1]);
    let child_high_order_id = structural_id("group", &[1, 0]);
    let child_low_order_id = structural_id("group", &[1, 1]);
    let root_id = structural_id("group", &[0]);

    assert_eq!(groups.len(), 4);
    assert_eq!(
        groups
            .iter()
            .map(|group| group.id.as_str())
            .collect::<Vec<_>>(),
        vec![
            parent_id.as_str(),
            child_low_order_id.as_str(),
            child_high_order_id.as_str(),
            root_id.as_str(),
        ]
    );

    let parent_group = groups
        .iter()
        .find(|group| group.id.as_str() == parent_id)
        .expect("parent group exists");
    assert_eq!(parent_group.parent_group_id, None);

    let child_low_order_group = groups
        .iter()
        .find(|group| group.id.as_str() == child_low_order_id)
        .expect("low-order child group exists");
    assert_eq!(
        child_low_order_group
            .parent_group_id
            .as_ref()
            .map(|value| value.as_str()),
        Some(parent_id.as_str())
    );

    let child_high_order_group = groups
        .iter()
        .find(|group| group.id.as_str() == child_high_order_id)
        .expect("high-order child group exists");
    assert_eq!(
        child_high_order_group
            .parent_group_id
            .as_ref()
            .map(|value| value.as_str()),
        Some(parent_id.as_str())
    );

    let root_group = groups.iter().find(|group| group.id.as_str() == root_id);
    let root_group = root_group.expect("root group exists");
    assert_eq!(root_group.parent_group_id, None);
}

#[test]
fn raw_input_pointer_selection_routes_one_opaque_tab_activation() {
    let projection = selectable_projection();
    let context = egui::Context::default();
    let mut adapter = SanitizedTabProjectionAdapter::from_projection(Some(&projection));

    let first = run_frame(&context, &mut adapter, Vec::new());
    let target = first
        .boundary_facts()
        .tab_rects
        .iter()
        .find(|(id, rect)| id == "sanitized-tab-0-1" && rect.width() > 0.0)
        .map(|(_, rect)| rect.center())
        .expect("second tab response exists");

    let _ = run_frame(&context, &mut adapter, vec![pointer_button(target, true)]);
    let released = run_frame(&context, &mut adapter, vec![pointer_button(target, false)]);
    let closed_events = released.into_closed_events();

    assert_eq!(closed_events.len(), 1);
    assert!(matches!(
        closed_events.as_slice(),
        [SanitizedTabProjectionClosedEvent::TabActivated(_)]
    ));
    assert_eq!(adapter.active_tab_id(), Some("sanitized-tab-0-1"));
}

#[test]
fn unknown_structural_widget_ids_fail_closed_for_tab_and_group_events() {
    let projection = selectable_projection();
    let (_, routes) = projection_to_state(&projection);

    assert!(routes
        .route_event(&CloseableTabStripEvent::TabSelected {
            tab_id: CloseableTabId::new("stale-tab-widget"),
        })
        .is_none());
    assert!(routes
        .route_event(&CloseableTabStripEvent::GroupCollapseChanged {
            group_id: CloseableTabGroupId::new("stale-group-widget"),
            collapsed: true,
        })
        .is_none());
    assert!(routes
        .route_event(&CloseableTabStripEvent::TabCloseRequested {
            tab_id: CloseableTabId::new("stale-tab-widget"),
        })
        .is_none());
}

#[test]
fn closed_route_events_and_public_declarations_hide_targets_and_widget_ids() {
    let projection = selectable_projection();
    let (_, routes) = projection_to_state(&projection);
    let event = routes
        .route_event(&CloseableTabStripEvent::TabSelected {
            tab_id: CloseableTabId::new("sanitized-tab-0-1"),
        })
        .expect("recognized internal route produces a closed event");

    let debug = format!("{event:?}");
    assert_eq!(debug, "SanitizedTabProjectionClosedEvent(..)");
    assert!(!debug.contains("de"));
    assert!(!debug.contains("sanitized-tab"));
    event.read_for_transport();
    assert!(matches!(
        &event,
        SanitizedTabProjectionClosedEvent::TabActivated(target)
            if format!("{target:?}") == "SanitizedRoutedTabTarget(..)"
    ));

    let source = include_str!("../sanitized_tab_projection_adapter.rs");
    let production_source = source.split("#[cfg(test)]").next().unwrap_or(source);
    for forbidden_public_declaration in [
        "pub enum SanitizedTabProjectionClosedEvent",
        "pub struct SanitizedRoutedTabTarget",
        "pub struct SanitizedRoutedGroupTarget",
    ] {
        assert!(
            !production_source.contains(forbidden_public_declaration),
            "closed routing leaked a public declaration: {forbidden_public_declaration}"
        );
    }
    assert!(!production_source.contains("CloseTab"));
    assert!(!production_source.contains("TabClosed"));
}
