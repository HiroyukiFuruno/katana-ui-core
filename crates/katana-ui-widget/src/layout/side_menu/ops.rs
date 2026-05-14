use crate::floem_view::FloemColor;
use crate::theme::Theme;
use crate::theme::color::Color;
use floem::event::{Event, EventListener};
use floem::keyboard::{Key, NamedKey};
use floem::views::{Decorators, container};
use floem::{IntoView, View};
use std::rc::Rc;

const BORDER_WIDTH: f32 = 1.0;
const CORNER_RADIUS: f32 = 6.0;
const MENU_PADDING: f32 = 6.0;
const MODAL_OVERLAY_ALPHA: u8 = 150;

pub(super) fn modal_overlay(
    content: Box<dyn View>,
    close: Rc<dyn Fn()>,
    theme: Theme,
) -> Box<dyn View> {
    let overlay_bg = Color {
        r: theme.color.bg.r,
        g: theme.color.bg.g,
        b: theme.color.bg.b,
        a: MODAL_OVERLAY_ALPHA,
    };
    let panel = container(content)
        .style(move |style| {
            style
                .background(FloemColor::from_token(theme.color.surface))
                .border(BORDER_WIDTH)
                .border_color(FloemColor::from_token(theme.color.border))
                .padding(MENU_PADDING)
                .border_radius(CORNER_RADIUS)
                .items_center()
        })
        .on_event_stop(EventListener::PointerDown, |_| {})
        .into_any();

    container(panel)
        .style(move |style| {
            style
                .width_full()
                .height_full()
                .items_center()
                .justify_center()
                .background(FloemColor::from_token(overlay_bg))
        })
        .keyboard_navigable()
        .on_event_stop(EventListener::PointerDown, {
            let close = Rc::clone(&close);
            move |_| {
                close();
            }
        })
        .on_event_stop(EventListener::KeyDown, {
            let close = Rc::clone(&close);
            move |event| {
                if let Event::KeyDown(event) = event
                    && event.key.logical_key == Key::Named(NamedKey::Escape)
                {
                    close();
                }
            }
        })
        .into_any()
}

pub(super) fn popover_overlay(
    content: Box<dyn View>,
    anchor: (f32, f32),
    close: Rc<dyn Fn()>,
    theme: Theme,
) -> Box<dyn View> {
    let pop = container(content)
        .style(move |style| {
            style
                .background(FloemColor::from_token(theme.color.surface))
                .border(BORDER_WIDTH)
                .border_color(FloemColor::from_token(theme.color.border))
                .padding(MENU_PADDING)
                .border_radius(CORNER_RADIUS)
                .items_center()
        })
        .on_event_stop(EventListener::PointerDown, |_| {})
        .into_any();

    container(container(pop).style(move |style| {
        style
            .inset_left(anchor.0.max(0.0) as f64)
            .inset_top(anchor.1.max(0.0) as f64)
            .absolute()
    }))
    .style(|style| style.width_full().height_full())
    .keyboard_navigable()
    .on_event_stop(EventListener::PointerDown, {
        let close = Rc::clone(&close);
        move |_| {
            close();
        }
    })
    .on_event_stop(EventListener::KeyDown, {
        let close = Rc::clone(&close);
        move |event| {
            if let Event::KeyDown(event) = event
                && event.key.logical_key == Key::Named(NamedKey::Escape)
            {
                close();
            }
        }
    })
    .into_any()
}
