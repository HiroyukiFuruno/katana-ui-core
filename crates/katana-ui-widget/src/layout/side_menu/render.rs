use super::expand_panel::expand_panel;
use super::helpers::{ActivePop, SideMenuSignals, build_buttons, rail_surface};
use super::overlay_effect::{SideMenuOverlayEffectArgs, bind_overlay_effect};
use super::types::{
    DEFAULT_EXPANDED_PANEL_WIDTH, DEFAULT_HOVER_HANDLE_WIDTH, SideMenuExpandMode,
    SideMenuItemPlacement, SideMenuPopMode, SideMenuProps, SideMenuSide,
};
use crate::overlay_lifecycle::{OverlayLifecycle, OverlayLifetime};
use crate::theme::Theme;
use floem::event::EventListener;
use floem::reactive::{SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{Decorators, container, h_stack, v_stack, v_stack_from_iter};
use floem::{IntoView, ViewId};
use std::rc::Rc;

const ICON_PANEL_PADDING: f32 = 6.0;

pub(super) fn render(props: SideMenuProps, theme: Theme) -> impl IntoView {
    let SideMenuProps {
        side,
        width,
        expand_mode,
        items,
        initial_pop,
    } = props;
    let collapsed_width = match expand_mode {
        SideMenuExpandMode::Hover { collapsed_width } => collapsed_width,
        SideMenuExpandMode::Fixed => width,
    };
    let is_hover_mode = matches!(expand_mode, SideMenuExpandMode::Hover { .. });
    let items = Rc::new(items);
    let hovered = create_rw_signal(false);
    let hover_cooldown = create_rw_signal(false);
    let active = create_rw_signal(initial_pop.map(|(index, mode)| ActivePop {
        index,
        mode,
        pinned: true,
    }));
    let anchor = create_rw_signal((0.0_f32, 0.0_f32));
    let overlay_id = create_rw_signal(None::<ViewId>);
    let overlay_lifetime = OverlayLifetime::new();

    let close_overlay: Rc<dyn Fn()> = {
        let overlay_lifetime = overlay_lifetime.clone();
        Rc::new(move || {
            if let Some(id) = overlay_id.try_update(|value| value.take()).flatten() {
                OverlayLifecycle::remove_overlay_next_tick(&overlay_lifetime, id);
            }
        })
    };
    let clear: Rc<dyn Fn()> = {
        let close_overlay = Rc::clone(&close_overlay);
        Rc::new(move || {
            active.set(None);
            close_overlay();
        })
    };
    let clear_for_popup: Rc<dyn Fn()> = Rc::clone(&clear);

    bind_overlay_effect(SideMenuOverlayEffectArgs {
        active,
        anchor,
        items: Rc::clone(&items),
        overlay_id,
        theme: theme.clone(),
        close_overlay: Rc::clone(&close_overlay),
        clear: clear_for_popup,
        overlay_lifetime: overlay_lifetime.clone(),
    });

    let is_expand_open = move || {
        matches!(
            active.get(),
            Some(ActivePop {
                index: _,
                mode: SideMenuPopMode::Expand,
                ..
            })
        )
    };
    let menu_open = move || !is_hover_mode || hovered.get() || is_expand_open();
    let icon_width = move || {
        if is_hover_mode && !menu_open() {
            collapsed_width
        } else {
            width
        }
    };
    let root_width = move || {
        let base = if is_hover_mode && !menu_open() {
            collapsed_width.max(DEFAULT_HOVER_HANDLE_WIDTH)
        } else {
            width
        };
        if is_expand_open() {
            base + DEFAULT_EXPANDED_PANEL_WIDTH
        } else {
            base
        }
    };
    let expand_width = move || {
        if is_expand_open() {
            DEFAULT_EXPANDED_PANEL_WIDTH
        } else {
            0.0
        }
    };

    let top_buttons = build_buttons(
        &items,
        SideMenuItemPlacement::Top,
        side,
        &theme,
        SideMenuSignals {
            active,
            hovered,
            hover_cooldown,
            anchor,
        },
        Rc::clone(&clear),
    );
    let bottom_buttons = build_buttons(
        &items,
        SideMenuItemPlacement::Bottom,
        side,
        &theme,
        SideMenuSignals {
            active,
            hovered,
            hover_cooldown,
            anchor,
        },
        Rc::clone(&clear),
    );
    let icon_panel = container(
        v_stack((
            v_stack_from_iter(top_buttons),
            v_stack_from_iter(bottom_buttons),
        ))
        .style(move |style| {
            style
                .width(icon_width())
                .min_width(icon_width())
                .height_full()
                .padding(ICON_PANEL_PADDING)
                .background(rail_surface())
                .justify_between()
        }),
    );

    let expand_panel = expand_panel(Rc::clone(&items), active, theme.clone(), expand_width);

    let body = match side {
        SideMenuSide::Left => h_stack((icon_panel, expand_panel)),
        SideMenuSide::Right => h_stack((expand_panel, icon_panel)),
    };

    container(body)
        .style(move |style| style.width(root_width()).height_full())
        .on_event_stop(EventListener::PointerEnter, {
            move |_| {
                if is_hover_mode {
                    hovered.set(true);
                }
            }
        })
        .on_event_stop(EventListener::PointerLeave, {
            let clear = Rc::clone(&clear);
            move |_| {
                hovered.set(false);
                if !matches!(
                    active.get(),
                    Some(ActivePop {
                        index: _,
                        pinned: true,
                        ..
                    })
                ) {
                    clear();
                }
            }
        })
        .on_cleanup({
            let close_overlay = Rc::clone(&close_overlay);
            move || {
                overlay_lifetime.dispose();
                close_overlay();
            }
        })
}
