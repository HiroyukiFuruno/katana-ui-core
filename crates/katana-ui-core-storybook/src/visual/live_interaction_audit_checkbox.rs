use crate::visual::window_interaction::{
    apply_click, apply_clickable_keyboard_activation_for_audit, apply_hover_at,
    focus_clickable_at_for_audit,
};
use crate::visual::{
    Canvas, dedicated_dod_form_binary_choice_live, layout_metrics::LayoutRect, preview_detail,
};

use super::{
    StorybookLiveInteractionScenario, component_body_pixel_diff, page_state, render_state, scenario,
};

const CHECKBOX_PAGE: &str = "checkbox";
const CLICK_OFFSET: usize = 4;
const CHECKBOX_ACCENT: u32 = 0x569cd6;
const CHECKBOX_GLYPH: u32 = 0xf8fafc;

pub(super) fn scenarios(page: &'static str) -> Vec<StorybookLiveInteractionScenario> {
    if page != CHECKBOX_PAGE {
        return Vec::new();
    }
    vec![
        checkbox_focus_scenario(),
        checkbox_hover_no_click_event_scenario(),
        checkbox_hover_secondary_row_scenario(),
        checkbox_keyboard_toggle_scenario(),
        checkbox_keyboard_toggle_off_scenario(),
        checkbox_keyboard_focused_secondary_row_scenario(),
        checkbox_control_toggle_reset_scenario(),
    ]
}

