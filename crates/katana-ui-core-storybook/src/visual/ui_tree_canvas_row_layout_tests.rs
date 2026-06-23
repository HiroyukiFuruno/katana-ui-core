#[test]
fn row_render_and_hit_collector_share_row_layout_contract() {
    let render_source = include_str!("ui_tree_canvas_layout.rs");
    let hit_source = include_str!("ui_tree_canvas_hit_methods.rs");

    assert!(render_source.contains("UiTreeRowLayout::children"));
    assert!(hit_source.contains("UiTreeRowLayout::children"));
    assert!(
        !render_source.contains("slot_width(child)"),
        "Row rendering must not rebuild child slot traversal outside UiTreeRowLayout"
    );
    assert!(
        !hit_source.contains("slot_width(child)"),
        "Row hit-test must not rebuild child slot traversal outside UiTreeRowLayout"
    );
}
