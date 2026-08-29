use katana_ui_core::event::DragEvent;
use katana_ui_core::interaction::drag_and_drop::{
    DndPoint, DndRect, DragData, DragMetadata, DragSource, DropEffect, DropTarget,
};
use katana_ui_core::render_model::UiNodeId;

const SOURCE_NODE_ID: &str = "storybook-dnd-source";
const TARGET_NODE_ID: &str = "storybook-dnd-target";
const DRAG_TAG: &str = "katana-ui-core/list-row";
const PAYLOAD_ID: &str = "item-02";
const TARGET_RECT_X: f32 = 0.0;
const TARGET_RECT_Y: f32 = 0.0;
const TARGET_RECT_WIDTH: f32 = 166.0;
const TARGET_RECT_HEIGHT: f32 = 54.0;
const TARGET_POINT_X: f32 = 90.0;
const TARGET_POINT_Y: f32 = 48.0;

pub(in crate::visual::window_interaction) fn drag_source(keyboard_draggable: bool) -> DragSource {
    DragSource::new(source_node_id(), drag_data())
        .allowed_effect(DropEffect::Copy)
        .keyboard_draggable(keyboard_draggable)
}

pub(in crate::visual::window_interaction) fn drop_target() -> DropTarget {
    DropTarget::new(target_node_id())
        .accepted_tag(DRAG_TAG)
        .effect(DropEffect::Move)
}

pub(in crate::visual::window_interaction) fn drag_data() -> DragData {
    DragData::new(DRAG_TAG, serde_json::json!({ "id": PAYLOAD_ID }))
        .metadata(DragMetadata::new().label(PAYLOAD_ID))
}

pub(in crate::visual::window_interaction) fn source_node_id() -> UiNodeId {
    UiNodeId::new(SOURCE_NODE_ID)
}

pub(in crate::visual::window_interaction) fn target_rect() -> DndRect {
    DndRect::new(
        TARGET_RECT_X,
        TARGET_RECT_Y,
        TARGET_RECT_WIDTH,
        TARGET_RECT_HEIGHT,
    )
}

pub(in crate::visual::window_interaction) fn target_point() -> DndPoint {
    DndPoint::new(TARGET_POINT_X, TARGET_POINT_Y)
}

pub(in crate::visual::window_interaction) fn drag_event_name(event: &DragEvent) -> &'static str {
    match event {
        DragEvent::DragStart { .. } => "drag_start",
        DragEvent::DragMove { .. } => "drag_move",
        DragEvent::DragEnter { .. } => "drag_enter",
        DragEvent::DragLeave { .. } => "drag_leave",
        DragEvent::DragOver { .. } => "drag_over",
        DragEvent::Drop { .. } => "drop",
        DragEvent::DragCancel { .. } => "drag_cancel",
        DragEvent::DragEnd { committed, .. } if *committed => "drag_end(committed=true)",
        DragEvent::DragEnd { .. } => "drag_end(committed=false)",
    }
}

fn target_node_id() -> UiNodeId {
    UiNodeId::new(TARGET_NODE_ID)
}

#[cfg(test)]
mod tests {
    use super::*;
    use katana_ui_core::interaction::drag_and_drop::DropAcceptance;

    #[test]
    fn drag_event_names_cover_every_public_event_variant() {
        let source = source_node_id();
        let target = target_node_id();
        let position = target_point();
        let cases = [
            (
                DragEvent::DragStart {
                    source: source.clone(),
                    data: drag_data(),
                },
                "drag_start",
            ),
            (
                DragEvent::DragMove {
                    source: source.clone(),
                    position,
                },
                "drag_move",
            ),
            (
                DragEvent::DragEnter {
                    target: target.clone(),
                    data: drag_data(),
                },
                "drag_enter",
            ),
            (
                DragEvent::DragLeave {
                    target: target.clone(),
                },
                "drag_leave",
            ),
            (
                DragEvent::DragOver {
                    target: target.clone(),
                    position,
                    acceptance: DropAcceptance::Reject,
                },
                "drag_over",
            ),
            (
                DragEvent::Drop {
                    target: target.clone(),
                    data: drag_data(),
                    effect: DropEffect::Move,
                },
                "drop",
            ),
            (
                DragEvent::DragCancel {
                    source: source.clone(),
                    reason: "cancelled".to_string(),
                },
                "drag_cancel",
            ),
            (
                DragEvent::DragEnd {
                    source: source.clone(),
                    committed: true,
                },
                "drag_end(committed=true)",
            ),
            (
                DragEvent::DragEnd {
                    source,
                    committed: false,
                },
                "drag_end(committed=false)",
            ),
        ];

        for (event, expected_name) in cases {
            assert_eq!(expected_name, drag_event_name(&event));
        }
    }
}
