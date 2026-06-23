use super::button_options::{StorybookButtonOptionControl, control_rect};
use super::canvas::Canvas;
use super::visual_interaction_test_support::component_body_pixel_diff;
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{preview_detail, render};

const BUTTON_FAMILY_PAGES: [&str; 4] = ["button", "text-button", "svg-button", "icon-text-button"];
const PRIMARY_INSTANCE: &str = "button.primary";
const SECONDARY_INSTANCE: &str = "button.secondary";
const DIFF_THRESHOLD: usize = 80;
const CLICK_INSET: usize = 1;

#[test]
fn button_family_window_interaction_keeps_instance_state_isolated_across_presets() {
    for page in BUTTON_FAMILY_PAGES {
        assert_button_page_keeps_instance_state_isolated(page);
    }
}

fn assert_button_page_keeps_instance_state_isolated(page: &'static str) {
    let mut state = state_for(page);

    state.select_instance(PRIMARY_INSTANCE);
    click_preview_button(&mut state, page);
    let primary_click = state.screen_state.clone();
    assert!(primary_click.button_pressed);
    assert_ne!("none", primary_click.last_action);

    state.select_instance(SECONDARY_INSTANCE);
    assert_eq!("none", state.screen_state.last_action);
    assert!(!state.screen_state.button_pressed);

    state.select_instance(PRIMARY_INSTANCE);
    assert_eq!(primary_click.last_action, state.screen_state.last_action);
    assert_eq!(
        primary_click.button_pressed,
        state.screen_state.button_pressed
    );

    click_label_option(&mut state);
    let primary_option = state.screen_state.clone();
    let primary_option_canvas = render_state(&state);
    assert_eq!("button_option_apply", primary_option.last_action);
    assert_eq!("label", primary_option.last_setting);
    assert_eq!("label=ja", primary_option.state_label);

    state.select_instance(SECONDARY_INSTANCE);
    assert_eq!("none", state.screen_state.last_action);
    assert_eq!("none", state.screen_state.last_setting);
    assert_eq!("idle", state.screen_state.state_label);
    let secondary_option_canvas = render_state(&state);

    state.select_instance(PRIMARY_INSTANCE);
    assert_eq!(primary_option.last_action, state.screen_state.last_action);
    assert_eq!(primary_option.last_setting, state.screen_state.last_setting);
    assert_eq!(primary_option.state_label, state.screen_state.state_label);
    assert!(
        component_body_pixel_diff(page, &primary_option_canvas, &secondary_option_canvas)
            > DIFF_THRESHOLD
    );
}

fn click_preview_button(state: &mut StorybookWindowState, page: &str) {
    let rect = preview_detail::button_action_hit_rect(page);

    assert!(apply_click(
        state,
        rect.x + CLICK_INSET,
        rect.y + CLICK_INSET,
    ));
}

fn click_label_option(state: &mut StorybookWindowState) {
    let rect = control_rect(StorybookButtonOptionControl::Label);

    assert!(apply_click(
        state,
        rect.x + CLICK_INSET,
        rect.y + CLICK_INSET,
    ));
}

fn state_for(page: &'static str) -> StorybookWindowState {
    StorybookWindowState {
        selected_page: page,
        ..StorybookWindowState::default()
    }
}

fn render_state(state: &StorybookWindowState) -> Canvas {
    render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    )
}
