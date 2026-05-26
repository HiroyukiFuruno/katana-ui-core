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
const PAGE: &str = "menu";
const MENU_ITEMS_PRESET: usize = 0;
const SHORTCUT_PRESET: usize = 1;
const DISABLED_PRESET: usize = 2;
const THEME_PRESET: usize = 3;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const ROW_X: usize = 36;
const FIRST_ROW_Y: usize = 38;
const ROW_SAMPLE_OFFSET: usize = 4;

#[test]
fn menu_exposes_leaf_presets_options_and_menu_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("menu_open", spec.action);
    assert_eq!("menu_opened", spec.event);
    assert_eq!("interaction.open", spec.option);
    assert_eq!("true", spec.after);
    assert_eq!("open=true", spec.state);
}

#[test]
fn menu_presets_render_distinct_menu_bodies() {
    let items = StorybookVisual.render_preset(DARK_THEME, PAGE, MENU_ITEMS_PRESET, 0);
    let shortcut = StorybookVisual.render_preset(DARK_THEME, PAGE, SHORTCUT_PRESET, 0);
    let disabled = StorybookVisual.render_preset(DARK_THEME, PAGE, DISABLED_PRESET, 0);
    let themed = StorybookVisual.render_preset(DARK_THEME, PAGE, THEME_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &items, &shortcut) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &shortcut, &disabled) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &items, &themed) > BODY_DIFF_THRESHOLD);
}

#[test]
fn menu_setting_option_updates_menu_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn menu_preview_action_updates_menu_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn menu_light_and_dark_row_uses_theme_surface() {
    assert_row_token(DARK_THEME, ThemeSnapshot::dark());
    assert_row_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_row_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, MENU_ITEMS_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert_eq!(
        Some(colors.surface),
        pixel_at(
            &canvas,
            component.x + ROW_X + ROW_SAMPLE_OFFSET,
            component.y + FIRST_ROW_Y + ROW_SAMPLE_OFFSET
        )
    );
}
