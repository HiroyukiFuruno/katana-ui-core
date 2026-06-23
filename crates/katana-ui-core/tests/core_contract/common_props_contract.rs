use katana_ui_core::atom::Button;
use katana_ui_core::render_model::{UiBorder, UiCommonProps, UiDimension, UiNode, UiZIndex};

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
