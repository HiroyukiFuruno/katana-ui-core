use crate::visual::dedicated_tabs_metrics::{STRIP_X, STRIP_Y};
use crate::visual::preview_detail;
use crate::visual::window_interaction::{
    apply_clickable_keyboard_activation_for_audit, apply_hover_at,
    apply_scroll_area_drag_for_audit, apply_scroll_area_resize_for_audit,
    apply_scroll_area_scroll_for_audit, apply_scroll_delta_x_at_for_audit,
    focus_clickable_at_for_audit,
};

use super::{
    StorybookLiveInteractionScenario, component_body_pixel_diff, page_state, render_state, scenario,
};

const TABS_PAGE: &str = "tabs";
const CLOSEABLE_TAB_STRIP_PAGE: &str = "closeable-tab-strip";
const SCROLL_AREA_PAGE: &str = "scroll-area";
const HORIZONTAL_SCROLL_DELTA: f32 = 96.0;
const CLICK_OFFSET: usize = 4;

pub(super) fn scenarios(page: &'static str) -> Vec<StorybookLiveInteractionScenario> {
    match page {
        TABS_PAGE => vec![tab_strip_scroll_scenario(TABS_PAGE)],
        CLOSEABLE_TAB_STRIP_PAGE => vec![tab_strip_scroll_scenario(CLOSEABLE_TAB_STRIP_PAGE)],
        SCROLL_AREA_PAGE => vec![
            scroll_area_scroll_scenario(),
            scroll_area_drag_scenario(),
            scroll_area_hover_scenario(),
            scroll_area_focus_scenario(),
            scroll_area_keyboard_scenario(),
            scroll_area_resize_scenario(),
        ],
        _ => Vec::new(),
    }
}

fn tab_strip_scroll_scenario(page: &'static str) -> StorybookLiveInteractionScenario {
    let mut state = page_state(page);
    let before = render_state(page, &state);
    let component = preview_detail::component_action_hit_rect(page);
    let scrolled = apply_scroll_delta_x_at_for_audit(
        &mut state,
        component.x + STRIP_X + 1,
        component.y + STRIP_Y + 1,
        HORIZONTAL_SCROLL_DELTA,
    );
    let after = render_state(page, &state);
    let body_pixel_diff = component_body_pixel_diff(page, &before, &after);
    let passed = scrolled
        && state.screen_state.last_action == "tab_strip_scroll"
        && state.screen_state.last_event == "closeable_tab_overflow_scrolled"
        && body_pixel_diff > 0;
    scenario(
        page,
        "tab_strip_horizontal_scroll",
        "scroll",
        scrolled,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn scroll_area_scroll_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SCROLL_AREA_PAGE);
    let before = render_state(SCROLL_AREA_PAGE, &state);
    let target = preview_detail::component_action_hit_rect(SCROLL_AREA_PAGE);
    let scrolled = apply_scroll_area_scroll_for_audit(
        &mut state,
        target.x + CLICK_OFFSET,
        target.y + CLICK_OFFSET,
    );
    let after = render_state(SCROLL_AREA_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SCROLL_AREA_PAGE, &before, &after);
    let passed = scrolled
        && state.screen_state.last_action == "scroll_area_scroll"
        && state.screen_state.last_event == "scroll_area_scrolled"
        && state.screen_state.state_label == "scroll=48"
        && state.screen_state.scroll_area.offset_y() > 0
        && body_pixel_diff > 0;
    scenario(
        SCROLL_AREA_PAGE,
        "scroll_area_scroll",
        "scroll",
        scrolled,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn scroll_area_drag_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SCROLL_AREA_PAGE);
    let before = render_state(SCROLL_AREA_PAGE, &state);
    let target = preview_detail::component_action_hit_rect(SCROLL_AREA_PAGE);
    let dragged = apply_scroll_area_drag_for_audit(
        &mut state,
        target.x + CLICK_OFFSET,
        target.y + CLICK_OFFSET,
    );
    let after = render_state(SCROLL_AREA_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SCROLL_AREA_PAGE, &before, &after);
    let passed = dragged
        && state.screen_state.last_action == "scroll_area_drag_thumb"
        && state.screen_state.last_event == "scroll_area_scrolled"
        && state.screen_state.state_label == "drag=72"
        && state.screen_state.scroll_area.dragging()
        && body_pixel_diff > 0;
    scenario(
        SCROLL_AREA_PAGE,
        "scroll_area_drag_thumb",
        "drag",
        dragged,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn scroll_area_hover_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SCROLL_AREA_PAGE);
    let before = render_state(SCROLL_AREA_PAGE, &state);
    let target = preview_detail::component_action_hit_rect(SCROLL_AREA_PAGE);
    let hovered = apply_hover_at(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(SCROLL_AREA_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SCROLL_AREA_PAGE, &before, &after);
    let passed = hovered
        && state.screen_state.last_action == "scroll_area_hover"
        && state.screen_state.last_event == "hover_start"
        && state.screen_state.state_label == "hover=viewport"
        && state.screen_state.preview_hovered
        && state.screen_state.scroll_area.hovered()
        && body_pixel_diff > 0;
    scenario(
        SCROLL_AREA_PAGE,
        "scroll_area_hover",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn scroll_area_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SCROLL_AREA_PAGE);
    let before = render_state(SCROLL_AREA_PAGE, &state);
    let target = preview_detail::component_action_hit_rect(SCROLL_AREA_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(SCROLL_AREA_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SCROLL_AREA_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "scroll_area_focus"
        && state.screen_state.last_event == "focus"
        && state.screen_state.state_label == "focus=viewport"
        && state.screen_state.is_button_focused()
        && state.screen_state.scroll_area.focused()
        && body_pixel_diff > 0;
    scenario(
        SCROLL_AREA_PAGE,
        "scroll_area_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn scroll_area_keyboard_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SCROLL_AREA_PAGE);
    let target = preview_detail::component_action_hit_rect(SCROLL_AREA_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before = render_state(SCROLL_AREA_PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(SCROLL_AREA_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SCROLL_AREA_PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "scroll_area_keyboard_scroll"
        && state.screen_state.last_event == "scroll_area_scrolled"
        && state.screen_state.state_label == "keyboard=36"
        && state.screen_state.scroll_area.offset_y() > 0
        && body_pixel_diff > 0;
    scenario(
        SCROLL_AREA_PAGE,
        "scroll_area_keyboard_scroll",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn scroll_area_resize_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SCROLL_AREA_PAGE);
    let before = render_state(SCROLL_AREA_PAGE, &state);
    let target = preview_detail::component_action_hit_rect(SCROLL_AREA_PAGE);
    let resized = apply_scroll_area_resize_for_audit(
        &mut state,
        target.x + CLICK_OFFSET,
        target.y + CLICK_OFFSET,
    );
    let after = render_state(SCROLL_AREA_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SCROLL_AREA_PAGE, &before, &after);
    let passed = resized
        && state.screen_state.last_action == "scrollbar_visibility_changed"
        && state.screen_state.last_event == "scroll_area_resized"
        && state.screen_state.state_label == "resize=viewport"
        && state.screen_state.scroll_area.resized()
        && body_pixel_diff > 0;
    scenario(
        SCROLL_AREA_PAGE,
        "scroll_area_resize",
        "resize",
        resized,
        passed,
        body_pixel_diff,
        &state,
    )
}
