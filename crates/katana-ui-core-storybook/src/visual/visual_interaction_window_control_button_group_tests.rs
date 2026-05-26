use super::interaction_spec::StorybookInteractionSpec;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at,
};
use super::{StorybookVisual, palette, preview_detail, storybook_ui_option_contract};
use crate::catalog::StoryPresetLabels;
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "window-control-button-group";
const MACOS_PRESET: usize = 0;
const WINDOWS_PRESET: usize = 1;
const LINUX_PRESET: usize = 2;
const FULLSCREEN_PRESET: usize = 3;
const CLOSE_ONLY_PRESET: usize = 4;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const SURFACE_TOKEN_X: usize = 338;
const SURFACE_TOKEN_Y: usize = 20;
const SURFACE_SAMPLE_X_OFFSET: usize = 120;
const SURFACE_SAMPLE_Y_OFFSET: usize = 12;

#[test]
fn window_controls_exposes_leaf_presets_options_and_press_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert_eq!(options.len(), REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert!(
        rows.iter()
            .any(|row| row.starts_with("window_control.position:"))
    );
    assert!(
        rows.iter()
            .any(|row| row.starts_with("window_control.visibility:"))
    );
    assert_eq!("window_control_press", spec.action);
    assert_eq!("window_control_pressed", spec.event);
    assert_eq!("window_controls.position", spec.option);
    assert_eq!("Leading", spec.after);
    assert_eq!("pressed=Close", spec.state);
}

#[test]
fn window_controls_presets_render_distinct_position_size_and_visibility_states() {
    let macos = StorybookVisual.render_preset(DARK_THEME, PAGE, MACOS_PRESET, 0);
    let windows = StorybookVisual.render_preset(DARK_THEME, PAGE, WINDOWS_PRESET, 0);
    let linux = StorybookVisual.render_preset(DARK_THEME, PAGE, LINUX_PRESET, 0);
    let fullscreen = StorybookVisual.render_preset(DARK_THEME, PAGE, FULLSCREEN_PRESET, 0);
    let close_only = StorybookVisual.render_preset(DARK_THEME, PAGE, CLOSE_ONLY_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &macos, &windows) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &windows, &linux) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &linux, &fullscreen) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &fullscreen, &close_only) > BODY_DIFF_THRESHOLD);
}

#[test]
fn window_controls_setting_option_updates_position_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn window_controls_preview_action_updates_pressed_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn window_controls_light_and_dark_surface_uses_theme_surface() {
    assert_window_surface_token(DARK_THEME, ThemeSnapshot::dark());
    assert_window_surface_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_window_surface_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, MACOS_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert_eq!(
        Some(colors.surface),
        pixel_at(
            &canvas,
            component.x + SURFACE_TOKEN_X + SURFACE_SAMPLE_X_OFFSET,
            component.y + SURFACE_TOKEN_Y + SURFACE_SAMPLE_Y_OFFSET
        )
    );
}
