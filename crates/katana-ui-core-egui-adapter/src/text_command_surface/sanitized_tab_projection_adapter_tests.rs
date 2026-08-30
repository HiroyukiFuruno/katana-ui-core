    use super::{
        SanitizedTabGroup, SanitizedTabProjectionAdapter, SanitizedTabProjectionClosedEvent,
        projection_to_state, projection_to_strip, structural_id,
    };
    use crate::text_command_surface::sanitized_document_root::sanitized_tab_projection::{
        SanitizedTab, SanitizedTabCapabilities, SanitizedTabClosePresentation,
        SanitizedTabGroupTarget, SanitizedTabProjection, SanitizedTabTarget,
    };
    use katana_ui_core::molecule::structured::{
        CloseableTabGroupId, CloseableTabId, CloseableTabStripEvent,
    };
    use katana_ui_core::render_model::UiIconProps;

    const SCREEN_SIZE: egui::Vec2 = egui::vec2(600.0, 160.0);

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

        assert!(
            routes
                .route_event(&CloseableTabStripEvent::TabSelected {
                    tab_id: CloseableTabId::new("stale-tab-widget"),
                })
                .is_none()
        );
        assert!(
            routes
                .route_event(&CloseableTabStripEvent::GroupCollapseChanged {
                    group_id: CloseableTabGroupId::new("stale-group-widget"),
                    collapsed: true,
                })
                .is_none()
        );
        assert!(
            routes
                .route_event(&CloseableTabStripEvent::TabCloseRequested {
                    tab_id: CloseableTabId::new("stale-tab-widget"),
                })
                .is_none()
        );
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

        let source = include_str!("sanitized_tab_projection_adapter.rs");
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
        assert!(released
            .boundary_facts()
            .events
            .iter()
            .all(|event| {
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

        assert!(
            routes
                .route_event(&CloseableTabStripEvent::TabAdded {
                    tab_id: CloseableTabId::new("sanitized-tab-unknown"),
                })
                .is_none()
        );
        assert!(
            routes
                .route_event(&CloseableTabStripEvent::TabClosed {
                    tab_id: CloseableTabId::new("sanitized-tab-0-1"),
                })
                .is_none()
        );
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
        let collapsed_fingerprint = super::super::super::sanitized_document_root_transport::tab_event_fingerprint(
            std::slice::from_ref(&event),
        );
        let expanded_event = routes
            .route_event(&CloseableTabStripEvent::GroupCollapseChanged {
                group_id: CloseableTabGroupId::new("sanitized-group-0"),
                collapsed: false,
            })
            .expect("the same group also routes its expanded state");
        let expanded_fingerprint = super::super::super::sanitized_document_root_transport::tab_event_fingerprint(
            std::slice::from_ref(&expanded_event),
        );
        assert_eq!(collapsed_fingerprint.len(), 64);
        assert_eq!(expanded_fingerprint.len(), 64);
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
        assert!(
            released
                .boundary_facts()
                .events
                .iter()
                .all(|event| { matches!(event, CloseableTabStripEvent::TabCloseRequested { .. }) })
        );
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

    fn empty_projection() -> SanitizedTabProjection {
        SanitizedTabProjection::new([])
    }

    fn closeable_projection() -> SanitizedTabProjection {
        SanitizedTabProjection::new([SanitizedTabGroup::new(
            SanitizedTabGroupTarget::from_opaque_bytes([0xaa]),
            0,
            "Documents",
        )
        .tab(
            SanitizedTab::new(SanitizedTabTarget::from_opaque_bytes([0x01]), 0, "First")
                .with_capabilities(SanitizedTabCapabilities::new().active_state(true)),
        )
        .tab(
            SanitizedTab::new(SanitizedTabTarget::from_opaque_bytes([0x02]), 1, "Second")
                .with_capabilities(SanitizedTabCapabilities::new().close_state(true))
                .with_close_presentation(SanitizedTabClosePresentation::new(
                    "×",
                    "Close tab",
                    "Close second tab",
                )),
        )])
    }

    fn selectable_projection() -> SanitizedTabProjection {
        SanitizedTabProjection::new([SanitizedTabGroup::new(
            SanitizedTabGroupTarget::from_opaque_bytes([0xaa, 0xbb]),
            0,
            "Documents",
        )
        .tab(
            SanitizedTab::new(
                SanitizedTabTarget::from_opaque_bytes([0x01, 0x02]),
                0,
                "First",
            )
            .with_capabilities(SanitizedTabCapabilities::new().active_state(true)),
        )
        .tab(SanitizedTab::new(
            SanitizedTabTarget::from_opaque_bytes([0xde, 0xad, 0xbe, 0xef]),
            1,
            "Second",
        )
        .with_icon(UiIconProps::new("<svg/>")))])
    }

    fn run_frame(
        context: &egui::Context,
        adapter: &mut SanitizedTabProjectionAdapter,
        events: Vec<egui::Event>,
    ) -> super::SanitizedTabProjectionFrame {
        let mut output = None;
        let mut platform_output = context.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, SCREEN_SIZE)),
                events,
                ..egui::RawInput::default()
            },
            |ui| output = Some(adapter.show(ui).expect("sanitized tab render succeeds")),
        );
        platform_output.textures_delta.clear();
        output.expect("sanitized tab frame is produced")
    }

    fn pointer_button(pos: egui::Pos2, pressed: bool) -> egui::Event {
        egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::default(),
        }
    }
