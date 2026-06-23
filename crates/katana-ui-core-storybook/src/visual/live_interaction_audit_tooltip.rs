use crate::visual::Canvas;
use crate::visual::window_interaction::{apply_hover_at, focus_clickable_at_for_audit};

use super::{
    StorybookLiveInteractionScenario, component_body_pixel_diff, page_state, render_state, scenario,
};

const PAGE: &str = "tooltip";
const CLICK_OFFSET: usize = 4;
const BUBBLE_X: usize = 112;
const BUBBLE_Y: usize = 34;
const BUBBLE_WIDTH: usize = 132;
const BUBBLE_HEIGHT: usize = 26;
const BUBBLE_SAMPLE_OFFSET: usize = 8;

pub(super) fn scenarios(page: &'static str) -> Vec<StorybookLiveInteractionScenario> {
    if page != PAGE {
        return Vec::new();
    }
    vec![
        tooltip_anchor_hover_open_scenario(),
        tooltip_idle_bubble_hidden_until_hover_scenario(),
        tooltip_hover_idempotent_scenario(),
        tooltip_hover_leave_close_scenario(),
        tooltip_window_hover_clear_close_scenario(),
        tooltip_hover_bubble_geometry_scenario(),
        tooltip_focus_scenario(),
    ]
}

fn tooltip_anchor_hover_open_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(PAGE);
    let before = render_state(PAGE, &state);
    let target = crate::visual::dedicated_tooltip::anchor_hit_rect(state.preset_index);
    let hovered = apply_hover_at(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(PAGE, &before, &after);
    let passed = hovered
        && state.screen_state.last_action == "tooltip_hover"
        && state.screen_state.last_event == "tooltip_opened"
        && state.screen_state.state_label == "hover=true focus=true"
        && body_pixel_diff > 0;
    scenario(
        PAGE,
        "tooltip_anchor_hover_open",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn tooltip_idle_bubble_hidden_until_hover_scenario() -> StorybookLiveInteractionScenario {
    let idle_state = page_state(PAGE);
    let before = render_state(PAGE, &idle_state);
    let mut state = page_state(PAGE);
    let target = crate::visual::dedicated_tooltip::anchor_hit_rect(state.preset_index);
    let hovered = apply_hover_at(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(PAGE, &state);
    let component = super::preview_detail::component_action_hit_rect(PAGE);
    let sample_x = component.x + BUBBLE_X + BUBBLE_SAMPLE_OFFSET;
    let sample_y = component.y + BUBBLE_Y + BUBBLE_SAMPLE_OFFSET;
    let hidden_before =
        pixel_at(&before, sample_x, sample_y) != pixel_at(&after, sample_x, sample_y);
    let body_pixel_diff = component_body_pixel_diff(PAGE, &before, &after);
    let passed = hovered
        && hidden_before
        && state.screen_state.last_action == "tooltip_hover"
        && state.screen_state.last_event == "tooltip_opened"
        && state.screen_state.state_label == "hover=true focus=true"
        && body_pixel_diff > 0;
    scenario(
        PAGE,
        "tooltip_idle_bubble_hidden_until_hover",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn tooltip_hover_idempotent_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(PAGE);
    let before = render_state(PAGE, &state);
    let target = crate::visual::dedicated_tooltip::anchor_hit_rect(state.preset_index);
    let hovered = apply_hover_at(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(PAGE, &before, &after);
    let action_count = state.screen_state.action_count;
    let repeated = apply_hover_at(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let passed = hovered
        && repeated
        && state.screen_state.action_count == action_count
        && state.screen_state.last_action == "tooltip_hover"
        && state.screen_state.last_event == "tooltip_opened"
        && state.screen_state.state_label == "hover=true focus=true"
        && body_pixel_diff > 0;
    scenario(
        PAGE,
        "tooltip_hover_idempotent",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn tooltip_hover_leave_close_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(PAGE);
    let target = crate::visual::dedicated_tooltip::anchor_hit_rect(state.preset_index);
    let hovered = apply_hover_at(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before_close = render_state(PAGE, &state);
    let component = super::preview_detail::component_action_hit_rect(PAGE);
    let closed = apply_hover_at(
        &mut state,
        component.x + CLICK_OFFSET,
        component.y + CLICK_OFFSET,
    );
    let after_close = render_state(PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(PAGE, &before_close, &after_close);
    let passed = hovered
        && closed
        && state.screen_state.last_action == "tooltip_hover"
        && state.screen_state.last_event == "tooltip_closed"
        && state.screen_state.state_label == "hover=false focus=false"
        && body_pixel_diff > 0;
    scenario(
        PAGE,
        "tooltip_hover_leave_close",
        "hover",
        closed,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn tooltip_window_hover_clear_close_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(PAGE);
    let target = crate::visual::dedicated_tooltip::anchor_hit_rect(state.preset_index);
    let hovered = apply_hover_at(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before_close = render_state(PAGE, &state);
    let closed = crate::visual::window::clear_hover_for_audit(&mut state);
    let after_close = render_state(PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(PAGE, &before_close, &after_close);
    let passed = hovered
        && closed
        && state.screen_state.last_action == "tooltip_hover"
        && state.screen_state.last_event == "tooltip_closed"
        && state.screen_state.state_label == "hover=false focus=false"
        && body_pixel_diff > 0;
    scenario(
        PAGE,
        "tooltip_window_hover_clear_close",
        "hover",
        closed,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn tooltip_hover_bubble_geometry_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(PAGE);
    let before = render_state(PAGE, &state);
    let target = crate::visual::dedicated_tooltip::anchor_hit_rect(state.preset_index);
    let hovered = apply_hover_at(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(PAGE, &state);
    let component = super::preview_detail::component_action_hit_rect(PAGE);
    let bubble_x = component.x + BUBBLE_X;
    let bubble_y = component.y + BUBBLE_Y;
    let anchor_center_x = target.x + target.width / 2;
    let body_pixel_diff = component_body_pixel_diff(PAGE, &before, &after);
    let bubble_sample_changed =
        pixel_at(&before, bubble_x, bubble_y) != pixel_at(&after, bubble_x, bubble_y);
    let bubble_inside = component.contains(bubble_x, bubble_y)
        && component.contains(bubble_x + BUBBLE_WIDTH - 1, bubble_y + BUBBLE_HEIGHT - 1);
    let covers_anchor = bubble_x <= anchor_center_x && anchor_center_x <= bubble_x + BUBBLE_WIDTH;
    let above_anchor = bubble_y + BUBBLE_HEIGHT < target.y;
    let passed = hovered
        && bubble_sample_changed
        && bubble_inside
        && covers_anchor
        && above_anchor
        && body_pixel_diff > 0;
    scenario(
        PAGE,
        "tooltip_hover_bubble_geometry",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn tooltip_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(PAGE);
    let before = render_state(PAGE, &state);
    let target = crate::visual::dedicated_tooltip::anchor_hit_rect(state.preset_index);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "tooltip_focus"
        && state.screen_state.last_event == "tooltip_focused"
        && state.screen_state.state_label == "hover=true focus=true"
        && body_pixel_diff > 0;
    scenario(
        PAGE,
        "tooltip_focus_open",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn pixel_at(canvas: &Canvas, x: usize, y: usize) -> Option<u32> {
    canvas.pixels().get(y * canvas.width() + x).copied()
}
