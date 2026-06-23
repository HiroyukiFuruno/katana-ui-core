use super::visual_interaction_test_support::component_body_pixel_diff;
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{preview_detail, render};

const DARK_THEME: &str = "dark";
const PAGE: &str = "dynamic-array-editor";
const PRIMARY_INSTANCE: &str = "dynamic-array.primary";
const SECONDARY_INSTANCE: &str = "dynamic-array.secondary";
const DIFF_THRESHOLD: usize = 80;
const CONTROL_X: usize = 246;
const ADD_Y: usize = 54;
const REORDER_Y: usize = 102;

#[test]
fn dynamic_array_editor_window_interaction_keeps_instance_state_isolated() {
    let mut state = page_state();

    state.select_instance(PRIMARY_INSTANCE);
    click_component_control(&mut state, ADD_Y);
    let primary = state.screen_state.clone();
    let primary_canvas = render_state(&state);
    assert_eq!("array_add", primary.last_action);
    assert_eq!(4, primary.dynamic_array_editor.item_count());
    assert_eq!("callback=add", primary.last_setting_value);

    state.select_instance(SECONDARY_INSTANCE);
    assert_eq!("none", state.screen_state.last_action);
    let secondary_initial_canvas = render_state(&state);
    click_component_control(&mut state, REORDER_Y);
    assert_eq!("array_reorder", state.screen_state.last_action);
    assert_eq!(3, state.screen_state.dynamic_array_editor.item_count());
    assert_eq!(
        "order=2,1,3",
        state.screen_state.dynamic_array_editor.order_label()
    );

    state.select_instance(PRIMARY_INSTANCE);
    assert_eq!(primary.last_action, state.screen_state.last_action);
    assert_eq!(primary.state_label, state.screen_state.state_label);
    assert_eq!(4, state.screen_state.dynamic_array_editor.item_count());
    assert!(
        component_body_pixel_diff(PAGE, &primary_canvas, &secondary_initial_canvas)
            > DIFF_THRESHOLD
    );
}

fn click_component_control(state: &mut StorybookWindowState, y: usize) {
    let component = preview_detail::component_action_hit_rect(PAGE);
    assert!(apply_click(
        state,
        component.x + CONTROL_X + 1,
        component.y + y + 1
    ));
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
