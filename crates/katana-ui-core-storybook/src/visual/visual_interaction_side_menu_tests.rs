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
const PAGE: &str = "side-menu";
const NAV_PRESET: usize = 0;
const SELECT_PRESET: usize = 1;
const COLLAPSE_PRESET: usize = 2;
const THEME_PRESET: usize = 3;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const PANEL_X: usize = 32;
const PANEL_Y: usize = 28;
const PANEL_SAMPLE_X_OFFSET: usize = 184;
const PANEL_SAMPLE_Y_OFFSET: usize = 14;

#[test]
fn side_menu_exposes_leaf_presets_options_and_route_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("side_menu_select", spec.action);
    assert_eq!("route_changed", spec.event);
    assert_eq!("interaction.selected_index", spec.option);
    assert_eq!("1", spec.after);
    assert_eq!("route=1", spec.state);
}

#[test]
fn side_menu_presets_render_distinct_nav_select_collapse_and_theme_states() {
    let nav = StorybookVisual.render_preset(DARK_THEME, PAGE, NAV_PRESET, 0);
    let select = StorybookVisual.render_preset(DARK_THEME, PAGE, SELECT_PRESET, 0);
    let collapse = StorybookVisual.render_preset(DARK_THEME, PAGE, COLLAPSE_PRESET, 0);
    let themed = StorybookVisual.render_preset(DARK_THEME, PAGE, THEME_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &nav, &select) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &select, &collapse) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &nav, &themed) > BODY_DIFF_THRESHOLD);
}

#[test]
fn side_menu_setting_option_updates_route_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn side_menu_preview_action_updates_route_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn side_menu_light_and_dark_panel_uses_theme_surface() {
    assert_panel_token(DARK_THEME, ThemeSnapshot::dark());
    assert_panel_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_panel_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, NAV_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert_eq!(
        Some(colors.surface),
        pixel_at(
            &canvas,
            component.x + PANEL_X + PANEL_SAMPLE_X_OFFSET,
            component.y + PANEL_Y + PANEL_SAMPLE_Y_OFFSET
        )
    );
}
