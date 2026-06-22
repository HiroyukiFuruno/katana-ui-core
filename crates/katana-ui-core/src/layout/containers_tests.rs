use super::{AlignCenter, AlignNode, Alignment, Column, LayoutAxis, Length, OverflowBehavior, Row};
use crate::atom::Text;
use crate::render_model::{
    UiAlignItems, UiDimension, UiDisplay, UiJustifyContent, UiLayoutAxis, UiNodeKind, UiOverflow,
    UiTree,
};

#[test]
fn layout_serializes_to_tree_shape() {
    let row = Row::new().gap(Length::px(8.0)).child(Text::new("A"));
    let tree = UiTree::new(Column::new().child(row));
    assert_eq!(1, tree.root().children().len());
}

#[test]
fn align_node_maps_to_common_layout_contract() {
    let tree = UiTree::new(AlignNode::left_center().child(Text::new("Label")));

    assert_eq!(UiNodeKind::AlignNode, tree.root().kind());
    assert_eq!(UiAlignItems::Center, tree.root().props().common.align_items);
    assert_eq!(
        UiJustifyContent::Start,
        tree.root().props().common.justify_content
    );
    assert_eq!(1, tree.root().children().len());
}

#[test]
fn align_center_projects_center_alignment_to_common_layout_contract() {
    let tree = UiTree::new(
        AlignCenter::new()
            .align(Alignment::Center)
            .child(Text::new("Label")),
    );

    assert_eq!(UiNodeKind::AlignCenter, tree.root().kind());
    assert_eq!(UiDisplay::Flex, tree.root().props().common.display);
    assert_eq!(UiAlignItems::Center, tree.root().props().common.align_items);
    assert_eq!(
        UiJustifyContent::Center,
        tree.root().props().common.justify_content
    );
    assert_eq!(1, tree.root().children().len());
}

#[test]
fn layout_models_project_axis_gap_alignment_and_overflow() {
    let tree = UiTree::new(
        Row::new()
            .axis(LayoutAxis::Vertical)
            .gap(Length::px(12.0))
            .align(Alignment::Center)
            .overflow(OverflowBehavior::Scroll)
            .child(Text::new("A")),
    );
    let common = &tree.root().props().common;

    assert_eq!(UiDisplay::Flex, common.display);
    assert_eq!(UiLayoutAxis::Vertical, common.layout_axis);
    assert_eq!(UiDimension::Px(12), common.gap);
    assert_eq!(UiAlignItems::Center, common.align_items);
    assert_eq!(UiJustifyContent::Center, common.justify_content);
    assert_eq!(UiOverflow::Scroll, common.overflow);
}
