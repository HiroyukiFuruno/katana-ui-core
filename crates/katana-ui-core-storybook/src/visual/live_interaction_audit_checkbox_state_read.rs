use crate::visual::{
    Canvas, dedicated_dod_form_binary_choice_live, preview_detail, render,
    window_interaction::apply_click,
};

use super::{StorybookLiveInteractionScenario, page_state, scenario};

const CHECKBOX_PAGE: &str = "checkbox";
const CLICK_OFFSET: usize = 4;
const PREVIEW_RIGHT_EDGE: usize = 1020;

pub(super) fn scenarios(page: &'static str) -> Vec<StorybookLiveInteractionScenario> {
    if page != CHECKBOX_PAGE {
        return Vec::new();
    }
    vec![
        checkbox_state_read_preserves_state_metadata_scenario(
            1,
            "checkbox_checked_state_read_preserves_checked_state_metadata",
            "checked=true",
        ),
        checkbox_disabled_state_read_is_blocked_scenario(),
        checkbox_state_read_preserves_state_metadata_scenario(
            3,
            "checkbox_focus_state_read_preserves_focus_state_metadata",
            "focused=true",
        ),
    ]
}

fn checkbox_disabled_state_read_is_blocked_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(CHECKBOX_PAGE);
    state.select_preset(2);
    let component = preview_detail::component_action_hit_rect(CHECKBOX_PAGE);
    let read = dedicated_dod_form_binary_choice_live::checkbox_state_read_button_rect(
        component.x,
        component.y,
    );
    let clicked = apply_click(&mut state, read.x + CLICK_OFFSET, read.y + CLICK_OFFSET);
    let rendered = render::render_storybook_canvas_with_screen_state(
        "dark",
        CHECKBOX_PAGE,
        state.preset_index,
        state.screen_state.clone(),
    );
    let passed = clicked
        && state.screen_state.action_count == 0
        && state.screen_state.last_action == "none"
        && state.screen_state.last_event == "none"
        && state.screen_state.state_label == "disabled=true"
        && has_preview_text(&rendered, "disabled=true")
        && has_inspector_text(&rendered, "screen: disabled=true");
    scenario(
        CHECKBOX_PAGE,
        "checkbox_disabled_state_read_control_is_blocked",
        "pointer",
        clicked,
        passed,
        0,
        &state,
    )
}

fn checkbox_state_read_preserves_state_metadata_scenario(
    preset_index: usize,
    operation: &'static str,
    expected_state: &'static str,
) -> StorybookLiveInteractionScenario {
    let mut state = page_state(CHECKBOX_PAGE);
    state.select_preset(preset_index);
    let component = preview_detail::component_action_hit_rect(CHECKBOX_PAGE);
    let read = dedicated_dod_form_binary_choice_live::checkbox_state_read_button_rect(
        component.x,
        component.y,
    );
    let clicked = apply_click(&mut state, read.x + CLICK_OFFSET, read.y + CLICK_OFFSET);
    let rendered = render::render_storybook_canvas_with_screen_state(
        "dark",
        CHECKBOX_PAGE,
        state.preset_index,
        state.screen_state.clone(),
    );
    let passed = clicked
        && state.screen_state.last_action == "checkbox_state_read"
        && state.screen_state.last_event == "checked_read"
        && state.screen_state.state_label == expected_state
        && has_preview_text(&rendered, expected_state)
        && has_inspector_text(&rendered, &format!("screen: {expected_state}"));
    scenario(
        CHECKBOX_PAGE,
        operation,
        "pointer",
        clicked,
        passed,
        0,
        &state,
    )
}

fn has_preview_text(canvas: &Canvas, text: &str) -> bool {
    canvas
        .text_runs()
        .iter()
        .any(|run| run.text() == text && run.x() < PREVIEW_RIGHT_EDGE)
}

fn has_inspector_text(canvas: &Canvas, text: &str) -> bool {
    canvas
        .text_runs()
        .iter()
        .any(|run| run.text() == text && run.x() > PREVIEW_RIGHT_EDGE)
}
