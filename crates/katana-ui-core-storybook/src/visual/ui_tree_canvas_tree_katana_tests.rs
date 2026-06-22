use super::ui_tree_canvas_palette::UiTreeCanvasPalette;
use super::{Canvas, UiTreeCanvasRenderer, UiTreeRenderArea};
use katana_ui_core::molecule::TreeView;
use katana_ui_core::render_model::{
    UiNode, UiNodeKind, UiTreeNodeKind, UiTreeNodeProps, UiTreeProps,
};
use katana_ui_core::theme::ThemeSnapshot;

const EXPECTED_TREE_ROW_HEIGHT: u32 = 22;
const TREE_TEST_CANVAS_WIDTH: usize = 160;
const TREE_TEST_CANVAS_HEIGHT: usize = 80;
const TREE_TEST_LAST_X: usize = TREE_TEST_CANVAS_WIDTH - 1;
const SELECTED_TREE_ROW_INDEX: usize = 2;
const TREE_ROW_MIDPOINT_OFFSET: usize = 11;
const FILE_DISCLOSURE_SAMPLE_X: usize = 35;
const FILE_ICON_SAMPLE_X: usize = 47;
const FILE_ROW_SAMPLE_Y_OFFSET: usize = 8;
const MARKDOWN_ICON_SAMPLE_X: usize = 25;
const MARKDOWN_ICON_SAMPLE_Y: usize = 9;

#[test]
fn tree_canvas_uses_katana_explorer_row_contract() {
    assert_eq!(EXPECTED_TREE_ROW_HEIGHT, TreeView::row_height());

    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let canvas = render_tree(theme, nested_tree());
    let selected_row_middle_y =
        TreeView::row_height() as usize * SELECTED_TREE_ROW_INDEX + TREE_ROW_MIDPOINT_OFFSET;

    assert_eq!(
        Some(palette.selection),
        pixel_at(&canvas, 0, selected_row_middle_y)
    );
    assert_eq!(
        Some(palette.selection),
        pixel_at(&canvas, TREE_TEST_LAST_X, selected_row_middle_y)
    );
}

#[test]
fn tree_canvas_file_rows_reserve_disclosure_space_before_file_icon() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let canvas = render_tree(theme, nested_tree());
    let selected_row_y = TreeView::row_height() as usize * SELECTED_TREE_ROW_INDEX;

    assert_eq!(
        Some(palette.selection),
        pixel_at(
            &canvas,
            FILE_DISCLOSURE_SAMPLE_X,
            selected_row_y + FILE_ROW_SAMPLE_Y_OFFSET
        )
    );
    assert_eq!(
        Some(palette.text),
        pixel_at(
            &canvas,
            FILE_ICON_SAMPLE_X,
            selected_row_y + FILE_ROW_SAMPLE_Y_OFFSET
        )
    );
}

#[test]
fn tree_canvas_uses_node_specific_file_icon_contract() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let canvas = render_tree(theme, file_tree_with_icon("markdown"));

    assert_eq!(
        Some(palette.text),
        pixel_at(&canvas, MARKDOWN_ICON_SAMPLE_X, MARKDOWN_ICON_SAMPLE_Y)
    );
}

fn render_tree(theme: ThemeSnapshot, props: UiTreeProps) -> Canvas {
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(
        TREE_TEST_CANVAS_WIDTH,
        TREE_TEST_CANVAS_HEIGHT,
        palette.background,
    );
    let root = UiNode::new(UiNodeKind::TreeView, "").tree(props);
    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: TREE_TEST_CANVAS_WIDTH,
            height: TREE_TEST_CANVAS_HEIGHT,
            scroll_y: 0.0,
        },
    );
    canvas
}

fn file_tree_with_icon(icon: &str) -> UiTreeProps {
    UiTreeProps {
        icons_visible: true,
        nodes: vec![
            UiTreeNodeProps::new("sample.md", "sample.md", 0, UiTreeNodeKind::File).icon(icon),
        ],
        ..UiTreeProps::default()
    }
}

fn nested_tree() -> UiTreeProps {
    UiTreeProps {
        line_display: true,
        icons_visible: true,
        nodes: vec![
            UiTreeNodeProps::new("assets", "assets", 0, UiTreeNodeKind::Directory),
            UiTreeNodeProps::new("assets/fixtures", "fixtures", 1, UiTreeNodeKind::Directory),
            UiTreeNodeProps::new(
                "assets/fixtures/sample.md",
                "sample.md",
                2,
                UiTreeNodeKind::File,
            )
            .selected(true),
        ],
        ..UiTreeProps::default()
    }
}

fn pixel_at(canvas: &Canvas, x: usize, y: usize) -> Option<u32> {
    canvas.pixels().get(y * canvas.width() + x).copied()
}
