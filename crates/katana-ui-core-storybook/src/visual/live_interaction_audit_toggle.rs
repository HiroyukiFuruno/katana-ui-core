use super::{
    CLICK_OFFSET, StorybookLiveInteractionScenario, component_body_pixel_diff, page_state,
    render_state, scenario,
};
use crate::visual::preview_detail;
use crate::visual::screen_state_segmented_toggle::SegmentedToggleScreenAction;
use crate::visual::window_interaction::{
    apply_click, apply_clickable_keyboard_activation_for_audit, apply_hover_at,
    focus_clickable_at_for_audit,
};

const TOGGLE_PAGE: &str = "toggle";
const SEGMENTED_TOGGLE_PAGE: &str = "segmented-toggle";

pub(super) fn scenarios(page: &'static str) -> Vec<StorybookLiveInteractionScenario> {
    if page == TOGGLE_PAGE {
        return vec![
            toggle_focus_scenario(),
            toggle_keyboard_scenario(),
            toggle_on_preset_first_click_returns_off_scenario(),
        ];
    }
    if page == SEGMENTED_TOGGLE_PAGE {
        return vec![
            segmented_toggle_hover_scenario(),
            segmented_toggle_focus_scenario(),
            segmented_toggle_keyboard_scenario(),
            segmented_toggle_disabled_select_scenario(),
        ];
    }
    Vec::new()
}

fn segmented_toggle_hover_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SEGMENTED_TOGGLE_PAGE);
    let before = render_state(SEGMENTED_TOGGLE_PAGE, &state);
    let target = preview_detail::component_action_hit_rect(SEGMENTED_TOGGLE_PAGE);
    let hovered = apply_hover_at(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(SEGMENTED_TOGGLE_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SEGMENTED_TOGGLE_PAGE, &before, &after);
    let passed = hovered
        && state.screen_state.last_action == "segment_hover"
        && state.screen_state.last_event == "hover_start"
        && state.screen_state.state_label == "hover=true"
        && state.screen_state.segmented_toggle.hovered
        && body_pixel_diff > 0;
    scenario(
        SEGMENTED_TOGGLE_PAGE,
        "segmented_toggle_hover",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn segmented_toggle_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SEGMENTED_TOGGLE_PAGE);
    let before = render_state(SEGMENTED_TOGGLE_PAGE, &state);
    let target = preview_detail::component_action_hit_rect(SEGMENTED_TOGGLE_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(SEGMENTED_TOGGLE_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SEGMENTED_TOGGLE_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "segment_focus"
        && state.screen_state.last_event == "focus"
        && state.screen_state.state_label == "focus=true"
        && state.screen_state.is_button_focused()
        && state.screen_state.segmented_toggle.focused
        && body_pixel_diff > 0;
    scenario(
        SEGMENTED_TOGGLE_PAGE,
        "segmented_toggle_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn segmented_toggle_keyboard_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SEGMENTED_TOGGLE_PAGE);
    let target = preview_detail::component_action_hit_rect(SEGMENTED_TOGGLE_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before = render_state(SEGMENTED_TOGGLE_PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(SEGMENTED_TOGGLE_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SEGMENTED_TOGGLE_PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "segment_keyboard_select"
        && state.screen_state.last_event == "segmented_toggle_selected"
        && state.screen_state.state_label == "segment=1"
        && state.screen_state.segmented_toggle.selected_index == 1
        && body_pixel_diff > 0;
    scenario(
        SEGMENTED_TOGGLE_PAGE,
        "segmented_toggle_keyboard_select",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn segmented_toggle_disabled_select_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SEGMENTED_TOGGLE_PAGE);
    let before = render_state(SEGMENTED_TOGGLE_PAGE, &state);
    state
        .screen_state
        .register_segmented_toggle_action(SegmentedToggleScreenAction::DisabledSelect);
    let after = render_state(SEGMENTED_TOGGLE_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SEGMENTED_TOGGLE_PAGE, &before, &after);
    let passed = state.screen_state.last_action == "segment_disabled_select"
        && state.screen_state.last_event == "segmented_toggle_ignored"
        && state.screen_state.state_label == "disabled=true"
        && state.screen_state.segmented_toggle.disabled_blocked;
    scenario(
        SEGMENTED_TOGGLE_PAGE,
        "segmented_toggle_disabled_select",
        "click",
        true,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn toggle_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(TOGGLE_PAGE);
    let before = render_state(TOGGLE_PAGE, &state);
    let target = preview_detail::component_action_hit_rect(TOGGLE_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(TOGGLE_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(TOGGLE_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.is_button_focused()
        && state.screen_state.last_action == "toggle_focus"
        && state.screen_state.last_event == "toggle_focused"
        && body_pixel_diff > 0;
    scenario(
        TOGGLE_PAGE,
        "toggle_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn toggle_keyboard_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(TOGGLE_PAGE);
    let target = preview_detail::component_action_hit_rect(TOGGLE_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before = render_state(TOGGLE_PAGE, &state);
    let typed = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(TOGGLE_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(TOGGLE_PAGE, &before, &after);
    let passed = focused
        && typed
        && state.screen_state.last_action == "toggle_keyboard_toggle"
        && state.screen_state.last_event == "toggle_changed"
        && state.screen_state.state_label == "checked=true"
        && body_pixel_diff > 0;
    scenario(
        TOGGLE_PAGE,
        "toggle_keyboard_toggle",
        "keyboard",
        typed,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn toggle_on_preset_first_click_returns_off_scenario() -> StorybookLiveInteractionScenario {
    const ON_PRESET_INDEX: usize = 1;
    let mut state = page_state(TOGGLE_PAGE);
    state.select_preset(ON_PRESET_INDEX);
    let before = render_state(TOGGLE_PAGE, &state);
    let target = preview_detail::component_action_hit_rect(TOGGLE_PAGE);
    let clicked = apply_click(
        &mut state,
        target.x + target.width / 2,
        target.y + target.height / 2,
    );
    let after = render_state(TOGGLE_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(TOGGLE_PAGE, &before, &after);
    let passed = clicked
        && state.screen_state.last_action == "toggle_change"
        && state.screen_state.last_event == "toggle_changed"
        && state.screen_state.state_label == "checked=false"
        && !state.screen_state.toggle_checked
        && body_pixel_diff > 0;
    scenario(
        TOGGLE_PAGE,
        "toggle_on_preset_first_click_returns_off",
        "pointer",
        clicked,
        passed,
        body_pixel_diff,
        &state,
    )
}
