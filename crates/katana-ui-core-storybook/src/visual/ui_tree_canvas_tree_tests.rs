use super::*;
use crate::test_assert::KucTestExpect;

#[test]
fn tree_canvas_requested_height_viewport_skip_uses_vertical_intersection() {
    let area = UiTreeRenderArea {
        x: 0,
        y: 40,
        width: 320,
        height: 100,
        scroll_y: 0.0,
    };

    assert!(is_outside_vertical_viewport(0, 40, area));
    assert!(!is_outside_vertical_viewport(0, 41, area));
    assert!(!is_outside_vertical_viewport(40, 24, area));
    assert!(!is_outside_vertical_viewport(139, 24, area));
    assert!(is_outside_vertical_viewport(140, 24, area));
}

#[test]
fn kdv_storybook_left_pane_regression_tree_canvas_renders_tree_selection_and_text_background() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(320, 100, palette.background);
    let root = left_pane_like_tree_canvas_root();

    UiTreeCanvasRenderer::new(theme.clone()).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 320,
            height: 100,
            scroll_y: 0.0,
        },
    );

    assert!(count_pixel(&canvas, palette.selection) > 200);
    assert!(count_pixel(&canvas, palette.code_background) > 80);
    assert_ne!(palette.selection, palette.code_background);
}

#[test]
fn kdv_storybook_left_pane_scroll_area_tree_view_is_top_aligned_without_extra_gap() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(360, 120, palette.background);
    let root = UiNode::new(UiNodeKind::ScrollArea, "")
        .scroll_area(UiScrollAreaProps {
            viewport_width: 360,
            viewport_height: 120,
            ..UiScrollAreaProps::default()
        })
        .child(UiNode::new(UiNodeKind::TreeView, "").tree(tree_nodes_for_test("", true)));

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 360,
            height: 120,
            scroll_y: 0.0,
        },
    );

    let top_row = first_row_for_color(&canvas, palette.selection).kuc_expect(
        "selection row should be rendered in scroll area for top alignment regression test",
    );
    assert_eq!(0, top_row);
}

#[test]
fn kdv_storybook_left_pane_tree_canvas_selection_row_uses_available_width() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(360, 60, palette.background);
    let root = UiNode::new(UiNodeKind::TreeView, "").tree(tree_nodes_for_test("src", true));

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 360,
            height: 60,
            scroll_y: 0.0,
        },
    );

    let top_row = first_row_for_color(&canvas, palette.selection)
        .kuc_expect("selected row should be visible in tree canvas");
    let sample_x = 340;
    assert_eq!(
        Some(palette.selection),
        pixel_at(&canvas, sample_x, top_row.saturating_add(8))
    );
}

#[test]
fn tree_canvas_draws_tree_icons_as_primitives_not_literal_icon_ids() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(240, 48, palette.background);
    let mut props = tree_nodes_for_test("src", false);
    props.icons_visible = true;
    props.directory_icon = "literal-folder-icon-id-must-not-render".to_string();
    let root = UiNode::new(UiNodeKind::TreeView, "").tree(props);

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 240,
            height: 48,
            scroll_y: 0.0,
        },
    );

    let rightmost = rightmost_content_x(&canvas, palette.background)
        .kuc_expect("tree row should draw an icon and label");
    assert!(
        rightmost < 90,
        "tree icon identifier leaked into rendered label; rightmost content x={rightmost}"
    );
    assert!(color_pixels_between_x(&canvas, palette.text, 0, 18) > 8);
}

#[test]
fn tree_canvas_draws_indent_lines_when_line_display_is_enabled() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(240, 80, palette.background);
    let root = UiNode::new(UiNodeKind::TreeView, "").tree(nested_tree_nodes(true));

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 240,
            height: 80,
            scroll_y: 0.0,
        },
    );

    assert_eq!(Some(palette.muted_border), pixel_at(&canvas, 6, 28));
    assert_eq!(Some(palette.muted_border), pixel_at(&canvas, 18, 52));
}

