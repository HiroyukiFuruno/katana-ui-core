use super::interaction_spec::StorybookInteractionSpec;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at,
};
use super::{StorybookVisual, preview_detail, storybook_ui_option_contract};
use crate::catalog::StoryPresetLabels;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "progress-bar";
const DEFAULT_PRESET: usize = 0;
const CHANGE_PRESET: usize = 1;
const EMPTY_PRESET: usize = 2;
const THEME_PRESET: usize = 3;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const SAMPLE_X_OFFSET: usize = 24;
const SAMPLE_Y_OFFSET: usize = 48;

#[test]
fn progress_bar_exposes_leaf_presets_options_and_progress_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("progress_change", spec.action);
    assert_eq!("progress_changed", spec.event);
    assert_eq!("percent=64", spec.state);
}

#[test]
fn progress_bar_presets_render_distinct_meter_bodies() {
    let determinate = StorybookVisual.render_preset(DARK_THEME, PAGE, DEFAULT_PRESET, 0);
    let changed = StorybookVisual.render_preset(DARK_THEME, PAGE, CHANGE_PRESET, 0);
    let empty = StorybookVisual.render_preset(DARK_THEME, PAGE, EMPTY_PRESET, 0);
    let themed = StorybookVisual.render_preset(DARK_THEME, PAGE, THEME_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &determinate, &changed) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &changed, &empty) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &empty, &themed) > BODY_DIFF_THRESHOLD);
}

#[test]
fn progress_bar_setting_option_updates_meter_width() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn progress_bar_preview_action_updates_meter_width() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn progress_bar_light_and_dark_surfaces_use_theme_tokens() {
    let dark = StorybookVisual.render_preset(DARK_THEME, PAGE, DEFAULT_PRESET, 0);
    let light = StorybookVisual.render_preset(LIGHT_THEME, PAGE, DEFAULT_PRESET, 0);
    let rect = preview_detail::component_action_hit_rect(PAGE);
    let sample_x = rect.x + SAMPLE_X_OFFSET;
    let sample_y = rect.y + SAMPLE_Y_OFFSET;

    assert_ne!(
        pixel_at(&dark, sample_x, sample_y),
        pixel_at(&light, sample_x, sample_y)
    );
}
