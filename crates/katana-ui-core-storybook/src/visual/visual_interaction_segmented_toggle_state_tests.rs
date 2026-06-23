use super::visual_interaction_test_support::component_body_pixel_diff;
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{preview_detail, render};

const DARK_THEME: &str = "dark";
const PAGE: &str = "segmented-toggle";
const PRIMARY_INSTANCE: &str = "segmented-toggle.primary";
const SECONDARY_INSTANCE: &str = "segmented-toggle.secondary";
const DISABLED_PRESET_INDEX: usize = 2;
const NO_BODY_DIFF: usize = 0;

#[test]
fn segmented_toggle_window_interaction_keeps_instance_state_isolated() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };

    state.select_instance(PRIMARY_INSTANCE);
    click_segmented_toggle(&mut state);
    let primary = state.screen_state.clone();
    let primary_canvas = render_state(&state);
    assert_eq!("segment_select", primary.last_action);
    assert_eq!("segment=1", primary.state_label);

    state.select_instance(SECONDARY_INSTANCE);
    let secondary_canvas = render_state(&state);
    assert_eq!("none", state.screen_state.last_action);
    assert_eq!("idle", state.screen_state.state_label);

    state.select_instance(PRIMARY_INSTANCE);
    assert_eq!(primary.last_action, state.screen_state.last_action);
    assert_eq!(primary.state_label, state.screen_state.state_label);
    assert!(
        component_body_pixel_diff(PAGE, &primary_canvas, &secondary_canvas) > 80,
        "segmented-toggle instance-local state must produce distinct rendered bodies"
    );
}

#[test]
fn segmented_toggle_window_interaction_disabled_click_does_not_mutate_state() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        preset_index: DISABLED_PRESET_INDEX,
        ..StorybookWindowState::default()
    };
    let before_state = state.screen_state.clone();
    let before_canvas = render_state(&state);

    assert!(click_segmented_toggle(&mut state));
    let after_canvas = render_state(&state);

    assert_eq!(before_state, state.screen_state);
    assert_eq!(
        NO_BODY_DIFF,
        component_body_pixel_diff(PAGE, &before_canvas, &after_canvas)
    );
}

fn click_segmented_toggle(state: &mut StorybookWindowState) -> bool {
    let rect = preview_detail::component_action_hit_rect(PAGE);
    apply_click(state, rect.x + 1, rect.y + 1)
}

fn render_state(state: &StorybookWindowState) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
        DARK_THEME,
        PAGE,
        state.preset_index,
        state.screen_state.clone(),
    )
}
