use katana_ui_core::atom::{DragHandle, DropIndicator};
use katana_ui_core::interaction::drag_and_drop::{
    DndRect, DropIndicatorKind, DropIndicatorOrientation, DropIndicatorVisual,
};
use katana_ui_core::molecule::DragPreview;
use katana_ui_core::render_model::{UiCursor, UiNode, UiNodeKind, UiRect, UiTone};
use katana_ui_core::widget;

#[test]
fn drag_handle_exposes_cursor_and_accessibility_contract() {
    let node: UiNode = DragHandle::new("drag row")
        .accessibility_label("行をドラッグ")
        .cursor_hint(UiCursor::Grab)
        .into();

    assert_eq!(UiNodeKind::DragHandle, node.kind());
    assert_eq!(UiCursor::Grab, node.props().common.cursor);
    assert!(node.props().common.focusable);
    assert_eq!("行をドラッグ", node.props().drag_handle.accessibility_label);
}

#[test]
fn drop_indicator_preserves_position_visual_and_tone_contract() {
    let rect = DndRect::new(8.0, 12.0, 240.0, 36.0);
    let node: UiNode = DropIndicator::new(DropIndicatorKind::Before, rect)
        .visual(DropIndicatorVisual::Glow)
        .orientation(DropIndicatorOrientation::Horizontal)
        .tone(UiTone::Accent)
        .visible(true)
        .into();

    assert_eq!(UiNodeKind::DropIndicator, node.kind());
    assert_eq!(DropIndicatorKind::Before, node.props().drop_indicator.kind);
    assert_eq!(
        DropIndicatorOrientation::Horizontal,
        node.props().drop_indicator.orientation
    );
    assert_eq!(
        DropIndicatorVisual::Glow,
        node.props().drop_indicator.visual
    );
    assert_eq!(UiTone::Accent, node.props().drop_indicator.tone);
    assert_eq!(
        UiRect::new(8, 12, 240, 36),
        node.props().drop_indicator.anchor_rect
    );
}

#[test]
fn drag_preview_carries_label_icon_count_and_opacity() {
    let node: UiNode = DragPreview::new("Tab A")
        .icon("file")
        .count_badge(3)
        .opacity_percent(72)
        .into();

    assert_eq!(UiNodeKind::DragPreview, node.kind());
    assert_eq!("Tab A", node.props().label);
    assert_eq!("file", node.props().drag_preview.icon);
    assert_eq!(3, node.props().drag_preview.count_badge);
    assert_eq!(72, node.props().drag_preview.opacity_percent);
}

#[test]
fn widget_layer_reexports_drag_and_drop_building_blocks() {
    let handle: UiNode = widget::atoms::DragHandle::new("handle").into();
    let indicator: UiNode = widget::atoms::DropIndicator::new(
        DropIndicatorKind::Inside,
        DndRect::new(0.0, 0.0, 0.0, 0.0),
    )
    .into();
    let preview: UiNode = widget::molecules::DragPreview::new("preview").into();

    assert_eq!(UiNodeKind::DragHandle, handle.kind());
    assert_eq!(UiNodeKind::DropIndicator, indicator.kind());
    assert_eq!(UiNodeKind::DragPreview, preview.kind());
}
