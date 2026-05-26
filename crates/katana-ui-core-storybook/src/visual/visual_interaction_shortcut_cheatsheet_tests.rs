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
const PAGE: &str = "shortcut-cheatsheet";
const SAMPLE_PRESET: usize = 0;
const CATEGORY_PRESET: usize = 1;
const TWO_COLUMN_PRESET: usize = 2;
const ONE_COLUMN_PRESET: usize = 3;
const SELECT_PRESET: usize = 4;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const SURFACE_TOKEN_X: usize = 338;
const SURFACE_TOKEN_Y: usize = 34;
const SURFACE_SAMPLE_X_OFFSET: usize = 120;
const SURFACE_SAMPLE_Y_OFFSET: usize = 12;

#[test]
fn shortcut_cheatsheet_exposes_leaf_presets_options_and_selection_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert_eq!(options.len(), REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert!(
        rows.iter()
            .any(|row| row.starts_with("shortcut_cheatsheet.group_layout:"))
    );
    assert!(
        rows.iter()
            .any(|row| row.starts_with("shortcut_cheatsheet.query:"))
    );
    assert_eq!("shortcut_filter_select", spec.action);
    assert_eq!("shortcut_selected", spec.event);
    assert_eq!("shortcut.query", spec.option);
    assert_eq!("format", spec.after);
    assert_eq!("selected=format", spec.state);
}

#[test]
fn shortcut_cheatsheet_presets_render_distinct_filter_layout_and_selection_states() {
    let sample = StorybookVisual.render_preset(DARK_THEME, PAGE, SAMPLE_PRESET, 0);
    let category = StorybookVisual.render_preset(DARK_THEME, PAGE, CATEGORY_PRESET, 0);
    let two_column = StorybookVisual.render_preset(DARK_THEME, PAGE, TWO_COLUMN_PRESET, 0);
    let one_column = StorybookVisual.render_preset(DARK_THEME, PAGE, ONE_COLUMN_PRESET, 0);
    let selected = StorybookVisual.render_preset(DARK_THEME, PAGE, SELECT_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &sample, &category) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &category, &two_column) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &two_column, &one_column) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &one_column, &selected) > BODY_DIFF_THRESHOLD);
}

#[test]
fn shortcut_cheatsheet_setting_option_updates_filter_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn shortcut_cheatsheet_preview_action_updates_selected_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn shortcut_cheatsheet_light_and_dark_surface_uses_theme_surface() {
    assert_cheatsheet_surface_token(DARK_THEME, ThemeSnapshot::dark());
    assert_cheatsheet_surface_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_cheatsheet_surface_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, SAMPLE_PRESET, 0);
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
