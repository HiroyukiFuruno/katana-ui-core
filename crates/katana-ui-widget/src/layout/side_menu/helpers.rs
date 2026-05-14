use super::interaction::{anchor_for_button, schedule_hover_open};
use super::ops::{modal_overlay, popover_overlay};
use super::types::{
    DEFAULT_EXPANDED_PANEL_WIDTH, SIDE_MENU_CLICK_COOLDOWN_MS, SideMenuItem, SideMenuItemPlacement,
    SideMenuItemPop, SideMenuPopMode, SideMenuSide,
};
use crate::floem_view::FloemColor;
use crate::primitive::icon::{Icon, IconSize};
use crate::theme::Theme;
use crate::theme::color::Color;
use floem::action::exec_after;
use floem::event::{Event, EventListener};
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::views::{Decorators, button, container, empty};
use floem::{IntoView, View};
use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

const ICON_BUTTON_SIZE: f32 = 34.0;
const ICON_BUTTON_PADDING: f32 = 6.0;
const CORNER_RADIUS: f32 = 6.0;
const POP_ANCHOR_OFFSET: f32 = 8.0;
const RAIL_ICON_COLOR: Color = Color {
    r: 214,
    g: 219,
    b: 226,
    a: u8::MAX,
};
const RAIL_SURFACE: Color = Color {
    r: 34,
    g: 34,
    b: 34,
    a: u8::MAX,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) struct ActivePop {
    pub(crate) index: usize,
    pub(crate) mode: SideMenuPopMode,
    pub(crate) pinned: bool,
}

#[derive(Clone, Copy)]
pub(super) struct SideMenuSignals {
    pub(crate) active: RwSignal<Option<ActivePop>>,
    pub(crate) hovered: RwSignal<bool>,
    pub(crate) hover_cooldown: RwSignal<bool>,
    pub(crate) anchor: RwSignal<(f32, f32)>,
}

pub(super) fn empty_slot() -> Box<dyn View> {
    container(empty()).into_any()
}

pub(super) fn overlay_for(
    pop: &SideMenuItemPop,
    mode: SideMenuPopMode,
    anchor: (f32, f32),
    close: Rc<dyn Fn()>,
    theme: Theme,
) -> Box<dyn View> {
    let content = (pop.content)();
    let anchor = (anchor.0 + POP_ANCHOR_OFFSET, anchor.1 + POP_ANCHOR_OFFSET);
    match mode {
        SideMenuPopMode::Modal => modal_overlay(content, close, theme),
        SideMenuPopMode::Popover => popover_overlay(content, anchor, close, theme),
        SideMenuPopMode::Expand => empty_slot(),
    }
}

pub(super) fn rail_surface() -> floem::peniko::Color {
    FloemColor::from_token(RAIL_SURFACE)
}

pub(super) fn build_buttons(
    items: &[SideMenuItem],
    placement: SideMenuItemPlacement,
    side: SideMenuSide,
    theme: &Theme,
    signals: SideMenuSignals,
    clear: Rc<dyn Fn()>,
) -> Vec<Box<dyn View>> {
    let theme = theme.clone();
    let accent = FloemColor::from_token(theme.color.accent_muted);
    let surface = rail_surface();
    let icon_color = RAIL_ICON_COLOR;

    items
        .iter()
        .enumerate()
        .filter(|(_, item)| item.placement == placement)
        .map(|(index, item)| {
            let selected = item.selected;
            let hover_token = Rc::new(Cell::new(0_u64));
            let icon = Icon::new(item.icon.clone())
                .size(IconSize::Lg)
                .color_override(icon_color)
                .view(theme.clone())
                .into_any();
            let pop_mode = item.pop.as_ref().map(|entry| entry.mode);
            let clear = Rc::clone(&clear);
            let on_activate = Rc::clone(&item.on_activate);
            let button_view = button(icon)
                .style(move |style| {
                    let is_active = selected
                        || matches!(
                            signals.active.try_get().flatten(),
                            Some(ActivePop {
                                index: active_index,
                                ..
                            }) if active_index == index
                        );
                    style
                        .width(ICON_BUTTON_SIZE)
                        .height(ICON_BUTTON_SIZE)
                        .padding(ICON_BUTTON_PADDING)
                        .background(if is_active { accent } else { surface })
                        .border_radius(CORNER_RADIUS)
                })
                .into_view();
            let button_id = button_view.id();
            button_view
                .on_event_cont(EventListener::PointerEnter, {
                    let hover_token = Rc::clone(&hover_token);
                    move |_| {
                        schedule_hover_open(
                            index,
                            pop_mode,
                            side,
                            button_id,
                            signals,
                            &hover_token,
                        );
                    }
                })
                .on_event_cont(EventListener::PointerMove, {
                    let hover_token = Rc::clone(&hover_token);
                    move |_| {
                        schedule_hover_open(
                            index,
                            pop_mode,
                            side,
                            button_id,
                            signals,
                            &hover_token,
                        );
                    }
                })
                .on_event_cont(EventListener::PointerLeave, {
                    let hover_token = Rc::clone(&hover_token);
                    move |_| {
                        hover_token.set(hover_token.get().wrapping_add(1));
                    }
                })
                .on_event_stop(EventListener::PointerDown, move |event| {
                    let Event::PointerDown(pointer_event) = event else {
                        return;
                    };
                    if !pointer_event.button.is_primary() {
                        return;
                    }
                    on_activate();
                    signals.hovered.set(true);
                    signals.hover_cooldown.set(true);
                    let cooldown = signals.hover_cooldown;
                    exec_after(
                        Duration::from_millis(SIDE_MENU_CLICK_COOLDOWN_MS),
                        move |_| {
                            cooldown.set(false);
                        },
                    );
                    signals.anchor.set(anchor_for_button(
                        side,
                        button_id,
                        DEFAULT_EXPANDED_PANEL_WIDTH,
                    ));
                    if let Some(mode) = pop_mode {
                        let next = ActivePop {
                            index,
                            mode,
                            pinned: true,
                        };
                        if signals.active.try_get().flatten() == Some(next) {
                            signals.active.set(None);
                        } else {
                            signals.active.set(Some(next));
                        }
                    } else {
                        clear();
                    }
                })
                .into_any()
        })
        .collect()
}
