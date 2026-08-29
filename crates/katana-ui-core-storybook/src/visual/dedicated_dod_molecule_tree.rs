use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Rect};
use super::dedicated_dod_metrics as m;
use super::dedicated_dod_molecule_tree_lines::{
    TreeGuideLayout, TreeLineOptions, TreeRowLayout, draw_indent_guides,
};
use super::dedicated_dod_molecule_tree_parts as parts;
use super::palette::VisualPalette;
use super::text::TextRenderer;
use katana_ui_core::molecule::{FileTree, FileTreeItem, FileTreeState};
use katana_ui_core::render_model::{UiNode, UiTreeNodeKind, UiTreeNodeProps, UiTreeProps};

const VISIBLE_TREE_ROWS: usize = 3;
const DEFAULT_TREE_SELECTED_ID: &str = "katana/a.md";
const NESTED_TREE_SELECTED_ID: &str = "katana/nested/b.md";
const PROJECTED_SCROLL_WINDOW_ID: &str = "storybook.projected-scroll-window";

pub(super) struct TreeViewRenderState<'a> {
    pub(super) scroll_offset_y: u32,
    pub(super) selected_id: &'a str,
    pub(super) focused_id: &'a str,
    pub(super) keyboard_committed: bool,
}

pub(super) fn tree_view(
    canvas: &mut Canvas,
    text: &TextRenderer,
    node: &UiNode,
    palette: &VisualPalette,
    state: TreeViewRenderState<'_>,
    x: usize,
    y: usize,
) {
    common::frame(canvas, text, palette, x, y, "TreeView");
    let tree = tree_props_for_state(
        node.props().tree.clone(),
        state.scroll_offset_y,
        state.selected_id,
    );
    draw_tree_panel(
        canvas,
        text,
        tree.clone(),
        palette,
        state.scroll_offset_y,
        state.focused_id,
        state.keyboard_committed,
        x,
        y,
    );
    if tree.empty_area_context_menu {
        parts::draw_context_menu(canvas, text, palette, x, y);
    }
    parts::draw_option_strip(canvas, text, palette, &tree, x, y);
}

fn tree_props_for_state(
    story_tree: UiTreeProps,
    scroll_offset_y: u32,
    selected_id: &str,
) -> UiTreeProps {
    let first_visible_row =
        usize::try_from(scroll_offset_y).unwrap_or(usize::MAX) / parts::ROW_HEIGHT;
    if selected_id == DEFAULT_TREE_SELECTED_ID && first_visible_row < story_tree.nodes.len() {
        return story_tree;
    }
    let rendered = FileTree::render_with_state_and_offset(
        &tree_items(),
        selected_id,
        parts::TREE_PANEL_WIDTH as u32,
        parts::TREE_PANEL_HEIGHT as u32,
        scroll_offset_y,
        &FileTreeState::default(),
    );
    let mut tree = rendered.root().children()[0].props().tree.clone();
    tree.line_display = story_tree.line_display;
    tree.line_style = story_tree.line_style;
    tree.line_width = story_tree.line_width;
    tree.icons_visible = story_tree.icons_visible;
    tree.empty_area_context_menu = story_tree.empty_area_context_menu;
    tree.toggle_trigger_area = story_tree.toggle_trigger_area;
    tree.default_open = story_tree.default_open;
    tree.active_id = PROJECTED_SCROLL_WINDOW_ID.to_string();
    tree
}

