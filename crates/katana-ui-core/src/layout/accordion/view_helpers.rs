use crate::theme::Theme;
use floem::IntoView;
use floem::View;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{Decorators, container, empty, h_stack, h_stack_from_iter};
use std::time::Duration;

use super::types::{AccordionProps, IndicatorPosition};

const HEADER_FONT_SIZE: f32 = 13.0;
const HEADER_PAD_V: f32 = 8.0;
const HEADER_PAD_H: f32 = 12.0;
const ANIMATION_MS: u32 = 180;
const BODY_MAX_HEIGHT: f32 = 240.0;
pub(super) const HEADER_GAP: f32 = 4.0;
pub(super) const HEADER_SLOT_WIDTH: f32 = 14.0;
const ANIMATION_STEP_MS: u64 = 16;
const ANIMATION_EPSILON: f32 = 0.001;
const TREE_ROW_HEIGHT: f32 = 24.0;
const TREE_INDENT: f32 = 16.0;
const TREE_LINE_WIDTH: f32 = 1.0;

#[derive(Clone)]
pub(super) enum OpenState {
    Controlled(RwSignal<bool>),
    Uncontrolled(RwSignal<bool>),
}

impl OpenState {
    fn get(&self) -> bool {
        match self {
            Self::Controlled(signal) => signal.get(),
            Self::Uncontrolled(signal) => signal.get(),
        }
    }

    fn set(&self, open: bool) {
        match self {
            Self::Controlled(signal) => signal.set(open),
            Self::Uncontrolled(signal) => signal.set(open),
        }
    }
}

pub(super) fn make_open_state(props: &AccordionProps) -> OpenState {
    match props.control_state {
        super::AccordionControlState::Controlled(signal) => OpenState::Controlled(signal),
        super::AccordionControlState::Uncontrolled => {
            OpenState::Uncontrolled(create_rw_signal(props.expanded))
        }
    }
}

pub(super) fn open_state_get(state: &OpenState) -> bool {
    state.get()
}

pub(super) fn open_state_set(state: &OpenState, value: bool) {
    state.set(value);
}

pub(super) fn animate_open(
    ratio: RwSignal<f32>,
    token: RwSignal<u32>,
    reduced_motion: bool,
    duration_ms: u32,
    target_open: bool,
) {
    let target = if target_open { 1.0 } else { 0.0 };
    let current = ratio.get_untracked();
    let current_token = token.get_untracked().wrapping_add(1);
    token.set(current_token);

    if reduced_motion || duration_ms == 0 {
        ratio.set(target);
        return;
    }

    if (current - target).abs() < ANIMATION_EPSILON {
        ratio.set(target);
        return;
    }

    let mut steps = (duration_ms as f64 / ANIMATION_STEP_MS as f64).ceil() as u32;
    if steps == 0 {
        steps = 1;
    }

    let delta = (target - current) / steps as f32;
    run_animation_step(ratio, token, current_token, steps, delta, target);
}

fn run_animation_step(
    ratio: RwSignal<f32>,
    token: RwSignal<u32>,
    current_token: u32,
    remaining: u32,
    delta: f32,
    target: f32,
) {
    if remaining == 0 {
        ratio.set(target);
        return;
    }

    ratio.try_update(|value| {
        *value = (*value + delta).clamp(0.0, 1.0);
    });

    if remaining == 1 {
        ratio.set(target);
        return;
    }

    let next = remaining - 1;
    floem::action::exec_after(Duration::from_millis(ANIMATION_STEP_MS), move |_| {
        if token.try_get().unwrap_or(0) != current_token {
            return;
        }
        run_animation_step(ratio, token, current_token, next, delta, target);
    });
}

pub(super) fn header_font_size() -> f32 {
    HEADER_FONT_SIZE
}

pub(super) fn header_padding() -> (f32, f32) {
    (HEADER_PAD_V, HEADER_PAD_H)
}

pub(super) fn animation_ms() -> u32 {
    ANIMATION_MS
}

pub(super) fn body_max_height() -> f32 {
    BODY_MAX_HEIGHT
}

pub(super) fn chevron_symbol(expanded: bool, position: IndicatorPosition) -> Option<&'static str> {
    match position {
        IndicatorPosition::None => None,
        _ => Some(if expanded { "▲" } else { "▼" }),
    }
}

pub(super) fn header_bg(disabled: bool, theme: &Theme) -> crate::theme::color::Color {
    if disabled {
        theme.color.surface
    } else {
        theme.color.bg
    }
}

pub(super) fn header_text(disabled: bool, theme: &Theme) -> crate::theme::color::Color {
    if disabled {
        theme.color.text_disabled
    } else {
        theme.color.text
    }
}

pub(super) fn border_color(theme: &Theme) -> crate::theme::color::Color {
    theme.color.border
}

pub(super) fn build_tree_prefix(
    depth: usize,
    show_lines: bool,
    line_color: floem::peniko::Color,
) -> impl IntoView {
    let width = (depth as f32 * TREE_INDENT).max(0.0);
    if !show_lines || depth == 0 {
        return empty().style(move |style| style.width(width)).into_any();
    }

    h_stack_from_iter(build_tree_indent(depth, line_color))
        .style(move |style| style.width(depth as f32 * TREE_INDENT))
        .into_any()
}

fn build_tree_indent(depth: usize, line_color: floem::peniko::Color) -> Vec<Box<dyn View>> {
    (0..depth)
        .map(|_| {
            h_stack((
                container(empty())
                    .style(move |style| {
                        style
                            .width(TREE_LINE_WIDTH)
                            .height(TREE_ROW_HEIGHT)
                            .background(line_color)
                    })
                    .into_any(),
                container(empty()).style(|style| {
                    style
                        .width(TREE_INDENT - TREE_LINE_WIDTH)
                        .height(TREE_ROW_HEIGHT)
                }),
            ))
            .into_any()
        })
        .collect()
}
