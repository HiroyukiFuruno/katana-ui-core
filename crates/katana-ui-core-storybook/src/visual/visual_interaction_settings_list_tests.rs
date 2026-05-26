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
const PAGE: &str = "settings-list";
const APP_PRESET: usize = 0;
const CHAT_PRESET: usize = 1;
const LINT_PRESET: usize = 2;
const DIRTY_PRESET: usize = 3;
const QUERY_PRESET: usize = 4;
const RESET_PRESET: usize = 5;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const SURFACE_TOKEN_X: usize = 338;
const SURFACE_TOKEN_Y: usize = 34;
const SURFACE_SAMPLE_X_OFFSET: usize = 120;
const SURFACE_SAMPLE_Y_OFFSET: usize = 12;

#[test]
fn settings_list_exposes_leaf_presets_options_and_field_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert!(
        rows.iter()
            .any(|row| row.starts_with("settings_list.density:"))
    );
    assert!(
        rows.iter()
            .any(|row| row.starts_with("settings_list.query:"))
    );
    assert_eq!("settings_filter_update_collapse", spec.action);
    assert_eq!("settings_field_changed", spec.event);
    assert_eq!("settings.query", spec.option);
    assert_eq!("font", spec.after);
    assert_eq!("dirty=font-size", spec.state);
}

#[test]
fn settings_list_presets_render_distinct_sections_dirty_query_and_reset_states() {
    let app = StorybookVisual.render_preset(DARK_THEME, PAGE, APP_PRESET, 0);
    let chat = StorybookVisual.render_preset(DARK_THEME, PAGE, CHAT_PRESET, 0);
    let lint = StorybookVisual.render_preset(DARK_THEME, PAGE, LINT_PRESET, 0);
    let dirty = StorybookVisual.render_preset(DARK_THEME, PAGE, DIRTY_PRESET, 0);
    let query = StorybookVisual.render_preset(DARK_THEME, PAGE, QUERY_PRESET, 0);
    let reset = StorybookVisual.render_preset(DARK_THEME, PAGE, RESET_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &app, &chat) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &chat, &lint) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &lint, &dirty) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &dirty, &query) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &query, &reset) > BODY_DIFF_THRESHOLD);
}

#[test]
fn settings_list_setting_option_updates_query_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn settings_list_preview_action_updates_field_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn settings_list_light_and_dark_surface_uses_theme_surface() {
    assert_settings_surface_token(DARK_THEME, ThemeSnapshot::dark());
    assert_settings_surface_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_settings_surface_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, APP_PRESET, 0);
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
