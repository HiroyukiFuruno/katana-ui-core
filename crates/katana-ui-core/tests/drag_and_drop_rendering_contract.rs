use katana_ui_core::atom::DropIndicator;
use katana_ui_core::interaction::drag_and_drop::{
    DndPoint, DndRect, DragData, DropIndicatorKind, DropTarget, OS_FILE_LIST_TAG,
};
use katana_ui_core::molecule::DragPreview;
use katana_ui_core::render_model::{UiNode, UiNodeId, UiNodeKind, UiRect};

#[test]
fn reorder_list_indicator_exposes_before_inside_and_after_geometry() {
    let target = DropTarget::new(UiNodeId::new("row-b")).accepted_tag("consumer/list-row");
    let data = DragData::new("consumer/list-row", serde_json::json!("row-a"));
    let rect = DndRect::new(4.0, 10.0, 220.0, 80.0);

    let before = target.accept(&data, DndPoint::new(8.0, 14.0), rect);
    let inside = target.accept(&data, DndPoint::new(8.0, 48.0), rect);
    let after = target.accept(&data, DndPoint::new(8.0, 88.0), rect);

    assert_eq!(Some(DropIndicatorKind::Before), before.indicator_kind());
    assert_eq!(Some(DropIndicatorKind::Inside), inside.indicator_kind());
    assert_eq!(Some(DropIndicatorKind::After), after.indicator_kind());
}

#[test]
fn file_drop_hover_indicator_keeps_os_tag_and_anchor_rect() {
    let rect = DndRect::new(12.0, 20.0, 320.0, 96.0);
    let target = DropTarget::new(UiNodeId::new("drop-zone")).accepted_tag(OS_FILE_LIST_TAG);
    let data = DragData::new(OS_FILE_LIST_TAG, serde_json::json!(["/tmp/a.md"]));
    let accepted = target.accept(&data, DndPoint::new(64.0, 58.0), rect);
    let indicator = accepted.indicator().expect("file drop must show indicator");
    let node: UiNode = DropIndicator::new(indicator.kind, indicator.anchor_rect).into();

    assert_eq!(UiNodeKind::DropIndicator, node.kind());
    assert_eq!(
        UiRect::new(12, 20, 320, 96),
        node.props().drop_indicator.anchor_rect
    );
}

#[test]
fn tab_reorder_ghost_uses_drag_preview_opacity_and_count_badge() {
    let preview: UiNode = DragPreview::new("settings.rs")
        .icon("tab")
        .count_badge(1)
        .opacity_percent(70)
        .into();

    assert_eq!(UiNodeKind::DragPreview, preview.kind());
    assert_eq!("tab", preview.props().drag_preview.icon);
    assert_eq!(1, preview.props().drag_preview.count_badge);
    assert_eq!(70, preview.props().drag_preview.opacity_percent);
}

#[test]
fn attachment_drop_zone_hover_uses_inside_indicator() {
    let target =
        DropTarget::new(UiNodeId::new("composer-attachments")).accepted_tag("consumer/attachment");
    let data = DragData::new("consumer/attachment", serde_json::json!("image.png"));
    let accepted = target.accept(
        &data,
        DndPoint::new(120.0, 48.0),
        DndRect::new(0.0, 0.0, 240.0, 96.0),
    );

    assert_eq!(Some(DropIndicatorKind::Inside), accepted.indicator_kind());
}
