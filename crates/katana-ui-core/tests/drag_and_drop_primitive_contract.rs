use katana_ui_core::event::drag::{
    DRAG_CANCEL_REASON_KEYBOARD_ESCAPE, DragEvent, DragEventRouteNode, DragEventRouting,
};
use katana_ui_core::interaction::drag_and_drop::{
    AutoScrollEngine, AutoScrollPolicy, CONSUMER_TAG_PREFIX, DndPoint, DndRect, DragData,
    DragMetadata, DragSource, DropAcceptance, DropEffect, DropIndicator, DropIndicatorKind,
    DropIndicatorOrientation, DropTarget, KUC_TAG_PREFIX, KeyboardDragContext, KeyboardDragKey,
    KeyboardDragPhase, KeyboardDragState, KeyboardDropTargetFocus, OS_FILE_LIST_TAG, OS_TAG_PREFIX,
};
use katana_ui_core::render_model::UiNodeId;

#[test]
fn drop_target_rejects_mismatched_tag_without_indicator() {
    let target = DropTarget::new(UiNodeId::new("attachments")).accepted_tag(OS_FILE_LIST_TAG);
    let data = DragData::new("consumer/tab-id", serde_json::json!("tab-a"));
    let acceptance = target.accept(
        &data,
        DndPoint::new(8.0, 8.0),
        DndRect::new(0.0, 0.0, 100.0, 40.0),
    );

    assert_eq!(DropAcceptance::Reject, acceptance);
    assert!(acceptance.indicator().is_none());
}

#[test]
fn drag_metadata_count_insert_and_reserved_prefixes_are_explicit() {
    let metadata = DragMetadata::new().count(3).insert("source", "explorer");
    assert_eq!(Some("3"), metadata.get("count"));
    assert_eq!(Some("explorer"), metadata.get("source"));

    for prefix in [OS_TAG_PREFIX, KUC_TAG_PREFIX, CONSUMER_TAG_PREFIX] {
        assert!(
            DragData::new(format!("{prefix}item"), serde_json::Value::Null).has_reserved_prefix()
        );
    }
    assert!(!DragData::new("custom/item", serde_json::Value::Null).has_reserved_prefix());
}

#[test]
fn drop_indicator_switches_between_before_inside_and_after() {
    let target = DropTarget::new(UiNodeId::new("tree-row")).accepted_tag("consumer/tree-node");
    let data = DragData::new("consumer/tree-node", serde_json::json!({"id": "a"}));
    let rect = DndRect::new(0.0, 10.0, 240.0, 100.0);

    let before = target.accept(&data, DndPoint::new(16.0, 20.0), rect);
    let inside = target.accept(&data, DndPoint::new(16.0, 60.0), rect);
    let after = target.accept(&data, DndPoint::new(16.0, 104.0), rect);

    assert_eq!(Some(DropIndicatorKind::Before), before.indicator_kind());
    assert_eq!(Some(DropIndicatorKind::Inside), inside.indicator_kind());
    assert_eq!(Some(DropIndicatorKind::After), after.indicator_kind());
}

#[test]
fn horizontal_indicator_zero_extent_and_hidden_acceptance_are_explicit() {
    let data = DragData::new("consumer/tree-node", serde_json::json!({"id": "a"}));
    let mut target = DropTarget::new(UiNodeId::new("tree-row"))
        .accepted_tag("consumer/tree-node")
        .auto_scroll(AutoScrollPolicy::default());
    target.indicator_orientation = DropIndicatorOrientation::Horizontal;
    let rect = DndRect::new(10.0, 20.0, 100.0, 40.0);

    assert_eq!(
        Some(DropIndicatorKind::Before),
        target
            .accept(&data, DndPoint::new(20.0, 30.0), rect)
            .indicator_kind()
    );
    assert_eq!(
        0.0,
        DndRect::new(0.0, 0.0, 0.0, 10.0).horizontal_ratio(DndPoint::new(2.0, 5.0))
    );

    let hidden = DropAcceptance::Accept {
        effect: DropEffect::Move,
        indicator: DropIndicator::hidden(rect),
    };
    assert!(hidden.indicator().is_none());
    assert_eq!(DropEffect::Move, hidden.effect());
    assert_eq!(DropEffect::None, DropAcceptance::Reject.effect());
}

#[test]
fn drag_source_adds_each_allowed_effect_only_once() {
    let source = DragSource::new(
        UiNodeId::new("source"),
        DragData::new("consumer/item", serde_json::Value::Null),
    )
    .allowed_effect(DropEffect::Copy)
    .allowed_effect(DropEffect::Copy);

    assert!(source.allows_effect(DropEffect::Copy));
    assert_eq!(2, source.allowed_effects.len());
}

