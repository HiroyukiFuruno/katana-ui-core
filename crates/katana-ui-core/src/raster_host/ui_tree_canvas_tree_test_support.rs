use super::ui_tree_canvas_palette::UiTreeCanvasPalette;
use super::{Canvas, UiTreeCanvasRenderer, UiTreeRenderArea};
use katana_ui_core::render_model::{
    UiContextMenuAnchor, UiContextMenuItem, UiContextMenuItemKind, UiContextMenuProps, UiNode,
    UiNodeKind, UiTextProps, UiTextWrapMode, UiTreeNodeKind, UiTreeNodeProps, UiTreeProps,
};
use katana_ui_core::theme::ThemeSnapshot;

const CONTEXT_MENU_ANCHOR_X: i32 = 10;
const CONTEXT_MENU_ANCHOR_Y: i32 = 12;
const CONTEXT_MENU_MIN_WIDTH: u32 = 180;

pub(crate) struct TreeTestSupport;

impl TreeTestSupport {
    pub(crate) fn left_pane_like_tree_canvas_root() -> UiNode {
        UiNode::new(UiNodeKind::Row, "")
            .child(
                UiNode::new(UiNodeKind::TreeView, "").tree(Self::tree_nodes_for_test("src", true)),
            )
            .child(code_text_node())
    }

    pub(crate) fn tree_nodes_for_test(label: &str, selected: bool) -> UiTreeProps {
        UiTreeProps {
            nodes: vec![
                UiTreeNodeProps::new("src", label, 0, UiTreeNodeKind::Directory).selected(selected),
            ],
            ..UiTreeProps::default()
        }
    }

    pub(crate) fn nested_tree_nodes(line_display: bool) -> UiTreeProps {
        UiTreeProps {
            line_display,
            icons_visible: true,
            nodes: vec![
                UiTreeNodeProps::new("assets", "assets", 0, UiTreeNodeKind::Directory),
                UiTreeNodeProps::new("assets/fixtures", "fixtures", 1, UiTreeNodeKind::Directory),
                UiTreeNodeProps::new(
                    "assets/fixtures/sample.md",
                    "sample.md",
                    2,
                    UiTreeNodeKind::File,
                ),
            ],
            ..UiTreeProps::default()
        }
    }

    pub(crate) fn context_menu_node_for_test() -> UiNode {
        UiNode::new(UiNodeKind::ContextMenu, "task-context-menu").context_menu(UiContextMenuProps {
            anchor: UiContextMenuAnchor::Pointer {
                x: CONTEXT_MENU_ANCHOR_X,
                y: CONTEXT_MENU_ANCHOR_Y,
            },
            min_width: CONTEXT_MENU_MIN_WIDTH,
            items: vec![
                UiContextMenuItem::new("[ ]", "未実施", UiContextMenuItemKind::Radio),
                UiContextMenuItem::new("[-]", "保留", UiContextMenuItemKind::Radio).checked(true),
                UiContextMenuItem::new("[/]", "実施中", UiContextMenuItemKind::Radio),
            ],
            ..UiContextMenuProps::default()
        })
    }

    pub(crate) fn count_pixel(canvas: &Canvas, expected: u32) -> usize {
        canvas
            .pixels()
            .iter()
            .filter(|pixel| **pixel == expected)
            .count()
    }

    pub(crate) fn color_pixels_between_x(
        canvas: &Canvas,
        expected: u32,
        minimum_x: usize,
        maximum_x: usize,
    ) -> usize {
        canvas
            .pixels()
            .iter()
            .enumerate()
            .filter(|(index, pixel)| {
                let x = index % canvas.width();
                **pixel == expected && x >= minimum_x && x <= maximum_x
            })
            .count()
    }

    pub(crate) fn first_row_for_color(canvas: &Canvas, expected: u32) -> Option<usize> {
        for y in 0..canvas.height() {
            for x in 0..canvas.width() {
                if Self::pixel_at(canvas, x, y) == Some(expected) {
                    return Some(y);
                }
            }
        }
        None
    }

    pub(crate) fn pixel_at(canvas: &Canvas, x: usize, y: usize) -> Option<u32> {
        canvas
            .pixels()
            .get(y.saturating_mul(canvas.width()) + x)
            .copied()
    }

    pub(crate) fn rightmost_content_x(canvas: &Canvas, background: u32) -> Option<usize> {
        canvas
            .pixels()
            .iter()
            .rposition(|pixel| *pixel != background)
            .map(|index| index % canvas.width())
    }
}

fn code_text_node() -> UiNode {
    UiNode::new(UiNodeKind::Text, "main.rs").text(UiTextProps {
        role: "code".to_owned(),
        wrap: UiTextWrapMode::NoWrap,
        ..UiTextProps::default()
    })
}

#[test]
fn tree_test_support_searches_only_the_requested_color() {
    let canvas = Canvas::new(2, 2, 0);

    assert_eq!(None, TreeTestSupport::first_row_for_color(&canvas, 1));
    assert_eq!(None, TreeTestSupport::rightmost_content_x(&canvas, 0));

    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut rendered = Canvas::new(64, 24, palette.background);
    let root = UiNode::new(UiNodeKind::TreeView, "")
        .tree(TreeTestSupport::tree_nodes_for_test("src", true));
    UiTreeCanvasRenderer::new(theme).render(
        &mut rendered,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 64,
            height: 24,
            scroll_y: 0.0,
        },
    );
    assert!(TreeTestSupport::count_pixel(&rendered, palette.background) < rendered.pixels().len());
}
