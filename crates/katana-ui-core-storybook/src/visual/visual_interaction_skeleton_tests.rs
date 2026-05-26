use super::interaction_spec::StorybookInteractionSpec;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at,
};
use super::{StorybookVisual, preview_detail, storybook_ui_option_contract};
use crate::catalog::StoryPresetLabels;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "skeleton";
const TEXT_PRESET: usize = 0;
const AVATAR_PRESET: usize = 1;
const RECT_PRESET: usize = 2;
const WAVE_PRESET: usize = 3;
const REDUCED_PRESET: usize = 4;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const SAMPLE_X_OFFSET: usize = 22;
const SAMPLE_Y_OFFSET: usize = 38;

#[test]
fn skeleton_exposes_leaf_presets_options_and_animation_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("reduced_motion_toggle", spec.action);
    assert_eq!("skeleton_animation_changed", spec.event);
    assert_eq!("reduced_motion=true", spec.state);
}

#[test]
fn skeleton_presets_render_distinct_placeholder_bodies() {
    let text = StorybookVisual.render_preset(DARK_THEME, PAGE, TEXT_PRESET, 0);
    let avatar = StorybookVisual.render_preset(DARK_THEME, PAGE, AVATAR_PRESET, 0);
    let rect = StorybookVisual.render_preset(DARK_THEME, PAGE, RECT_PRESET, 0);
    let wave = StorybookVisual.render_preset(DARK_THEME, PAGE, WAVE_PRESET, 0);
    let reduced = StorybookVisual.render_preset(DARK_THEME, PAGE, REDUCED_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &text, &avatar) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &avatar, &rect) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &rect, &wave) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &wave, &reduced) > BODY_DIFF_THRESHOLD);
}

#[test]
fn skeleton_setting_option_updates_placeholder_style() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn skeleton_preview_action_updates_placeholder_style() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn skeleton_light_and_dark_surfaces_use_theme_tokens() {
    let dark = StorybookVisual.render_preset(DARK_THEME, PAGE, TEXT_PRESET, 0);
    let light = StorybookVisual.render_preset(LIGHT_THEME, PAGE, TEXT_PRESET, 0);
    let rect = preview_detail::component_action_hit_rect(PAGE);
    let sample_x = rect.x + SAMPLE_X_OFFSET;
    let sample_y = rect.y + SAMPLE_Y_OFFSET;

    assert_ne!(
        pixel_at(&dark, sample_x, sample_y),
        pixel_at(&light, sample_x, sample_y)
    );
}
