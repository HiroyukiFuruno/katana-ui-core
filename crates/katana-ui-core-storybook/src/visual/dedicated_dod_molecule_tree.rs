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
    for (index, node) in tree.nodes.iter().take(VISIBLE_TREE_ROWS).enumerate() {
        draw_tree_row(
            canvas,
            text,
            palette,
            node,
            index,
            &tree.nodes,
            tree.line_style,
            tree.line_width,
            tree.line_display,
            tree.icons_visible,
            x,
            y,
        );
    }
}

fn draw_tree_row(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    node: &UiTreeNodeProps,
    index: usize,
    rows: &[UiTreeNodeProps],
    line_style: UiTreeLineStyle,
    line_width: u8,
    line_display: bool,
    icons_visible: bool,
    x: usize,
    y: usize,
) {
    let row_y = y + parts::TREE_PANEL_Y + m::PX_6 + index * parts::ROW_HEIGHT;
    let row_center_y = row_y + m::PX_8;
    let disclosure_x = x + parts::DISCLOSURE_X + node.depth * parts::INDENT_STEP;
    let marker_x = x + parts::NODE_ICON_X + node.depth * parts::INDENT_STEP;
    let label_x = if icons_visible {
        x + parts::LABEL_X + node.depth * parts::INDENT_STEP
    } else {
        x + parts::NODE_ICON_X + node.depth * parts::INDENT_STEP
    };
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
    if matches!(node.kind, UiTreeNodeKind::Directory) {
        parts::draw_disclosure(canvas, palette.muted, disclosure_x, row_y, node.expanded);
    }
    if line_display {
        draw_indent_guides(
            canvas,
            palette,
            node,
            rows,
            index,
            row_center_y,
            row_y,
            x,
            line_style,
            usize::from(line_width.max(1)),
        );
    }
    if icons_visible && matches!(node.kind, UiTreeNodeKind::Directory) {
        parts::branch_marker(canvas, marker_x, row_y);
    } else if icons_visible {
        parts::leaf_marker(canvas, marker_x, row_y);
    }
    text.draw(
        canvas,
        &node.label,
        label_x,
        row_y + m::PX_2,
        m::FONT_8,
        palette.text,
    );
}

fn draw_indent_guides(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    node: &UiTreeNodeProps,
    rows: &[UiTreeNodeProps],
    row_index: usize,
    row_center_y: usize,
    row_y: usize,
    x: usize,
    line_style: UiTreeLineStyle,
    line_width: usize,
) {
    let row_center_x =
        x + parts::DISCLOSURE_X + m::PX_4 + m::PX_1 + node.depth * parts::INDENT_STEP;
    let previous_depth = row_index
        .checked_sub(1)
        .and_then(|index| rows.get(index).map(|node| node.depth));
    let next_depth = rows.get(row_index + 1).map(|node| node.depth);
    let current_depth = node.depth;

    for level in 0..=node.depth {
        let level_center_x =
            row_center_x - node.depth * parts::INDENT_STEP + level * parts::INDENT_STEP;
        let has_up = if current_depth > level {
            previous_depth.is_some_and(|depth| depth >= level)
        } else {
            false
        };
        let has_down = next_depth.is_some_and(|depth| depth > level);
        let start_y = if has_up {
            row_y.saturating_sub(m::PX_2)
        } else {
            row_center_y
        };
        let end_y = if has_down {
            row_y + parts::ROW_HEIGHT + m::PX_2
        } else {
            row_center_y + 1
        };

        if has_up || has_down {
            let stroke_length = end_y.saturating_sub(start_y);
            draw_styled_line(
                canvas,
                level_center_x,
                start_y,
                stroke_length,
                line_width,
                line_style,
                true,
                palette.border,
            );
        }
    }
    draw_styled_line(
        canvas,
        row_center_x - (parts::INDENT_STEP - m::PX_4),
        row_center_y,
        parts::INDENT_STEP - m::PX_4,
        line_width,
        line_style,
        false,
        palette.border,
    );
}

fn draw_styled_line(
    canvas: &mut Canvas,
    x: usize,
    y: usize,
    length: usize,
    width: usize,
    style: UiTreeLineStyle,
    vertical: bool,
    color: u32,
) {
    let segment_length = 4usize;
    let segment_gap = 6usize;
    match style {
        UiTreeLineStyle::Solid => {
            draw_segment(canvas, x, y, length, width, 0, width, vertical, color)
        }
        UiTreeLineStyle::Dotted => draw_segment(
            canvas,
            x,
            y,
            length,
            width,
            segment_length,
            segment_gap,
            vertical,
            color,
        ),
        UiTreeLineStyle::Dashed => {
            draw_segment(canvas, x, y, length, width, 10, 6, vertical, color)
        }
    }
}

