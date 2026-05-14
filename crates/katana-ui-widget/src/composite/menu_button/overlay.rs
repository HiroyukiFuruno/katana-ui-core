use crate::composite::menu_button::types::MenuButtonContentFactory;
use crate::floem_view::FloemColor;
use crate::layout::popover::{AnchorRect, Placement, PlacementResolver};
use crate::theme::Theme;
use floem::event::{Event, EventListener};
use floem::keyboard::{Key, NamedKey};
use floem::views::{Decorators, container};
use floem::{IntoView, View};
use std::rc::Rc;

use super::style::{MENU_GAP, MENU_OFFSET_Y, MENU_PADDING, MENU_RADIUS};

const MENU_OVERLAY_WIDTH: f32 = 120.0;
const MENU_OVERLAY_HEIGHT: f32 = 128.0;
const MENU_OVERLAY_VIEWPORT_WIDTH: f32 = 1024.0;
const MENU_OVERLAY_VIEWPORT_HEIGHT: f32 = 768.0;

pub(super) fn build_overlay(
    content: MenuButtonContentFactory,
    close: Rc<dyn Fn()>,
    anchor: AnchorRect,
    placement: Placement,
    theme: Theme,
) -> Box<dyn View> {
    let origin = PlacementResolver::resolve_origin(
        placement,
        anchor,
        MENU_OFFSET_Y,
        MENU_OVERLAY_WIDTH.max(anchor.width),
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
                    .min_width(anchor.width)
                    .inset_left(origin.x)
                    .inset_top(origin.y)
            })
            .on_event_stop(EventListener::PointerDown, |_| {}),
    )
    .keyboard_navigable()
    .on_event_stop(EventListener::PointerDown, move |_| {
        close_outside();
    })
    .on_event_stop(EventListener::KeyDown, move |event| {
        if let Event::KeyDown(key_event) = event
            && key_event.key.logical_key == Key::Named(NamedKey::Escape)
        {
            close_esc();
        }
    })
    .style(|style| style.width_full().height_full())
    .into_any()
}
