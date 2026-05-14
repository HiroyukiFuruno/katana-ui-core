use super::types::{CommandPaletteItem, OnExecute, OnSelection};
use crate::primitive::icon::{Icon, IconSource};
use crate::theme::Theme;
use floem::IntoView;
use floem::View;
use floem::event::EventListener;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate, SignalWith};
use floem::views::{
    Decorators, button, container, h_stack, label, scroll, v_stack, v_stack_from_iter,
};
use std::rc::Rc;

const ITEM_GAP: f32 = 4.0;
const ITEM_PAD_V: f32 = 8.0;
const ITEM_PAD_H: f32 = 10.0;
const ITEM_RADIUS: f32 = 8.0;
const ITEM_MIN_HEIGHT: f32 = 34.0;
const ITEM_ICON_SIZE: f32 = 16.0;
const ITEM_FONT_SIZE: f32 = 14.0;
const ITEM_SCORE_FONT_SIZE: f32 = 11.0;
const ITEM_SCORE_PREFIX: &str = "score: ";
const ITEM_SHORTCUT_FONT_SIZE: f32 = 12.0;
const ITEM_STACK_GAP: f32 = 3.0;
const ITEM_ROW_GAP: f32 = 8.0;
const ITEM_PADDING_EMPTY: f32 = 0.0;
const ITEM_NO_RESULTS_PADDING_VERT: f32 = 10.0;
const ITEM_NO_RESULTS_PADDING_HORIZ: f32 = 12.0;
const ITEM_LIST_MAX_HEIGHT: f32 = 240.0;
const ITEM_BORDER_WIDTH: f32 = 1.0;

fn icon_node(icon: Option<IconSource>, theme: &Theme) -> Box<dyn View> {
    match icon {
        Some(source) => Icon::new(source).view(theme.clone()).into_any(),
        None => container(label(|| ""))
            .style(|style| style.width(ITEM_ICON_SIZE).height(ITEM_ICON_SIZE))
            .into_any(),
    }
}

fn shortcut_node(shortcut: Option<String>, theme: &Theme) -> Box<dyn View> {
    let muted = theme.color.text_muted;
    match shortcut {
        Some(value) if !value.is_empty() => label(move || value.clone())
            .style(move |style| {
                style
                    .font_size(ITEM_SHORTCUT_FONT_SIZE)
                    .color(crate::floem_view::FloemColor::from_token(muted))
            })
            .into_any(),
        _ => container(label(|| ""))
            .style(|style| style.width(ITEM_PADDING_EMPTY))
            .into_any(),
    }
}

pub(super) fn execute_selected<P: Clone + 'static>(
    index: usize,
    query_signal: &RwSignal<String>,
    results_signal: &RwSignal<Vec<CommandPaletteItem<P>>>,
    selected_signal: &RwSignal<usize>,
    on_selection_change: &OnSelection,
    on_execute: &OnExecute<P>,
) {
    let query = query_signal.get();
    let selected = results_signal.with(|rows| rows.get(index).cloned());
    if let Some(item) = selected {
        selected_signal.set(index);
        on_selection_change(query.clone(), index);
        on_execute(query, index, item.payload);
    }
}

pub(super) fn rows_view<P: Clone + 'static>(
    theme: Theme,
    items: Vec<CommandPaletteItem<P>>,
    selected_index: usize,
    disabled: bool,
    on_execute: Rc<dyn Fn(usize)>,
) -> Box<dyn View> {
    if items.is_empty() {
        let muted = theme.color.text_muted;
        return container(label(|| "候補がありません"))
            .style(move |style| {
                style
                    .color(crate::floem_view::FloemColor::from_token(muted))
                    .padding_vert(ITEM_NO_RESULTS_PADDING_VERT)
                    .padding_horiz(ITEM_NO_RESULTS_PADDING_HORIZ)
            })
            .into_any();
    }

    scroll(
        v_stack_from_iter(items.into_iter().enumerate().map(|(index, item)| {
            let is_selected = selected_index == index;
            let execute_row = Rc::clone(&on_execute);
            let fg = if is_selected {
                theme.color.accent
            } else {
                theme.color.text
            };
            let bg = if is_selected {
                theme.color.accent_muted
            } else {
                theme.color.bg
            };
            let row_label = item.label.clone();
            let row_score = item.score;
            let row_shortcut = item.shortcut.clone();
            let row_icon = item.icon;
            let row = h_stack((
                icon_node(row_icon, &theme),
                v_stack((
                    label(move || row_label.clone())
                        .style(move |style| {
                            style
                                .font_size(ITEM_FONT_SIZE)
                                .color(crate::floem_view::FloemColor::from_token(fg))
                        })
                        .into_any(),
                    label(move || format!("{ITEM_SCORE_PREFIX}{row_score}"))
                        .style(move |style| {
                            style.font_size(ITEM_SCORE_FONT_SIZE).color(
                                crate::floem_view::FloemColor::from_token(theme.color.text_muted),
                            )
                        })
                        .into_any(),
                ))
                .style(|style| style.gap(ITEM_STACK_GAP)),
                shortcut_node(row_shortcut, &theme),
            ))
            .style(|style| {
                style
                    .width_full()
                    .items_center()
                    .gap(ITEM_ROW_GAP)
                    .justify_between()
                    .min_height(ITEM_MIN_HEIGHT)
            });

            button(row)
                .on_event_stop(EventListener::PointerDown, |_| {})
                .action(move || {
                    if !disabled {
                        execute_row(index);
                    }
                })
                .style(move |style| {
                    style
                        .width_full()
                        .padding_vert(ITEM_PAD_V)
                        .padding_horiz(ITEM_PAD_H)
                        .gap(ITEM_GAP)
                        .border(ITEM_BORDER_WIDTH)
                        .border_color(crate::floem_view::FloemColor::from_token(
                            theme.color.border,
                        ))
                        .border_radius(ITEM_RADIUS)
                        .background(crate::floem_view::FloemColor::from_token(bg))
                })
                .into_any()
        }))
        .style(|style| style.gap(ITEM_GAP)),
    )
    .style(|style| style.width_full().height(ITEM_LIST_MAX_HEIGHT))
    .into_any()
}
