use super::SelectionListItem;
use super::ops::{SelectionListItemPath, SelectionListOps};
use crate::theme::Theme;
use floem::IntoView;
use floem::View;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::style::Display;
use floem::views::{Decorators, button, container, empty, h_stack, label, v_stack};
use std::rc::Rc;

const CORNER_RADIUS: f32 = 6.0;
const ITEM_TITLE_SIZE: f32 = 14.0;
const MARKER_GAP: f32 = 8.0;
const MARKER_RADIUS_DIVISOR: f32 = 2.0;
const MARKER_SIZE: f32 = 8.0;
const ROW_BORDER_WIDTH: f32 = 1.0;
const ROW_GAP_SMALL: f32 = 4.0;
const ROW_PADDING_H: f32 = 10.0;
const ROW_PADDING_V: f32 = 10.0;

fn marker_node(color: crate::theme::color::Color) -> impl floem::IntoView {
    container(empty())
        .style(move |style| {
            let marker = crate::floem_view::FloemColor::from_token(color);
            style
                .width(MARKER_SIZE)
                .height(MARKER_SIZE)
                .border_radius(MARKER_SIZE / MARKER_RADIUS_DIVISOR)
                .background(marker)
        })
        .into_any()
}

fn row_display(hidden: bool, hidden_revealed: RwSignal<bool>) -> Display {
    if SelectionListOps::item_visible(hidden, hidden_revealed.try_get().unwrap_or(false)) {
        Display::Flex
    } else {
        Display::None
    }
}

pub(super) fn item_row(
    item: SelectionListItem,
    theme: Theme,
    path: SelectionListItemPath,
    selected_path: RwSignal<Option<SelectionListItemPath>>,
    hidden_revealed: RwSignal<bool>,
    initial_selected_path: Option<SelectionListItemPath>,
) -> Box<dyn View> {
    let on_select: Rc<dyn Fn()> = Rc::clone(&item.on_select);
    let disabled = item.disabled;
    let hidden = item.hidden;
    let row_label = item.label.clone();
    let text_disabled = theme.color.text_disabled;
    let text_selected = theme.color.accent;
    let text_default = theme.color.text;
    let selected_bg = theme.color.accent_muted;
    let default_bg = theme.color.bg;
    let border_color = theme.color.border;

    let body = if let Some(extra) = item.content {
        v_stack((
            label(move || row_label.clone()).style(move |style| style.font_size(ITEM_TITLE_SIZE)),
            extra,
        ))
        .style(move |style| style.gap(ROW_GAP_SMALL))
        .into_any()
    } else {
        label(move || row_label.clone())
            .style(move |style| style.font_size(ITEM_TITLE_SIZE))
            .into_any()
    };

    let row = h_stack((marker_node(item.marker_color), body)).style(move |style| {
        let is_selected = selected_path.try_get().unwrap_or(initial_selected_path) == Some(path);
        let text_color = if disabled {
            text_disabled
        } else if is_selected {
            text_selected
        } else {
            text_default
        };
        let row_bg = if is_selected { selected_bg } else { default_bg };

        style
            .gap(MARKER_GAP)
            .display(row_display(hidden, hidden_revealed))
            .items_center()
            .padding_vert(ROW_PADDING_V)
            .padding_horiz(ROW_PADDING_H)
            .width_full()
            .border_radius(CORNER_RADIUS)
            .background(crate::floem_view::FloemColor::from_token(row_bg))
            .border(ROW_BORDER_WIDTH)
            .border_color(crate::floem_view::FloemColor::from_token(border_color))
            .color(crate::floem_view::FloemColor::from_token(text_color))
    });

    button(row)
        .action(move || {
            if !disabled {
                selected_path.set(Some(path));
                on_select();
            }
        })
        .into_any()
}
