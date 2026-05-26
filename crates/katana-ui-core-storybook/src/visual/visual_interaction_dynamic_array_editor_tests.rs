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
const PAGE: &str = "dynamic-array-editor";
const ROWS_PRESET: usize = 0;
const ADD_REMOVE_PRESET: usize = 1;
const REORDER_PRESET: usize = 2;
const THEME_PRESET: usize = 3;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const SURFACE_TOKEN_X: usize = 338;
const SURFACE_TOKEN_Y: usize = 34;
const SURFACE_SAMPLE_X_OFFSET: usize = 120;
const SURFACE_SAMPLE_Y_OFFSET: usize = 12;

#[test]
fn dynamic_array_editor_exposes_leaf_presets_options_and_add_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert_eq!(options.len(), REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert!(rows.iter().any(|row| row.starts_with("array.rows:")));
    assert!(rows.iter().any(|row| row.starts_with("array.reorder:")));
    assert_eq!("array_add", spec.action);
    assert_eq!("array_changed", spec.event);
    assert_eq!("interaction.value", spec.option);
    assert_eq!("3 rows", spec.after);
    assert_eq!("rows=3", spec.state);
}

#[test]
fn dynamic_array_editor_presets_render_distinct_rows_add_reorder_and_theme_states() {
    let rows = StorybookVisual.render_preset(DARK_THEME, PAGE, ROWS_PRESET, 0);
    let add_remove = StorybookVisual.render_preset(DARK_THEME, PAGE, ADD_REMOVE_PRESET, 0);
    let reorder = StorybookVisual.render_preset(DARK_THEME, PAGE, REORDER_PRESET, 0);
    let theme = StorybookVisual.render_preset(DARK_THEME, PAGE, THEME_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &rows, &add_remove) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &add_remove, &reorder) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &reorder, &theme) > BODY_DIFF_THRESHOLD);
}

#[test]
fn dynamic_array_editor_setting_option_updates_row_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn dynamic_array_editor_preview_action_updates_row_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn dynamic_array_editor_light_and_dark_surface_uses_theme_surface() {
    assert_dynamic_array_surface_token(DARK_THEME, ThemeSnapshot::dark());
    assert_dynamic_array_surface_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_dynamic_array_surface_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, ROWS_PRESET, 0);
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
