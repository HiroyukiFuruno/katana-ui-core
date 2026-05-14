mod controls;
mod paint_util;
mod panel;
mod plane;
mod preview;
mod trigger;

use super::ColorPickerRgba;
use super::ops;
use super::types::{
    ColorPickerValue, InlineColorPicker, LabeledColorPicker, ResolvedInlineColorPicker,
    ResolvedLabeledColorPicker, RgbaChannel,
};
use crate::overlay_lifecycle::OverlayLifecycle;
use crate::theme::Theme;
use crate::theme::color::Color;
use floem::IntoView;
use floem::event::{Event, EventListener};
use floem::keyboard::{Key, NamedKey};
use floem::peniko::kurbo::Point;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate, create_effect, create_rw_signal};
use floem::views::{Decorators, container, empty, h_stack, label};
use floem::{View, ViewId};
use std::cell::Cell;
use std::rc::Rc;

const LABEL_WIDTH: f32 = 130.0;
const ROW_HEIGHT: f32 = 24.0;
const STACK_GAP: f32 = 8.0;
const OVERLAY_OFFSET: f32 = 6.0;
const OVERLAY_SAFE_WIDTH: f32 = 1280.0;
const OVERLAY_SAFE_HEIGHT: f32 = 900.0;
const OVERLAY_Z_INDEX: i32 = 3000;

impl InlineColorPicker {
    #[must_use]
    pub fn view(self, theme: Theme) -> impl IntoView {
        let resolved = self.resolve(&theme);
        inline_picker_view(resolved, theme)
    }
}

impl LabeledColorPicker {
    #[must_use]
    pub fn view(self, theme: Theme) -> impl IntoView {
        let resolved = self.resolve(&theme);
        labeled_picker_view(resolved, theme)
    }
}

impl ColorPickerRgba {
    #[must_use]
    pub fn view(self, theme: Theme) -> impl IntoView {
        let resolved = self.resolve(&theme);
        inline_picker_view(resolved, theme)
    }
}

fn labeled_picker_view(resolved: ResolvedLabeledColorPicker, theme: Theme) -> impl IntoView {
    h_stack((
        label(move || resolved.label.clone())
            .style(|style| style.width(LABEL_WIDTH).height(ROW_HEIGHT).items_center()),
        container(empty()).style(move |style| style.width(resolved.spacing)),
        container(inline_picker_view(resolved.picker, theme.clone()))
            .style(move |style| style.margin_top(resolved.offset_y)),
    ))
    .style(|style| style.height(ROW_HEIGHT).items_center().gap(STACK_GAP))
}

