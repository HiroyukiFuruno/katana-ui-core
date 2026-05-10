use super::types::Direction;
use super::{SplitPane, ops};
use crate::theme::Theme;
use crate::theme::color::Color;
use floem::IntoView;
use floem::event::{Event, EventListener};
use floem::reactive::{SignalGet, SignalUpdate, create_rw_signal};
use floem::style::CursorStyle;
use floem::views::{Decorators, button, container, h_stack, label, v_stack};
use floem::{View, peniko::kurbo::Point};

const HANDLE_THICKNESS: f32 = 4.0;
const HANDLE_HOVER_ALPHA: u8 = 80;
const HANDLE_ACTIVE_ALPHA: u8 = 160;
const CURSOR_HORIZONTAL: &str = "col-resize";
const CURSOR_VERTICAL: &str = "row-resize";
const RATIO_STEP: f32 = 0.05;
const SPLIT_CONTROL_GAP: f32 = crate::floem_view::GAP_XS;
const SPLIT_CONTENT_GAP: f32 = crate::floem_view::GAP_SM;
const SPLIT_HANDLE_MIN_LENGTH: f32 = 48.0;
const MIN_PANE_GROW: f32 = 0.001;

pub(super) fn handle_thickness() -> f32 {
    HANDLE_THICKNESS
}

pub(super) fn handle_color(theme: &Theme) -> Color {
    theme.color.border
}

pub(super) fn handle_hover_color(theme: &Theme) -> Color {
    Color {
        r: theme.color.accent.r,
        g: theme.color.accent.g,
        b: theme.color.accent.b,
        a: HANDLE_HOVER_ALPHA,
    }
}

pub(super) fn handle_active_color(theme: &Theme) -> Color {
    Color {
        r: theme.color.accent.r,
        g: theme.color.accent.g,
        b: theme.color.accent.b,
        a: HANDLE_ACTIVE_ALPHA,
    }
}

pub(super) fn handle_cursor(direction: Direction) -> &'static str {
    match direction {
        Direction::Horizontal => CURSOR_HORIZONTAL,
        Direction::Vertical => CURSOR_VERTICAL,
    }
}

fn handle_cursor_style(direction: Direction) -> CursorStyle {
    match direction {
        Direction::Horizontal => CursorStyle::ColResize,
        Direction::Vertical => CursorStyle::RowResize,
    }
}

fn axis_position(direction: Direction, point: Point) -> f64 {
    match direction {
        Direction::Horizontal => point.x,
        Direction::Vertical => point.y,
    }
}

