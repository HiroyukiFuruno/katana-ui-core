use super::{
    CLICK_OFFSET, SHORTCUT_CHEATSHEET_PAGE, StorybookLiveInteractionScenario,
    apply_clickable_keyboard_activation_for_audit, apply_hover_at,
    apply_shortcut_cheatsheet_scroll_for_audit, component_body_pixel_diff,
    focus_clickable_at_for_audit, page_state, render_state, scenario,
};

pub(super) fn shortcut_cheatsheet_hover_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SHORTCUT_CHEATSHEET_PAGE);
    let before = render_state(SHORTCUT_CHEATSHEET_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(SHORTCUT_CHEATSHEET_PAGE);
    let hovered = apply_hover_at(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(SHORTCUT_CHEATSHEET_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SHORTCUT_CHEATSHEET_PAGE, &before, &after);
    let passed = hovered
        && state.screen_state.last_action == "shortcut_cheatsheet_hover"
        && state.screen_state.last_event == "hover_start"
        && state.screen_state.state_label == "hover=true"
        && state.screen_state.shortcut_cheatsheet.hovered
        && body_pixel_diff > 0;
    scenario(
        SHORTCUT_CHEATSHEET_PAGE,
        "shortcut_cheatsheet_hover",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn shortcut_cheatsheet_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SHORTCUT_CHEATSHEET_PAGE);
    let before = render_state(SHORTCUT_CHEATSHEET_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(SHORTCUT_CHEATSHEET_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(SHORTCUT_CHEATSHEET_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SHORTCUT_CHEATSHEET_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "shortcut_cheatsheet_focus"
        && state.screen_state.last_event == "focus"
        && state.screen_state.state_label == "focus=true"
        && state.screen_state.is_button_focused()
        && body_pixel_diff > 0;
    scenario(
        SHORTCUT_CHEATSHEET_PAGE,
        "shortcut_cheatsheet_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn shortcut_cheatsheet_keyboard_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SHORTCUT_CHEATSHEET_PAGE);
    let target = crate::visual::preview_detail::component_action_hit_rect(SHORTCUT_CHEATSHEET_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before = render_state(SHORTCUT_CHEATSHEET_PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(SHORTCUT_CHEATSHEET_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SHORTCUT_CHEATSHEET_PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "shortcut_filter_select"
        && state.screen_state.last_event == "shortcut_selected"
        && state.screen_state.state_label == "selected=format"
        && body_pixel_diff > 0;
    scenario(
        SHORTCUT_CHEATSHEET_PAGE,
        "shortcut_cheatsheet_keyboard_select",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn shortcut_cheatsheet_scroll_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SHORTCUT_CHEATSHEET_PAGE);
    let before = render_state(SHORTCUT_CHEATSHEET_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(SHORTCUT_CHEATSHEET_PAGE);
    let scrolled = apply_shortcut_cheatsheet_scroll_for_audit(
        &mut state,
        target.x + CLICK_OFFSET,
        target.y + CLICK_OFFSET,
    );
    let after = render_state(SHORTCUT_CHEATSHEET_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SHORTCUT_CHEATSHEET_PAGE, &before, &after);
    let passed = scrolled
        && state.screen_state.last_action == "shortcut_cheatsheet_scroll"
        && state.screen_state.last_event == "scroll_by"
        && state.screen_state.state_label == "scroll=1"
        && body_pixel_diff > 0;
    scenario(
        SHORTCUT_CHEATSHEET_PAGE,
        "shortcut_cheatsheet_scroll",
        "scroll",
        scrolled,
        passed,
        body_pixel_diff,
        &state,
    )
}
