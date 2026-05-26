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
fn slide_control_light_and_dark_track_uses_theme_tokens() {
    assert_track_tokens(DARK_THEME, ThemeSnapshot::dark());
    assert_track_tokens(LIGHT_THEME, ThemeSnapshot::light());
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