fn draw_tree_panel(
    canvas: &mut Canvas,
    text: &TextRenderer,
    tree: UiTreeProps,
    palette: &VisualPalette,
    scroll_offset_y: u32,
    focused_id: &str,
    keyboard_committed: bool,
    x: usize,
    y: usize,
) {
    let panel = Rect::new(
        x + parts::TREE_PANEL_X,
        y + parts::TREE_PANEL_Y,
        parts::TREE_PANEL_WIDTH,
        parts::TREE_PANEL_HEIGHT,
    );
    common::fill(canvas, panel, palette.surface);
    common::outline(canvas, palette, panel);
    let line_options = TreeLineOptions {
        rows: &tree.nodes,
        style: tree.line_style,
        width: usize::from(tree.line_width.max(1)),
        visible: tree.line_display,
        icons_visible: tree.icons_visible,
    };
    let first_visible_row = if tree.active_id == PROJECTED_SCROLL_WINDOW_ID {
        0
    } else {
        usize::try_from(scroll_offset_y).unwrap_or(usize::MAX) / parts::ROW_HEIGHT
    };
    for (index, node) in tree
        .nodes
        .iter()
        .skip(first_visible_row)
        .take(VISIBLE_TREE_ROWS)
        .enumerate()
    {
        draw_tree_row(
            canvas,
            text,
            palette,
            node,
            TreeRowLayout {
                index: first_visible_row + index,
                visual_index: index,
                x,
                y,
            },
            line_options,
            focused_id,
            keyboard_committed,
        );
    }
}

