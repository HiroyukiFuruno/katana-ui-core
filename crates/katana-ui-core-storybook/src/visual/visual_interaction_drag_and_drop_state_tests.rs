use super::visual_interaction_test_support::component_body_pixel_diff;
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{dedicated_drag_and_drop, preview_detail, render};

const DARK_THEME: &str = "dark";
const PAGE: &str = "drag-and-drop";
const PRIMARY_INSTANCE: &str = "drag.primary";
const SECONDARY_INSTANCE: &str = "drag.secondary";
const DIFF_THRESHOLD: usize = 80;

#[test]
fn drag_and_drop_window_interaction_keeps_instance_state_isolated() {
    let mut state = page_state();

    state.select_instance(PRIMARY_INSTANCE);
    click_source(&mut state);
    let primary = state.screen_state.clone();
    let primary_canvas = render_state(&state);
    assert_eq!("drag_start", primary.last_action);
    assert_eq!("dragging=true", primary.state_label);
    assert!(primary.drag_and_drop.is_dragging());

    state.select_instance(SECONDARY_INSTANCE);
    assert_eq!("none", state.screen_state.last_action);
    assert!(!state.screen_state.drag_and_drop.is_dragging());
    click_target(&mut state);
    let secondary_canvas = render_state(&state);
    assert_eq!("drop", state.screen_state.last_action);
    assert_eq!("committed=true", state.screen_state.state_label);
    assert!(state.screen_state.drag_and_drop.committed());

    state.select_instance(PRIMARY_INSTANCE);
    assert_eq!(primary.last_action, state.screen_state.last_action);
    assert_eq!(primary.state_label, state.screen_state.state_label);
    assert!(state.screen_state.drag_and_drop.is_dragging());
    assert!(!state.screen_state.drag_and_drop.committed());
    assert!(component_body_pixel_diff(PAGE, &primary_canvas, &secondary_canvas) > DIFF_THRESHOLD);
}

fn click_source(state: &mut StorybookWindowState) {
    let component = preview_detail::component_action_hit_rect(PAGE);
    let source = dedicated_drag_and_drop::source_rect(component.x, component.y);

    assert!(apply_click(state, source.x + 1, source.y + 1));
}

fn click_target(state: &mut StorybookWindowState) {
    let component = preview_detail::component_action_hit_rect(PAGE);
    let target = dedicated_drag_and_drop::target_rect(component.x, component.y);

    assert!(apply_click(state, target.x + 1, target.y + 1));
}

fn page_state() -> StorybookWindowState {
    StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    }
}

fn render_state(state: &StorybookWindowState) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
        DARK_THEME,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    )
}
