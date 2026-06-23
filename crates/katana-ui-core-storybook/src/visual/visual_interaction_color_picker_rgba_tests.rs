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
const PAGE: &str = "color-picker-rgba";
const RGBA_PANEL_PRESET: usize = 0;
const REQUIRED_PRESET_COUNT: usize = 15;
const REQUIRED_OPTION_COUNT: usize = 15;
const BODY_DIFF_THRESHOLD: usize = 80;
const PANEL_X: usize = 176;
const PANEL_Y: usize = 74;
const PANEL_SAMPLE_OFFSET: usize = 4;

#[test]
fn color_picker_exposes_leaf_presets_options_and_rgba_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("color_drag", spec.action);
    assert_eq!("rgba_changed", spec.event);
    assert_eq!("color_swatch.selected_color", spec.option);
    assert_eq!("rgba(64,128,255,204)", spec.after);
    assert_eq!("rgba=accent", spec.state);
}

#[test]
fn color_picker_presets_render_distinct_rgba_bodies() {
    let mut previous = StorybookVisual.render_preset(DARK_THEME, PAGE, RGBA_PANEL_PRESET, 0);

    for preset in 1..REQUIRED_PRESET_COUNT {
        let current = StorybookVisual.render_preset(DARK_THEME, PAGE, preset, 0);
        assert!(
            component_body_pixel_diff(PAGE, &previous, &current) > BODY_DIFF_THRESHOLD,
            "preset {preset} did not repaint color picker body"
        );
        previous = current;
    }
}

#[test]
fn color_picker_setting_option_updates_rgba_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn color_picker_preview_action_updates_rgba_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn color_picker_light_and_dark_panel_uses_theme_surface() {
    assert_panel_token(DARK_THEME, ThemeSnapshot::dark());
    assert_panel_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_panel_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, RGBA_PANEL_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert_eq!(
        Some(colors.surface),
        pixel_at(
            &canvas,
            component.x + PANEL_X + PANEL_SAMPLE_OFFSET,
            component.y + PANEL_Y + PANEL_SAMPLE_OFFSET
        )
    );
}