fn checkbox_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(CHECKBOX_PAGE);
    let before = render_state(CHECKBOX_PAGE, &state);
    let target = checkbox_focus_target();
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(CHECKBOX_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(CHECKBOX_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "checkbox_focus"
        && state.screen_state.last_event == "checkbox_focused"
        && state.screen_state.is_checkbox_focused()
        && body_pixel_diff > 0;
    scenario(
        CHECKBOX_PAGE,
        "checkbox_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn checkbox_hover_no_click_event_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(CHECKBOX_PAGE);
    let before = render_state(CHECKBOX_PAGE, &state);
    let target = checkbox_focus_target();
    let hovered = apply_hover_at(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(CHECKBOX_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(CHECKBOX_PAGE, &before, &after);
    let passed = hovered
        && body_pixel_diff > 0
        && state.screen_state.action_count == 0
        && state.screen_state.last_action == "none"
        && state.screen_state.last_event == "none"
        && !state.screen_state.is_checkbox_checked();
    scenario(
        CHECKBOX_PAGE,
        "checkbox_hover_no_click_event",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn checkbox_hover_secondary_row_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(CHECKBOX_PAGE);
    let before = render_state(CHECKBOX_PAGE, &state);
    let target = checkbox_focus_target_at(1);
    let hovered = apply_hover_at(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(CHECKBOX_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(CHECKBOX_PAGE, &before, &after);
    let passed = hovered
        && body_pixel_diff > 0
        && state.screen_state.action_count == 0
        && state.screen_state.checkbox_hovered_index() == Some(1)
        && state.screen_state.last_action == "none"
        && state.screen_state.last_event == "none";
    scenario(
        CHECKBOX_PAGE,
        "checkbox_hover_secondary_row",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn checkbox_keyboard_toggle_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(CHECKBOX_PAGE);
    let target = checkbox_focus_target();
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before = render_state(CHECKBOX_PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(CHECKBOX_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(CHECKBOX_PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "checkbox_keyboard_toggle"
        && state.screen_state.last_event == "checked_changed"
        && state.screen_state.is_checkbox_checked()
        && body_pixel_diff > 0;
    scenario(
        CHECKBOX_PAGE,
        "checkbox_keyboard_toggle",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn checkbox_keyboard_toggle_off_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(CHECKBOX_PAGE);
    let target = checkbox_focus_target();
    let mark = checkbox_mark_target();
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let first_activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let checked = render_state(CHECKBOX_PAGE, &state);
    let checked_accent = count_color_in_rect(&checked, mark, CHECKBOX_ACCENT);
    let checked_glyph = count_color_in_rect(&checked, mark, CHECKBOX_GLYPH);
    let second_activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let unchecked = render_state(CHECKBOX_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(CHECKBOX_PAGE, &checked, &unchecked);
    let passed = focused
        && first_activated
        && second_activated
        && state.screen_state.last_action == "checkbox_keyboard_toggle"
        && state.screen_state.last_event == "checked_changed"
        && state.screen_state.state_label == "before=true after=false"
        && !state.screen_state.is_checkbox_checked()
        && checked_accent > 0
        && checked_glyph > 0
        && count_color_in_rect(&unchecked, mark, CHECKBOX_ACCENT) == 0
        && count_color_in_rect(&unchecked, mark, CHECKBOX_GLYPH) == 0
        && body_pixel_diff > 0;
    scenario(
        CHECKBOX_PAGE,
        "checkbox_keyboard_toggle_off",
        "keyboard",
        second_activated,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn checkbox_keyboard_focused_secondary_row_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(CHECKBOX_PAGE);
    let first_mark = checkbox_mark_target_at(0);
    let second_target = checkbox_focus_target_at(1);
    let second_mark = checkbox_mark_target_at(1);
    let focused = focus_clickable_at_for_audit(
        &mut state,
        second_target.x + CLICK_OFFSET,
        second_target.y + CLICK_OFFSET,
    );
    let before = render_state(CHECKBOX_PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(CHECKBOX_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(CHECKBOX_PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "checkbox_keyboard_toggle"
        && state.screen_state.last_event == "checked_changed"
        && !state.screen_state.is_checkbox_checked_at(0)
        && state.screen_state.is_checkbox_checked_at(1)
        && count_color_in_rect(&after, first_mark, CHECKBOX_ACCENT) == 0
        && count_color_in_rect(&after, first_mark, CHECKBOX_GLYPH) == 0
        && count_color_in_rect(&after, second_mark, CHECKBOX_ACCENT) > 0
        && count_color_in_rect(&after, second_mark, CHECKBOX_GLYPH) > 0
        && body_pixel_diff > 0;
    scenario(
        CHECKBOX_PAGE,
        "checkbox_keyboard_focused_secondary_row",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn checkbox_control_toggle_reset_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(CHECKBOX_PAGE);
    let mark = checkbox_mark_target();
    let toggle = checkbox_toggle_target();
    let reset = checkbox_reset_target();
    let before = render_state(CHECKBOX_PAGE, &state);
    let toggled = apply_click(&mut state, toggle.x + CLICK_OFFSET, toggle.y + CLICK_OFFSET);
    let checked = render_state(CHECKBOX_PAGE, &state);
    let checked_accent = count_color_in_rect(&checked, mark, CHECKBOX_ACCENT);
    let checked_glyph = count_color_in_rect(&checked, mark, CHECKBOX_GLYPH);
    let reset_clicked = apply_click(&mut state, reset.x + CLICK_OFFSET, reset.y + CLICK_OFFSET);
    let unchecked = render_state(CHECKBOX_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(CHECKBOX_PAGE, &before, &unchecked)
        + component_body_pixel_diff(CHECKBOX_PAGE, &checked, &unchecked);
    let passed = toggled
        && reset_clicked
        && state.screen_state.last_action == "checkbox_reset"
        && state.screen_state.last_event == "checked_changed"
        && state.screen_state.state_label == "before=true after=false"
        && !state.screen_state.is_checkbox_checked()
        && checked_accent > 0
        && checked_glyph > 0
        && count_color_in_rect(&unchecked, mark, CHECKBOX_ACCENT) == 0
        && count_color_in_rect(&unchecked, mark, CHECKBOX_GLYPH) == 0
        && body_pixel_diff > 0;
    scenario(
        CHECKBOX_PAGE,
        "checkbox_control_toggle_reset",
        "pointer",
        reset_clicked,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn checkbox_focus_target() -> LayoutRect {
    checkbox_focus_target_at(0)
}

fn checkbox_focus_target_at(index: usize) -> LayoutRect {
    let component = preview_detail::component_action_hit_rect(CHECKBOX_PAGE);
    dedicated_dod_form_binary_choice_live::row_rect(index, component.x, component.y)
}

fn checkbox_mark_target() -> LayoutRect {
    checkbox_mark_target_at(0)
}

fn checkbox_mark_target_at(index: usize) -> LayoutRect {
    let component = preview_detail::component_action_hit_rect(CHECKBOX_PAGE);
    dedicated_dod_form_binary_choice_live::checkbox_mark_rect(index, component.x, component.y)
}

fn checkbox_toggle_target() -> LayoutRect {
    let component = preview_detail::component_action_hit_rect(CHECKBOX_PAGE);
    dedicated_dod_form_binary_choice_live::checkbox_toggle_button_rect(component.x, component.y)
}

fn checkbox_reset_target() -> LayoutRect {
    let component = preview_detail::component_action_hit_rect(CHECKBOX_PAGE);
    dedicated_dod_form_binary_choice_live::checkbox_reset_button_rect(component.x, component.y)
}

fn count_color_in_rect(canvas: &Canvas, rect: LayoutRect, color: u32) -> usize {
    (rect.y..rect.bottom())
        .flat_map(|y| (rect.x..rect.right()).map(move |x| (x, y)))
        .filter(|(x, y)| pixel_at(canvas, *x, *y) == Some(color))
        .count()
}

fn pixel_at(canvas: &Canvas, x: usize, y: usize) -> Option<u32> {
    (x < canvas.width() && y < canvas.height())
        .then(|| y * canvas.width() + x)
        .and_then(|index| canvas.pixels().get(index))
        .copied()
}
