use super::{
    CLICK_OFFSET, StorybookLiveInteractionScenario, component_body_pixel_diff, page_state,
    render_state, scenario,
};
use crate::visual::preview_detail;
use crate::visual::window_interaction::{
    apply_clickable_keyboard_activation_for_audit, apply_hover_at, apply_list_scroll_for_audit,
    apply_selection_list_scroll_for_audit, focus_clickable_at_for_audit,
};

const PAGE: &str = "list";
const DYNAMIC_ARRAY_PAGE: &str = "dynamic-array-editor";
const SELECTION_LIST_PAGE: &str = "selection-list";

pub(super) fn scenarios(page: &'static str) -> Vec<StorybookLiveInteractionScenario> {
    match page {
        PAGE => vec![
            list_focus_scenario(),
            list_keyboard_scenario(),
            list_scroll_scenario(),
        ],
        DYNAMIC_ARRAY_PAGE => vec![
            dynamic_array_hover_scenario(),
            dynamic_array_focus_scenario(),
            dynamic_array_keyboard_scenario(),
        ],
        SELECTION_LIST_PAGE => vec![
            selection_list_hover_scenario(),
            selection_list_focus_scenario(),
            selection_list_keyboard_scenario(),
            selection_list_scroll_scenario(),
        ],
        _ => Vec::new(),
    }
}

fn selection_list_hover_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SELECTION_LIST_PAGE);
    let before = render_state(SELECTION_LIST_PAGE, &state);
    let target = preview_detail::component_action_hit_rect(SELECTION_LIST_PAGE);
    let hovered = apply_hover_at(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(SELECTION_LIST_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SELECTION_LIST_PAGE, &before, &after);
    let passed = hovered
        && state.screen_state.last_action == "selection_list_hover"
        && state.screen_state.last_event == "hover_start"
        && state.screen_state.state_label == "hover=true"
        && state.screen_state.selection.selection_list_hovered
        && body_pixel_diff > 0;
    scenario(
        SELECTION_LIST_PAGE,
        "selection_list_hover",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn selection_list_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SELECTION_LIST_PAGE);
    let before = render_state(SELECTION_LIST_PAGE, &state);
    let target = preview_detail::component_action_hit_rect(SELECTION_LIST_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(SELECTION_LIST_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SELECTION_LIST_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "selection_list_focus"
        && state.screen_state.last_event == "focus"
        && state.screen_state.state_label == "single=none multi=none focus=0"
        && state.screen_state.is_button_focused()
        && body_pixel_diff > 0;
    scenario(
        SELECTION_LIST_PAGE,
        "selection_list_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn selection_list_keyboard_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SELECTION_LIST_PAGE);
    let target = preview_detail::component_action_hit_rect(SELECTION_LIST_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before = render_state(SELECTION_LIST_PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(SELECTION_LIST_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SELECTION_LIST_PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "selection_list_keyboard_next"
        && state.screen_state.last_event == "set_selected_index"
        && state.screen_state.state_label == "single=1 multi=none focus=1"
        && body_pixel_diff > 0;
    scenario(
        SELECTION_LIST_PAGE,
        "selection_list_keyboard_next",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn selection_list_scroll_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SELECTION_LIST_PAGE);
    let before = render_state(SELECTION_LIST_PAGE, &state);
    let target = preview_detail::component_action_hit_rect(SELECTION_LIST_PAGE);
    let scrolled = apply_selection_list_scroll_for_audit(
        &mut state,
        target.x + CLICK_OFFSET,
        target.y + CLICK_OFFSET,
    );
    let after = render_state(SELECTION_LIST_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SELECTION_LIST_PAGE, &before, &after);
    let passed = scrolled
        && state.screen_state.last_action == "selection_list_scroll"
        && state.screen_state.last_event == "scroll_by"
        && state.screen_state.state_label == "scroll=1"
        && body_pixel_diff > 0;
    scenario(
        SELECTION_LIST_PAGE,
        "selection_list_scroll",
        "scroll",
        scrolled,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn list_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(PAGE);
    let before = render_state(PAGE, &state);
    let target = preview_detail::component_action_hit_rect(PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "list_focus"
        && state.screen_state.last_event == "list_focused"
        && state.screen_state.state_label == "focused=1"
        && state.screen_state.is_button_focused()
        && body_pixel_diff > 0;
    scenario(
        PAGE,
        "list_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn list_keyboard_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(PAGE);
    let target = preview_detail::component_action_hit_rect(PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before = render_state(PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "list_keyboard_next"
        && state.screen_state.last_event == "selection_changed"
        && state.screen_state.state_label == "selected=2"
        && body_pixel_diff > 0;
    scenario(
        PAGE,
        "list_keyboard_next",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn list_scroll_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(PAGE);
    let before = render_state(PAGE, &state);
    let target = preview_detail::component_action_hit_rect(PAGE);
    let scrolled =
        apply_list_scroll_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(PAGE, &before, &after);
    let passed = scrolled
        && state.screen_state.last_action == "list_scroll"
        && state.screen_state.last_event == "list_virtual_range_changed"
        && state.screen_state.state_label == "virtual=48/200"
        && body_pixel_diff > 0;
    scenario(
        PAGE,
        "list_scroll",
        "scroll",
        scrolled,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn dynamic_array_hover_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(DYNAMIC_ARRAY_PAGE);
    let before = render_state(DYNAMIC_ARRAY_PAGE, &state);
    let target = preview_detail::component_action_hit_rect(DYNAMIC_ARRAY_PAGE);
    let hovered = apply_hover_at(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(DYNAMIC_ARRAY_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(DYNAMIC_ARRAY_PAGE, &before, &after);
    let passed = hovered
        && state.screen_state.last_action == "array_hover"
        && state.screen_state.last_event == "array_hovered"
        && state.screen_state.state_label == "hover=true"
        && state.screen_state.preview_hovered
        && body_pixel_diff > 0;
    scenario(
        DYNAMIC_ARRAY_PAGE,
        "dynamic_array_hover",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn dynamic_array_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(DYNAMIC_ARRAY_PAGE);
    let before = render_state(DYNAMIC_ARRAY_PAGE, &state);
    let target = preview_detail::component_action_hit_rect(DYNAMIC_ARRAY_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(DYNAMIC_ARRAY_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(DYNAMIC_ARRAY_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "array_focus"
        && state.screen_state.last_event == "array_focused"
        && state.screen_state.state_label == "focus=true"
        && state.screen_state.is_button_focused()
        && body_pixel_diff > 0;
    scenario(
        DYNAMIC_ARRAY_PAGE,
        "dynamic_array_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn dynamic_array_keyboard_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(DYNAMIC_ARRAY_PAGE);
    let target = preview_detail::component_action_hit_rect(DYNAMIC_ARRAY_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before = render_state(DYNAMIC_ARRAY_PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(DYNAMIC_ARRAY_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(DYNAMIC_ARRAY_PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "array_keyboard_edit"
        && state.screen_state.last_event == "array_changed"
        && state.screen_state.state_label == "edited=row-1"
        && body_pixel_diff > 0;
    scenario(
        DYNAMIC_ARRAY_PAGE,
        "dynamic_array_keyboard_edit",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}
