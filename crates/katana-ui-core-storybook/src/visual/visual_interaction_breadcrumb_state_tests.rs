use super::visual_interaction_test_support::component_body_pixel_diff;
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{dedicated_breadcrumb, preview_detail, render};

const DARK_THEME: &str = "dark";
const PAGE: &str = "breadcrumb";
const PRIMARY_INSTANCE: &str = "breadcrumb.primary";
const SECONDARY_INSTANCE: &str = "breadcrumb.secondary";
const ROOT_INDEX: usize = 0;
const FILE_INDEX: usize = 2;

#[test]
fn breadcrumb_window_interaction_keeps_instance_state_isolated() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };

    state.select_instance(PRIMARY_INSTANCE);
    click_crumb(&mut state, FILE_INDEX);
    let primary = state.screen_state.clone();
    let primary_canvas = render_state(&state);
    assert_eq!(FILE_INDEX, primary.breadcrumb_selected_index);
    assert_eq!("route=2", primary.state_label);

    state.select_instance(SECONDARY_INSTANCE);
    click_crumb(&mut state, ROOT_INDEX);
    let secondary = state.screen_state.clone();
    let secondary_canvas = render_state(&state);
    assert_eq!(ROOT_INDEX, secondary.breadcrumb_selected_index);
    assert_eq!("route=0", secondary.state_label);

    state.select_instance(PRIMARY_INSTANCE);
    assert_eq!(
        primary.breadcrumb_selected_index,
        state.screen_state.breadcrumb_selected_index
    );
    assert_eq!(primary.state_label, state.screen_state.state_label);
    assert!(
        component_body_pixel_diff(PAGE, &primary_canvas, &secondary_canvas) > 80,
        "breadcrumb instance-local state must produce distinct rendered bodies"
    );
}

fn click_crumb(state: &mut StorybookWindowState, index: usize) {
    let component = preview_detail::component_action_hit_rect(PAGE);
    let rect = match index {
        ROOT_INDEX => dedicated_breadcrumb::root_crumb_rect(component.x, component.y),
        _ => dedicated_breadcrumb::file_crumb_rect(component.x, component.y),
    };

    assert!(apply_click(state, rect.x + 1, rect.y + 1));
}

fn render_state(state: &StorybookWindowState) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
        DARK_THEME,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    )
}