#[test]
fn keyboard_drag_drops_after_space_pickup_arrow_focus_and_space_drop() {
    let source = DragSource::new(
        UiNodeId::new("tab-a"),
        DragData::new("katana-ui-core/tab-id", serde_json::json!("tab-a"))
            .metadata(DragMetadata::new().label("Tab A")),
    )
    .keyboard_draggable(true)
    .allowed_effect(DropEffect::Move);
    let target = DropTarget::new(UiNodeId::new("tab-b"))
        .accepted_tag("katana-ui-core/tab-id")
        .effect(DropEffect::Move);
    let focus = KeyboardDropTargetFocus::new(
        target,
        DndRect::new(0.0, 0.0, 120.0, 32.0),
        DndPoint::new(90.0, 16.0),
    );
    let state = KeyboardDragState::idle();

    let picked_up = state.handle_key(
        KeyboardDragKey::Space,
        KeyboardDragContext::focused_source(source),
    );
    let moved = picked_up.state.handle_key(
        KeyboardDragKey::ArrowRight,
        KeyboardDragContext::focused_target(focus.clone()),
    );
    let dropped = moved.state.handle_key(
        KeyboardDragKey::Space,
        KeyboardDragContext::focused_target(focus),
    );

    assert!(matches!(
        picked_up.events.as_slice(),
        [DragEvent::DragStart { .. }]
    ));
    assert!(matches!(
        moved.events.as_slice(),
        [DragEvent::DragEnter { .. }, DragEvent::DragOver { .. }]
    ));
    assert!(matches!(
        dropped.events.as_slice(),
        [
            DragEvent::Drop {
                effect: DropEffect::Move,
                ..
            },
            DragEvent::DragEnd {
                committed: true,
                ..
            }
        ]
    ));
}

#[test]
fn keyboard_escape_emits_cancel_then_uncommitted_end() {
    let source = DragSource::new(
        UiNodeId::new("tree-row-a"),
        DragData::new("consumer/tree-node", serde_json::json!({"id": "a"})),
    )
    .keyboard_draggable(true);
    let state = KeyboardDragState::idle()
        .handle_key(
            KeyboardDragKey::Enter,
            KeyboardDragContext::focused_source(source),
        )
        .state;

    let cancelled = state.handle_key(
        KeyboardDragKey::Escape,
        KeyboardDragContext::empty(UiNodeId::new("tree-row-a")),
    );

    assert!(matches!(cancelled.events.as_slice(), [
            DragEvent::DragCancel { reason, .. },
            DragEvent::DragEnd { committed: false, .. }
        ] if reason == DRAG_CANCEL_REASON_KEYBOARD_ESCAPE));
}

#[test]
fn keyboard_drag_ignores_invalid_starts_idle_movement_and_missing_targets() {
    let idle = KeyboardDragState::idle();
    assert_eq!(KeyboardDragPhase::Idle, idle.phase());

    let idle_move = idle.handle_key(
        KeyboardDragKey::ArrowLeft,
        KeyboardDragContext::empty(UiNodeId::new("none")),
    );
    assert_eq!(KeyboardDragPhase::Idle, idle_move.state.phase());
    assert!(idle_move.events.is_empty());
    assert!(idle_move.announcement.is_none());

    let missing_source = idle.handle_key(
        KeyboardDragKey::Space,
        KeyboardDragContext::empty(UiNodeId::new("none")),
    );
    assert_eq!(KeyboardDragPhase::Idle, missing_source.state.phase());

    let disabled_source = DragSource::new(
        UiNodeId::new("disabled"),
        DragData::new("consumer/item", serde_json::json!("disabled")),
    );
    let disabled = idle.handle_key(
        KeyboardDragKey::Enter,
        KeyboardDragContext::focused_source(disabled_source),
    );
    assert_eq!(KeyboardDragPhase::Idle, disabled.state.phase());

    let active_source = DragSource::new(
        UiNodeId::new("source"),
        DragData::new("consumer/item", serde_json::json!("source")),
    )
    .keyboard_draggable(true);
    let dragging = idle
        .handle_key(
            KeyboardDragKey::Space,
            KeyboardDragContext::focused_source(active_source),
        )
        .state;
    assert_eq!(KeyboardDragPhase::Dragging, dragging.phase());
    let no_target = dragging.handle_key(
        KeyboardDragKey::ArrowDown,
        KeyboardDragContext::empty(UiNodeId::new("none")),
    );
    assert_eq!(KeyboardDragPhase::Dragging, no_target.state.phase());
    assert!(no_target.events.is_empty());
    let no_drop_target = no_target.state.handle_key(
        KeyboardDragKey::Space,
        KeyboardDragContext::empty(UiNodeId::new("none")),
    );
    assert_eq!(KeyboardDragPhase::Dragging, no_drop_target.state.phase());
    assert!(no_drop_target.events.is_empty());
}

