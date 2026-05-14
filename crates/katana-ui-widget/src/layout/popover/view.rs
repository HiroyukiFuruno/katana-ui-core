use super::types::{PopoverChildren, ResolvedPopover};
use super::{AnchorRect, PlacementResolver, PopoverOverlay, ViewAnchor};
use crate::floem_view::FloemColor;
use crate::overlay_lifecycle::{OverlayLifecycle, OverlayLifetime};
use crate::theme::Theme;
use crate::theme::color::Color;
use floem::event::{Event, EventListener};
use floem::keyboard::{Key, NamedKey};
use floem::peniko::kurbo::Point;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate, create_effect, create_rw_signal};
use floem::views::{Decorators, button, container, label};
use floem::{IntoView, View, ViewId};
use std::rc::Rc;

const CORNER_RADIUS: f32 = 6.0;
const DEFAULT_OFFSET: f32 = 4.0;
const SHADOW_ALPHA: u8 = 40;
const POPOVER_PADDING: f32 = crate::floem_view::GAP_SM;
const POPOVER_GAP: f32 = crate::floem_view::GAP_XS;
const ESTIMATED_PANEL_HEIGHT: f32 = 96.0;
const DEFAULT_ANCHOR_WIDTH: f32 = 120.0;
const DEFAULT_ANCHOR_HEIGHT: f32 = 32.0;
const POPOVER_VIEWPORT_WIDTH: f32 = 1024.0;
const POPOVER_VIEWPORT_HEIGHT: f32 = 768.0;
const POPOVER_OVERLAY_CATCH_PLANE_SIZE: f32 = 10000.0;
const POPOVER_OVERLAY_ROOT_Z_INDEX: i32 = 1000;
const POPOVER_OVERLAY_PANEL_Z_INDEX: i32 = 1001;

