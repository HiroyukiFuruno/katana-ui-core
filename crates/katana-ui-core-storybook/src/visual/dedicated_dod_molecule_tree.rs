use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Rect};
use super::dedicated_dod_metrics as m;
use super::dedicated_dod_molecule_tree_parts as parts;
use super::palette::VisualPalette;
use super::text::TextRenderer;
use katana_ui_core::render_model::{
    UiNode, UiTreeLineStyle, UiTreeNodeKind, UiTreeNodeProps, UiTreeProps,
};

const VISIBLE_TREE_ROWS: usize = 3;
const LINE_DASH_STEP: usize = 8;
const LINE_DASH_HEIGHT: usize = 4;
const LINE_DOT_STEP: usize = 6;
const LINE_DOT_HEIGHT: usize = 2;

pub(super) fn tree_view(
    canvas: &mut Canvas,
    text: &TextRenderer,
    node: &UiNode,
    palette: &VisualPalette,
    x: usize,
    y: usize,
) {
    common::frame(canvas, text, palette, x, y, "TreeView");
    draw_tree_panel(canvas, text, node.props().tree.clone(), palette, x, y);
    if node.props().tree.empty_area_context_menu {
        parts::draw_context_menu(canvas, text, palette, x, y);
    }
    parts::draw_option_strip(canvas, text, palette, &node.props().tree, x, y);
}

fn draw_tree_panel(
    canvas: &mut Canvas,
    text: &TextRenderer,
    tree: UiTreeProps,
    palette: &VisualPalette,
    x: usize,
    y: usize,
) {
    let panel = Rect::new(
        x + parts::TREE_PANEL_X,
        y + parts::TREE_PANEL_Y,
        parts::TREE_PANEL_WIDTH,
        parts::TREE_PANEL_HEIGHT,
    );
    common::outline(canvas, palette, panel);
    if tree.line_display {
        draw_tree_lines(canvas, palette, &tree, x, y);
    }
    for (index, node) in tree.nodes.iter().take(VISIBLE_TREE_ROWS).enumerate() {
        draw_tree_row(canvas, text, palette, node, index, x, y);
    }
}

fn draw_tree_lines(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    tree: &UiTreeProps,
    x: usize,
    y: usize,
) {
    draw_styled_vertical_line(canvas, palette, tree, x, y);
    for offset in [m::PX_22, m::PX_34, m::PX_46] {
        common::fill(
            canvas,
            Rect::new(
                x + parts::LINE_X + m::PX_20,
                y + parts::TREE_PANEL_Y + offset,
                m::PX_10,
                usize::from(tree.line_width.max(1)),
            ),
            palette.border,
        );
    }
}

fn draw_styled_vertical_line(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    tree: &UiTreeProps,
    x: usize,
    y: usize,
) {
    let line_width = usize::from(tree.line_width.max(1));
    let start_y = y + parts::TREE_PANEL_Y + m::PX_16;
    let height = m::PX_42;
    match tree.line_style {
        UiTreeLineStyle::Solid => common::fill(
            canvas,
            Rect::new(x + parts::LINE_X, start_y, line_width, height),
            palette.border,
        ),
        UiTreeLineStyle::Dashed => draw_line_segments(
            canvas,
            palette,
            x,
            start_y,
            line_width,
            LINE_DASH_STEP,
            LINE_DASH_HEIGHT,
        ),
        UiTreeLineStyle::Dotted => draw_line_segments(
            canvas,
            palette,
            x,
            start_y,
            line_width,
            LINE_DOT_STEP,
            LINE_DOT_HEIGHT,
        ),
    }
}

fn draw_line_segments(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    x: usize,
    start_y: usize,
    line_width: usize,
    step: usize,
    segment_height: usize,
) {
    for offset in (m::PX_0..m::PX_42).step_by(step) {
        common::fill(
            canvas,
            Rect::new(
                x + parts::LINE_X,
                start_y + offset,
                line_width,
                segment_height,
            ),
            palette.border,
        );
    }
}

fn draw_tree_row(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    node: &UiTreeNodeProps,
    index: usize,
    x: usize,
    y: usize,
) {
    let row_y = y + parts::TREE_PANEL_Y + m::PX_6 + index * parts::ROW_HEIGHT;
    if node.selected {
        common::fill(
            canvas,
            Rect::new(
                x + parts::TREE_PANEL_X + m::PX_2,
                row_y - m::PX_2,
                parts::TREE_PANEL_WIDTH - m::PX_2 - m::PX_2,
                parts::ROW_HEIGHT,
            ),
            palette.accent,
        );
    }
    text.draw(
        canvas,
        toggle_label(node),
        x + m::PX_22,
        row_y + m::PX_2,
        m::FONT_8,
        palette.text,
    );
    let icon_x = parts::NODE_ICON_X + node.depth * m::PX_20;
    if matches!(node.kind, UiTreeNodeKind::Directory) {
        parts::branch_marker(canvas, x + icon_x, row_y);
    } else {
        parts::leaf_marker(canvas, x + icon_x, row_y);
    }
    text.draw(
        canvas,
        &node.label,
        x + parts::LABEL_X + node.depth * m::PX_20,
        row_y + m::PX_2,
        m::FONT_8,
        palette.text,
    );
}

fn toggle_label(node: &UiTreeNodeProps) -> &'static str {
    if !matches!(node.kind, UiTreeNodeKind::Directory) {
        return "";
    }
    if node.expanded { "v" } else { ">" }
}