impl SplitPane {
    #[must_use]
    pub fn view(
        self,
        theme: Theme,
        first: impl IntoView + 'static,
        second: impl IntoView + 'static,
    ) -> impl IntoView {
        let resolved = self.resolve(&theme);
        let ratio = create_rw_signal(resolved.ratio);
        let drag_start = create_rw_signal(None::<(f64, f32)>);
        let handle = crate::floem_view::FloemColor::from_token(resolved.handle_color);
        let handle_hover = crate::floem_view::FloemColor::from_token(resolved.handle_hover_color);
        let handle_active = crate::floem_view::FloemColor::from_token(resolved.handle_active_color);
        let cursor = handle_cursor_style(resolved.direction);
        let direction = resolved.direction;

        let first_pane = container(first).style(move |style| {
            let current_ratio = ratio.try_get().unwrap_or(resolved.ratio);
            style
                .flex_basis(0.0)
                .flex_grow(current_ratio.max(MIN_PANE_GROW))
                .min_width(0.0)
                .min_height(0.0)
        });
        let second_pane = container(second).style(move |style| {
            let current_ratio = ratio.try_get().unwrap_or(resolved.ratio);
            style
                .flex_basis(0.0)
                .flex_grow((1.0 - current_ratio).max(MIN_PANE_GROW))
                .min_width(0.0)
                .min_height(0.0)
        });

        let handle_view = label(|| "").style(move |style| {
            let style = style
                .background(handle)
                .cursor(cursor)
                .hover(move |style| style.background(handle_hover))
                .active(move |style| style.background(handle_active));
            match direction {
                Direction::Horizontal => style
                    .width(resolved.handle_thickness)
                    .min_height(SPLIT_HANDLE_MIN_LENGTH),
                Direction::Vertical => style
                    .height(resolved.handle_thickness)
                    .min_width(SPLIT_HANDLE_MIN_LENGTH),
            }
        });
        let handle_id = handle_view.id();
        let handle_view = handle_view
            .on_event_stop(EventListener::PointerDown, move |event| {
                if let Event::PointerDown(pointer_event) = event
                    && pointer_event.button.is_primary()
                {
                    handle_id.request_active();
                    drag_start.set(Some((
                        axis_position(direction, pointer_event.pos),
                        ratio.get_untracked(),
                    )));
                }
            })
            .on_event_stop(EventListener::PointerMove, move |event| {
                if let Event::PointerMove(pointer_event) = event {
                    let Some((start_position, start_ratio)) = drag_start.get_untracked() else {
                        return;
                    };
                    let Some(parent_size) = handle_id.parent_size() else {
                        return;
                    };
                    let total_size = match direction {
                        Direction::Horizontal => parent_size.width as f32,
                        Direction::Vertical => parent_size.height as f32,
                    };
                    let delta =
                        (axis_position(direction, pointer_event.pos) - start_position) as f32;
                    ratio.set(ops::clamp_ratio(
                        ops::drag_ratio(start_ratio, delta, total_size),
                        resolved.min_ratio,
                        resolved.max_ratio,
                    ));
                }
            })
            .on_event_stop(EventListener::PointerUp, move |_| {
                drag_start.set(None);
                handle_id.clear_active();
            })
            .on_event_stop(EventListener::DoubleClick, move |_| {
                ratio.set(ops::clamp_ratio(
                    ops::reset_ratio(),
                    resolved.min_ratio,
                    resolved.max_ratio,
                ));
            });

        let controls = h_stack((
            button(label(move || {
                format!("{:.0}%", ratio.try_get().unwrap_or(resolved.ratio) * 100.0)
            })),
            button(label(|| "Nudge -")).action(move || {
                ratio.update(|value| {
                    *value = ops::clamp_ratio(
                        *value - RATIO_STEP,
                        resolved.min_ratio,
                        resolved.max_ratio,
                    );
                });
            }),
            button(label(|| "Nudge +")).action(move || {
                ratio.update(|value| {
                    *value = ops::clamp_ratio(
                        *value + RATIO_STEP,
                        resolved.min_ratio,
                        resolved.max_ratio,
                    );
                });
            }),
            button(label(|| "Reset")).action(move || {
                ratio.set(ops::clamp_ratio(
                    ops::reset_ratio(),
                    resolved.min_ratio,
                    resolved.max_ratio,
                ));
            }),
        ))
        .style(|style| style.gap(SPLIT_CONTROL_GAP).items_center());

        match resolved.direction {
            Direction::Horizontal => v_stack((
                h_stack((first_pane, handle_view, second_pane)).style(|style| {
                    style
                        .gap(SPLIT_CONTENT_GAP)
                        .width_full()
                        .min_height(SPLIT_HANDLE_MIN_LENGTH)
                }),
                controls,
            ))
            .into_any(),
            Direction::Vertical => v_stack((
                v_stack((first_pane, handle_view, second_pane)).style(|style| {
                    style
                        .gap(SPLIT_CONTENT_GAP)
                        .width_full()
                        .min_height(SPLIT_HANDLE_MIN_LENGTH)
                }),
                controls,
            ))
            .style(|style| style.gap(SPLIT_CONTENT_GAP))
            .into_any(),
        }
    }
}
