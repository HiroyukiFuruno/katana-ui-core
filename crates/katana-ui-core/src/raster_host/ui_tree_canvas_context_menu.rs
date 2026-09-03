use super::canvas::Canvas;
use super::text::TextRenderer;
use super::ui_tree_canvas_palette::UiTreeCanvasPalette;
use katana_ui_core::render_model::{
    UiContextMenuAnchor, UiContextMenuItem, UiContextMenuItemKind, UiHostActionPlan, UiNode,
    UiNodeKind,
};

const ITEM_HEIGHT: usize = 24;
const TEXT_SIZE: f32 = 14.0;
const CHECK_SIZE: usize = 8;
const CHECK_X: usize = 8;
const LABEL_X: usize = 24;
const LABEL_Y: usize = 5;
const DIVIDER_X_INSET: usize = 8;
const DIVIDER_WIDTH_INSET: usize = 16;
const DIVIDER_HEIGHT: usize = 1;
const CHECK_Y_OFFSET: usize = 8;
const SHORTCUT_COLUMN_WIDTH: usize = 64;

pub(super) struct UiTreeContextMenuRenderer;

impl UiTreeContextMenuRenderer {
    pub(super) fn draw(
        canvas: &mut Canvas,
        text: &TextRenderer,
        node: &UiNode,
        y: &mut usize,
        palette: UiTreeCanvasPalette,
    ) {
        let rect = ContextMenuRect::from_node(node);
        canvas.fill_rect(
            rect.x,
            rect.y,
            rect.width,
            rect.height,
            palette.preview_background,
        );
        canvas.stroke_rect(
            rect.x,
            rect.y,
            rect.width,
            rect.height,
            palette.muted_border,
        );
        for (index, item) in node.props().context_menu.items.iter().enumerate() {
            Self::draw_item(canvas, text, item, index, rect, node, palette);
        }
        *y = rect.y.saturating_add(rect.height);
    }

    pub(super) fn item_id_at(node: &UiNode, x: f32, y: f32) -> Option<String> {
        let (_, _, item) = Self::item_at(node, x, y)?;
        Some(item.id.clone())
    }

    pub(super) fn host_action_at(node: &UiNode, x: f32, y: f32) -> Option<UiHostActionPlan> {
        let (context_node, index, item) = Self::item_at(node, x, y)?;
        UiHostActionPlan::from_context_menu_item(
            context_node.id().clone(),
            item,
            node_enabled(context_node),
            &[index],
        )
    }

    pub(super) fn item_center_for_id(node: &UiNode, item_id: &str) -> Option<(f32, f32)> {
        let context_node = find_context_menu(node)?;
        let rect = ContextMenuRect::from_node(context_node);
        let index = context_node
            .props()
            .context_menu
            .items
            .iter()
            .position(|item| item.id == item_id)?;
        let x = rect.x + rect.width / 2;
        let y = rect.y + index * ITEM_HEIGHT + ITEM_HEIGHT / 2;
        Some((x as f32, y as f32))
    }

    fn item_at(node: &UiNode, x: f32, y: f32) -> Option<(&UiNode, usize, &UiContextMenuItem)> {
        let context_node = find_context_menu(node)?;
        let rect = ContextMenuRect::from_node(context_node);
        if !rect.contains(x, y) {
            return None;
        }
        let index = ((y - rect.y as f32) / ITEM_HEIGHT as f32).floor() as usize;
        let item = context_node.props().context_menu.items.get(index)?;
        if item.disabled
            || matches!(
                item.kind,
                UiContextMenuItemKind::Divider | UiContextMenuItemKind::Section
            )
        {
            return None;
        }
        Some((context_node, index, item))
    }

    fn draw_item(
        canvas: &mut Canvas,
        text: &TextRenderer,
        item: &UiContextMenuItem,
        index: usize,
        rect: ContextMenuRect,
        node: &UiNode,
        palette: UiTreeCanvasPalette,
    ) {
        let item_y = rect.y.saturating_add(index.saturating_mul(ITEM_HEIGHT));
        if highlighted(node, index) {
            canvas.fill_rect(
                rect.x + 1,
                item_y,
                rect.width.saturating_sub(2),
                ITEM_HEIGHT,
                palette.hover_background,
            );
        }
        if item.kind == UiContextMenuItemKind::Divider {
            canvas.fill_rect(
                rect.x + DIVIDER_X_INSET,
                item_y + ITEM_HEIGHT / 2,
                rect.width.saturating_sub(DIVIDER_WIDTH_INSET),
                DIVIDER_HEIGHT,
                palette.muted_border,
            );
            return;
        }
        if item.checked {
            canvas.fill_rect(
                rect.x + CHECK_X,
                item_y + CHECK_Y_OFFSET,
                CHECK_SIZE,
                CHECK_SIZE,
                palette.selection,
            );
        }
        let color = if item.destructive {
            palette.danger_accent
        } else if item.disabled {
            palette.muted_border
        } else {
            palette.text
        };
        text.draw(
            canvas,
            &item.label,
            rect.x + LABEL_X,
            item_y + LABEL_Y,
            TEXT_SIZE,
            color,
        );
        if !item.shortcut.is_empty() {
            let shortcut_x = rect.x + rect.width.saturating_sub(SHORTCUT_COLUMN_WIDTH);
            text.draw(
                canvas,
                &item.shortcut,
                shortcut_x,
                item_y + LABEL_Y,
                TEXT_SIZE,
                palette.muted_border,
            );
        }
    }
}

