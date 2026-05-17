use crate::composite::menu_button::types::MenuButtonContentFactory;
use crate::floem_view::FloemColor;
use crate::layout::popover::{AnchorRect, Placement, PlacementResolver};
use crate::theme::Theme;
use floem::event::{Event, EventListener};
use floem::views::{Decorators, container};
use floem::{IntoView, View};
use std::rc::Rc;

use super::ops::{self, CloseIntent};
use super::style::{MENU_GAP, MENU_OFFSET_Y, MENU_PADDING, MENU_RADIUS};

const MENU_OVERLAY_WIDTH: f32 = 120.0;
const MENU_OVERLAY_HEIGHT: f32 = 128.0;
const MENU_OVERLAY_VIEWPORT_WIDTH: f32 = 1024.0;
const MENU_OVERLAY_VIEWPORT_HEIGHT: f32 = 768.0;
const MENU_OVERLAY_CATCH_PLANE_SIZE: f32 = 10000.0;
const MENU_OVERLAY_ROOT_Z_INDEX: i32 = 1000;
const MENU_OVERLAY_PANEL_Z_INDEX: i32 = 1001;

pub(super) fn build_overlay(
    content: MenuButtonContentFactory,
    close: Rc<dyn Fn()>,
    anchor: AnchorRect,
    placement: Placement,
    theme: Theme,
) -> Box<dyn View> {
    let overlay_width = MENU_OVERLAY_WIDTH.max(anchor.width);
    let origin = PlacementResolver::resolve_origin(
        placement,
        anchor,
        MENU_OFFSET_Y,
        overlay_width,
        MENU_OVERLAY_HEIGHT,
        MENU_OVERLAY_VIEWPORT_WIDTH,
        MENU_OVERLAY_VIEWPORT_HEIGHT,
    );
    let menu = content(Rc::clone(&close));
    let close_outside = Rc::clone(&close);
    let close_esc = Rc::clone(&close);

    container(
        container(menu)
            .style(move |style| {
                style
                    .background(FloemColor::from_token(theme.color.surface))
                    .border(1.0)
                    .border_color(FloemColor::from_token(theme.color.border))
                    .border_radius(MENU_RADIUS)
                    .padding(MENU_PADDING)
                    .gap(MENU_GAP)
                    .absolute()
                    .width(overlay_width)
                    .max_width(overlay_width)
                    .min_width(anchor.width)
                    .max_height(MENU_OVERLAY_HEIGHT)
                    .inset_left(origin.x)
                    .inset_top(origin.y)
                    .z_index(MENU_OVERLAY_PANEL_Z_INDEX)
            })
            .on_event_stop(EventListener::PointerDown, |_| {}),
    )
    .keyboard_navigable()
    .on_event_stop(EventListener::PointerDown, move |_| {
        if matches!(
            ops::close_intent_for_outside_pointer(ops::should_close_on_outside_pointer()),
            CloseIntent::Close
        ) {
            close_outside();
        }
    })
    .on_event_stop(EventListener::KeyDown, move |event| {
        if let Event::KeyDown(key_event) = event
            && matches!(
                ops::close_intent_for_key(&key_event.key.logical_key, true),
                CloseIntent::Close
            )
        {
            close_esc();
        }
    })
    .style(|style| {
        style
            .absolute()
            .inset_left(0.0)
            .inset_top(0.0)
            .width(MENU_OVERLAY_CATCH_PLANE_SIZE)
            .height(MENU_OVERLAY_CATCH_PLANE_SIZE)
            .z_index(MENU_OVERLAY_ROOT_Z_INDEX)
    })
    .into_any()
}
