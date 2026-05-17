use crate::composite::tabs::types::TabItem;
use crate::floem_view::FloemColor;
use crate::primitive::icon::{Icon, IconSize};
use crate::theme::Theme;
use floem::IntoView;
use floem::View;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::style::Display;
use floem::views::{
    Decorators, button, dyn_container, empty, h_stack, h_stack_from_iter, label, scroll,
};
use std::rc::Rc;

const TAB_FONT_SIZE: f32 = 13.0;
const TAB_ICON_SIZE: f32 = 12.0;
const TAB_GAP: f32 = 8.0;
const TAB_PADDING_H: f32 = 12.0;
const TAB_PADDING_V: f32 = 6.0;
const TAB_RADIUS: f32 = 10.0;
const TAB_DOT_SIZE: f32 = 10.0;
const TAB_CLOSE_BUTTON_PADDING: f32 = 2.0;
const TAB_CLOSE_BUTTON_SIZE: f32 = 20.0;
const TAB_TITLE_ICON_GAP: f32 = 6.0;
const TAB_CLOSE_BUTTON_GAP: f32 = 2.0;
const TAB_OVERFLOW_HEIGHT: f32 = 44.0;
const TAB_BORDER_WIDTH: f32 = 1.0;

type ContentBuilder = Rc<dyn Fn() -> Box<dyn View>>;
type ContentBuilderList = Rc<Vec<Option<ContentBuilder>>>;
type TabCloseContext = (
    usize,
    RwSignal<usize>,
    RwSignal<bool>,
    Rc<Vec<RwSignal<bool>>>,
    Rc<Vec<bool>>,
);

fn colors(
    selected: bool,
    disabled: bool,
    theme: &Theme,
) -> (
    crate::theme::color::Color,
    crate::theme::color::Color,
    crate::theme::color::Color,
) {
    if selected {
        if disabled {
            (
                theme.color.border,
                theme.color.border,
                theme.color.text_disabled,
            )
        } else {
            (theme.color.accent, theme.color.surface, theme.color.bg)
        }
    } else if disabled {
        (
            theme.color.bg,
            theme.color.border,
            theme.color.text_disabled,
        )
    } else {
        (theme.color.surface, theme.color.border, theme.color.text)
    }
}

fn close_button(on_close: Rc<dyn Fn()>, disabled: bool, theme: &Theme) -> Box<dyn View> {
    let fg = FloemColor::from_token(if disabled {
        theme.color.text_disabled
    } else {
        theme.color.text_muted
    });

    button(label(|| "x").style(move |style| style.font_size(TAB_DOT_SIZE).color(fg)))
        .action(move || {
            if !disabled {
                on_close();
            }
        })
        .style(|style| {
            style
                .padding(TAB_CLOSE_BUTTON_PADDING)
                .width(TAB_CLOSE_BUTTON_SIZE)
                .height(TAB_CLOSE_BUTTON_SIZE)
                .border(0.0)
                .items_center()
                .justify_center()
        })
        .into_any()
}

fn title_with_icon(
    label_text: String,
    icon: Option<crate::primitive::icon::IconSource>,
    disabled: bool,
    theme: &Theme,
) -> Box<dyn View> {
    let label_color = FloemColor::from_token(if disabled {
        theme.color.text_disabled
    } else {
        theme.color.text
    });

    let mut nodes: Vec<Box<dyn View>> = Vec::new();

    if let Some(src) = icon {
        nodes.push(
            Icon::new(src)
                .size(IconSize::Pt(TAB_ICON_SIZE))
                .color_override(if disabled {
                    theme.color.text_disabled
                } else {
                    theme.color.text
                })
                .view(theme.clone())
                .into_any(),
        );
    }

    nodes.push(
        label(move || label_text.clone())
            .style(move |style| style.font_size(TAB_FONT_SIZE).color(label_color))
            .into_any(),
    );

    h_stack_from_iter(nodes)
        .style(|style| style.gap(TAB_TITLE_ICON_GAP).items_center())
        .into_any()
}

