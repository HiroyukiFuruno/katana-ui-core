mod render_model {
    pub use katana_ui_core::render_model::*;
}

#[path = "../src/interaction/drag_and_drop/mod.rs"]
pub mod drag_and_drop;

pub mod interaction {
    pub use crate::drag_and_drop;
}

#[path = "../src/event/drag.rs"]
pub mod drag_event;

pub mod event {
    pub use crate::drag_event as drag;
}

use event::drag::{
    DRAG_CANCEL_REASON_KEYBOARD_ESCAPE, DragEvent, DragEventRouteNode, DragEventRouting,
};
use interaction::drag_and_drop::{
    AutoScrollEngine, AutoScrollPolicy, DndPoint, DndRect, DragData, DragMetadata, DragSource,
    DropAcceptance, DropEffect, DropIndicatorKind, DropTarget, KeyboardDragContext,
    KeyboardDragKey, KeyboardDragState, KeyboardDropTargetFocus, OS_FILE_LIST_TAG,
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
