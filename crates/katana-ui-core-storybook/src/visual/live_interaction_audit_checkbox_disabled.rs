use crate::visual::window_interaction::{
    apply_click, apply_clickable_keyboard_activation_for_audit, focus_clickable_at_for_audit,
};
use crate::visual::{
    Canvas, StorybookVisual, dedicated_dod_form_binary_choice_live, layout_metrics::LayoutRect,
    preview_detail,
};

use super::{
    StorybookLiveInteractionScenario, component_body_pixel_diff, page_state, render_state, scenario,
};

const CHECKBOX_PAGE: &str = "checkbox";
const CLICK_OFFSET: usize = 4;
const DISABLED_PRESET: usize = 2;
const CHECKBOX_ACCENT: u32 = 0x569cd6;
const CHECKBOX_GLYPH: u32 = 0xf8fafc;
const MIN_CONTROL_BOTTOM_PADDING: usize = 8;

pub(super) fn scenarios(page: &'static str) -> Vec<StorybookLiveInteractionScenario> {
    if page != CHECKBOX_PAGE {
        return Vec::new();
    }
    vec![
        checkbox_disabled_focus_keyboard_block_scenario(),
        checkbox_disabled_pointer_block_scenario(),
        checkbox_no_runtime_overlay_over_controls_scenario(),
        checkbox_controls_bottom_padding_scenario(),
        checkbox_disabled_snapshot_click_block_scenario(),
    ]
}

fn checkbox_disabled_focus_keyboard_block_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(CHECKBOX_PAGE);
    state.select_preset(DISABLED_PRESET);
    let target = checkbox_focus_target();
    let mark = checkbox_mark_target();
    let before = render_state(CHECKBOX_PAGE, &state);
    let before_accent = count_color_in_rect(&before, mark, CHECKBOX_ACCENT);
    let before_glyph = count_color_in_rect(&before, mark, CHECKBOX_GLYPH);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(CHECKBOX_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(CHECKBOX_PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "checkbox_keyboard_blocked"
        && state.screen_state.last_event == "checkbox_keyboard_ignored"
        && state.screen_state.state_label == "disabled=true"
        && !state.screen_state.is_checkbox_checked()
        && !state.screen_state.is_checkbox_focused()
        && before_accent == count_color_in_rect(&after, mark, CHECKBOX_ACCENT)
        && before_glyph == count_color_in_rect(&after, mark, CHECKBOX_GLYPH);
    scenario(
        CHECKBOX_PAGE,
        "disabled_focus_keyboard_block",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn checkbox_disabled_pointer_block_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(CHECKBOX_PAGE);
    state.select_preset(DISABLED_PRESET);
    let target = checkbox_focus_target();
    let mark = checkbox_mark_target();
    let before = render_state(CHECKBOX_PAGE, &state);
    let before_accent = count_color_in_rect(&before, mark, CHECKBOX_ACCENT);
    let before_glyph = count_color_in_rect(&before, mark, CHECKBOX_GLYPH);
    let clicked = apply_click(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(CHECKBOX_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(CHECKBOX_PAGE, &before, &after);
    let passed = clicked
        && state.screen_state.action_count == 0
        && state.screen_state.last_action == "none"
        && state.screen_state.last_event == "none"
        && state.screen_state.state_label == "disabled=true"
        && !state.screen_state.is_checkbox_checked()
        && before_accent == count_color_in_rect(&after, mark, CHECKBOX_ACCENT)
        && before_glyph == count_color_in_rect(&after, mark, CHECKBOX_GLYPH);
    scenario(
        CHECKBOX_PAGE,
        "checkbox_disabled_pointer_block",
        "pointer",
        clicked,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn checkbox_no_runtime_overlay_over_controls_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(CHECKBOX_PAGE);
    let target = checkbox_focus_target();
    let clicked = apply_click(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(CHECKBOX_PAGE, &state);
    let passed = clicked
        && after
            .text_runs()
            .iter()
            .all(|run| run.text() != "clicked 1");
    scenario(
        CHECKBOX_PAGE,
        "checkbox_no_runtime_overlay_over_controls",
        "visual",
        clicked,
        passed,
        0,
        &state,
    )
}

fn checkbox_controls_bottom_padding_scenario() -> StorybookLiveInteractionScenario {
    let state = page_state(CHECKBOX_PAGE);
    let component = preview_detail::component_action_hit_rect(CHECKBOX_PAGE);
    let reset =
        dedicated_dod_form_binary_choice_live::checkbox_reset_button_rect(component.x, component.y);
    let passed = reset.bottom() + MIN_CONTROL_BOTTOM_PADDING <= component.bottom();
    scenario(
        CHECKBOX_PAGE,
        "checkbox_controls_bottom_padding",
        "visual",
        true,
        passed,
        0,
        &state,
    )
}

fn checkbox_disabled_snapshot_click_block_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(CHECKBOX_PAGE);
    state.select_preset(DISABLED_PRESET);
    let clicked = StorybookVisual.render_clicked_preset_with_scrollbar(
        "dark",
        CHECKBOX_PAGE,
        DISABLED_PRESET,
        0,
        true,
    );
    let mark = checkbox_mark_target();
    let passed = count_color_in_rect(&clicked, mark, CHECKBOX_GLYPH) == 0
        && clicked
            .text_runs()
            .iter()
            .any(|run| run.text() == "checked=false")
        && clicked
            .text_runs()
            .iter()
            .any(|run| run.text() == "count 0");
    scenario(
        CHECKBOX_PAGE,
        "checkbox_disabled_snapshot_click_block",
        "visual",
        true,
        passed,
        0,
        &state,
    )
}

fn checkbox_focus_target() -> LayoutRect {
    let component = preview_detail::component_action_hit_rect(CHECKBOX_PAGE);
    dedicated_dod_form_binary_choice_live::row_rect(0, component.x, component.y)
}

fn checkbox_mark_target() -> LayoutRect {
    let component = preview_detail::component_action_hit_rect(CHECKBOX_PAGE);
    dedicated_dod_form_binary_choice_live::checkbox_mark_rect(0, component.x, component.y)
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
