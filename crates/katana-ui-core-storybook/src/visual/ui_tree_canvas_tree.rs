use super::canvas::Canvas;
use super::text::TextRenderer;
use super::ui_tree_canvas_hover::UiTreeCanvasHover;
use super::ui_tree_canvas_palette::UiTreeCanvasPalette;
use super::ui_tree_canvas_tree_parts as parts;
use super::ui_tree_canvas_types::UiTreeRenderArea;
use katana_ui_core::render_model::{UiNode, UiTreeLineStyle};

const DOTTED_LINE_PERIOD: usize = 4;
const DASHED_LINE_PERIOD: usize = 8;
const DASHED_LINE_VISIBLE_LENGTH: usize = 5;

pub(super) struct UiTreeViewRenderer;

impl UiTreeViewRenderer {
    pub(super) fn draw(
        canvas: &mut Canvas,
        text: &TextRenderer,
        node: &UiNode,
        x: usize,
        y: &mut usize,
        area: UiTreeRenderArea,
        palette: UiTreeCanvasPalette,
    ) {
        if !node.props().label.trim().is_empty() {
            text.draw(
                canvas,
                &node.props().label,
                x,
                *y,
                parts::TEXT_SIZE,
                palette.text,
            );
            *y = y.saturating_add(parts::ROW_HEIGHT);
        }
        for tree_node in &node.props().tree.nodes {
            if y.saturating_add(parts::ROW_HEIGHT) > area.y.saturating_add(area.height) {
                *y = area.y.saturating_add(area.height);
                break;
            }
            let row_width = parts::row_background_width(area.width, area.x, x);
            Self::draw_indent_lines(canvas, node, tree_node.depth, x, *y, palette.muted_border);
            if tree_node.id == node.props().tree.hovered_id {
                canvas.fill_rect(
                    x,
                    *y,
                    row_width,
                    parts::ROW_HEIGHT,
                    palette.hover_background,
                );
            }
            if tree_node.selected || tree_node.active {
                canvas.fill_rect(x, *y, row_width, parts::ROW_HEIGHT, palette.selection);
            }
            if tree_node.id == node.props().tree.hovered_id {
                UiTreeCanvasHover::draw_border(
                    canvas,
                    &node.props().tree.row_hover_border,
                    x,
                    *y,
                    row_width,
                    parts::ROW_HEIGHT,
                    palette,
                );
            }
            let label_x = if node.props().tree.icons_visible {
                parts::draw_affordance(
                    canvas,
                    tree_node.kind,
                    tree_node.expanded,
                    &node.props().tree.directory_icon,
                    Self::file_icon(node, &tree_node.icon),
                    parts::content_x(x, tree_node.depth),
                    *y,
                    palette.text,
                );
                parts::label_x(x, tree_node.depth, true)
            } else {
                parts::label_x(x, tree_node.depth, false)
            };
            text.draw(
                canvas,
                &tree_node.label,
                label_x,
                *y,
                parts::TEXT_SIZE,
                palette.text,
            );
            *y = y.saturating_add(parts::ROW_HEIGHT);
        }
        *y = y.saturating_add(parts::NODE_GAP);
    }

    fn file_icon<'a>(node: &'a UiNode, icon: &'a str) -> &'a str {
        if icon.trim().is_empty() {
            return node.props().tree.file_icon.as_str();
        }
        icon
    }

    fn draw_indent_lines(
        canvas: &mut Canvas,
        node: &UiNode,
        depth: usize,
        x: usize,
        y: usize,
        color: u32,
    ) {
        if !node.props().tree.line_display || depth == 0 {
            return;
        }
        let width = usize::from(node.props().tree.line_width).max(1);
        for level in 0..depth {
            Self::draw_vertical_line(
                canvas,
                parts::line_x(x, level),
                y,
                width,
                parts::ROW_HEIGHT,
                node.props().tree.line_style,
                color,
            );
        }
    }

    fn draw_vertical_line(
        canvas: &mut Canvas,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        style: UiTreeLineStyle,
        color: u32,
    ) {
        for offset in 0..height {
            if !Self::line_pixel_visible(style, offset) {
                continue;
            }
            canvas.fill_rect(x, y.saturating_add(offset), width, 1, color);
        }
    }

    fn line_pixel_visible(style: UiTreeLineStyle, offset: usize) -> bool {
        match style {
            UiTreeLineStyle::Solid => true,
            UiTreeLineStyle::Dotted => offset.is_multiple_of(DOTTED_LINE_PERIOD),
            UiTreeLineStyle::Dashed => offset % DASHED_LINE_PERIOD < DASHED_LINE_VISIBLE_LENGTH,
        }
    }
}
