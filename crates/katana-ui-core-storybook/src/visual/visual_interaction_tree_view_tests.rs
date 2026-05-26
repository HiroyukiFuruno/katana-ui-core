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
const PAGE: &str = "tree-view";
const FOLDERS_PRESET: usize = 0;
const TOGGLE_PRESET: usize = 1;
const CONTEXT_PRESET: usize = 2;
const THEME_TREE_PRESET: usize = 3;
const VIRTUALIZATION_PRESET: usize = 4;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const TREE_PANEL_X: usize = 14;
const TREE_PANEL_Y: usize = 30;
const TREE_SAMPLE_X_OFFSET: usize = 168;
const TREE_SAMPLE_Y_OFFSET: usize = 62;

#[test]
fn tree_view_exposes_leaf_presets_options_and_toggle_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert!(rows.iter().any(|row| row.starts_with("line:")));
    assert!(rows.iter().any(|row| row.starts_with("trigger:")));
    assert_eq!("tree_click_toggle", spec.action);
    assert_eq!("tree_toggled", spec.event);
    assert_eq!("interaction.open", spec.option);
    assert_eq!("false", spec.after);
    assert_eq!("open=false", spec.state);
}

#[test]
fn tree_view_presets_render_distinct_folder_toggle_context_theme_and_virtual_states() {
    let folders = StorybookVisual.render_preset(DARK_THEME, PAGE, FOLDERS_PRESET, 0);
    let toggle = StorybookVisual.render_preset(DARK_THEME, PAGE, TOGGLE_PRESET, 0);
    let context = StorybookVisual.render_preset(DARK_THEME, PAGE, CONTEXT_PRESET, 0);
    let themed = StorybookVisual.render_preset(DARK_THEME, PAGE, THEME_TREE_PRESET, 0);
    let virtualized = StorybookVisual.render_preset(DARK_THEME, PAGE, VIRTUALIZATION_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &folders, &toggle) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &toggle, &context) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &context, &themed) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &themed, &virtualized) > BODY_DIFF_THRESHOLD);
}

#[test]
fn tree_view_setting_option_updates_tree_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn tree_view_preview_action_updates_toggle_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn tree_view_light_and_dark_panel_uses_theme_surface() {
    assert_tree_token(DARK_THEME, ThemeSnapshot::dark());
    assert_tree_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_tree_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, FOLDERS_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert_eq!(
        Some(colors.surface),
        pixel_at(
            &canvas,
            component.x + TREE_PANEL_X + TREE_SAMPLE_X_OFFSET,
            component.y + TREE_PANEL_Y + TREE_SAMPLE_Y_OFFSET
        )
    );
}
