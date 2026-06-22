use super::{
    CLICK_OFFSET, StorybookLiveInteractionScenario, component_body_pixel_diff, page_state,
    render_state, scenario,
};
use crate::visual::preview_detail;
use crate::visual::window_interaction::{
    apply_clickable_keyboard_activation_for_audit, apply_hover_at,
    apply_settings_list_scroll_for_audit, apply_side_menu_scroll_for_audit,
    focus_clickable_at_for_audit,
};

const SETTINGS_LIST_PAGE: &str = "settings-list";
const SIDE_MENU_PAGE: &str = "side-menu";

pub(super) fn scenarios(page: &'static str) -> Vec<StorybookLiveInteractionScenario> {
    match page {
        SETTINGS_LIST_PAGE => vec![
            settings_list_hover_scenario(),
            settings_list_focus_scenario(),
            settings_list_keyboard_scenario(),
            settings_list_scroll_scenario(),
        ],
        SIDE_MENU_PAGE => vec![
            side_menu_hover_scenario(),
            side_menu_focus_scenario(),
            side_menu_keyboard_scenario(),
            side_menu_scroll_scenario(),
        ],
        _ => Vec::new(),
    }
}

fn settings_list_hover_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SETTINGS_LIST_PAGE);
    let before = render_state(SETTINGS_LIST_PAGE, &state);
    let target = preview_detail::component_action_hit_rect(SETTINGS_LIST_PAGE);
    let hovered = apply_hover_at(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(SETTINGS_LIST_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SETTINGS_LIST_PAGE, &before, &after);
    let passed = hovered
        && state.screen_state.last_action == "settings_hover_field"
        && state.screen_state.last_event == "hover_start"
        && state.screen_state.state_label == "hover=app.font-size"
        && state.screen_state.settings_list.hovered
        && body_pixel_diff > 0;
    scenario(
        SETTINGS_LIST_PAGE,
        "settings_list_hover",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn settings_list_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SETTINGS_LIST_PAGE);
    let before = render_state(SETTINGS_LIST_PAGE, &state);
    let target = preview_detail::component_action_hit_rect(SETTINGS_LIST_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(SETTINGS_LIST_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SETTINGS_LIST_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "settings_focus_field"
        && state.screen_state.last_event == "settings_field_focused"
        && state.screen_state.state_label == "focus=app.font-size"
        && state.screen_state.is_button_focused()
        && state.screen_state.settings_list.focused
        && body_pixel_diff > 0;
    scenario(
        SETTINGS_LIST_PAGE,
        "settings_list_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn settings_list_keyboard_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SETTINGS_LIST_PAGE);
    let target = preview_detail::component_action_hit_rect(SETTINGS_LIST_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before = render_state(SETTINGS_LIST_PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(SETTINGS_LIST_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SETTINGS_LIST_PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "settings_keyboard_next"
        && state.screen_state.last_event == "settings_field_focused"
        && state.screen_state.state_label == "focus=next"
        && state.screen_state.settings_list.focused
        && body_pixel_diff > 0;
    scenario(
        SETTINGS_LIST_PAGE,
        "settings_list_keyboard_next",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn settings_list_scroll_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SETTINGS_LIST_PAGE);
    let before = render_state(SETTINGS_LIST_PAGE, &state);
    let target = preview_detail::component_action_hit_rect(SETTINGS_LIST_PAGE);
    let scrolled = apply_settings_list_scroll_for_audit(
        &mut state,
        target.x + CLICK_OFFSET,
        target.y + CLICK_OFFSET,
    );
    let after = render_state(SETTINGS_LIST_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SETTINGS_LIST_PAGE, &before, &after);
    let passed = scrolled
        && state.screen_state.last_action == "settings_scroll"
        && state.screen_state.last_event == "scroll_by"
        && state.screen_state.state_label == "scroll=1"
        && state.screen_state.settings_list.scroll_offset == 1
        && body_pixel_diff > 0;
    scenario(
        SETTINGS_LIST_PAGE,
        "settings_list_scroll",
        "scroll",
        scrolled,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn side_menu_hover_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SIDE_MENU_PAGE);
    let before = render_state(SIDE_MENU_PAGE, &state);
    let target = preview_detail::component_action_hit_rect(SIDE_MENU_PAGE);
    let hovered = apply_hover_at(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(SIDE_MENU_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SIDE_MENU_PAGE, &before, &after);
    let passed = hovered
        && state.screen_state.last_action == "side_menu_hover"
        && state.screen_state.last_event == "hover_start"
        && state.screen_state.state_label == "hover=true"
        && state.screen_state.side_menu.hovered
        && body_pixel_diff > 0;
    scenario(
        SIDE_MENU_PAGE,
        "side_menu_hover",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn side_menu_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SIDE_MENU_PAGE);
    let before = render_state(SIDE_MENU_PAGE, &state);
    let target = preview_detail::component_action_hit_rect(SIDE_MENU_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(SIDE_MENU_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SIDE_MENU_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "side_menu_focus"
        && state.screen_state.last_event == "focus"
        && state.screen_state.state_label == "route=none focus=0"
        && state.screen_state.is_button_focused()
        && body_pixel_diff > 0;
    scenario(
        SIDE_MENU_PAGE,
        "side_menu_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn side_menu_keyboard_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SIDE_MENU_PAGE);
    let target = preview_detail::component_action_hit_rect(SIDE_MENU_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before = render_state(SIDE_MENU_PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(SIDE_MENU_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SIDE_MENU_PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "side_menu_keyboard_next"
        && state.screen_state.last_event == "set_selected_index"
        && state.screen_state.state_label == "route=1 focus=1"
        && body_pixel_diff > 0;
    scenario(
        SIDE_MENU_PAGE,
        "side_menu_keyboard_next",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn side_menu_scroll_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SIDE_MENU_PAGE);
    let before = render_state(SIDE_MENU_PAGE, &state);
    let target = preview_detail::component_action_hit_rect(SIDE_MENU_PAGE);
    let scrolled = apply_side_menu_scroll_for_audit(
        &mut state,
        target.x + CLICK_OFFSET,
        target.y + CLICK_OFFSET,
    );
    let after = render_state(SIDE_MENU_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SIDE_MENU_PAGE, &before, &after);
    let passed = scrolled
        && state.screen_state.last_action == "side_menu_scroll"
        && state.screen_state.last_event == "scroll_by"
        && state.screen_state.state_label == "scroll=1"
        && body_pixel_diff > 0;
    scenario(
        SIDE_MENU_PAGE,
        "side_menu_scroll",
        "scroll",
        scrolled,
        passed,
        body_pixel_diff,
        &state,
    )
}
