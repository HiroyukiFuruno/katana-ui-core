use super::Popover;
use crate::theme::Theme;
use crate::theme::color::Color;
use floem::action::{add_overlay, remove_overlay};
use floem::event::{Event, EventListener};
use floem::keyboard::{Key, NamedKey};
use floem::peniko::kurbo::Point;
use floem::reactive::{SignalGet, SignalUpdate, create_effect, create_rw_signal};
use floem::views::{Decorators, button, container, label, v_stack};
use floem::{IntoView, View, ViewId};
use std::rc::Rc;

const CORNER_RADIUS: f32 = 6.0;
const DEFAULT_OFFSET: f32 = 4.0;
const SHADOW_ALPHA: u8 = 40;
const POPOVER_PADDING: f32 = crate::floem_view::GAP_SM;
const POPOVER_GAP: f32 = crate::floem_view::GAP_XS;
const POPOVER_ESTIMATED_WIDTH: f32 = 240.0;
const POPOVER_ESTIMATED_HEIGHT: f32 = 96.0;
const POPOVER_ESTIMATED_VIEWPORT: f32 = 4096.0;

#[derive(Debug, Clone, Copy)]
pub(super) struct PopoverViewStyle {
    pub popover_bg: Color,
    pub popover_border: Color,
    pub shadow_color: Color,
    pub corner_radius: f32,
}

/// Resolved layout for popover overlay placement.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PopoverOverlay {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub placement: super::Placement,
    pub popover_bg: Color,
    pub popover_border: Color,
    pub shadow_color: Color,
    pub corner_radius: f32,
}

pub(super) fn popover_bg(theme: &Theme) -> Color {
    theme.color.surface
}

pub(super) fn popover_border(theme: &Theme) -> Color {
    theme.color.border
}

pub(super) fn shadow_color(theme: &Theme) -> Color {
    Color {
        r: theme.color.bg.r,
        g: theme.color.bg.g,
        b: theme.color.bg.b,
        a: SHADOW_ALPHA,
    }
}

pub(super) fn corner_radius() -> f32 {
    CORNER_RADIUS
}

pub(super) fn default_offset() -> f32 {
    DEFAULT_OFFSET
}

pub(super) fn style(theme: &Theme) -> PopoverViewStyle {
    PopoverViewStyle {
        popover_bg: popover_bg(theme),
        popover_border: popover_border(theme),
        shadow_color: shadow_color(theme),
        corner_radius: corner_radius(),
    }
}

impl Popover {
    #[must_use]
    pub fn view(self, theme: Theme, anchor_label: impl Into<String>) -> impl IntoView {
        let resolved = self.resolve(&theme);
        let open = create_rw_signal(resolved.open);
        let overlay_id = create_rw_signal::<Option<ViewId>>(None);
        let anchor_label = anchor_label.into();
        let children_text = resolved.children.clone().unwrap_or_default();
        let dismiss_on_outside_click = resolved.dismiss_on_outside_click;
        let dismiss_on_esc = resolved.dismiss_on_esc;
        let anchor = resolved.anchor;

        let close_overlay = {
            let on_close = Rc::clone(&resolved.on_close);
            Rc::new(move || {
                let changed = open
                    .try_update(|is_open| {
                        if *is_open {
                            *is_open = false;
                            true
                        } else {
                            false
                        }
                    })
                    .unwrap_or(false);

                if changed {
                    if let Some(id) = overlay_id
                        .try_update(|overlay_id| overlay_id.take())
                        .flatten()
                    {
                        remove_overlay(id);
                    }
                    (on_close)();
                }
            })
        };

        let remove_overlay_if_open = {
            move || {
                if let Some(id) = overlay_id
                    .try_update(|overlay_id| overlay_id.take())
                    .flatten()
                {
                    remove_overlay(id);
                }
            }
        };

        create_effect({
            let close_overlay = Rc::clone(&close_overlay);
            move |_| {
                if !open.try_get().unwrap_or(false) {
                    if let Some(id) = overlay_id
                        .try_update(|overlay_id| overlay_id.take())
                        .flatten()
                    {
                        remove_overlay(id);
                    }
                    return;
                }

                if overlay_id.try_get().unwrap_or(None).is_some() || anchor.is_none() {
                    return;
                }

                let Some(layout) = resolved.overlay_layout(
                    POPOVER_ESTIMATED_WIDTH,
                    POPOVER_ESTIMATED_HEIGHT,
                    POPOVER_ESTIMATED_VIEWPORT,
                    POPOVER_ESTIMATED_VIEWPORT,
                ) else {
                    return;
                };

                let popover_bg = crate::floem_view::FloemColor::from_token(layout.popover_bg);
                let popover_border =
                    crate::floem_view::FloemColor::from_token(layout.popover_border);
                let close_overlay_for_events = Rc::clone(&close_overlay);
                let close_overlay_for_events_esc = Rc::clone(&close_overlay);
                let children_text = children_text.clone();

                let overlay_view_id = add_overlay(Point::new(0., 0.), move |_| {
                    let panel = container(
                        v_stack((label(move || children_text.clone()),))
                            .style(move |style| {
                                style
                                    .background(popover_bg)
                                    .border(1.0)
                                    .border_color(popover_border)
                                    .border_radius(layout.corner_radius)
                                    .padding(POPOVER_PADDING)
                                    .gap(POPOVER_GAP)
                                    .absolute()
                                    .inset_left(layout.x)
                                    .inset_top(layout.y)
                            })
                            .on_event_stop(EventListener::PointerDown, |_| {}),
                    );
                    let panel_id = panel.id();
                    panel_id.request_focus();

                    container(panel)
                        .style(|style| style.width_full().height_full())
                        .on_event_stop(EventListener::PointerDown, move |_| {
                            if dismiss_on_outside_click {
                                (close_overlay_for_events)();
                            }
                        })
                        .keyboard_navigable()
                        .on_event_stop(EventListener::KeyDown, move |event| {
                            if let Event::KeyDown(key_event) = event
                                && dismiss_on_esc
                                && key_event.key.logical_key == Key::Named(NamedKey::Escape)
                            {
                                (close_overlay_for_events_esc)();
                            }
                        })
                        .into_any()
                });

                overlay_id.set(Some(overlay_view_id));
            }
        });

        v_stack((button(label(move || anchor_label.clone())).action({
            let close_overlay = Rc::clone(&close_overlay);
            move || {
                let currently_open = open.try_get().unwrap_or(false);
                if currently_open {
                    close_overlay();
                } else {
                    let _ = open.try_update(|value| *value = true);
                }
            }
        }),))
        .on_cleanup(move || {
            remove_overlay_if_open();
        })
    }
}
