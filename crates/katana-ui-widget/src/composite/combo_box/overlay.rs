use crate::layout::popover::{AnchorRect, Placement, PlacementResolver};
use crate::theme::Theme;
use floem::IntoView;
use floem::View;
use floem::event::{Event, EventListener};
use floem::keyboard::{Key, NamedKey};
use floem::views::Decorators;
use floem::views::container;
use std::rc::Rc;

const OVERLAY_BORDER_WIDTH: f32 = 1.0;
const OVERLAY_BORDER_RADIUS: f32 = 8.0;
const OVERLAY_GAP: f32 = 4.0;
const OVERLAY_PADDING: f32 = 8.0;
const OVERLAY_WIDTH: f32 = 280.0;
const OVERLAY_MAX_HEIGHT: f32 = 260.0;
const OVERLAY_ANCHOR_Y_OFFSET: f32 = 2.0;
const OVERLAY_VIEWPORT_WIDTH: f32 = 1024.0;
const OVERLAY_VIEWPORT_HEIGHT: f32 = 768.0;

pub(super) fn build_overlay(
    rows: Box<dyn View>,
    anchor: AnchorRect,
    placement: Placement,
    theme: Theme,
    close_overlay: Rc<dyn Fn()>,
) -> Box<dyn View> {
    let close_outer = Rc::clone(&close_overlay);
    let close_esc = close_overlay;
    let overlay_width = anchor.width.max(OVERLAY_WIDTH);
    let origin = PlacementResolver::resolve_origin(
        placement,
        anchor,
        OVERLAY_ANCHOR_Y_OFFSET,
        overlay_width,
        OVERLAY_MAX_HEIGHT,
        OVERLAY_VIEWPORT_WIDTH,
        OVERLAY_VIEWPORT_HEIGHT,
    );
    container(
        container(rows)
            .style({
                let theme = theme.clone();
                move |style| {
                    style
                        .background(crate::floem_view::FloemColor::from_token(
                            theme.color.surface,
                        ))
                        .border(OVERLAY_BORDER_WIDTH)
                        .border_color(crate::floem_view::FloemColor::from_token(
                            theme.color.border,
                        ))
                        .border_radius(OVERLAY_BORDER_RADIUS)
                        .padding(OVERLAY_PADDING)
                        .gap(OVERLAY_GAP)
                        .width(overlay_width)
                        .max_width(overlay_width)
                        .max_height(OVERLAY_MAX_HEIGHT)
                        .absolute()
                        .inset_left(origin.x)
                        .inset_top(origin.y)
                }
            })
            .on_event_stop(EventListener::PointerDown, |_| {}),
    )
    .style(|style| style.width_full().height_full())
    .keyboard_navigable()
    .on_event_stop(EventListener::PointerDown, move |_| {
        close_outer();
    })
    .on_event_stop(EventListener::KeyDown, move |event| {
        if let Event::KeyDown(key_event) = event
            && key_event.key.logical_key == Key::Named(NamedKey::Escape)
        {
            close_esc();
        }
    })
    .into_any()
}