#[test]
fn tree_canvas_omits_indent_lines_when_line_display_is_disabled() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(240, 80, palette.background);
    let root = UiNode::new(UiNodeKind::TreeView, "").tree(nested_tree_nodes(false));

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 240,
            height: 80,
            scroll_y: 0.0,
        },
    );

    assert_ne!(Some(palette.muted_border), pixel_at(&canvas, 8, 28));
    assert_ne!(Some(palette.muted_border), pixel_at(&canvas, 24, 52));
}

#[test]
fn tree_canvas_renders_context_menu_node_and_returns_item_hit() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(260, 120, palette.background);
    let root = context_menu_node_for_test();

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 260,
            height: 120,
            scroll_y: 0.0,
        },
    );

    assert!(count_pixel(&canvas, palette.preview_background) > 200);
    assert!(count_pixel(&canvas, palette.selection) > 20);
    assert_eq!(
        Some("[-]".to_string()),
        UiTreeCanvasRenderer::context_menu_item_id_at(&root, 24.0, 48.0)
    );
    assert_eq!(
        Some((100.0, 48.0)),
        UiTreeCanvasRenderer::context_menu_item_center_for_id(&root, "[-]")
    );
}

#[test]
fn tree_canvas_returns_context_menu_typed_action_hit() {
    let root = UiNode::new(UiNodeKind::ContextMenu, "task-context-menu").context_menu(
        UiContextMenuProps {
            anchor: UiContextMenuAnchor::Pointer { x: 20, y: 24 },
            min_width: 160,
            items: vec![
                UiContextMenuItem::new("legacy-empty", "未実施", UiContextMenuItemKind::Radio)
                    .host_action(UiHostActionSpec::task_control_state(
                        "未実施",
                        "list",
                        1,
                        "[ ]",
                    )),
                UiContextMenuItem::new("legacy-done", "完了", UiContextMenuItemKind::Radio)
                    .host_action(UiHostActionSpec::task_control_state(
                        "完了", "list", 1, "[x]",
                    )),
            ],
            ..UiContextMenuProps::default()
        },
    );

    let action = UiTreeCanvasRenderer::context_menu_host_action_at(&root, 48.0, 60.0)
        .kuc_expect("typed context menu action");
    assert_eq!(UI_TASK_SET_STATE_ACTION_ID, action.action_id);
    assert!(matches!(
        &action.typed_payload,
        UiHostActionPayload::TaskControlState(_)
    ));
    let UiHostActionPayload::TaskControlState(payload) = &action.typed_payload else {
        return;
    };
    assert_eq!("list", payload.node_id);
    assert_eq!(1, payload.row_index);
    assert_eq!("[x]", payload.marker);
}

#[test]
fn tree_canvas_draws_hover_row_background_from_tree_hovered_id() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(240, 80, palette.background);
    let mut props = nested_tree_nodes(true);
    props.hovered_id = "assets/fixtures/sample.md".to_string();
    let root = UiNode::new(UiNodeKind::TreeView, "").tree(props);

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 240,
            height: 80,
            scroll_y: 0.0,
        },
    );

    assert_eq!(Some(palette.hover_background), pixel_at(&canvas, 40, 52));
    assert_ne!(Some(palette.selection), pixel_at(&canvas, 40, 52));
}

#[test]
fn tree_canvas_draws_tree_view_row_hover_border_from_kuc_contract() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(240, 80, palette.background);
    let mut props = nested_tree_nodes(true);
    props.hovered_id = "assets/fixtures/sample.md".to_string();
    props.row_hover_border = UiBorder::solid(1, 4, "control.hover.border");
    let root = UiNode::new(UiNodeKind::TreeView, "").tree(props);

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 240,
            height: 80,
            scroll_y: 0.0,
        },
    );

    assert_eq!(Some(palette.visual.hover_border), pixel_at(&canvas, 0, 52));
    assert_eq!(Some(palette.hover_background), pixel_at(&canvas, 180, 52));
}
