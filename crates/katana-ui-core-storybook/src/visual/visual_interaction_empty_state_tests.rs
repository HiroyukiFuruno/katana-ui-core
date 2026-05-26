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
const PAGE: &str = "empty-state";
const EXPLORER_PRESET: usize = 0;
const SEARCH_PRESET: usize = 1;
const CLEAN_PRESET: usize = 2;
const HISTORY_PRESET: usize = 3;
const ERROR_PRESET: usize = 4;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const SURFACE_TOKEN_X: usize = 338;
const SURFACE_TOKEN_Y: usize = 34;
const SURFACE_SAMPLE_X_OFFSET: usize = 120;
const SURFACE_SAMPLE_Y_OFFSET: usize = 12;

#[test]
fn empty_state_exposes_leaf_presets_options_and_action_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert!(rows.iter().any(|row| row.starts_with("empty_state.tone:")));
    assert!(
        rows.iter()
            .any(|row| row.starts_with("empty_state.alignment:"))
    );
    assert_eq!("empty_state_primary", spec.action);
    assert_eq!("empty_state_actioned", spec.event);
    assert_eq!("empty_state.primary_action", spec.option);
    assert_eq!("reload", spec.after);
    assert_eq!("action=reload", spec.state);
}

#[test]
fn empty_state_presets_render_distinct_empty_clean_history_and_error_states() {
    let explorer = StorybookVisual.render_preset(DARK_THEME, PAGE, EXPLORER_PRESET, 0);
    let search = StorybookVisual.render_preset(DARK_THEME, PAGE, SEARCH_PRESET, 0);
    let clean = StorybookVisual.render_preset(DARK_THEME, PAGE, CLEAN_PRESET, 0);
    let history = StorybookVisual.render_preset(DARK_THEME, PAGE, HISTORY_PRESET, 0);
    let error = StorybookVisual.render_preset(DARK_THEME, PAGE, ERROR_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &explorer, &search) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &search, &clean) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &clean, &history) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &history, &error) > BODY_DIFF_THRESHOLD);
}

#[test]
fn empty_state_setting_option_updates_alignment_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn empty_state_preview_action_updates_primary_action_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn empty_state_light_and_dark_surface_uses_theme_surface() {
    assert_empty_state_surface_token(DARK_THEME, ThemeSnapshot::dark());
    assert_empty_state_surface_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_empty_state_surface_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, EXPLORER_PRESET, 0);
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