fn draw_tree_row(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    node: &UiTreeNodeProps,
    layout: TreeRowLayout,
    line_options: TreeLineOptions<'_>,
    focused_id: &str,
    keyboard_committed: bool,
) {
    let row_y = layout.y + parts::TREE_PANEL_Y + m::PX_6 + layout.visual_index * parts::ROW_HEIGHT;
    let row_center_y = row_y + m::PX_8;
    let disclosure_x = layout.x + parts::DISCLOSURE_X + node.depth * parts::INDENT_STEP;
    let marker_x = layout.x + parts::NODE_ICON_X + node.depth * parts::INDENT_STEP;
    let label_x = if line_options.icons_visible {
        layout.x + parts::LABEL_X + node.depth * parts::INDENT_STEP
    } else {
        layout.x + parts::NODE_ICON_X + node.depth * parts::INDENT_STEP
    };
    if node.selected {
        common::fill(
            canvas,
            Rect::new(
                layout.x + parts::TREE_PANEL_X + m::PX_2,
                row_y - m::PX_2,
                parts::TREE_PANEL_WIDTH - m::PX_2 - m::PX_2,
                parts::ROW_HEIGHT,
            ),
            if keyboard_committed {
                common::SUCCESS
            } else {
                palette.accent
            },
        );
    }
    if node.id == focused_id {
        common::outline(
            canvas,
            palette,
            Rect::new(
                layout.x + parts::TREE_PANEL_X + m::PX_1,
                row_y - m::PX_3,
                parts::TREE_PANEL_WIDTH - m::PX_2,
                parts::ROW_HEIGHT + m::PX_2,
            ),
        );
    }
    if matches!(node.kind, UiTreeNodeKind::Directory) {
        parts::draw_disclosure(canvas, palette.muted, disclosure_x, row_y, node.expanded);
    }
    if line_options.visible {
        draw_indent_guides(
            canvas,
            palette,
            TreeGuideLayout {
                node,
                rows: line_options.rows,
                row_index: layout.index,
                row_center_y,
                row_y,
                x: layout.x,
                style: line_options.style,
                width: line_options.width,
                draw_horizontal_connector: matches!(node.kind, UiTreeNodeKind::Directory),
            },
        );
    }
    if line_options.icons_visible && matches!(node.kind, UiTreeNodeKind::Directory) {
        parts::branch_marker(canvas, marker_x, row_y);
    } else if line_options.icons_visible {
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

fn tree_items() -> Vec<FileTreeItem> {
    vec![
        FileTreeItem::new(DEFAULT_TREE_SELECTED_ID, DEFAULT_TREE_SELECTED_ID),
        FileTreeItem::new(NESTED_TREE_SELECTED_ID, NESTED_TREE_SELECTED_ID).icon("markdown"),
    ]
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
        render_tree_with_style_and_offset(
            line_display,
            line_style,
            line_width,
            icons_visible,
            nodes,
            0,
        )
    }

    fn render_tree_with_style_and_offset(
        line_display: bool,
        line_style: TreeLineStyle,
        line_width: u8,
        icons_visible: bool,
        nodes: &[(usize, &str, bool)],
        scroll_offset_y: u32,
    ) -> (Canvas, VisualPalette) {
        let facade = UiCoreFacade::new(ThemeSnapshot::dark());
        let theme_palette = VisualPalette::from_theme(facade.theme());
        let text = TextRenderer::load(&facade, facade.default_font_role());
        let mut canvas = Canvas::new(220, 130, theme_palette.background);
        let node = sample_tree(line_display, line_style, line_width, icons_visible, nodes);
        tree_view(
            &mut canvas,
            &text,
            &node,
            &theme_palette,
            TreeViewRenderState {
                scroll_offset_y,
                selected_id: DEFAULT_TREE_SELECTED_ID,
                focused_id: "",
                keyboard_committed: false,
            },
            0,
            0,
        );
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
    fn tree_view_scroll_offset_changes_visible_rows() {
        let nodes = [
            (0, "root", true),
            (1, "child", true),
            (2, "grandchild", false),
            (0, "after", false),
        ];
        let (top, _) =
            render_tree_with_style_and_offset(true, TreeLineStyle::Solid, 1, true, &nodes, 0);
        let (scrolled, _) = render_tree_with_style_and_offset(
            true,
            TreeLineStyle::Solid,
            1,
            true,
            &nodes,
            parts::ROW_HEIGHT as u32,
        );

        assert!(
            top.text_runs().iter().any(|run| run.text() == "root"),
            "top viewport should show the root row"
        );
        assert!(
            !scrolled.text_runs().iter().any(|run| run.text() == "root"),
            "scrolled viewport must not keep rendering the first row"
        );
        assert!(
            scrolled.text_runs().iter().any(|run| run.text() == "after"),
            "scrolled viewport should reveal a later row"
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
                pixel_at(&canvas, x, row_center_y + 1).filter(|&color| color == palette.border)
            );
        }
    }

    #[test]
    fn tree_view_draws_focus_outline_for_the_focused_row() {
        let facade = UiCoreFacade::new(ThemeSnapshot::dark());
        let palette = VisualPalette::from_theme(facade.theme());
        let text = TextRenderer::load(&facade, facade.default_font_role());
        let node = sample_tree(true, TreeLineStyle::Solid, 1, true, &[(0, "root", true)]);
        let mut unfocused = Canvas::new(220, 130, palette.background);
        let mut focused = Canvas::new(220, 130, palette.background);
        let base_state = TreeViewRenderState {
            scroll_offset_y: 0,
            selected_id: DEFAULT_TREE_SELECTED_ID,
            focused_id: "",
            keyboard_committed: false,
        };
        tree_view(&mut unfocused, &text, &node, &palette, base_state, 0, 0);
        tree_view(
            &mut focused,
            &text,
            &node,
            &palette,
            TreeViewRenderState {
                scroll_offset_y: 0,
                selected_id: DEFAULT_TREE_SELECTED_ID,
                focused_id: "id-0",
                keyboard_committed: false,
            },
            0,
            0,
        );

        assert_ne!(unfocused.pixels(), focused.pixels());
    }

    #[test]
    fn tree_view_draws_horizontal_elbow_for_directory_rows() {
        let nodes = [(0, "root", true), (1, "child", true)];
        let (canvas, palette) = render_tree_with_style(true, TreeLineStyle::Solid, 1, true, &nodes);
        let child_row_center_y = row_center_y(1);
        let child_connector_x = horizontal_connector_sample_x(1);

        assert_eq!(
            Some(palette.border),
            pixel_at(&canvas, child_connector_x, child_row_center_y)
        );
    }

    #[test]
    fn tree_view_skips_horizontal_elbow_for_leaf_rows() {
        let nodes = [(0, "root", true), (1, "child", false)];
        let (canvas, palette) = render_tree_with_style(true, TreeLineStyle::Solid, 1, true, &nodes);
        let child_row_center_y = row_center_y(1);
        let child_connector_x = horizontal_connector_sample_x(1);

        assert_ne!(
            Some(palette.border),
            pixel_at(&canvas, child_connector_x, child_row_center_y)
        );
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

    fn horizontal_connector_sample_x(depth: usize) -> usize {
        let depth_center_x = parts::DISCLOSURE_X + m::PX_4 + m::PX_1 + depth * parts::INDENT_STEP;
        depth_center_x - (parts::INDENT_STEP - m::PX_4) + 1
    }
}
