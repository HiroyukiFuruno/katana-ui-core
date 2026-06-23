use super::interaction_spec::StorybookInteractionSpec;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at,
};
use super::window_interaction::{
    StorybookWindowState, apply_clickable_keyboard_activation_for_audit, apply_hover_at,
    apply_slide_drag_for_audit, focus_clickable_at_for_audit,
};
use super::{StorybookVisual, palette, preview_detail, storybook_ui_option_contract};
use crate::catalog::StoryPresetLabels;
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "slide-control";
const TRACK_PRESET: usize = 0;
const DRAG_PRESET: usize = 1;
const STEP_PRESET: usize = 2;
const THEME_PRESET: usize = 3;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const TRACK_X: usize = 24;
const TRACK_Y: usize = 54;
const TRACK_FILL_SAMPLE_X_OFFSET: usize = 4;
const TRACK_FILL_SAMPLE_Y_OFFSET: usize = 2;

#[test]
fn slide_control_exposes_leaf_presets_options_and_slide_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("slide_drag", spec.action);
    assert_eq!("slide_changed", spec.event);
    assert_eq!("interaction.value", spec.option);
    assert_eq!("64", spec.after);
    assert_eq!("value=64", spec.state);
}

#[test]
fn slide_control_presets_render_distinct_slider_bodies() {
    let track = StorybookVisual.render_preset(DARK_THEME, PAGE, TRACK_PRESET, 0);
    let drag = StorybookVisual.render_preset(DARK_THEME, PAGE, DRAG_PRESET, 0);
    let step = StorybookVisual.render_preset(DARK_THEME, PAGE, STEP_PRESET, 0);
    let themed = StorybookVisual.render_preset(DARK_THEME, PAGE, THEME_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &track, &drag) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &drag, &step) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &track, &themed) > BODY_DIFF_THRESHOLD);
}

#[test]
fn slide_control_setting_option_updates_slider_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn slide_control_preview_action_updates_slider_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn slide_control_hover_focus_keyboard_and_drag_update_body_and_state() {
    let mut hover = page_state();
    let target = preview_detail::component_action_hit_rect(PAGE);
    let before_hover = render_state(&hover);

    assert!(apply_hover_at(&mut hover, target.x + 1, target.y + 1));
    let after_hover = render_state(&hover);
    assert!(hover.screen_state.preview_hovered);
    assert!(component_body_pixel_diff(PAGE, &before_hover, &after_hover) > 0);

    let mut keyboard = page_state();
    let before_focus = render_state(&keyboard);
    assert!(focus_clickable_at_for_audit(
        &mut keyboard,
        target.x + 1,
        target.y + 1
    ));
    let after_focus = render_state(&keyboard);
    assert_eq!("slide_focus", keyboard.screen_state.last_action);
    assert_eq!("slide_focused", keyboard.screen_state.last_event);
    assert_eq!("focused=true", keyboard.screen_state.state_label);
    assert!(component_body_pixel_diff(PAGE, &before_focus, &after_focus) > 0);

    let before_keyboard = render_state(&keyboard);
    assert!(apply_clickable_keyboard_activation_for_audit(&mut keyboard));
    let after_keyboard = render_state(&keyboard);
    assert_eq!(
        "slide_keyboard_increment",
        keyboard.screen_state.last_action
    );
    assert_eq!("slide_changed", keyboard.screen_state.last_event);
    assert_eq!("value=64", keyboard.screen_state.state_label);
    assert!(component_body_pixel_diff(PAGE, &before_keyboard, &after_keyboard) > 0);

    let mut drag = page_state();
    let before_drag = render_state(&drag);
    assert!(apply_slide_drag_for_audit(
        &mut drag,
        target.x + 1,
        target.y + 1
    ));
    let after_drag = render_state(&drag);
    assert_eq!("slide_drag", drag.screen_state.last_action);
    assert_eq!("slide_changed", drag.screen_state.last_event);
    assert_eq!("value=64", drag.screen_state.state_label);
    assert!(component_body_pixel_diff(PAGE, &before_drag, &after_drag) > 0);
}

#[test]
fn slide_control_light_and_dark_track_uses_theme_tokens() {
    assert_track_tokens(DARK_THEME, ThemeSnapshot::dark());
    assert_track_tokens(LIGHT_THEME, ThemeSnapshot::light());
}

fn page_state() -> StorybookWindowState {
    StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    }
}

fn render_state(state: &StorybookWindowState) -> super::Canvas {
    super::render::render_storybook_canvas_with_screen_state(
        DARK_THEME,
        PAGE,
        state.preset_index,
        state.screen_state.clone(),
    )
}

fn assert_track_tokens(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, TRACK_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let component = preview_detail::component_action_hit_rect(PAGE);
    let track_x = component.x + TRACK_X;
    let track_y = component.y + TRACK_Y;

    assert_eq!(
        Some(colors.accent),
        pixel_at(
            &canvas,
            track_x + TRACK_FILL_SAMPLE_X_OFFSET,
            track_y + TRACK_FILL_SAMPLE_Y_OFFSET
        )
    );
}
