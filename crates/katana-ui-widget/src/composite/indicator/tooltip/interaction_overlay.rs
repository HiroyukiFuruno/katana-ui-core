use crate::layout::popover::{AnchorRect, FreePlacement, Placement};
use crate::overlay_lifecycle::OverlayLifecycle;
use floem::event::{Event, EventListener};
use floem::keyboard::{Key, NamedKey};
use floem::peniko::Color;
use floem::peniko::kurbo::Point;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate, create_effect};
use floem::views::{Decorators, container, label};
use floem::{IntoView, View, ViewId};
use std::rc::Rc;

use super::interaction_arrow::{TooltipArrowConfig, arrow_view};
use super::view::{
    default_anchor_height, default_anchor_width, estimate_overlay_height, overlay_layout_detail,
    overlay_offset, viewport_height, viewport_width,
};
use std::cell::Cell;

const TOOLTIP_RADIUS: f32 = crate::floem_view::CORNER_RADIUS_SM;

pub(super) struct TooltipOverlayConfig {
    pub(super) placement: super::Placement,
    pub(super) tooltip_label: String,
    pub(super) tooltip_max_width: f32,
    pub(super) tooltip_font_size: f32,
    pub(super) tooltip_pad_v: f32,
    pub(super) tooltip_pad_h: f32,
    pub(super) tooltip_bg: Color,
    pub(super) tooltip_text_color: Color,
    pub(super) anchor: RwSignal<AnchorRect>,
    pub(super) parent_origin: RwSignal<(f32, f32)>,
    pub(super) show_arrow: bool,
}

#[derive(Clone)]
struct TooltipOverlayViewConfig {
    overlay_label: String,
    tooltip_font_size: f32,
    tooltip_bg: Color,
    tooltip_text_color: Color,
    tooltip_max_width: f32,
    tooltip_pad_v: f32,
    tooltip_pad_h: f32,
    x: f32,
    y: f32,
    tooltip_height: f32,
    placement: Placement,
    anchor: AnchorRect,
    show_arrow: bool,
    close_overlay_for_outside: Rc<dyn Fn()>,
    close_overlay_for_escape: Rc<dyn Fn()>,
}

pub(super) fn bind_overlay_effect(
    visible: RwSignal<bool>,
    overlay_id: RwSignal<Option<ViewId>>,
    config: TooltipOverlayConfig,
    close_overlay: Rc<dyn Fn()>,
) {
    let overlay_pending = Rc::new(Cell::new(false));
    create_effect({
        let close_overlay = Rc::clone(&close_overlay);
        let overlay_pending = Rc::clone(&overlay_pending);
        move |_| {
            if !visible.try_get().unwrap_or(false) {
                overlay_pending.set(false);
                if let Some(id) = overlay_id.try_update(|id| id.take()).flatten() {
                    OverlayLifecycle::remove_overlay_next_tick(id);
                }
                return;
            }

            if overlay_id.try_get().unwrap_or(None).is_some() || overlay_pending.get() {
                return;
            }
            overlay_pending.set(true);

            let current_anchor = config.anchor.try_get().unwrap_or(AnchorRect::new(
                0.0,
                0.0,
                default_anchor_width(),
                default_anchor_height(),
            ));
            let placement = placement_for_parent_origin(
                config.placement,
                config.parent_origin.try_get().unwrap_or((0.0, 0.0)),
            );
            let tooltip_height = estimate_overlay_height(
                &config.tooltip_label,
                config.tooltip_max_width,
                config.tooltip_font_size,
                config.tooltip_pad_v,
                config.tooltip_pad_h,
            );
            let layout = overlay_layout_detail(
                placement,
                current_anchor,
                config.tooltip_max_width,
                tooltip_height,
                viewport_width(),
                viewport_height(),
                overlay_offset(),
            );

            let tooltip_label = config.tooltip_label.clone();
            let tooltip_font_size = config.tooltip_font_size;
            let tooltip_bg = config.tooltip_bg;
            let tooltip_text_color = config.tooltip_text_color;
            let tooltip_max_width = config.tooltip_max_width;
            let tooltip_pad_v = config.tooltip_pad_v;
            let tooltip_pad_h = config.tooltip_pad_h;
            let view_config = TooltipOverlayViewConfig {
                overlay_label: tooltip_label,
                tooltip_font_size,
                tooltip_bg,
                tooltip_text_color,
                tooltip_max_width,
                tooltip_pad_v,
                tooltip_pad_h,
                x: layout.x,
                y: layout.y,
                tooltip_height,
                placement: layout.placement,
                anchor: current_anchor,
                show_arrow: config.show_arrow,
                close_overlay_for_outside: Rc::clone(&close_overlay),
                close_overlay_for_escape: Rc::clone(&close_overlay),
            };
            OverlayLifecycle::add_overlay_next_tick(
                Point::new(0.0, 0.0),
                move |_| overlay_view(view_config.clone()).into_any(),
                {
                    let overlay_pending = Rc::clone(&overlay_pending);
                    move |overlay_view_id| {
                        overlay_pending.set(false);
                        if visible.try_get().unwrap_or(false)
                            && overlay_id.try_get().unwrap_or(None).is_none()
                        {
                            overlay_id.set(Some(overlay_view_id));
                        } else {
                            OverlayLifecycle::remove_overlay_next_tick(overlay_view_id);
                        }
                    }
                },
            );
        }
    });
}

fn placement_for_parent_origin(placement: Placement, parent_origin: (f32, f32)) -> Placement {
    match placement {
        Placement::Free(FreePlacement::ParentOffset { x, y }) => {
            Placement::Free(FreePlacement::ParentOffset {
                x: parent_origin.0 + x,
                y: parent_origin.1 + y,
            })
        }
        _ => placement,
    }
}

fn overlay_view(config: TooltipOverlayViewConfig) -> Box<dyn View> {
    let arrow = arrow_view(TooltipArrowConfig {
        tooltip_bg: config.tooltip_bg,
        tooltip_max_width: config.tooltip_max_width,
        tooltip_height: config.tooltip_height,
        x: config.x,
        y: config.y,
        placement: config.placement,
        anchor: config.anchor,
        show_arrow: config.show_arrow,
    });
    let panel = container(
        label(move || config.overlay_label.clone())
            .style(move |style| {
                style
                    .font_size(config.tooltip_font_size)
                    .background(config.tooltip_bg)
                    .color(config.tooltip_text_color)
                    .padding_vert(config.tooltip_pad_v)
                    .padding_horiz(config.tooltip_pad_h)
                    .width(config.tooltip_max_width)
                    .max_width(config.tooltip_max_width)
                    .border_radius(TOOLTIP_RADIUS)
            })
            .on_event_stop(EventListener::PointerDown, |_| {}),
    )
    .style(move |style| style.absolute().inset_left(config.x).inset_top(config.y));

    container((arrow, panel))
        .style(|style| style.width_full().height_full())
        .on_event_stop(EventListener::PointerDown, move |_| {
            (config.close_overlay_for_outside)();
        })
        .keyboard_navigable()
        .on_event_stop(EventListener::KeyDown, move |event| {
            if let Event::KeyDown(key_event) = event
                && key_event.key.logical_key == Key::Named(NamedKey::Escape)
            {
                (config.close_overlay_for_escape)();
            }
        })
        .into_any()
}