#[derive(Debug, Clone, Copy)]
pub(super) struct PopoverViewStyle {
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

fn corner_radius() -> f32 {
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

fn render_popover_content(children: &Option<PopoverChildren>) -> Box<dyn View> {
    children
        .as_ref()
        .map(|child| child())
        .unwrap_or_else(|| container(label(|| "")).into_any())
}

fn close_if_open(open: RwSignal<bool>, on_close: &Rc<dyn Fn()>, on_focus_out: &Rc<dyn Fn()>) {
    let closed = open
        .try_update(|is_open| {
            if !*is_open {
                return false;
            }

            *is_open = false;
            true
        })
        .unwrap_or(false);

    if closed {
        on_focus_out();
        on_close();
    }
}

fn open_if_closed(open: RwSignal<bool>, on_focus_in: &Rc<dyn Fn()>) {
    let opened = open
        .try_update(|is_open| {
            if *is_open {
                return false;
            }

            *is_open = true;
            true
        })
        .unwrap_or(false);

    if opened {
        on_focus_in();
    }
}

pub(super) fn render(
    resolved: ResolvedPopover,
    _theme: Theme,
    anchor_label: impl Into<String>,
) -> impl IntoView {
    let open = create_rw_signal(resolved.open);
    let overlay_id = create_rw_signal::<Option<ViewId>>(None);
    let trigger_anchor = create_rw_signal(default_anchor());
    let overlay_lifetime = OverlayLifetime::new();
    let anchor_label = anchor_label.into();
    let on_close = Rc::clone(&resolved.on_close);
    let on_focus_in = Rc::clone(&resolved.on_focus_in);
    let on_focus_out = Rc::clone(&resolved.on_focus_out);

    let remove_overlay_if_open: Rc<dyn Fn()> = {
        let overlay_lifetime = overlay_lifetime.clone();
        Rc::new(move || {
            if let Some(id) = overlay_id.try_update(|id| id.take()).flatten() {
                OverlayLifecycle::remove_overlay_next_tick(&overlay_lifetime, id);
            }
        })
    };

    bind_overlay_effect(
        open,
        overlay_id,
        trigger_anchor,
        resolved.clone(),
        Rc::clone(&remove_overlay_if_open),
        overlay_lifetime.clone(),
    );

    let trigger = button(label(move || anchor_label.clone()));
    let trigger_id = trigger.id();
    trigger
        .action({
            let on_close = Rc::clone(&on_close);
            let on_focus_in = Rc::clone(&on_focus_in);
            let on_focus_out = Rc::clone(&on_focus_out);
            move || {
                if open.try_get().unwrap_or(false) {
                    close_if_open(open, &on_close, &on_focus_out);
                } else {
                    trigger_anchor.set(ViewAnchor::rect_for_view(trigger_id, default_anchor()));
                    open_if_closed(open, &on_focus_in);
                }
            }
        })
        .keyboard_navigable()
        .on_event_stop(EventListener::KeyDown, {
            let on_close = Rc::clone(&on_close);
            let on_focus_out = Rc::clone(&on_focus_out);
            let dismiss_on_esc = resolved.dismiss_on_esc;
            move |event| {
                let Event::KeyDown(key_event) = event else {
                    return;
                };
                if dismiss_on_esc && key_event.key.logical_key == Key::Named(NamedKey::Escape) {
                    close_if_open(open, &on_close, &on_focus_out);
                }
            }
        })
        .on_cleanup(move || {
            overlay_lifetime.dispose();
            remove_overlay_if_open();
        })
}

fn default_anchor() -> AnchorRect {
    AnchorRect::new(0.0, 0.0, DEFAULT_ANCHOR_WIDTH, DEFAULT_ANCHOR_HEIGHT)
}

fn bind_overlay_effect(
    open: RwSignal<bool>,
    overlay_id: RwSignal<Option<ViewId>>,
    trigger_anchor: RwSignal<AnchorRect>,
    resolved: ResolvedPopover,
    remove_overlay_if_open: Rc<dyn Fn()>,
    overlay_lifetime: OverlayLifetime,
) {
    create_effect({
        move |_| {
            if !open.try_get().unwrap_or(false) {
                remove_overlay_if_open();
                return;
            }

            if overlay_id.try_get().unwrap_or(None).is_some() {
                return;
            }

            let anchor = resolved
                .anchor
                .unwrap_or_else(|| trigger_anchor.try_get().unwrap_or(default_anchor()));
            let layout = overlay_layout_for(&resolved, anchor);
            let overlay_lifetime_for_added = overlay_lifetime.clone();
            let resolved_for_view = resolved.clone();
            OverlayLifecycle::add_overlay_next_tick(
                &overlay_lifetime,
                Point::new(0.0, 0.0),
                move |_| overlay_view(open, layout, resolved_for_view).into_any(),
                move |view_id| {
                    if open.try_get().unwrap_or(false)
                        && overlay_id.try_get().unwrap_or(None).is_none()
                    {
                        overlay_id.set(Some(view_id));
                    } else {
                        OverlayLifecycle::remove_overlay_next_tick(
                            &overlay_lifetime_for_added,
                            view_id,
                        );
                    }
                },
            );
        }
    });
}

fn overlay_layout_for(resolved: &ResolvedPopover, anchor: AnchorRect) -> PopoverOverlay {
    let origin = PlacementResolver::resolve_origin(
        resolved.placement,
        anchor,
        resolved.offset,
        resolved.width,
        ESTIMATED_PANEL_HEIGHT,
        POPOVER_VIEWPORT_WIDTH,
        POPOVER_VIEWPORT_HEIGHT,
    );
    PopoverOverlay {
        x: origin.x,
        y: origin.y,
        width: resolved.width,
        height: ESTIMATED_PANEL_HEIGHT,
        placement: origin.placement,
        popover_bg: resolved.popover_bg,
        popover_border: resolved.popover_border,
        shadow_color: resolved.shadow_color,
        corner_radius: resolved.corner_radius,
    }
}

fn overlay_view(
    open: RwSignal<bool>,
    layout: PopoverOverlay,
    resolved: ResolvedPopover,
) -> Box<dyn View> {
    let on_close = Rc::clone(&resolved.on_close);
    let on_focus_out = Rc::clone(&resolved.on_focus_out);
    let panel = container(render_popover_content(&resolved.children))
        .style(move |style| {
            style
                .absolute()
                .inset_left(layout.x)
                .inset_top(layout.y)
                .width(layout.width)
                .min_height(layout.height)
                .background(FloemColor::from_token(layout.popover_bg))
                .border(1.0)
                .border_color(FloemColor::from_token(layout.popover_border))
                .border_radius(layout.corner_radius)
                .padding(POPOVER_PADDING)
                .gap(POPOVER_GAP)
                .z_index(POPOVER_OVERLAY_PANEL_Z_INDEX)
        })
        .on_event_stop(EventListener::PointerDown, |_| {});

    container(panel)
        .style(|style| {
            style
                .absolute()
                .inset_left(0.0)
                .inset_top(0.0)
                .width(POPOVER_OVERLAY_CATCH_PLANE_SIZE)
                .height(POPOVER_OVERLAY_CATCH_PLANE_SIZE)
                .z_index(POPOVER_OVERLAY_ROOT_Z_INDEX)
        })
        .on_event_stop(EventListener::PointerDown, move |_| {
            if resolved.dismiss_on_outside_click {
                close_if_open(open, &on_close, &on_focus_out);
            }
        })
        .keyboard_navigable()
        .into_any()
}

#[cfg(test)]
mod tests {
    use super::super::{Placement, Popover};
    use super::*;

    #[test]
    fn bottom_layout_places_panel_below_anchor() {
        let resolved = Popover::new()
            .open(true)
            .placement(Placement::Bottom)
            .resolve(&Theme::default_light());
        let layout = overlay_layout_for(&resolved, AnchorRect::new(100.0, 100.0, 120.0, 32.0));

        assert_eq!(layout.x, 40.0);
        assert_eq!(layout.y, 136.0);
    }

    #[test]
    fn side_layout_places_panel_outside_anchor() {
        let left = Popover::new()
            .open(true)
            .placement(Placement::Left)
            .resolve(&Theme::default_light());
        let right = Popover::new()
            .open(true)
            .placement(Placement::Right)
            .resolve(&Theme::default_light());
        let anchor = AnchorRect::new(300.0, 100.0, 120.0, 32.0);

        assert_eq!(overlay_layout_for(&left, anchor).x, 56.0);
        assert_eq!(overlay_layout_for(&right, anchor).x, 424.0);
    }
}
