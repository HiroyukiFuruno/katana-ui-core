use katana_ui_core::atom::Button;
use katana_ui_core::render_model::{
    UiBorder, UiCommonProps, UiCursor, UiDimension, UiDisplay, UiEdgeInsets, UiJustifyContent,
    UiNode, UiPointerEvents, UiPosition, UiZIndex,
};

#[test]
fn common_props_cover_material_layout_and_state_dto_fields() {
    let props = UiCommonProps::default()
        .visible(false)
        .width(UiDimension::percent(100))
        .height(UiDimension::px(40))
        .border(UiBorder::solid(1, 8, "outline"))
        .tab_index(0)
        .z_index(UiZIndex::token("popover"));

    assert!(!props.visible);
    assert_eq!(UiDimension::percent(100), props.width);
    assert_eq!(UiDimension::px(40), props.height);
    assert_eq!(UiBorder::solid(1, 8, "outline"), props.border);
    assert_eq!(Some(0), props.tab_index);
    assert_eq!(UiZIndex::token("popover"), props.z_index);
}

#[test]
fn common_border_can_be_applied_to_any_node() {
    let button = UiNode::from(Button::new("Save"))
        .border(UiBorder::solid(2, 10, "accent"))
        .width(UiDimension::Fill);

    assert_eq!(
        UiBorder::solid(2, 10, "accent"),
        button.props().common.border
    );
    assert_eq!(UiDimension::Fill, button.props().common.width);
}

#[test]
fn border_fluent_overrides_preserve_explicit_visibility_radius_and_color() {
    let border = UiBorder::solid(2, 4, "outline")
        .visible(false)
        .radius_px(9)
        .color_token("accent");

    assert!(!border.visible);
    assert_eq!(9, border.radius_px);
    assert_eq!("accent", border.color_token);
}

#[test]
fn node_common_builder_applies_the_complete_host_layout_contract() {
    let common = UiCommonProps::default()
        .visible(false)
        .width(UiDimension::token("content"))
        .height(UiDimension::FitContent)
        .border(UiBorder::solid(1, 2, "border"))
        .hover_border(UiBorder::solid(2, 4, "hover"))
        .margin(UiEdgeInsets::axis(UiDimension::px(8), UiDimension::px(4)))
        .display(UiDisplay::Flex)
        .position(UiPosition::Relative)
        .justify_content(UiJustifyContent::SpaceBetween)
        .tab_index(3)
        .z_index(UiZIndex::value(7))
        .cursor(UiCursor::Pointer)
        .pointer_events(UiPointerEvents::None)
        .selectable(true)
        .accessibility_label("Common label")
        .theme_slot("common.surface");
    assert_eq!("common.surface", common.theme_slot);
    let node = UiNode::from(Button::new("Save"))
        .common(common)
        .disabled(true)
        .focusable(true)
        .accessibility_label("Save document")
        .visible(true)
        .width(UiDimension::Fill)
        .height(UiDimension::px(32))
        .border(UiBorder::solid(1, 4, "accent"))
        .hover_border(UiBorder::solid(2, 4, "accent-hover"))
        .margin(UiEdgeInsets::all(UiDimension::px(6)))
        .display(UiDisplay::Grid)
        .position(UiPosition::Absolute)
        .justify_content(UiJustifyContent::Center)
        .tab_index(0)
        .z_index(UiZIndex::token("overlay"))
        .cursor(UiCursor::Grab)
        .pointer_events(UiPointerEvents::Auto)
        .command_action("save", "Save")
        .surface_control_action("zoom-in", "Zoom in", "preview")
        .task_control_action("Retry", "task-1", 2)
        .selectable(false);

    assert!(node.props().disabled);
    assert!(node.props().focusable);
    assert!(node.props().common.visible);
    assert_eq!(UiDimension::Fill, node.props().common.width);
    assert_eq!(UiDimension::px(32), node.props().common.height);
    assert_eq!(UiDisplay::Grid, node.props().common.display);
    assert_eq!(UiPosition::Absolute, node.props().common.position);
    assert_eq!(Some(0), node.props().common.tab_index);
    assert_eq!(UiCursor::Grab, node.props().common.cursor);
    assert_eq!(3, node.props().common.host_actions.len());
    assert!(node.has_host_action());
    assert!(!node.props().common.selectable);
}
