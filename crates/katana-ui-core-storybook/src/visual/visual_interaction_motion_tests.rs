use super::interaction_spec::StorybookInteractionSpec;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at,
};
use super::{StorybookVisual, preview_detail, storybook_ui_option_contract};
use crate::catalog::StoryPresetLabels;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "motion";
const DEFAULT_PRESET: usize = 0;
const REDUCED_PRESET: usize = 1;
const FORCE_PRESET: usize = 2;
const PER_MOLECULE_PRESET: usize = 3;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const SAMPLE_X_OFFSET: usize = 26;
const SAMPLE_Y_OFFSET: usize = 42;

#[test]
fn motion_exposes_leaf_presets_options_and_reduced_motion_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("motion_reduce", spec.action);
    assert_eq!("motion_snapshot_changed", spec.event);
    assert_eq!("instant=true", spec.state);
}

#[test]
fn motion_presets_render_distinct_runtime_bodies() {
    let primitive = StorybookVisual.render_preset(DARK_THEME, PAGE, DEFAULT_PRESET, 0);
    let reduced = StorybookVisual.render_preset(DARK_THEME, PAGE, REDUCED_PRESET, 0);
    let force = StorybookVisual.render_preset(DARK_THEME, PAGE, FORCE_PRESET, 0);
    let per_molecule = StorybookVisual.render_preset(DARK_THEME, PAGE, PER_MOLECULE_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &primitive, &reduced) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &reduced, &force) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &force, &per_molecule) > BODY_DIFF_THRESHOLD);
}

#[test]
fn motion_setting_option_updates_runtime_style() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn motion_preview_action_updates_runtime_phase() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn motion_light_and_dark_surfaces_use_theme_tokens() {
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
