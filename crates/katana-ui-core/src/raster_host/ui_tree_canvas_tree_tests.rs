use super::ui_tree_canvas::is_outside_vertical_viewport;
use super::ui_tree_canvas_palette::UiTreeCanvasPalette;
use super::ui_tree_canvas_tree_test_support::TreeTestSupport;
use super::{Canvas, UiTreeCanvasRenderer, UiTreeRenderArea};
use crate::test_assert::KucTestExpect;
use katana_ui_core::render_model::{UiNode, UiNodeKind, UiScrollAreaProps};
use katana_ui_core::theme::ThemeSnapshot;

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
fn labelled_tree_stops_rows_at_the_viewport_after_drawing_its_label() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(240, 24, palette.background);
    let root = UiNode::new(UiNodeKind::TreeView, "Files")
        .tree(TreeTestSupport::tree_nodes_for_test("src", true));

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 240,
            height: 24,
            scroll_y: 0.0,
        },
    );

    assert!(canvas.non_background_pixels(palette.background) > 0);
    assert_eq!(0, TreeTestSupport::count_pixel(&canvas, palette.selection));
}

#[test]
fn kdv_storybook_left_pane_regression_tree_canvas_renders_tree_selection_and_text_background() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(320, 100, palette.background);
    let root = TreeTestSupport::left_pane_like_tree_canvas_root();

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

    assert!(TreeTestSupport::count_pixel(&canvas, palette.selection) > 200);
    assert!(TreeTestSupport::count_pixel(&canvas, palette.code_background) > 80);
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
        .child(
            UiNode::new(UiNodeKind::TreeView, "")
                .tree(TreeTestSupport::tree_nodes_for_test("", true)),
        );

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

    let top_row = TreeTestSupport::first_row_for_color(&canvas, palette.selection).kuc_expect(
        "selection row should be rendered in scroll area for top alignment regression test",
    );
    assert_eq!(0, top_row);
}

#[test]
fn kdv_storybook_left_pane_tree_canvas_selection_row_uses_available_width() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(360, 60, palette.background);
    let root = UiNode::new(UiNodeKind::TreeView, "")
        .tree(TreeTestSupport::tree_nodes_for_test("src", true));

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

    let top_row = TreeTestSupport::first_row_for_color(&canvas, palette.selection)
        .kuc_expect("selected row should be visible in tree canvas");
    let sample_x = 340;
    assert_eq!(
        Some(palette.selection),
        TreeTestSupport::pixel_at(&canvas, sample_x, top_row.saturating_add(8))
    );
}

#[test]
fn tree_canvas_draws_tree_icons_as_primitives_not_literal_icon_ids() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(240, 48, palette.background);
    let mut props = TreeTestSupport::tree_nodes_for_test("src", false);
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

    let rightmost = TreeTestSupport::rightmost_content_x(&canvas, palette.background)
        .kuc_expect("tree row should draw an icon and label");
    assert!(
        rightmost < 90,
        "tree icon identifier leaked into rendered label; rightmost content x={rightmost}"
    );
    assert!(TreeTestSupport::color_pixels_between_x(&canvas, palette.text, 0, 18) > 8);
}

#[test]
fn tree_canvas_draws_indent_lines_when_line_display_is_enabled() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(240, 80, palette.background);
    let root = UiNode::new(UiNodeKind::TreeView, "").tree(TreeTestSupport::nested_tree_nodes(true));

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

    assert_eq!(
        Some(palette.muted_border),
        TreeTestSupport::pixel_at(&canvas, 6, 28)
    );
    assert_eq!(
        Some(palette.muted_border),
        TreeTestSupport::pixel_at(&canvas, 18, 52)
    );
}

#[test]
fn tree_canvas_omits_indent_lines_when_line_display_is_disabled() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(240, 80, palette.background);
    let root =
        UiNode::new(UiNodeKind::TreeView, "").tree(TreeTestSupport::nested_tree_nodes(false));

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

    assert_ne!(
        Some(palette.muted_border),
        TreeTestSupport::pixel_at(&canvas, 8, 28)
    );
    assert_ne!(
        Some(palette.muted_border),
        TreeTestSupport::pixel_at(&canvas, 24, 52)
    );
}