fn inline_picker_view(resolved: ResolvedInlineColorPicker, theme: Theme) -> impl IntoView {
    let state = create_rw_signal(ops::ColorPickerOps::new_value(
        resolved.value,
        resolved.alpha,
    ));
    let open = create_rw_signal(resolved.open);
    let locked = resolved.disabled || resolved.readonly;
    let on_change = Rc::clone(&resolved.on_change);
    let allows_alpha = resolved.alpha.allows_alpha();
    let theme_for_trigger = theme.clone();
    let overlay_id = create_rw_signal::<Option<ViewId>>(None);
    let overlay_pending = Rc::new(Cell::new(false));

    let trigger = trigger::trigger_button(
        state,
        open,
        resolved.clone(),
        theme_for_trigger.clone(),
        locked,
    );
    let trigger_id = trigger.id();
    let close_overlay = close_overlay_handler(open, overlay_id, trigger_id);

    create_effect({
        let close_overlay = Rc::clone(&close_overlay);
        let overlay_pending = Rc::clone(&overlay_pending);
        move |_| {
            if !open.try_get().unwrap_or(false) {
                overlay_pending.set(false);
                remove_overlay_if_present(overlay_id);
                return;
            }

            if overlay_id.try_get().unwrap_or(None).is_some() || overlay_pending.get() {
                return;
            }
            overlay_pending.set(true);

            let panel_scale = resolved.panel_scale;
            let position = panel_position(trigger_id, panel_scale);
            let panel_theme = theme.clone();
            let panel_resolved = resolved.clone();
            let panel_on_change = Rc::clone(&on_change);
            let close_on_outside = Rc::clone(&close_overlay);
            let close_on_escape = Rc::clone(&close_overlay);

            OverlayLifecycle::add_overlay_next_tick(
                Point::new(0.0, 0.0),
                move |_| {
                    let panel = panel::panel_view(panel::PanelViewArgs {
                        state,
                        open,
                        resolved: panel_resolved.clone(),
                        on_change: Rc::clone(&panel_on_change),
                        locked,
                        allows_alpha,
                        panel_scale,
                        theme: panel_theme.clone(),
                    })
                    .on_event_stop(EventListener::PointerDown, |_| {});
                    let panel_id = panel.id();
                    OverlayLifecycle::request_focus_next_tick(panel_id);

                    let floating_panel = container(panel).style(move |style| {
                        style
                            .absolute()
                            .inset_left(position.x)
                            .inset_top(position.y)
                            .z_index(OVERLAY_Z_INDEX)
                    });

                    container(floating_panel)
                        .style(|style| {
                            style
                                .width_full()
                                .height_full()
                                .absolute()
                                .z_index(OVERLAY_Z_INDEX)
                        })
                        .keyboard_navigable()
                        .on_event_stop(EventListener::PointerDown, move |_| {
                            (close_on_outside)();
                        })
                        .on_event_stop(EventListener::KeyDown, move |event| match event {
                            Event::KeyDown(key_event)
                                if key_event.key.logical_key == Key::Named(NamedKey::Escape) =>
                            {
                                (close_on_escape)();
                            }
                            _ => (),
                        })
                        .into_any()
                },
                {
                    let overlay_pending = Rc::clone(&overlay_pending);
                    move |overlay_view_id| {
                        overlay_pending.set(false);
                        if open.try_get().unwrap_or(false)
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

    container(trigger).on_cleanup(move || {
        (close_overlay)();
    })
}

pub(super) fn apply_state(
    state: &RwSignal<ColorPickerValue>,
    on_change: Rc<dyn Fn(Color)>,
    next: ColorPickerValue,
) {
    if let Some(Some(next_color)) = state.try_update(|current| {
        if *current == next {
            return None;
        }
        *current = next;
        Some(current.color)
    }) {
        on_change(next_color);
    }
}

pub(super) fn channel_to_color_name(channel: RgbaChannel) -> &'static str {
    match channel {
        RgbaChannel::Red => "R",
        RgbaChannel::Green => "G",
        RgbaChannel::Blue => "B",
        RgbaChannel::Alpha => "A",
    }
}

#[derive(Clone, Copy)]
struct PanelPosition {
    x: f32,
    y: f32,
}

fn panel_position(trigger_id: ViewId, panel_scale: f32) -> PanelPosition {
    let layout = trigger_id.get_layout().unwrap_or_default();
    let panel_width = panel::panel_width(panel_scale);
    let panel_height = panel::panel_estimated_height(panel_scale);
    let x = layout
        .location
        .x
        .clamp(0.0, OVERLAY_SAFE_WIDTH - panel_width);
    let below = layout.location.y + layout.size.height + OVERLAY_OFFSET;
    let y = if below + panel_height > OVERLAY_SAFE_HEIGHT {
        (layout.location.y - panel_height - OVERLAY_OFFSET).max(0.0)
    } else {
        below
    };

    PanelPosition { x, y }
}

fn close_overlay_handler(
    open: RwSignal<bool>,
    overlay_id: RwSignal<Option<ViewId>>,
    trigger_id: ViewId,
) -> Rc<dyn Fn()> {
    Rc::new(move || {
        remove_overlay_if_present(overlay_id);
        let _ = open.try_update(|is_open| *is_open = false);
        OverlayLifecycle::request_focus_next_tick(trigger_id);
    })
}

fn remove_overlay_if_present(overlay_id: RwSignal<Option<ViewId>>) {
    if let Some(id) = overlay_id
        .try_update(|current_overlay_id| current_overlay_id.take())
        .flatten()
    {
        OverlayLifecycle::remove_overlay_next_tick(id);
    }
}