#[test]
fn keyboard_drag_tracks_target_changes_and_rejects_unsupported_drop_effects() {
    let source = DragSource::new(
        UiNodeId::new("source"),
        DragData::new("consumer/item", serde_json::json!("source")),
    )
    .keyboard_draggable(true);
    let first_focus = KeyboardDropTargetFocus::new(
        DropTarget::new(UiNodeId::new("first")).accepted_tag("consumer/item"),
        DndRect::new(0.0, 0.0, 100.0, 40.0),
        DndPoint::new(50.0, 20.0),
    );
    let second_focus = KeyboardDropTargetFocus::new(
        DropTarget::new(UiNodeId::new("second")).accepted_tag("consumer/item"),
        DndRect::new(0.0, 0.0, 100.0, 40.0),
        DndPoint::new(50.0, 20.0),
    );
    let picked_up = KeyboardDragState::idle().handle_key(
        KeyboardDragKey::Space,
        KeyboardDragContext::focused_source(source),
    );
    assert_eq!(
        Some("Picked up source"),
        picked_up
            .announcement
            .as_ref()
            .map(|announcement| announcement.message.as_str())
    );

    let first = picked_up.state.handle_key(
        KeyboardDragKey::ArrowRight,
        KeyboardDragContext::focused_target(first_focus.clone()),
    );
    let same = first.state.handle_key(
        KeyboardDragKey::ArrowRight,
        KeyboardDragContext::focused_target(first_focus),
    );
    assert!(matches!(
        same.events.as_slice(),
        [DragEvent::DragOver { .. }]
    ));

    let second = same.state.handle_key(
        KeyboardDragKey::ArrowRight,
        KeyboardDragContext::focused_target(second_focus),
    );
    assert!(matches!(
        second.events.as_slice(),
        [
            DragEvent::DragLeave { target },
            DragEvent::DragEnter { .. },
            DragEvent::DragOver { .. }
        ] if target.as_str() == "first"
    ));

    let copy_focus = KeyboardDropTargetFocus::new(
        DropTarget::new(UiNodeId::new("copy"))
            .accepted_tag("consumer/item")
            .effect(DropEffect::Copy),
        DndRect::new(0.0, 0.0, 100.0, 40.0),
        DndPoint::new(50.0, 20.0),
    );
    let unsupported = second.state.handle_key(
        KeyboardDragKey::Space,
        KeyboardDragContext::focused_target(copy_focus),
    );
    assert_eq!(KeyboardDragPhase::Dragging, unsupported.state.phase());
    assert!(unsupported.events.is_empty());

    let rejected_focus = KeyboardDropTargetFocus::new(
        DropTarget::new(UiNodeId::new("rejected")).accepted_tag("other"),
        DndRect::new(0.0, 0.0, 100.0, 40.0),
        DndPoint::new(50.0, 20.0),
    );
    let rejected = unsupported.state.handle_key(
        KeyboardDragKey::Enter,
        KeyboardDragContext::focused_target(rejected_focus),
    );
    assert_eq!(KeyboardDragPhase::Dragging, rejected.state.phase());
    assert!(rejected.events.is_empty());
}

#[test]
fn autoscroll_request_is_emitted_inside_edge_zone_and_accelerates() {
    let policy = AutoScrollPolicy::default()
        .edge_zone_px(24.0)
        .max_speed_px_per_tick(18.0);
    let viewport = DndRect::new(0.0, 0.0, 320.0, 200.0);
    let first = AutoScrollEngine::request(&policy, viewport, DndPoint::new(12.0, 6.0), 1);
    let later = AutoScrollEngine::request(&policy, viewport, DndPoint::new(12.0, 6.0), 5);

    assert!(first.is_some());
    assert!(later.is_some());
    let first_speed = first.map_or(0.0, |request| request.speed_px_per_tick);
    let later_speed = later.map_or(0.0, |request| request.speed_px_per_tick);
    assert!(later_speed > first_speed);
    assert!(
        AutoScrollEngine::request(
            &AutoScrollPolicy::disabled(),
            viewport,
            DndPoint::new(12.0, 6.0),
            5
        )
        .is_none()
    );
}

#[test]
fn disabled_nodes_are_skipped_in_drag_bubbling() {
    let route = DragEventRouting::bubble_route(
        UiNodeId::new("drop-zone"),
        vec![
            DragEventRouteNode::enabled(UiNodeId::new("panel")),
            DragEventRouteNode::disabled(UiNodeId::new("disabled-shell")),
            DragEventRouteNode::enabled(UiNodeId::new("root")),
        ],
    );
    let actual: Vec<&str> = route.iter().map(UiNodeId::as_str).collect();

    assert_eq!(["drop-zone", "panel", "root"], actual.as_slice());
}
