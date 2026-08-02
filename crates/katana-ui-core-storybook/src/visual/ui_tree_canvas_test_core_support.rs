use super::*;

pub(in crate::visual) fn render_canvas(root: UiNode) -> Canvas {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(
        RENDER_CANVAS_WIDTH,
        RENDER_CANVAS_HEIGHT,
        palette.background,
    );
    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: RENDER_CANVAS_WIDTH,
            height: RENDER_CANVAS_HEIGHT,
            scroll_y: 0.0,
        },
    );
    canvas
}

pub(in crate::visual) fn left_pane_like_tree_canvas_root() -> UiNode {
    UiNode::new(UiNodeKind::Row, "")
        .child(UiNode::new(UiNodeKind::TreeView, "").tree(tree_nodes()))
        .child(code_text_node())
}

pub(in crate::visual) fn tree_nodes() -> UiTreeProps {
    tree_nodes_for_test("src", true)
}

pub(in crate::visual) fn tree_nodes_for_test(label: &str, selected: bool) -> UiTreeProps {
    UiTreeProps {
        nodes: vec![
            UiTreeNodeProps::new("src", label, 0, UiTreeNodeKind::Directory).selected(selected),
        ],
        ..UiTreeProps::default()
    }
}

pub(in crate::visual) fn nested_tree_nodes(line_display: bool) -> UiTreeProps {
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

pub(in crate::visual) fn context_menu_node_for_test() -> UiNode {
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

pub(in crate::visual) fn settings_list_root() -> UiNode {
    SettingsList::new("KDV settings")
        .density(SettingsListDensity::Compact)
        .section(
            SettingsSection::new("display", "Display")
                .field(SettingsField::new(
                    "dark",
                    "Dark",
                    SettingsControl::Toggle { checked: true },
                ))
                .field(SettingsField::new(
                    "theme",
                    "Theme",
                    SettingsControl::Select {
                        options: vec![SettingsControlOption::new("dark", "Dark")],
                        selected: "dark".to_string(),
                    },
                ))
                .field(SettingsField::new(
                    "slide",
                    "Slide",
                    SettingsControl::Input {
                        value: "2/5".to_string(),
                    },
                )),
        )
        .into()
}

pub(in crate::visual) fn hovered_interaction() -> UiInteractionState {
    UiInteractionState {
        hovered: true,
        ..UiInteractionState::default()
    }
}

pub(in crate::visual) fn count_pixel(canvas: &Canvas, expected: u32) -> usize {
    canvas.pixels().iter().filter(|it| **it == expected).count()
}

pub(in crate::visual) fn foreground_pixels_in_rows(
    canvas: &Canvas,
    palette: UiTreeCanvasPalette,
    start_y: usize,
    end_y: usize,
) -> usize {
    let start = start_y.min(canvas.height()).saturating_mul(canvas.width());
    let end = end_y
        .min(canvas.height())
        .saturating_mul(canvas.width())
        .min(canvas.pixels().len());
    canvas.pixels()[start..end]
        .iter()
        .filter(|pixel| {
            **pixel != palette.background
                && **pixel != palette.code_background
                && **pixel != palette.muted_border
        })
        .count()
}

pub(in crate::visual) fn color_pixels_between_x(
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

pub(in crate::visual) fn first_row_for_color(canvas: &Canvas, expected: u32) -> Option<usize> {
    let width = canvas.width();
    for y in 0..canvas.height() {
        for x in 0..width {
            if pixel_at(canvas, x, y) == Some(expected) {
                return Some(y);
            }
        }
    }
    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::visual) struct VerticalBounds {
    pub(in crate::visual) top: usize,
    pub(in crate::visual) bottom: usize,
}

impl VerticalBounds {
    pub(in crate::visual) const fn center_twice(self) -> usize {
        self.top + self.bottom
    }
}

pub(in crate::visual) fn pixel_at(canvas: &Canvas, x: usize, y: usize) -> Option<u32> {
    canvas
        .pixels()
        .get(y.saturating_mul(canvas.width()) + x)
        .copied()
}

pub(in crate::visual) fn first_content_x(canvas: &Canvas) -> Option<usize> {
    let background = UiTreeCanvasPalette::from_theme(&ThemeSnapshot::dark()).background;
    canvas
        .pixels()
        .iter()
        .position(|pixel| *pixel != background)
        .map(|index| index % canvas.width())
}

pub(in crate::visual) fn rightmost_content_x(canvas: &Canvas, background: u32) -> Option<usize> {
    canvas
        .pixels()
        .iter()
        .rposition(|pixel| *pixel != background)
        .map(|index| index % canvas.width())
}

pub(in crate::visual) fn first_row_for_non_background(
    canvas: &Canvas,
    background: u32,
) -> Option<usize> {
    canvas
        .pixels()
        .iter()
        .position(|pixel| *pixel != background)
        .map(|index| index / canvas.width())
}

pub(in crate::visual) fn first_row_containing_color_after(
    canvas: &Canvas,
    color: u32,
    minimum_y: usize,
) -> Option<usize> {
    let start = minimum_y.saturating_mul(canvas.width());
    canvas
        .pixels()
        .iter()
        .enumerate()
        .skip(start)
        .find(|(_, pixel)| **pixel == color)
        .map(|(index, _)| index / canvas.width())
}

pub(in crate::visual) fn first_row_content_x_after(
    canvas: &Canvas,
    color: u32,
    minimum_x: usize,
) -> Option<usize> {
    for y in 0..canvas.height() {
        for x in minimum_x..canvas.width() {
            if pixel_at(canvas, x, y) == Some(color) {
                return Some(x);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::{Canvas, first_row_content_x_after, first_row_for_color};

    #[test]
    fn color_search_helpers_return_none_when_the_color_is_absent() {
        let canvas = Canvas::new(2, 2, 0);

        assert_eq!(None, first_row_for_color(&canvas, 1));
        assert_eq!(None, first_row_content_x_after(&canvas, 1, 0));
    }
}