#[derive(Clone, Copy)]
struct ContextMenuRect {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl ContextMenuRect {
    fn from_node(node: &UiNode) -> Self {
        let props = &node.props().context_menu;
        let (x, y) = match props.anchor {
            UiContextMenuAnchor::Pointer { x, y } => (non_negative(x), non_negative(y)),
            UiContextMenuAnchor::VirtualRect(rect) => (non_negative(rect.x), non_negative(rect.y)),
            UiContextMenuAnchor::NodeId(_) => (0, 0),
        };
        let item_height = props.items.len().saturating_mul(ITEM_HEIGHT);
        let max_height = props.max_height as usize;
        Self {
            x,
            y,
            width: props.min_width as usize,
            height: item_height.min(max_height).max(ITEM_HEIGHT),
        }
    }

    fn contains(self, x: f32, y: f32) -> bool {
        x >= self.x as f32
            && x <= self.x.saturating_add(self.width) as f32
            && y >= self.y as f32
            && y < self.y.saturating_add(self.height) as f32
    }
}

fn find_context_menu(node: &UiNode) -> Option<&UiNode> {
    if node.kind() == UiNodeKind::ContextMenu {
        return Some(node);
    }
    node.children().iter().find_map(find_context_menu)
}

fn node_enabled(node: &UiNode) -> bool {
    !node.props().disabled && !node.props().common.disabled
}

fn highlighted(node: &UiNode, index: usize) -> bool {
    node.props()
        .context_menu
        .highlighted_path
        .first()
        .is_some_and(|selected| *selected == index)
}

fn non_negative(value: i32) -> usize {
    value.max(0) as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    use katana_ui_core::facade::UiCoreFacade;
    use katana_ui_core::render_model::{UiContextMenuProps, UiContextMenuRect, UiHostActionSpec};
    use katana_ui_core::theme::ThemeSnapshot;

    #[test]
    fn context_menu_draws_highlight_divider_shortcut_and_semantic_item_tones() {
        let palette = UiTreeCanvasPalette::from_theme(&ThemeSnapshot::dark());
        let node = menu_node(UiContextMenuAnchor::VirtualRect(UiContextMenuRect::new(
            4, 6, 160, 96,
        )));
        let text = TextRenderer::load(&UiCoreFacade::default(), "body");
        let mut canvas = Canvas::new(200, 140, palette.background);
        let mut y = 0;

        UiTreeContextMenuRenderer::draw(&mut canvas, &text, &node, &mut y, palette);

        assert_eq!(102, y);
        assert!(count_color(&canvas, palette.hover_background) > 0);
        assert!(count_color(&canvas, palette.muted_border) > 0);
        assert!(count_color(&canvas, palette.selection) > 0);
        assert!(count_color(&canvas, palette.danger_accent) > 0);
    }

    #[test]
    fn context_menu_hit_test_rejects_outside_disabled_and_divider_rows() {
        let node = menu_node(UiContextMenuAnchor::Pointer { x: 4, y: 6 });

        assert_eq!(None, UiTreeContextMenuRenderer::item_id_at(&node, 3.0, 7.0));
        assert_eq!(
            None,
            UiTreeContextMenuRenderer::item_id_at(&node, 10.0, 42.0)
        );
        assert_eq!(
            None,
            UiTreeContextMenuRenderer::item_id_at(&node, 10.0, 90.0)
        );
        assert_eq!(
            Some("open".to_string()),
            UiTreeContextMenuRenderer::item_id_at(&node, 10.0, 18.0)
        );
        assert!(UiTreeContextMenuRenderer::host_action_at(&node, 10.0, 18.0).is_some());
        assert_eq!(
            Some((84.0, 18.0)),
            UiTreeContextMenuRenderer::item_center_for_id(&node, "open")
        );
        assert_eq!(
            None,
            UiTreeContextMenuRenderer::item_center_for_id(&node, "missing")
        );
    }

    #[test]
    fn node_anchor_defaults_to_origin_and_nested_lookup_finds_context_menu() {
        let menu = menu_node(UiContextMenuAnchor::NodeId("trigger".to_string()));
        let root = UiNode::new(UiNodeKind::Panel, "root").child(menu);

        assert_eq!(
            Some("open".to_string()),
            UiTreeContextMenuRenderer::item_id_at(&root, 10.0, 10.0)
        );
        assert_eq!(
            None,
            UiTreeContextMenuRenderer::item_id_at(&root, 181.0, 10.0)
        );
    }

    fn menu_node(anchor: UiContextMenuAnchor) -> UiNode {
        let items = vec![
            UiContextMenuItem::action("open", "Open")
                .checked(true)
                .shortcut("Cmd+O")
                .host_action(UiHostActionSpec::command("open", "Open")),
            UiContextMenuItem::new("divider", "", UiContextMenuItemKind::Divider),
            UiContextMenuItem::action("delete", "Delete").destructive(true),
            UiContextMenuItem::action("disabled", "Disabled").disabled(true),
        ];
        UiNode::new(UiNodeKind::ContextMenu, "menu").context_menu(UiContextMenuProps {
            anchor,
            min_width: 160,
            max_height: 120,
            highlighted_path: vec![0],
            items,
            ..UiContextMenuProps::default()
        })
    }

    fn count_color(canvas: &Canvas, color: u32) -> usize {
        canvas
            .pixels()
            .iter()
            .filter(|pixel| **pixel == color)
            .count()
    }
}
