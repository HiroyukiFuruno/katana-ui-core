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
const PAGE: &str = "startup-state-panel";
const BOOT_PRESET: usize = 0;
const SESSION_PRESET: usize = 1;
const UPDATE_PRESET: usize = 2;
const ERROR_PRESET: usize = 3;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const SURFACE_TOKEN_X: usize = 338;
const SURFACE_TOKEN_Y: usize = 20;
const SURFACE_SAMPLE_X_OFFSET: usize = 120;
const SURFACE_SAMPLE_Y_OFFSET: usize = 12;

#[test]
fn startup_state_panel_exposes_leaf_presets_options_and_error_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert!(
        rows.iter()
            .any(|row| row.starts_with("startup_state.state:"))
    );
    assert!(
        rows.iter()
            .any(|row| row.starts_with("startup_state.retry:"))
    );
    assert_eq!("startup_state_error", spec.action);
    assert_eq!("startup_state_changed", spec.event);
    assert_eq!("startup.state", spec.option);
    assert_eq!("Error", spec.after);
    assert_eq!("retry=true", spec.state);
}

#[test]
fn startup_state_panel_presets_render_distinct_progress_error_and_action_states() {
    let boot = StorybookVisual.render_preset(DARK_THEME, PAGE, BOOT_PRESET, 0);
    let session = StorybookVisual.render_preset(DARK_THEME, PAGE, SESSION_PRESET, 0);
    let update = StorybookVisual.render_preset(DARK_THEME, PAGE, UPDATE_PRESET, 0);
    let error = StorybookVisual.render_preset(DARK_THEME, PAGE, ERROR_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &boot, &session) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &session, &update) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &update, &error) > BODY_DIFF_THRESHOLD);
}

#[test]
fn startup_state_panel_setting_option_updates_error_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn startup_state_panel_preview_action_updates_retry_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn startup_state_panel_light_and_dark_surface_uses_theme_surface() {
    assert_startup_surface_token(DARK_THEME, ThemeSnapshot::dark());
    assert_startup_surface_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_startup_surface_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, BOOT_PRESET, 0);
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
