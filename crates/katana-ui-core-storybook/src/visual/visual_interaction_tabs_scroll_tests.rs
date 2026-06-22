use super::visual_interaction_test_support::require_some;
use super::window_interaction::{StorybookWindowState, apply_scroll_delta_x_at_for_test};
use super::{dedicated_tabs, preview_detail, screen_state_tabs::TabsScreenState};

const ACTIVE_FOLLOW_PRESET: usize = 6;
const HORIZONTAL_SCROLL_DELTA: f32 = 96.0;

#[test]
fn tabs_active_follow_preset_scrolls_current_tab_into_strip() -> Result<(), String> {
    let state = TabsScreenState::for_preset(ACTIVE_FOLLOW_PRESET);
    let strip = dedicated_tabs::strip_rect_for_test();
    let active = require_some(
        dedicated_tabs::tab_rect_for_test(&state, "theme.rs"),
        "active follow tab rect",
    )?;

    assert_eq!("theme.rs", state.active_tab_id);
    assert!(dedicated_tabs::scroll_x_for_test(&state) > 0);
    assert!(active.x >= strip.x);
    assert!(active.right() <= strip.x + strip.width);
    Ok(())
}

#[test]
fn tabs_default_workspace_preset_does_not_scroll_when_tabs_fit() {
    let state = TabsScreenState::default();

    assert_eq!(0, dedicated_tabs::scroll_x_for_test(&state));
}

#[test]
fn tabs_scroll_measured_order_matches_render_layout_order() {
    let state = TabsScreenState::default();

    assert_eq!(
        dedicated_tabs::layout_item_ids_for_test(&state),
        dedicated_tabs::measured_item_ids_for_test(&state)
    );
}

#[test]
fn tabs_strip_horizontal_wheel_scrolls_overflowing_tab_row() {
    let mut state = StorybookWindowState {
        selected_page: "tabs",
        ..StorybookWindowState::default()
    };
    state.screen_state.tabs.add_many_for_overflow();
    let component = preview_detail::component_action_hit_rect("tabs");
    let strip = dedicated_tabs::strip_rect_for_test();

    assert!(apply_scroll_delta_x_at_for_test(
        &mut state,
        component.x + strip.x + 1,
        component.y + strip.y + 1,
        HORIZONTAL_SCROLL_DELTA,
    ));
    assert_eq!("tab_strip_scroll", state.screen_state.last_action);
    assert_eq!(
        "closeable_tab_overflow_scrolled",
        state.screen_state.last_event
    );
    assert!(dedicated_tabs::scroll_x_for_test(&state.screen_state.tabs) > 0);
}
