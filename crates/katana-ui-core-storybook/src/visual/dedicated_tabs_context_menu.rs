use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common, Rect};
use super::dedicated_dod_metrics as m;
use super::layout_metrics::LayoutRect;
use super::palette::VisualPalette;
use super::screen_state_tabs::{TabsContextMenuCommand, TabsScreenState};
use super::screen_state_tabs_types::TabsContextMenuState;
use super::text::{TextBox, TextRenderer};
use katana_ui_core::widget::molecules::ContextMenuItem;

const MENU_WIDTH: usize = 168;
const MENU_ROW_HEIGHT: usize = 22;
const MENU_PADDING_X: usize = 8;
const MENU_OFFSET_Y: usize = 2;

pub(super) fn draw_context_menu(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    state: &TabsScreenState,
    x: usize,
    y: usize,
) {
    let Some(menu) = state.context_menu.as_ref() else {
        return;
    };
    let rect = menu_rect(x, y, state);
    common::fill(canvas, rect_to_common(rect), palette.surface);
    common::outline(canvas, palette, rect_to_common(rect));
    for (index, item) in visible_items(menu).into_iter().enumerate() {
        draw_row(
            canvas,
            text,
            palette,
            rect,
            index,
            item.depth,
            item.item.label.as_str(),
        );
    }
}

pub(super) fn menu_rect(origin_x: usize, origin_y: usize, state: &TabsScreenState) -> LayoutRect {
    let Some(menu) = state.context_menu.as_ref() else {
        return LayoutRect::new(origin_x, origin_y, 0, 0);
    };
    LayoutRect::new(
        origin_x + menu.x,
        origin_y + menu.y + MENU_OFFSET_Y,
        MENU_WIDTH,
        visible_items(menu).len() * MENU_ROW_HEIGHT,
    )
}

pub(super) fn command_at(
    origin_x: usize,
    origin_y: usize,
    x: usize,
    y: usize,
    state: &TabsScreenState,
) -> Option<TabsContextMenuCommand> {
    let menu = state.context_menu.as_ref()?;
    let rect = menu_rect(origin_x, origin_y, state);
    if !rect.contains(x, y) {
        return None;
    }
    let index = (y - rect.y) / MENU_ROW_HEIGHT;
    let items = visible_items(menu);
    let item_id = items.get(index).map(|item| item.item.id.as_str())?;
    TabsContextMenuCommand::from_item_id(item_id, menu.group_id.is_some())
}

#[cfg(test)]
pub(super) fn menu_labels_for_test(state: &TabsScreenState) -> Vec<&str> {
    state.context_menu.as_ref().map_or_else(Vec::new, |menu| {
        visible_items(menu)
            .into_iter()
            .map(|item| item.item.label.as_str())
            .collect()
    })
}

struct VisibleMenuItem<'a> {
    item: &'a ContextMenuItem,
    depth: usize,
}

fn visible_items(menu: &TabsContextMenuState) -> Vec<VisibleMenuItem<'_>> {
    let mut items = Vec::new();
    for item in &menu.items {
        push_visible_item(&mut items, item, 0);
    }
    items
}

fn push_visible_item<'a>(
    items: &mut Vec<VisibleMenuItem<'a>>,
    item: &'a ContextMenuItem,
    depth: usize,
) {
    items.push(VisibleMenuItem { item, depth });
    for child in &item.children {
        push_visible_item(items, child, depth + 1);
    }
}

fn draw_row(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    rect: LayoutRect,
    index: usize,
    depth: usize,
    label: &str,
) {
    let row = LayoutRect::new(
        rect.x,
        rect.y + index * MENU_ROW_HEIGHT,
        rect.width,
        MENU_ROW_HEIGHT,
    );
    let label_x = row.x + MENU_PADDING_X + depth * MENU_PADDING_X;
    text.draw_in_box(
        canvas,
        label,
        TextBox::new(
            label_x,
            row.y,
            row.width.saturating_sub(label_x - row.x),
            row.height,
        ),
        m::FONT_7,
        palette.text,
    );
}

fn rect_to_common(rect: LayoutRect) -> Rect {
    Rect::new(rect.x, rect.y, rect.width, rect.height)
}

#[cfg(test)]
mod tests {
    use super::{TabsScreenState, command_at, menu_rect};

    #[test]
    fn absent_and_outside_context_menu_coordinates_are_total() {
        let state = TabsScreenState::default();
        assert_eq!(0, menu_rect(4, 5, &state).width);

        let mut opened = TabsScreenState::default();
        let tab_id = opened.tabs[0].id.clone();
        opened.open_context_menu_for_tab(tab_id.as_str(), 10, 20);
        assert_eq!(None, command_at(0, 0, usize::MAX, usize::MAX, &opened));
    }
}
