use super::ui_tree_canvas_palette::UiTreeCanvasPalette;
use super::ui_tree_canvas_tree_test_support::TreeTestSupport;
use super::{Canvas, UiTreeCanvasRenderer, UiTreeRenderArea};
use crate::test_assert::KucTestExpect;
use katana_ui_core::render_model::{
    UI_TASK_SET_STATE_ACTION_ID, UiBorder, UiContextMenuAnchor, UiContextMenuItem,
    UiContextMenuItemKind, UiContextMenuProps, UiHostActionPayload, UiHostActionSpec, UiNode,
    UiNodeKind,
};
use katana_ui_core::theme::ThemeSnapshot;

#[test]
fn tree_canvas_renders_context_menu_node_and_returns_item_hit() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(260, 120, palette.background);
    let root = TreeTestSupport::context_menu_node_for_test();

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

    assert!(TreeTestSupport::count_pixel(&canvas, palette.preview_background) > 200);
    assert!(TreeTestSupport::count_pixel(&canvas, palette.selection) > 20);
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
    let mut props = TreeTestSupport::nested_tree_nodes(true);
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

    assert_eq!(
        Some(palette.hover_background),
        TreeTestSupport::pixel_at(&canvas, 40, 52)
    );
    assert_ne!(
        Some(palette.selection),
        TreeTestSupport::pixel_at(&canvas, 40, 52)
    );
}

#[test]
fn tree_canvas_draws_tree_view_row_hover_border_from_kuc_contract() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(240, 80, palette.background);
    let mut props = TreeTestSupport::nested_tree_nodes(true);
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

    assert_eq!(
        Some(palette.visual.hover_border),
        TreeTestSupport::pixel_at(&canvas, 0, 52)
    );
    assert_eq!(
        Some(palette.hover_background),
        TreeTestSupport::pixel_at(&canvas, 180, 52)
    );
}
