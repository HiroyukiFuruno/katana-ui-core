use std::cell::Cell;
use std::rc::Rc;

use crate::floem_view::FloemColor;
use crate::layout::popover::AnchorRect;
use crate::overlay_lifecycle::{OverlayLifecycle, OverlayLifetime};
use floem::event::EventListener;
use floem::reactive::{SignalUpdate, create_rw_signal};
use floem::views::Decorators;
use floem::{IntoView, View, ViewId};

use super::ResolvedTooltip;
use super::view::{default_anchor_height, default_anchor_width, visible_after_focus_loss};
use super::{interaction_events, interaction_overlay};

pub(super) fn build_view(
    resolved: ResolvedTooltip,
    child: impl IntoView + 'static,
) -> impl IntoView {
    let child_view = child.into_view();
    let child_id = child_view.id();
    let visible = create_rw_signal(resolved.visible);
    let hover_ready = Rc::new(Cell::new(false));
    let focus_ready = Rc::new(Cell::new(false));
    let hover_token = Rc::new(Cell::new(0_u64));
    let overlay_id = create_rw_signal::<Option<ViewId>>(None);
    let mounted = Rc::new(Cell::new(true));
    let overlay_lifetime = OverlayLifetime::new();

    let tooltip_label = resolved.label.clone();
    let close_overlay: Rc<dyn Fn()> = {
        let overlay_lifetime = overlay_lifetime.clone();
        Rc::new(move || {
            if let Some(id) = overlay_id.try_update(|id| id.take()).flatten() {
                OverlayLifecycle::remove_overlay_next_tick(&overlay_lifetime, id);
            }
            let _ = visible.try_update(|is_visible| {
                *is_visible = false;
            });
        })
    };
    let tooltip_bg = FloemColor::from_token(resolved.bg_color);
    let tooltip_text_color = FloemColor::from_token(resolved.text_color);
    let tooltip_max_width = resolved.max_width;
    let tooltip_font_size = resolved.font_size;
    let tooltip_pad_v = resolved.pad_v;
    let tooltip_pad_h = resolved.pad_h;
    let delay_ms = resolved.delay_ms;
    let dismiss_on_pointer_leave = resolved.dismiss_on_pointer_leave;
    let dismiss_on_focus_loss = resolved.dismiss_on_focus_loss;
    let show_arrow = resolved.show_arrow;

    let anchor = create_rw_signal(AnchorRect::new(
        0.0,
        0.0,
        default_anchor_width(),
        default_anchor_height(),
    ));
    let parent_origin = create_rw_signal((0.0_f32, 0.0_f32));

    interaction_overlay::bind_overlay_effect(
        visible,
        overlay_id,
        interaction_overlay::TooltipOverlayConfig {
            placement: resolved.placement,
            tooltip_label,
            tooltip_max_width,
            tooltip_font_size,
            tooltip_pad_v,
            tooltip_pad_h,
            tooltip_bg,
            tooltip_text_color,
            anchor,
            parent_origin,
            show_arrow,
        },
        Rc::clone(&close_overlay),
        overlay_lifetime.clone(),
    );

    child_view
        .on_event_cont(EventListener::PointerMove, {
            let hover_ready = Rc::clone(&hover_ready);
            let hover_token = Rc::clone(&hover_token);
            let mounted = Rc::clone(&mounted);
            move |event| {
                interaction_events::apply_pointer_move(
                    event,
                    interaction_events::PointerMoveState {
                        anchor_id: child_id,
                        anchor,
                        parent_origin,
                        hover_ready: Rc::clone(&hover_ready),
                        hover_token: Rc::clone(&hover_token),
                        mounted: Rc::clone(&mounted),
                        visible,
                        delay_ms,
                    },
                );
            }
        })
        .on_event_cont(EventListener::PointerLeave, {
            let hover_ready = Rc::clone(&hover_ready);
            let focus_ready = Rc::clone(&focus_ready);
            let hover_token = Rc::clone(&hover_token);
            move |_| {
                hover_token.set(hover_token.get().wrapping_add(1));
                hover_ready.set(false);
                if dismiss_on_pointer_leave {
                    visible.set(focus_ready.get());
                }
            }
        })
        .on_event_cont(EventListener::FocusGained, {
            let focus_ready = Rc::clone(&focus_ready);
            move |_| {
                focus_ready.set(true);
                visible.set(true);
            }
        })
        .on_event_cont(EventListener::FocusLost, {
            let focus_ready = Rc::clone(&focus_ready);
            let hover_ready = Rc::clone(&hover_ready);
            move |_| {
                focus_ready.set(false);
                if dismiss_on_focus_loss {
                    visible.set(visible_after_focus_loss(
                        hover_ready.get(),
                        dismiss_on_focus_loss,
                    ));
                }
            }
        })
        .on_cleanup(move || {
            mounted.set(false);
            overlay_lifetime.dispose();
            if let Some(id) = overlay_id.try_update(|id| id.take()).flatten() {
                OverlayLifecycle::remove_overlay_next_tick(&overlay_lifetime, id);
            }
        })
        .into_any()
}
