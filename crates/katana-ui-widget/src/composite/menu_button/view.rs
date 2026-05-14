use crate::composite::menu_button::types::{MenuButtonProps, MenuButtonVariant};
use crate::layout::popover::{AnchorRect, ViewAnchor};
use crate::overlay_lifecycle::{OverlayLifecycle, OverlayLifetime};
use crate::theme::Theme;
use floem::event::{Event, EventListener};
use floem::peniko::kurbo::Point;
use floem::reactive::{SignalGet, SignalUpdate, create_effect, create_rw_signal};
use floem::views::{Decorators, button};
use floem::{IntoView, View, ViewId};
use std::rc::Rc;

use super::overlay::build_overlay;
use super::style::{MENU_GAP, MENU_RADIUS, menu_style};
use super::trigger::build_trigger;
use std::cell::Cell;

const MENU_BUTTON_TRIGGER_PADDING_HORIZ: f32 = 10.0;
const MENU_BUTTON_TRIGGER_PADDING_VERT: f32 = 6.0;
const MENU_BUTTON_TRIGGER_GAP_ZERO: f32 = 0.0;
const MENU_BUTTON_OVERLAY_COORDINATE: f64 = 0.0;
const MENU_BUTTON_FALLBACK_SIZE: f32 = 1.0;

pub(super) fn build_view(props: MenuButtonProps, theme: Theme) -> impl IntoView {
    let MenuButtonProps {
        variant,
        trigger,
        content,
        on_open,
        on_close,
        placement,
        open,
    } = props;
    let is_open = create_rw_signal(open);
    let overlay_id = create_rw_signal::<Option<ViewId>>(None);
    let trigger_anchor = create_rw_signal(default_anchor());
    let overlay_pending = Rc::new(Cell::new(false));
    let overlay_lifetime = OverlayLifetime::new();

    let remove_overlay_if_open = {
        let overlay_lifetime = overlay_lifetime.clone();
        Rc::new(move || {
            if let Some(id) = overlay_id.try_update(|value| value.take()).flatten() {
                OverlayLifecycle::remove_overlay_next_tick(&overlay_lifetime, id);
            }
        })
    };

    let close_overlay: Rc<dyn Fn()> = {
        let on_close = Rc::clone(&on_close);
        let remove_overlay_if_open = Rc::clone(&remove_overlay_if_open);
        Rc::new(move || {
            let changed = is_open
                .try_update(|open| {
                    if *open {
                        *open = false;
                        true
                    } else {
                        false
                    }
                })
                .unwrap_or(false);
            if !changed {
                return;
            }
            remove_overlay_if_open();
            on_close();
        })
    };

    let close_overlay_for_effect = Rc::clone(&close_overlay);
    create_effect({
        let remove_overlay_if_open = Rc::clone(&remove_overlay_if_open);
        let content = Rc::clone(&content);
        let theme = theme.clone();
        let overlay_pending = Rc::clone(&overlay_pending);
        let overlay_lifetime = overlay_lifetime.clone();
        move |_| {
            if !is_open.try_get().unwrap_or(false) {
                overlay_pending.set(false);
                remove_overlay_if_open();
                return;
            }

            if overlay_id.try_get().unwrap_or(None).is_some() || overlay_pending.get() {
                return;
            }
            overlay_pending.set(true);

            let overlay_anchor = trigger_anchor.try_get().unwrap_or(default_anchor());
            let content = Rc::clone(&content);
            let close_overlay = Rc::clone(&close_overlay_for_effect);
            let overlay_theme = theme.clone();
            let overlay_lifetime_for_added = overlay_lifetime.clone();
            OverlayLifecycle::add_overlay_next_tick(
                &overlay_lifetime,
                Point::new(
                    MENU_BUTTON_OVERLAY_COORDINATE,
                    MENU_BUTTON_OVERLAY_COORDINATE,
                ),
                move |_| {
                    build_overlay(
                        content.clone(),
                        Rc::clone(&close_overlay),
                        overlay_anchor,
                        placement,
                        overlay_theme.clone(),
                    )
                },
                {
                    let overlay_pending = Rc::clone(&overlay_pending);
                    move |view_id| {
                        overlay_pending.set(false);
                        if is_open.try_get().unwrap_or(false)
                            && overlay_id.try_get().unwrap_or(None).is_none()
                        {
                            overlay_id.set(Some(view_id));
                        } else {
                            OverlayLifecycle::remove_overlay_next_tick(
                                &overlay_lifetime_for_added,
                                view_id,
                            );
                        }
                    }
                },
            );
        }
    });

    let (text_color, bg_color, border_color, border_width) = menu_style(variant, &theme);
    let close_overlay = Rc::clone(&close_overlay);
    let base_button = button(build_trigger(&trigger, &theme));
    let trigger_id = base_button.id();
    let mut trigger_button = base_button
        .on_event_stop(EventListener::PointerDown, {
            let close_overlay = Rc::clone(&close_overlay);
            let on_open = Rc::clone(&on_open);
            move |event| {
                let Event::PointerDown(pointer_event) = event else {
                    return;
                };
                if !pointer_event.button.is_primary() {
                    return;
                }

                let currently_open = is_open.try_get().unwrap_or(false);
                if currently_open {
                    close_overlay();
                    return;
                }

                trigger_anchor.set(ViewAnchor::rect_for_view(trigger_id, default_anchor()));
                if is_open.try_update(|open| *open = true).is_some() {
                    on_open();
                }
            }
        })
        .style(move |style| {
            let style = match variant {
                MenuButtonVariant::Framed => style
                    .background(bg_color)
                    .border(border_width)
                    .border_color(border_color)
                    .border_radius(MENU_RADIUS),
                MenuButtonVariant::Unframed => style
                    .background(bg_color)
                    .border(border_width)
                    .border_color(border_color),
            };
            style
                .padding_horiz(MENU_BUTTON_TRIGGER_PADDING_HORIZ)
                .padding_vert(MENU_BUTTON_TRIGGER_PADDING_VERT)
                .gap(MENU_GAP)
                .color(text_color)
        });

    if matches!(variant, MenuButtonVariant::Unframed) {
        trigger_button = trigger_button.style(|style| style.padding(MENU_BUTTON_TRIGGER_GAP_ZERO));
    }

    trigger_button.on_cleanup(move || {
        overlay_lifetime.dispose();
        remove_overlay_if_open();
    })
}

fn default_anchor() -> AnchorRect {
    AnchorRect::new(
        0.0,
        0.0,
        MENU_BUTTON_FALLBACK_SIZE,
        MENU_BUTTON_FALLBACK_SIZE,
    )
}