fn first_open_enabled(closed: &[RwSignal<bool>], disabled: &[bool]) -> Option<usize> {
    closed
        .iter()
        .enumerate()
        .find(|(index, signal)| !signal.get() && !disabled[*index])
        .map(|(index, _)| index)
}

fn tab_button(
    item: &TabItem,
    index: usize,
    selected_index: RwSignal<usize>,
    closed: RwSignal<bool>,
    theme: &Theme,
) -> Box<dyn View> {
    let disabled = item.disabled;
    let on_select = Rc::clone(&item.on_select);
    let style_theme = theme.clone();

    button(title_with_icon(
        item.label.clone(),
        item.icon.clone(),
        disabled,
        theme,
    ))
    .action(move || {
        if !disabled && !closed.get() {
            selected_index.set(index);
            on_select();
        }
    })
    .style(move |style| {
        let selected = selected_index.get() == index;
        let (bg, border, fg_color) = colors(selected, disabled, &style_theme);
        let display = if closed.get() {
            Display::None
        } else {
            Display::Flex
        };
        style
            .display(display)
            .background(FloemColor::from_token(bg))
            .border(TAB_BORDER_WIDTH)
            .border_color(FloemColor::from_token(border))
            .color(FloemColor::from_token(fg_color))
            .border_radius(TAB_RADIUS)
            .padding_horiz(TAB_PADDING_H)
            .padding_vert(TAB_PADDING_V)
    })
    .into_any()
}

fn closeable_tab(
    tab: Box<dyn View>,
    item: &TabItem,
    context: TabCloseContext,
    theme: &Theme,
) -> Box<dyn View> {
    let (index, selected_index, closed, closed_states, disabled_flags) = context;
    let Some(on_close) = item.on_close.as_ref().cloned() else {
        return tab;
    };

    let close = close_button(
        Rc::new(move || {
            closed.set(true);
            on_close();
            if selected_index.get() == index
                && let Some(next) = first_open_enabled(&closed_states, &disabled_flags)
            {
                selected_index.set(next);
            }
        }),
        item.disabled,
        theme,
    );

    h_stack((tab, close))
        .style(move |style| {
            let display = if closed.get() {
                Display::None
            } else {
                Display::Flex
            };
            style
                .display(display)
                .gap(TAB_CLOSE_BUTTON_GAP)
                .items_center()
        })
        .into_any()
}

pub(super) fn tab_node(
    item: &TabItem,
    index: usize,
    selected_index: RwSignal<usize>,
    closed: RwSignal<bool>,
    closed_states: Rc<Vec<RwSignal<bool>>>,
    disabled_flags: Rc<Vec<bool>>,
    theme: &Theme,
) -> Box<dyn View> {
    let tab = tab_button(item, index, selected_index, closed, theme);
    closeable_tab(
        tab,
        item,
        (index, selected_index, closed, closed_states, disabled_flags),
        theme,
    )
}

pub(super) fn tabs_container(
    items: Vec<Box<dyn View>>,
    overflow: bool,
    theme: Theme,
) -> Box<dyn View> {
    let bar = h_stack_from_iter(items).style(|style| style.gap(TAB_GAP).items_center());
    let bg = theme.color.bg;

    if overflow {
        scroll(bar)
            .style(move |style| {
                style
                    .width_full()
                    .height(TAB_OVERFLOW_HEIGHT)
                    .background(FloemColor::from_token(bg))
            })
            .into_any()
    } else {
        bar.into_any()
    }
}

pub(super) fn selected_content(
    selected_index: RwSignal<usize>,
    closed_states: Rc<Vec<RwSignal<bool>>>,
    content_builders: ContentBuilderList,
) -> Box<dyn View> {
    dyn_container(
        move || {
            (
                selected_index.get(),
                closed_states
                    .iter()
                    .map(|closed| closed.get())
                    .collect::<Vec<_>>(),
            )
        },
        move |(index, closed)| {
            if closed.get(index).copied().unwrap_or(true) {
                return empty().into_any();
            }

            content_builders
                .get(index)
                .and_then(|content| content.as_ref())
                .map(|content| content())
                .unwrap_or_else(|| empty().into_any())
        },
    )
    .into_any()
}
