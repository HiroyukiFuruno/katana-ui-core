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
const PAGE: &str = "context-menu";
const EDITOR_PRESET: usize = 0;
const EXPLORER_PRESET: usize = 1;
const TAB_BAR_PRESET: usize = 2;
const ICON_SHORTCUT_PRESET: usize = 4;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const MENU_X: usize = 128;
const MENU_Y: usize = 30;
const MENU_SAMPLE_OFFSET: usize = 4;

#[test]
fn context_menu_exposes_leaf_presets_options_and_context_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("context_menu_open", spec.action);
    assert_eq!("context_menu_opened", spec.event);
    assert_eq!("context_menu.anchor", spec.option);
    assert_eq!("Pointer(192,128)", spec.after);
    assert_eq!("context_menu=open", spec.state);
}

#[test]
fn context_menu_presets_render_distinct_anchor_and_menu_bodies() {
    let editor = StorybookVisual.render_preset(DARK_THEME, PAGE, EDITOR_PRESET, 0);
    let explorer = StorybookVisual.render_preset(DARK_THEME, PAGE, EXPLORER_PRESET, 0);
    let tab_bar = StorybookVisual.render_preset(DARK_THEME, PAGE, TAB_BAR_PRESET, 0);
    let shortcut = StorybookVisual.render_preset(DARK_THEME, PAGE, ICON_SHORTCUT_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &editor, &explorer) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &explorer, &tab_bar) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &tab_bar, &shortcut) > BODY_DIFF_THRESHOLD);
}

#[test]
fn context_menu_setting_option_updates_context_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn context_menu_preview_action_updates_context_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn context_menu_light_and_dark_panel_uses_theme_panel() {
    assert_menu_panel_token(DARK_THEME, ThemeSnapshot::dark());
    assert_menu_panel_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_menu_panel_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, EDITOR_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert_eq!(
        Some(colors.panel),
        pixel_at(
            &canvas,
            component.x + MENU_X + MENU_SAMPLE_OFFSET,
            component.y + MENU_Y + MENU_SAMPLE_OFFSET
        )
    );
}