fn draw_segment(
    canvas: &mut Canvas,
    x: usize,
    y: usize,
    length: usize,
    width: usize,
    on_length: usize,
    off_length: usize,
    vertical: bool,
    color: u32,
) {
    if on_length == 0 {
        if vertical {
            common::fill(canvas, Rect::new(x, y, width, length), color);
        } else {
            common::fill(canvas, Rect::new(x, y, length, width), color);
        }
        return;
    }
    let pitch = on_length + off_length;
    for offset in (0..length).step_by(pitch) {
        let segment_length = on_length.min(length.saturating_sub(offset));
        if vertical {
            common::fill(
                canvas,
                Rect::new(x, y + offset, width, segment_length),
                color,
            );
        } else {
            common::fill(
                canvas,
                Rect::new(x + offset, y, segment_length, width),
                color,
            );
        }
    }
    // Prevent full-width gaps at the end due to `step_by`.
    if length > 0 && on_length > 0 {
        let tail_offset = (length / pitch) * pitch;
        if tail_offset < length {
            let tail = length.saturating_sub(tail_offset);
            if tail > 0 && tail <= on_length {
                if vertical {
                    common::fill(canvas, Rect::new(x, y + tail_offset, width, tail), color);
                } else {
                    common::fill(canvas, Rect::new(x + tail_offset, y, tail, width), color);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use katana_ui_core::{
        facade::UiCoreFacade,
        molecule::{TreeLineStyle, TreeNode, TreeView},
        theme::ThemeSnapshot,
    };

    fn sample_tree(
        line_display: bool,
        line_style: TreeLineStyle,
        line_width: u8,
        icons_visible: bool,
        nodes: &[(usize, &str, bool)],
    ) -> UiNode {
        let mut tree = TreeView::new("Tree settings").line_display(line_display);
        tree = tree.line_style(line_style);
        tree = tree.line_width(line_width);
        tree = tree.icons_visible(icons_visible);
        for (index, (depth, label, directory)) in nodes.iter().enumerate() {
            let node = if *directory {
                TreeNode::new(format!("id-{index}"), *label, *depth).directory()
            } else {
                TreeNode::new(format!("id-{index}"), *label, *depth)
            };
            tree = tree.item(node);
        }
        tree.into()
    }

    fn render_tree(line_display: bool, nodes: &[(usize, &str, bool)]) -> (Canvas, VisualPalette) {
        render_tree_with_style(line_display, TreeLineStyle::Solid, 1, true, nodes)
    }

    fn render_tree_with_style(
        line_display: bool,
        line_style: TreeLineStyle,
        line_width: u8,
        icons_visible: bool,
        nodes: &[(usize, &str, bool)],
    ) -> (Canvas, VisualPalette) {
        let facade = UiCoreFacade::new(ThemeSnapshot::dark());
        let theme_palette = VisualPalette::from_theme(&facade.theme());
        let text = TextRenderer::load(&facade, facade.default_font_role());
        let mut canvas = Canvas::new(220, 130, theme_palette.background);
        let node = sample_tree(line_display, line_style, line_width, icons_visible, nodes);
        tree_view(&mut canvas, &text, &node, &theme_palette, 0, 0);
        (canvas, theme_palette)
    }

    fn pixel_at(canvas: &Canvas, x: usize, y: usize) -> Option<u32> {
        canvas.pixels().get(y * canvas.width() + x).copied()
    }

    fn row_center_y(row_index: usize) -> usize {
        parts::TREE_PANEL_Y + m::PX_6 + m::PX_8 + row_index * parts::ROW_HEIGHT
    }

    #[test]
    fn tree_view_hides_guides_when_line_display_is_false() {
        let nodes = [
            (0, "root", true),
            (1, "child", true),
            (2, "grandchild", false),
        ];
        let (canvas, theme_palette) = render_tree(false, &nodes);
        let child_center_x = parts::DISCLOSURE_X + m::PX_4 + m::PX_1;

        assert_eq!(
            None,
            pixel_at(&canvas, child_center_x, row_center_y(1))
                .filter(|&color| color == theme_palette.border)
        );
        assert_eq!(
            None,
            pixel_at(
                &canvas,
                child_center_x + parts::INDENT_STEP,
                row_center_y(2)
            )
            .filter(|&color| color == theme_palette.border)
        );
    }

    #[test]
    fn tree_view_stops_connector_extension_when_next_node_depth_drops() {
        let nodes = [(0, "root", true), (1, "child", false), (0, "after", false)];
        let (canvas, palette) = render_tree(true, &nodes);
        let parent_column_x = parts::DISCLOSURE_X + m::PX_4 + m::PX_1;
        let child_row_center_y = row_center_y(1);
        let below_child_center_y = child_row_center_y + m::PX_2;

        assert_eq!(
            Some(palette.border),
            pixel_at(&canvas, parent_column_x, child_row_center_y)
        );
        assert_eq!(
            None,
            pixel_at(&canvas, parent_column_x, below_child_center_y)
                .filter(|&color| color == palette.border)
        );
    }

    #[test]
    fn tree_view_draws_lines_with_styled_pattern() {
        let nodes = [(0, "root", true), (1, "child", false)];
        let (solid_canvas, palette) =
            render_tree_with_style(true, TreeLineStyle::Solid, 1, true, &nodes);
        let (dotted_canvas, _) =
            render_tree_with_style(true, TreeLineStyle::Dotted, 1, true, &nodes);
        let (dashed_canvas, _) =
            render_tree_with_style(true, TreeLineStyle::Dashed, 1, true, &nodes);
        let line_x = parts::DISCLOSURE_X + m::PX_4 + m::PX_1;
        let from = row_center_y(0);
        let to = from + parts::ROW_HEIGHT + m::PX_2;
        let solid_count = line_pixel_count(&solid_canvas, line_x, from, to, palette.border);
        let dotted_count = line_pixel_count(&dotted_canvas, line_x, from, to, palette.border);
        let dashed_count = line_pixel_count(&dashed_canvas, line_x, from, to, palette.border);

        assert!(solid_count > dashed_count);
        assert!(dashed_count > dotted_count);
        assert!(dotted_count > 0);
    }

    #[test]
    fn tree_view_applies_line_width() {
        let nodes = [(0, "root", true), (1, "child", false)];
        let (canvas, palette) = render_tree_with_style(true, TreeLineStyle::Solid, 2, true, &nodes);
        let line_x = parts::DISCLOSURE_X + m::PX_4 + m::PX_1;
        let sample_y = row_center_y(0);

        assert_eq!(Some(palette.border), pixel_at(&canvas, line_x, sample_y));
        assert_eq!(
            Some(palette.border),
            pixel_at(&canvas, line_x + 1, sample_y)
        );
    }

    #[test]
    fn tree_view_draws_solid_horizontal_connector_without_vertical_bleed() {
        let nodes = [(0, "root", true), (1, "child", false)];
        let (canvas, palette) = render_tree_with_style(true, TreeLineStyle::Solid, 1, true, &nodes);
        let row_center_y = row_center_y(0);
        let node_depth_center_x = parts::DISCLOSURE_X + m::PX_4 + m::PX_1;
        let connector_start_x = node_depth_center_x - (parts::INDENT_STEP - m::PX_4);
        let connector_end_x = connector_start_x + (parts::INDENT_STEP - m::PX_4);

        for x in connector_start_x..connector_end_x {
            assert_eq!(
                Some(palette.border),
                pixel_at(&canvas, x, row_center_y),
                "horizontal connector should draw border at ({x}, {row_center_y})"
            );
            assert_eq!(
                None,
                pixel_at(&canvas, x, row_center_y + 1).filter(|&color| color == palette.border),
                "horizontal connector should not bleed vertically at ({x}, {})",
                row_center_y + 1
            );
        }
    }

    #[test]
    fn tree_view_hides_icons_when_icons_visible_is_false() {
        let nodes = [(0, "", true)];
        let (with_icons, _) = render_tree_with_style(true, TreeLineStyle::Solid, 1, true, &nodes);
        let (without_icons, palette) =
            render_tree_with_style(true, TreeLineStyle::Solid, 1, false, &nodes);
        let icon_sample_x = parts::NODE_ICON_X + 2;
        let icon_sample_y = parts::TREE_PANEL_Y + m::PX_6 + m::PX_4;

        assert_ne!(
            Some(palette.panel),
            pixel_at(&with_icons, icon_sample_x, icon_sample_y)
        );
        assert_eq!(
            Some(palette.panel),
            pixel_at(&without_icons, icon_sample_x, icon_sample_y)
        );
    }

    fn line_pixel_count(
        canvas: &Canvas,
        x: usize,
        from_y: usize,
        to_y: usize,
        color: u32,
    ) -> usize {
        (from_y..to_y)
            .filter(|&y| pixel_at(canvas, x, y) == Some(color))
            .count()
    }
}
