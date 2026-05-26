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
const PAGE: &str = "virtualization";
const FIXED_PRESET: usize = 0;
const VARIABLE_PRESET: usize = 1;
const FOCUSED_PRESET: usize = 2;
const MEASURED_PRESET: usize = 3;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const VIEWPORT_X: usize = 36;
const VIEWPORT_Y: usize = 30;
const VIEWPORT_SAMPLE_X_OFFSET: usize = 286;
const VIEWPORT_SAMPLE_Y_OFFSET: usize = 78;

#[test]
fn virtualization_exposes_leaf_presets_options_and_scroll_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert!(
        rows.iter()
            .any(|row| row.starts_with("virtualization.overscan:"))
    );
    assert!(
        rows.iter()
            .any(|row| row.starts_with("virtualization.row_height_provider:"))
    );
    assert_eq!("virtualized_scroll", spec.action);
    assert_eq!("virtual_range_changed", spec.event);
    assert_eq!("viewport.offset", spec.option);
    assert_eq!("1260", spec.after);
    assert_eq!("rows=visible", spec.state);
}

#[test]
fn virtualization_presets_render_distinct_fixed_variable_focus_and_measurement_states() {
    let fixed = StorybookVisual.render_preset(DARK_THEME, PAGE, FIXED_PRESET, 0);
    let variable = StorybookVisual.render_preset(DARK_THEME, PAGE, VARIABLE_PRESET, 0);
    let focused = StorybookVisual.render_preset(DARK_THEME, PAGE, FOCUSED_PRESET, 0);
    let measured = StorybookVisual.render_preset(DARK_THEME, PAGE, MEASURED_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &fixed, &variable) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &variable, &focused) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &focused, &measured) > BODY_DIFF_THRESHOLD);
}

#[test]
fn virtualization_setting_option_updates_viewport_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn virtualization_preview_action_updates_scroll_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn virtualization_light_and_dark_viewport_uses_theme_surface() {
    assert_viewport_token(DARK_THEME, ThemeSnapshot::dark());
    assert_viewport_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_viewport_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, FIXED_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert_eq!(
        Some(colors.surface),
        pixel_at(
            &canvas,
            component.x + VIEWPORT_X + VIEWPORT_SAMPLE_X_OFFSET,
            component.y + VIEWPORT_Y + VIEWPORT_SAMPLE_Y_OFFSET
        )
    );
}
