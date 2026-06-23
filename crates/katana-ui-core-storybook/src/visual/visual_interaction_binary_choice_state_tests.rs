use super::visual_interaction_test_support::component_body_pixel_diff;
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{dedicated_dod_form_binary_choice_live, preview_detail, render};

const DARK_THEME: &str = "dark";
const CHECKBOX_PAGE: &str = "checkbox";
const RADIO_PAGE: &str = "radio";
const CHECKBOX_PRIMARY_INSTANCE: &str = "checkbox.primary";
const CHECKBOX_SECONDARY_INSTANCE: &str = "checkbox.secondary";
const RADIO_PRIMARY_INSTANCE: &str = "radio.primary";
const RADIO_SECONDARY_INSTANCE: &str = "radio.secondary";
const CLICK_CENTER: usize = 2;
const DISABLED_PRESET_INDEX: usize = 2;
const NO_BODY_DIFF: usize = 0;

#[test]
fn checkbox_window_interaction_keeps_instance_state_isolated() {
    let mut state = page_state(CHECKBOX_PAGE);

    state.select_instance(CHECKBOX_PRIMARY_INSTANCE);
    click_checkbox_toggle(&mut state);
    let primary = state.screen_state.clone();
    let primary_canvas = render_state(CHECKBOX_PAGE, &state);
    assert_eq!("checkbox_toggle", primary.last_action);
    assert_eq!("before=false after=true", primary.state_label);
    assert!(primary.is_checkbox_checked());

    state.select_instance(CHECKBOX_SECONDARY_INSTANCE);
    assert_eq!("none", state.screen_state.last_action);
    assert!(!state.screen_state.is_checkbox_checked());
    click_checkbox_state_read(&mut state);
    let secondary = state.screen_state.clone();
    let secondary_canvas = render_state(CHECKBOX_PAGE, &state);
    assert_eq!("checkbox_state_read", secondary.last_action);
    assert_eq!("checked=false", secondary.state_label);
    assert!(!secondary.is_checkbox_checked());

    state.select_instance(CHECKBOX_PRIMARY_INSTANCE);
    assert_eq!(primary.last_action, state.screen_state.last_action);
    assert_eq!(primary.state_label, state.screen_state.state_label);
    assert!(state.screen_state.is_checkbox_checked());
    assert!(
        component_body_pixel_diff(CHECKBOX_PAGE, &primary_canvas, &secondary_canvas) > 80,
        "checkbox instance-local state must produce distinct rendered bodies"
    );
}

#[test]
fn radio_window_interaction_keeps_instance_state_isolated() {
    let mut state = page_state(RADIO_PAGE);

    state.select_instance(RADIO_PRIMARY_INSTANCE);
    click_radio_select(&mut state);
    let primary = state.screen_state.clone();
    let primary_canvas = render_state(RADIO_PAGE, &state);
    assert_eq!("radio_select", primary.last_action);
    assert_eq!("before=false after=true", primary.state_label);
    assert!(primary.is_radio_selected());

    state.select_instance(RADIO_SECONDARY_INSTANCE);
    assert_eq!("none", state.screen_state.last_action);
    assert!(!state.screen_state.is_radio_selected());
    click_radio_state_read(&mut state);
    let secondary = state.screen_state.clone();
    let secondary_canvas = render_state(RADIO_PAGE, &state);
    assert_eq!("radio_state_read", secondary.last_action);
    assert_eq!("before=false after=false", secondary.state_label);
    assert!(!secondary.is_radio_selected());

    state.select_instance(RADIO_PRIMARY_INSTANCE);
    assert_eq!(primary.last_action, state.screen_state.last_action);
    assert_eq!(primary.state_label, state.screen_state.state_label);
    assert!(state.screen_state.is_radio_selected());
    assert!(
        component_body_pixel_diff(RADIO_PAGE, &primary_canvas, &secondary_canvas) > 80,
        "radio instance-local state must produce distinct rendered bodies"
    );
}

#[test]
fn checkbox_window_interaction_disabled_toggle_does_not_mutate_state() {
    let mut state = page_state(CHECKBOX_PAGE);
    state.preset_index = DISABLED_PRESET_INDEX;
    let before_state = state.screen_state.clone();
    let before_canvas = render_state(CHECKBOX_PAGE, &state);

    assert!(click_checkbox_toggle(&mut state));
    let after_canvas = render_state(CHECKBOX_PAGE, &state);

    assert_eq!(before_state, state.screen_state);
    assert_eq!(
        NO_BODY_DIFF,
        component_body_pixel_diff(CHECKBOX_PAGE, &before_canvas, &after_canvas)
    );
}

fn click_checkbox_toggle(state: &mut StorybookWindowState) -> bool {
    let origin = preview_detail::component_action_hit_rect(CHECKBOX_PAGE);
    let rect =
        dedicated_dod_form_binary_choice_live::checkbox_toggle_button_rect(origin.x, origin.y);
    apply_click(state, rect.x + CLICK_CENTER, rect.y + CLICK_CENTER)
}

fn click_checkbox_state_read(state: &mut StorybookWindowState) {
    let origin = preview_detail::component_action_hit_rect(CHECKBOX_PAGE);
    let rect =
        dedicated_dod_form_binary_choice_live::checkbox_state_read_button_rect(origin.x, origin.y);
    assert!(apply_click(
        state,
        rect.x + CLICK_CENTER,
        rect.y + CLICK_CENTER
    ));
}

fn click_radio_select(state: &mut StorybookWindowState) {
    let origin = preview_detail::component_action_hit_rect(RADIO_PAGE);
    let rect = dedicated_dod_form_binary_choice_live::radio_select_button_rect(origin.x, origin.y);
    assert!(apply_click(
        state,
        rect.x + CLICK_CENTER,
        rect.y + CLICK_CENTER
    ));
}

fn click_radio_state_read(state: &mut StorybookWindowState) {
    let origin = preview_detail::component_action_hit_rect(RADIO_PAGE);
    let rect =
        dedicated_dod_form_binary_choice_live::radio_state_read_button_rect(origin.x, origin.y);
    assert!(apply_click(
        state,
        rect.x + CLICK_CENTER,
        rect.y + CLICK_CENTER
    ));
}

fn render_state(page: &'static str, state: &StorybookWindowState) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
        DARK_THEME,
        page,
        state.preset_index,
        state.screen_state.clone(),
    )
}

fn page_state(page: &'static str) -> StorybookWindowState {
    StorybookWindowState {
        selected_page: page,
        ..StorybookWindowState::default()
    }
}
