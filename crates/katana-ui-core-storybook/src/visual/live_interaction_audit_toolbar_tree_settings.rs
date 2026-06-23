use super::{
    CLICK_OFFSET, SETTINGS_LIST_PAGE, StorybookLiveInteractionScenario, TREE_VIEW_PAGE,
    apply_clickable_keyboard_activation_for_audit, apply_hover_at,
    apply_settings_list_scroll_for_audit, apply_tree_view_scroll_for_audit,
    component_body_pixel_diff, focus_clickable_at_for_audit, page_state, render_state, scenario,
};

pub(super) fn tree_view_hover_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(TREE_VIEW_PAGE);
    let before = render_state(TREE_VIEW_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(TREE_VIEW_PAGE);
    let hovered = apply_hover_at(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(TREE_VIEW_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(TREE_VIEW_PAGE, &before, &after);
    let passed = hovered
        && state.screen_state.last_action == "tree_hover_item"
        && state.screen_state.last_event == "hover_start"
        && state.screen_state.state_label == "hover=katana/a.md"
        && state.screen_state.preview_hovered
        && body_pixel_diff > 0;
    scenario(
        TREE_VIEW_PAGE,
        "tree_hover_item",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn tree_view_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(TREE_VIEW_PAGE);
    let before = render_state(TREE_VIEW_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(TREE_VIEW_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(TREE_VIEW_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(TREE_VIEW_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "tree_focus_item"
        && state.screen_state.last_event == "tree_item_focused"
        && state.screen_state.state_label == "focus=katana/a.md"
        && state.screen_state.is_button_focused()
        && body_pixel_diff > 0;
    scenario(
        TREE_VIEW_PAGE,
        "tree_focus_item",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn tree_view_keyboard_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(TREE_VIEW_PAGE);
    let target = crate::visual::preview_detail::component_action_hit_rect(TREE_VIEW_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before = render_state(TREE_VIEW_PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(TREE_VIEW_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(TREE_VIEW_PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "tree_keyboard_select"
        && state.screen_state.last_event == "tree_selected"
        && state.screen_state.state_label == "selected=katana/a.md"
        && body_pixel_diff > 0;
    scenario(
        TREE_VIEW_PAGE,
        "tree_keyboard_select",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn tree_view_scroll_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(TREE_VIEW_PAGE);
    let before = render_state(TREE_VIEW_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(TREE_VIEW_PAGE);
    let scrolled = apply_tree_view_scroll_for_audit(
        &mut state,
        target.x + CLICK_OFFSET,
        target.y + CLICK_OFFSET,
    );
    let after = render_state(TREE_VIEW_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(TREE_VIEW_PAGE, &before, &after);
    let passed = scrolled
        && state.screen_state.last_action == "tree_scroll_retained"
        && state.screen_state.last_event == "tree_scroll_offset_kept"
        && state.screen_state.state_label == "scroll=retained"
        && body_pixel_diff > 0;
    scenario(
        TREE_VIEW_PAGE,
        "tree_scroll_retained",
        "scroll",
        scrolled,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn settings_list_hover_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SETTINGS_LIST_PAGE);
    let before = render_state(SETTINGS_LIST_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(SETTINGS_LIST_PAGE);
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

pub(super) fn settings_list_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SETTINGS_LIST_PAGE);
    let before = render_state(SETTINGS_LIST_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(SETTINGS_LIST_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(SETTINGS_LIST_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SETTINGS_LIST_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "settings_focus_field"
        && state.screen_state.last_event == "settings_field_focused"
        && state.screen_state.state_label == "focus=app.font-size"
        && state.screen_state.is_button_focused()
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

pub(super) fn settings_list_keyboard_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SETTINGS_LIST_PAGE);
    let target = crate::visual::preview_detail::component_action_hit_rect(SETTINGS_LIST_PAGE);
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

pub(super) fn settings_list_scroll_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SETTINGS_LIST_PAGE);
    let before = render_state(SETTINGS_LIST_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(SETTINGS_LIST_PAGE);
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
