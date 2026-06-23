use super::{
    CLICK_OFFSET, StorybookLiveInteractionScenario, component_body_pixel_diff, page_state,
    render_state, scenario,
};
use crate::visual::layout_metrics::LayoutRect;
use crate::visual::preview_detail;
use crate::visual::selection_control_metrics;
use crate::visual::window_interaction::{
    apply_clickable_keyboard_activation_for_audit, apply_hover_at, apply_select_scroll_for_audit,
    focus_clickable_at_for_audit,
};

const COMBO_BOX_PAGE: &str = "combo-box";
const SELECT_BOX_PAGE: &str = "select-box";

pub(super) fn scenarios(page: &'static str) -> Vec<StorybookLiveInteractionScenario> {
    match page {
        COMBO_BOX_PAGE => vec![
            combo_box_hover_scenario(),
            combo_box_focus_scenario(),
            combo_box_keyboard_scenario(),
        ],
        SELECT_BOX_PAGE => vec![
            select_box_hover_scenario(),
            select_box_focus_scenario(),
            select_box_keyboard_scenario(),
            select_box_scroll_scenario(),
        ],
        _ => Vec::new(),
    }
}

fn select_box_hover_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SELECT_BOX_PAGE);
    let before = render_state(SELECT_BOX_PAGE, &state);
    let field = select_box_field();
    let hovered = apply_hover_at(&mut state, field.x + CLICK_OFFSET, field.y + CLICK_OFFSET);
    let after = render_state(SELECT_BOX_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SELECT_BOX_PAGE, &before, &after);
    let passed = hovered
        && state.screen_state.last_action == "select_hover"
        && state.screen_state.last_event == "hover_start"
        && state.screen_state.state_label == "hover=true"
        && state.screen_state.selection.select_hovered
        && body_pixel_diff > 0;
    scenario(
        SELECT_BOX_PAGE,
        "select_box_hover",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn select_box_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SELECT_BOX_PAGE);
    let before = render_state(SELECT_BOX_PAGE, &state);
    let field = select_box_field();
    let focused =
        focus_clickable_at_for_audit(&mut state, field.x + CLICK_OFFSET, field.y + CLICK_OFFSET);
    let after = render_state(SELECT_BOX_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SELECT_BOX_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "select_focus"
        && state.screen_state.last_event == "focus"
        && state.screen_state.state_label == "focus=true"
        && state.screen_state.is_button_focused()
        && state.screen_state.selection.select_focused
        && body_pixel_diff > 0;
    scenario(
        SELECT_BOX_PAGE,
        "select_box_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn select_box_keyboard_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SELECT_BOX_PAGE);
    let field = select_box_field();
    let focused =
        focus_clickable_at_for_audit(&mut state, field.x + CLICK_OFFSET, field.y + CLICK_OFFSET);
    let before = render_state(SELECT_BOX_PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(SELECT_BOX_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SELECT_BOX_PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "select_keyboard_select"
        && state.screen_state.last_event == "select_box_selected"
        && state.screen_state.state_label == "selected=light"
        && state.screen_state.selection.select_selected_index == Some(1)
        && body_pixel_diff > 0;
    scenario(
        SELECT_BOX_PAGE,
        "select_box_keyboard_select",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn select_box_scroll_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SELECT_BOX_PAGE);
    let before = render_state(SELECT_BOX_PAGE, &state);
    let field = select_box_field();
    let scrolled =
        apply_select_scroll_for_audit(&mut state, field.x + CLICK_OFFSET, field.y + CLICK_OFFSET);
    let after = render_state(SELECT_BOX_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SELECT_BOX_PAGE, &before, &after);
    let passed = scrolled
        && state.screen_state.last_action == "select_option_scroll"
        && state.screen_state.last_event == "select_options_scrolled"
        && state.screen_state.state_label == "scroll=1"
        && body_pixel_diff > 0;
    scenario(
        SELECT_BOX_PAGE,
        "select_box_option_scroll",
        "scroll",
        scrolled,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn combo_box_hover_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(COMBO_BOX_PAGE);
    let before = render_state(COMBO_BOX_PAGE, &state);
    let field = combo_box_field();
    let hovered = apply_hover_at(&mut state, field.x + CLICK_OFFSET, field.y + CLICK_OFFSET);
    let after = render_state(COMBO_BOX_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(COMBO_BOX_PAGE, &before, &after);
    let passed = hovered
        && state.screen_state.last_action == "combo_hover"
        && state.screen_state.last_event == "combo_hovered"
        && state.screen_state.state_label == "hover=true"
        && state.screen_state.preview_hovered
        && body_pixel_diff > 0;
    scenario(
        COMBO_BOX_PAGE,
        "combo_box_hover",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn combo_box_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(COMBO_BOX_PAGE);
    let before = render_state(COMBO_BOX_PAGE, &state);
    let field = combo_box_field();
    let focused =
        focus_clickable_at_for_audit(&mut state, field.x + CLICK_OFFSET, field.y + CLICK_OFFSET);
    let after = render_state(COMBO_BOX_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(COMBO_BOX_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "combo_focus"
        && state.screen_state.last_event == "combo_focused"
        && state.screen_state.state_label == "focus=true"
        && state.screen_state.is_button_focused()
        && body_pixel_diff > 0;
    scenario(
        COMBO_BOX_PAGE,
        "combo_box_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn combo_box_keyboard_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(COMBO_BOX_PAGE);
    let field = combo_box_field();
    let focused =
        focus_clickable_at_for_audit(&mut state, field.x + CLICK_OFFSET, field.y + CLICK_OFFSET);
    let before = render_state(COMBO_BOX_PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(COMBO_BOX_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(COMBO_BOX_PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "combo_keyboard_select"
        && state.screen_state.last_event == "combo_selected"
        && state.screen_state.state_label == "selected=two"
        && body_pixel_diff > 0;
    scenario(
        COMBO_BOX_PAGE,
        "combo_box_keyboard_select",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn combo_box_field() -> LayoutRect {
    let origin = preview_detail::component_action_hit_rect(COMBO_BOX_PAGE);
    selection_control_metrics::trigger_rect(origin)
}

fn select_box_field() -> LayoutRect {
    let origin = preview_detail::component_action_hit_rect(SELECT_BOX_PAGE);
    selection_control_metrics::trigger_rect(origin)
}
