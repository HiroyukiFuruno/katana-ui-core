use super::types::{FreePlacement, Placement, PopoverChildren, ResolvedPopover};
use crate::floem_view::FloemColor;
use crate::theme::Theme;
use crate::theme::color::Color;
use floem::event::{Event, EventListener};
use floem::keyboard::{Key, NamedKey};
use floem::reactive::{RwSignal, SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{Decorators, button, container, empty, label, stack};
use floem::{IntoView, View};
use std::rc::Rc;

const CORNER_RADIUS: f32 = 6.0;
const DEFAULT_OFFSET: f32 = 4.0;
const SHADOW_ALPHA: u8 = 40;
const POPOVER_PADDING: f32 = crate::floem_view::GAP_SM;
const POPOVER_GAP: f32 = crate::floem_view::GAP_XS;
const ESTIMATED_TRIGGER_WIDTH: f32 = 120.0;
const ESTIMATED_TRIGGER_HEIGHT: f32 = 32.0;
const ESTIMATED_PANEL_HEIGHT: f32 = 96.0;
const ROOT_EXTRA_SPACE: f32 = 12.0;

#[derive(Debug, Clone, Copy)]
pub(super) struct PopoverViewStyle {
    pub popover_bg: Color,
    pub popover_border: Color,
    pub shadow_color: Color,
    pub corner_radius: f32,
}

#[derive(Debug, Clone, Copy)]
struct LocalPanelLayout {
    x: f32,
    y: f32,
    width: f32,
    min_height: f32,
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

fn local_panel_layout(placement: Placement, offset: f32, width: f32) -> LocalPanelLayout {
    let end_x = ESTIMATED_TRIGGER_WIDTH - width;
    let bottom_y = ESTIMATED_TRIGGER_HEIGHT + offset;
    let top_y = -(ESTIMATED_PANEL_HEIGHT + offset);
    let start_x = -(width + offset);
    let end_side_x = ESTIMATED_TRIGGER_WIDTH + offset;
    let (x, y) = match placement {
        Placement::Bottom | Placement::Auto => (0.0, bottom_y),
        Placement::BottomStart => (0.0, bottom_y),
        Placement::BottomEnd => (end_x, bottom_y),
        Placement::Top => (0.0, top_y),
        Placement::TopStart => (0.0, top_y),
        Placement::TopEnd => (end_x, top_y),
        Placement::Start => (start_x, 0.0),
        Placement::End => (end_side_x, 0.0),
        Placement::Free(FreePlacement::AnchorOffset { x, y }) => (x, y),
        Placement::Free(FreePlacement::ParentOffset { x, y }) => (x, y),
    };

    LocalPanelLayout {
        x,
        y,
        width,
        min_height: ESTIMATED_PANEL_HEIGHT,
    }
}

fn render_popover_content(children: &Option<PopoverChildren>) -> Box<dyn View> {
    children
        .as_ref()
        .map(|child| child())
        .unwrap_or_else(|| container(label(|| "")).into_any())
}

fn hidden_panel() -> Box<dyn View> {
    container(empty())
        .style(|style| style.width(0.0).height(0.0))
        .into_any()
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
    let anchor_label = anchor_label.into();
    let children = resolved.children.clone();
    let dismiss_on_esc = resolved.dismiss_on_esc;
    let on_close = Rc::clone(&resolved.on_close);
    let on_focus_in = Rc::clone(&resolved.on_focus_in);
    let on_focus_out = Rc::clone(&resolved.on_focus_out);
    let layout = local_panel_layout(resolved.placement, resolved.offset, resolved.width);
    let popover_bg = FloemColor::from_token(resolved.popover_bg);
    let popover_border = FloemColor::from_token(resolved.popover_border);
    let corner_radius = resolved.corner_radius;

    let trigger = button(label(move || anchor_label.clone())).action({
        let on_close = Rc::clone(&on_close);
        let on_focus_in = Rc::clone(&on_focus_in);
        let on_focus_out = Rc::clone(&on_focus_out);
        move || {
            if open.try_get().unwrap_or(false) {
                close_if_open(open, &on_close, &on_focus_out);
            } else {
                open_if_closed(open, &on_focus_in);
            }
        }
    });

    let panel = floem::views::dyn_container(
        move || open.get(),
        move |is_open| {
            if !is_open {
                return hidden_panel();
            }

            let content = render_popover_content(&children);
            container(content)
                .style(move |style| {
                    style
                        .absolute()
                        .inset_left(layout.x)
                        .inset_top(layout.y)
                        .width(layout.width)
                        .min_height(layout.min_height)
                        .background(popover_bg)
                        .border(1.0)
                        .border_color(popover_border)
                        .border_radius(corner_radius)
                        .padding(POPOVER_PADDING)
                        .gap(POPOVER_GAP)
                })
                .into_any()
        },
    );

    stack((trigger, panel))
        .keyboard_navigable()
        .on_event_stop(EventListener::KeyDown, {
            let on_close = Rc::clone(&on_close);
            let on_focus_out = Rc::clone(&on_focus_out);
            move |event| {
                let Event::KeyDown(key_event) = event else {
                    return;
                };
                if dismiss_on_esc && key_event.key.logical_key == Key::Named(NamedKey::Escape) {
                    close_if_open(open, &on_close, &on_focus_out);
                }
            }
        })
        .style(move |style| {
            style
                .min_width(resolved.width.max(ESTIMATED_TRIGGER_WIDTH))
                .min_height(ESTIMATED_TRIGGER_HEIGHT + ESTIMATED_PANEL_HEIGHT + ROOT_EXTRA_SPACE)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bottom_layout_places_panel_below_trigger() {
        let layout = local_panel_layout(Placement::Bottom, 4.0, 240.0);

        assert_eq!(layout.x, 0.0);
        assert_eq!(layout.y, ESTIMATED_TRIGGER_HEIGHT + 4.0);
    }

    #[test]
    fn side_layout_places_panel_outside_trigger() {
        let start = local_panel_layout(Placement::Start, 4.0, 240.0);
        let end = local_panel_layout(Placement::End, 4.0, 240.0);

        assert_eq!(start.x, -244.0);
        assert_eq!(end.x, ESTIMATED_TRIGGER_WIDTH + 4.0);
    }

    #[test]
    fn free_layout_uses_requested_offsets() {
        let layout = local_panel_layout(
            Placement::Free(FreePlacement::AnchorOffset { x: 12.0, y: 16.0 }),
            4.0,
            240.0,
        );

        assert_eq!(layout.x, 12.0);
        assert_eq!(layout.y, 16.0);
    }
}
