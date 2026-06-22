use crate::visual::window_interaction::apply_click;
use crate::visual::{Canvas, dedicated_dod_form_binary_choice_live, layout_metrics::LayoutRect};

use super::{
    StorybookLiveInteractionScenario, component_body_pixel_diff, page_state, preview_detail,
    render_state, scenario,
};

const CHECKBOX_PAGE: &str = "checkbox";
const CLICK_OFFSET: usize = 4;
const CHECKBOX_ACCENT: u32 = 0x569cd6;
const CHECKBOX_GLYPH: u32 = 0xf8fafc;

pub(super) fn scenarios(page: &'static str) -> Vec<StorybookLiveInteractionScenario> {
    if page != CHECKBOX_PAGE {
        return Vec::new();
    }
    vec![checkbox_pointer_checks_both_rows_scenario()]
}

fn checkbox_pointer_checks_both_rows_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(CHECKBOX_PAGE);
    let first_target = checkbox_row_target_at(0);
    let second_target = checkbox_row_target_at(1);
    let first_mark = checkbox_mark_target_at(0);
    let second_mark = checkbox_mark_target_at(1);
    let before = render_state(CHECKBOX_PAGE, &state);
    let first_clicked = apply_click(
        &mut state,
        first_target.x + CLICK_OFFSET,
        first_target.y + CLICK_OFFSET,
    );
    let first_checked = render_state(CHECKBOX_PAGE, &state);
    let second_clicked = apply_click(
        &mut state,
        second_target.x + CLICK_OFFSET,
        second_target.y + CLICK_OFFSET,
    );
    let both_checked = render_state(CHECKBOX_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(CHECKBOX_PAGE, &before, &first_checked)
        + component_body_pixel_diff(CHECKBOX_PAGE, &first_checked, &both_checked);
    let passed = first_clicked
        && second_clicked
        && state.screen_state.last_action == "checkbox_toggle"
        && state.screen_state.last_event == "checked_changed"
        && state.screen_state.is_checkbox_checked_at(0)
        && state.screen_state.is_checkbox_checked_at(1)
        && checked_mark_visible(&both_checked, first_mark)
        && checked_mark_visible(&both_checked, second_mark)
        && body_pixel_diff > 0;
    scenario(
        CHECKBOX_PAGE,
        "checkbox_pointer_checks_both_rows",
        "pointer",
        second_clicked,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn checked_mark_visible(canvas: &Canvas, mark: LayoutRect) -> bool {
    count_color_in_rect(canvas, mark, CHECKBOX_ACCENT) > 0
        && count_color_in_rect(canvas, mark, CHECKBOX_GLYPH) > 0
}

fn checkbox_row_target_at(index: usize) -> LayoutRect {
    let component = preview_detail::component_action_hit_rect(CHECKBOX_PAGE);
    dedicated_dod_form_binary_choice_live::row_rect(index, component.x, component.y)
}

fn checkbox_mark_target_at(index: usize) -> LayoutRect {
    let component = preview_detail::component_action_hit_rect(CHECKBOX_PAGE);
    dedicated_dod_form_binary_choice_live::checkbox_mark_rect(index, component.x, component.y)
}

fn count_color_in_rect(canvas: &Canvas, rect: LayoutRect, color: u32) -> usize {
    (rect.y..rect.bottom())
        .flat_map(|y| (rect.x..rect.right()).map(move |x| (x, y)))
        .filter(|(x, y)| pixel_at(canvas, *x, *y) == Some(color))
        .count()
}

fn pixel_at(canvas: &Canvas, x: usize, y: usize) -> Option<u32> {
    if x >= canvas.width() || y >= canvas.height() {
        return None;
    }
    Some(canvas.pixels()[y * canvas.width() + x])
}
