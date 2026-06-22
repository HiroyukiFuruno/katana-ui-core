use super::*;
use katana_ui_core::render_model::{
    UiAlignItems, UiDimension, UiDisplay, UiJustifyContent, UiLayoutAxis, UiOverflow,
};

#[test]
fn row_story_exposes_layout_alignment_interaction_contract() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let story = examples
        .iter()
        .find(|it| it.page == "row")
        .ok_or("row page missing")?;
    let root = story.tree.root();
    let labels = page_children(&examples, "row").ok_or("row page missing")?;

    assert_eq!(UiDisplay::Flex, root.props().common.display);
    assert_eq!(UiLayoutAxis::Horizontal, root.props().common.layout_axis);
    assert_eq!(UiAlignItems::Start, root.props().common.align_items);
    assert_eq!(UiJustifyContent::Start, root.props().common.justify_content);
    assert_eq!(UiOverflow::Visible, root.props().common.overflow);
    assert_eq!(UiDimension::Px(8), root.props().common.gap);
    assert_eq!("alignment=start", root.props().interaction.value);
    assert!(
        labels.iter().any(|it| it.contains("axis=Horizontal")),
        "row preview should expose axis/gap config"
    );
    assert!(
        labels
            .iter()
            .any(|it| it == "settings: axis=Horizontal gap=8 overflow=fit"),
        "row preview should expose settings text"
    );
    assert!(
        story
            .callback_logs
            .iter()
            .any(|it| it.action == "row_align" && it.after.contains("event=layout_changed")),
        "row callback log lacks row_align evidence"
    );
    Ok(())
}

#[test]
fn column_story_exposes_layout_alignment_interaction_contract() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let story = examples
        .iter()
        .find(|it| it.page == "column")
        .ok_or("column page missing")?;
    let root = story.tree.root();
    let labels = page_children(&examples, "column").ok_or("column page missing")?;

    assert_eq!(UiDisplay::Flex, root.props().common.display);
    assert_eq!(UiLayoutAxis::Vertical, root.props().common.layout_axis);
    assert_eq!(UiAlignItems::Start, root.props().common.align_items);
    assert_eq!(UiJustifyContent::Start, root.props().common.justify_content);
    assert_eq!(UiOverflow::Visible, root.props().common.overflow);
    assert_eq!(UiDimension::Px(8), root.props().common.gap);
    assert_eq!("alignment=start", root.props().interaction.value);
    assert!(
        labels.iter().any(|it| it.contains("axis=Vertical")),
        "column preview should expose axis/gap config"
    );
    assert!(
        labels
            .iter()
            .any(|it| it == "settings: axis=Vertical gap=8 overflow=fit"),
        "column preview should expose settings text"
    );
    assert!(
        story
            .callback_logs
            .iter()
            .any(|it| it.action == "column_align" && it.after.contains("event=layout_changed")),
        "column callback log lacks column_align evidence"
    );
    Ok(())
}

#[test]
fn stack_story_exposes_reorder_interaction_contract() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let story = examples
        .iter()
        .find(|it| it.page == "stack")
        .ok_or("stack page missing")?;
    let root = story.tree.root();
    let labels = page_children(&examples, "stack").ok_or("stack page missing")?;

    assert_eq!(UiDisplay::Flex, root.props().common.display);
    assert_eq!(UiLayoutAxis::Overlay, root.props().common.layout_axis);
    assert_eq!(UiAlignItems::Start, root.props().common.align_items);
    assert_eq!(UiJustifyContent::Start, root.props().common.justify_content);
    assert_eq!(UiOverflow::Visible, root.props().common.overflow);
    assert_eq!(UiDimension::Px(0), root.props().common.gap);
    assert_eq!(0, root.props().interaction.selected_index);
    assert!(
        labels.iter().any(|it| it.contains("action: stack_reorder")),
        "stack preview should expose reorder action"
    );
    assert!(
        story
            .callback_logs
            .iter()
            .any(|it| it.action == "stack_reorder" && it.after.contains("event=z_order_changed")),
        "stack callback log lacks stack_reorder evidence"
    );
    Ok(())
}

#[test]
fn grid_story_exposes_select_interaction_contract() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let story = examples
        .iter()
        .find(|it| it.page == "grid")
        .ok_or("grid page missing")?;
    let root = story.tree.root();
    let labels = page_children(&examples, "grid").ok_or("grid page missing")?;

    assert_eq!(UiDisplay::Grid, root.props().common.display);
    assert_eq!(UiLayoutAxis::Both, root.props().common.layout_axis);
    assert_eq!(UiAlignItems::Start, root.props().common.align_items);
    assert_eq!(UiJustifyContent::Start, root.props().common.justify_content);
    assert_eq!(UiOverflow::Visible, root.props().common.overflow);
    assert_eq!(UiDimension::Px(12), root.props().common.gap);
    assert_eq!("selected=0", root.props().interaction.value);
    assert!(
        labels.iter().any(|it| it.contains("Grid item 3")),
        "grid preview should expose grid item children"
    );
    assert!(
        story
            .callback_logs
            .iter()
            .any(|it| it.action == "grid_select" && it.after.contains("event=grid_cell_selected")),
        "grid callback log lacks grid_select evidence"
    );
    Ok(())
}
