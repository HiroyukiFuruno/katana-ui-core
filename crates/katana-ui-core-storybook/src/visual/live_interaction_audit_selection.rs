use super::{
    CLICK_OFFSET, StorybookLiveInteractionScenario, component_body_pixel_diff, page_state,
    render_state, scenario,
};
use crate::visual::preview_detail;
use crate::visual::selection_screen_state::SelectionScreenAction;
use crate::visual::window_interaction::{
    apply_clickable_keyboard_activation_for_audit, apply_hover_at, apply_select_scroll_for_audit,
    focus_clickable_at_for_audit,
};

const SELECT_BOX_PAGE: &str = "select-box";
const SELECTION_LIST_PAGE: &str = "selection-list";

pub(super) fn scenarios(page: &'static str) -> Vec<StorybookLiveInteractionScenario> {
    if page == SELECT_BOX_PAGE {
        return vec![
            select_hover_scenario(),
            select_focus_scenario(),
            select_keyboard_scenario(),
            select_scroll_scenario(),
        ];
    }
    if page == SELECTION_LIST_PAGE {
        return vec![
            selection_list_hover_scenario(),
            selection_list_focus_scenario(),
            selection_list_scroll_scenario(),
        ];
    }
    Vec::new()
}

fn select_hover_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SELECT_BOX_PAGE);
    let before = render_state(SELECT_BOX_PAGE, &state);
    let target = preview_detail::component_action_hit_rect(SELECT_BOX_PAGE);
    let hovered = apply_hover_at(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
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
        "select_hover",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn selection_list_hover_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SELECTION_LIST_PAGE);
    let before = render_state(SELECTION_LIST_PAGE, &state);
    state
        .screen_state
        .register_selection_action(SelectionScreenAction::SelectionListHover);
    let after = render_state(SELECTION_LIST_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SELECTION_LIST_PAGE, &before, &after);
    let passed = state.screen_state.last_action == "selection_list_hover"
        && state.screen_state.last_event == "hover_start"
        && state.screen_state.state_label == "hover=true"
        && state.screen_state.selection.selection_list_hovered
        && body_pixel_diff > 0;
    scenario(
        SELECTION_LIST_PAGE,
        "selection_list_hover",
        "hover",
        true,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn selection_list_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SELECTION_LIST_PAGE);
    let before = render_state(SELECTION_LIST_PAGE, &state);
    state
        .screen_state
        .register_selection_action(SelectionScreenAction::SelectionListFocus);
    let after = render_state(SELECTION_LIST_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SELECTION_LIST_PAGE, &before, &after);
    let passed = state.screen_state.last_action == "selection_list_focus"
        && state.screen_state.last_event == "focus"
        && state.screen_state.selection.selection_list_focus_index == Some(0)
        && body_pixel_diff > 0;
    scenario(
        SELECTION_LIST_PAGE,
        "selection_list_focus",
        "focus",
        true,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn selection_list_scroll_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SELECTION_LIST_PAGE);
    let before = render_state(SELECTION_LIST_PAGE, &state);
    state
        .screen_state
        .register_selection_action(SelectionScreenAction::SelectionListScroll);
    let after = render_state(SELECTION_LIST_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SELECTION_LIST_PAGE, &before, &after);
    let passed = state.screen_state.last_action == "selection_list_scroll"
        && state.screen_state.last_event == "scroll_by"
        && state.screen_state.state_label == "scroll=1"
        && state.screen_state.selection.selection_list_scroll_offset == 1
        && body_pixel_diff > 0;
    scenario(
        SELECTION_LIST_PAGE,
        "selection_list_scroll",
        "scroll",
        true,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn select_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SELECT_BOX_PAGE);
    let before = render_state(SELECT_BOX_PAGE, &state);
    let target = preview_detail::component_action_hit_rect(SELECT_BOX_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(SELECT_BOX_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SELECT_BOX_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "select_focus"
        && state.screen_state.last_event == "focus"
        && state.screen_state.state_label == "focus=true"
        && state.screen_state.selection.select_focused
        && state.screen_state.is_button_focused()
        && body_pixel_diff > 0;
    scenario(
        SELECT_BOX_PAGE,
        "select_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn select_keyboard_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SELECT_BOX_PAGE);
    let target = preview_detail::component_action_hit_rect(SELECT_BOX_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
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
        "select_keyboard_select",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn select_scroll_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SELECT_BOX_PAGE);
    let before = render_state(SELECT_BOX_PAGE, &state);
    let target = preview_detail::component_action_hit_rect(SELECT_BOX_PAGE);
    let scrolled =
        apply_select_scroll_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(SELECT_BOX_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SELECT_BOX_PAGE, &before, &after);
    let passed = scrolled
        && state.screen_state.last_action == "select_option_scroll"
        && state.screen_state.last_event == "select_options_scrolled"
        && state.screen_state.state_label == "scroll=1"
        && state.screen_state.selection.select_open
        && state.screen_state.selection.select_scroll_offset == 1
        && body_pixel_diff > 0;
    scenario(
        SELECT_BOX_PAGE,
        "select_option_scroll",
        "scroll",
        scrolled,
        passed,
        body_pixel_diff,
        &state,
    )
}
