use super::button_options::{StorybookButtonOptionControl, control_rect};
use super::render;
use super::visual_interaction_test_support::component_body_pixel_diff;
use super::window_interaction::{StorybookWindowState, apply_click};

const SVG_BUTTON_PAGE: &str = "svg-button";
const SVG_SLOT_DIFF_THRESHOLD: usize = 12;

#[test]
fn svg_button_external_svg_source_option_updates_state_and_body() {
    let mut state = svg_button_state();
    let before = render_state(&state);
    let control = control_rect(StorybookButtonOptionControl::SvgSource);

    assert!(apply_click(&mut state, control.x + 1, control.y + 1));
    let after = render_state(&state);

    assert_eq!("button_option_apply", state.screen_state.last_action);
    assert_eq!("button.svg_source", state.screen_state.last_setting);
    assert_eq!("custom-svg", state.screen_state.last_setting_value);
    assert_eq!("svg_source=custom", state.screen_state.state_label);
    assert!(component_body_pixel_diff(SVG_BUTTON_PAGE, &before, &after) > SVG_SLOT_DIFF_THRESHOLD);
}

#[test]
fn svg_button_aria_label_option_updates_state_and_body() {
    let mut state = svg_button_state();
    let before = render_state(&state);
    let control = control_rect(StorybookButtonOptionControl::AriaLabel);

    assert!(apply_click(&mut state, control.x + 1, control.y + 1));
    let after = render_state(&state);

    assert_eq!("button_option_apply", state.screen_state.last_action);
    assert_eq!("button.aria_label", state.screen_state.last_setting);
    assert_eq!("Close panel", state.screen_state.last_setting_value);
    assert_eq!("aria_label=custom", state.screen_state.state_label);
    assert!(component_body_pixel_diff(SVG_BUTTON_PAGE, &before, &after) > SVG_SLOT_DIFF_THRESHOLD);
}

fn svg_button_state() -> StorybookWindowState {
    StorybookWindowState {
        selected_page: SVG_BUTTON_PAGE,
        ..StorybookWindowState::default()
    }
}

fn render_state(state: &StorybookWindowState) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    )
}
