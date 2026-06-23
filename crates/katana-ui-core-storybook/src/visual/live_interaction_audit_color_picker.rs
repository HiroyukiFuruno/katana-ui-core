use super::{
    CLICK_OFFSET, StorybookLiveInteractionScenario, component_body_pixel_diff, page_state,
    render_state, scenario,
};
use crate::visual::preview_detail;
use crate::visual::window_interaction::{
    apply_click, apply_hover_at, focus_clickable_at_for_audit,
};

const PAGE: &str = "color-picker-rgba";
const ALPHA_PRESET_INDEX: usize = 4;
const EYEDROPPER_PRESET_INDEX: usize = 12;

pub(super) fn scenarios(page: &'static str) -> Vec<StorybookLiveInteractionScenario> {
    if page != PAGE {
        return Vec::new();
    }
    vec![
        color_picker_hover_scenario(),
        color_picker_focus_scenario(),
        color_picker_alpha_drag_scenario(),
        color_picker_eyedropper_scenario(),
    ]
}

fn color_picker_hover_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(PAGE);
    let before = render_state(PAGE, &state);
    let target = preview_detail::component_action_hit_rect(PAGE);
    let hovered = apply_hover_at(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(PAGE, &before, &after);
    let passed = hovered
        && state.screen_state.last_action == "color_picker_hover"
        && state.screen_state.last_event == "color_picker_hovered"
        && state.screen_state.state_label == "hover=true"
        && state.screen_state.preview_hovered
        && body_pixel_diff > 0;
    scenario(
        PAGE,
        "color_picker_hover",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn color_picker_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(PAGE);
    let before = render_state(PAGE, &state);
    let target = preview_detail::component_action_hit_rect(PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "color_picker_focus"
        && state.screen_state.last_event == "color_picker_focused"
        && state.screen_state.state_label == "focus=true"
        && state.screen_state.is_button_focused()
        && body_pixel_diff > 0;
    scenario(
        PAGE,
        "color_picker_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn color_picker_alpha_drag_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(PAGE);
    state.preset_index = ALPHA_PRESET_INDEX;
    let before = render_state(PAGE, &state);
    let target = preview_detail::component_action_hit_rect(PAGE);
    let dragged = apply_click(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(PAGE, &before, &after);
    let passed = dragged
        && state.screen_state.last_action == "color_alpha_drag"
        && state.screen_state.last_event == "alpha_changed"
        && state.screen_state.state_label == "color_picker.alpha=188"
        && body_pixel_diff > 0;
    scenario(
        PAGE,
        "color_picker_alpha_drag",
        "drag",
        dragged,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn color_picker_eyedropper_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(PAGE);
    state.preset_index = EYEDROPPER_PRESET_INDEX;
    let before = render_state(PAGE, &state);
    let target = preview_detail::component_action_hit_rect(PAGE);
    let clicked = apply_click(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(PAGE, &before, &after);
    let passed = clicked
        && state.screen_state.last_action == "color_eyedropper_request"
        && state.screen_state.last_event == "eyedropper_requested"
        && state.screen_state.state_label == "color_picker.eyedropper=storybook-eyedropper"
        && body_pixel_diff > 0;
    scenario(
        PAGE,
        "color_picker_eyedropper",
        "pointer",
        clicked,
        passed,
        body_pixel_diff,
        &state,
    )
}
