use crate::visual::window_interaction::focus_clickable_at_for_audit;

use super::{
    StorybookLiveInteractionScenario, component_body_pixel_diff, page_state, render_state, scenario,
};

const PAGE: &str = "form-field";
const CLICK_OFFSET: usize = 4;

pub(super) fn scenarios(page: &'static str) -> Vec<StorybookLiveInteractionScenario> {
    if page != PAGE {
        return Vec::new();
    }
    vec![form_field_focus_scenario()]
}

fn form_field_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(PAGE);
    let before = render_state(PAGE, &state);
    let target = super::preview_detail::component_action_hit_rect(PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "form_field_focus_link"
        && state.screen_state.last_event == "form_field_control_focused"
        && state.screen_state.state_label == "focus=control"
        && body_pixel_diff > 0;
    scenario(
        PAGE,
        "form_field_focus_link",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}
