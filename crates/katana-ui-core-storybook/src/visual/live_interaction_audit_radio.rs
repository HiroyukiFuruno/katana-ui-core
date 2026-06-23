use crate::visual::window_interaction::{
    apply_clickable_keyboard_activation_for_audit, focus_clickable_at_for_audit,
};
use crate::visual::{
    dedicated_dod_form_binary_choice_live, layout_metrics::LayoutRect, preview_detail,
};

use super::{
    StorybookLiveInteractionScenario, component_body_pixel_diff, page_state, render_state, scenario,
};

const RADIO_PAGE: &str = "radio";
const CLICK_OFFSET: usize = 4;

pub(super) fn scenarios(page: &'static str) -> Vec<StorybookLiveInteractionScenario> {
    if page != RADIO_PAGE {
        return Vec::new();
    }
    vec![radio_focus_scenario(), radio_keyboard_select_scenario()]
}

fn radio_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(RADIO_PAGE);
    let before = render_state(RADIO_PAGE, &state);
    let target = radio_focus_target();
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(RADIO_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(RADIO_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "radio_focus"
        && state.screen_state.last_event == "radio_focused"
        && state.screen_state.is_radio_focused()
        && body_pixel_diff > 0;
    scenario(
        RADIO_PAGE,
        "radio_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn radio_keyboard_select_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(RADIO_PAGE);
    let target = radio_focus_target();
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before = render_state(RADIO_PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(RADIO_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(RADIO_PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "radio_keyboard_select"
        && state.screen_state.last_event == "radio_selected"
        && state.screen_state.is_radio_selected()
        && body_pixel_diff > 0;
    scenario(
        RADIO_PAGE,
        "radio_keyboard_select",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn radio_focus_target() -> LayoutRect {
    let component = preview_detail::component_action_hit_rect(RADIO_PAGE);
    dedicated_dod_form_binary_choice_live::row_rect(0, component.x, component.y)
}
