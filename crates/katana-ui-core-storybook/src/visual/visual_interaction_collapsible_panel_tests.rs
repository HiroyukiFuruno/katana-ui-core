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
const PAGE: &str = "collapsible-panel";
const EXPLORER_PRESET: usize = 0;
const CHAT_HISTORY_PRESET: usize = 1;
const TOC_PRESET: usize = 2;
const FLOATING_PRESET: usize = 3;
const ICON_ONLY_PRESET: usize = 4;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const PANEL_X: usize = 34;
const PANEL_Y: usize = 30;
const PANEL_SAMPLE_X_OFFSET: usize = 226;
const PANEL_SAMPLE_Y_OFFSET: usize = 84;

#[test]
fn collapsible_panel_exposes_leaf_presets_options_and_panel_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert!(
        rows.iter()
            .any(|row| row.starts_with("collapsible_panel.mode:"))
    );
    assert!(
        rows.iter()
            .any(|row| row.starts_with("collapsible_panel.width:"))
    );
    assert_eq!("collapsible_panel_resize", spec.action);
    assert_eq!("collapsible_panel_width_changed", spec.event);
    assert_eq!("collapsible_panel.width", spec.option);
    assert_eq!("320", spec.after);
    assert_eq!("mode=floating_overlay", spec.state);
}

#[test]
fn collapsible_panel_presets_render_distinct_sidebar_modes() {
    let explorer = StorybookVisual.render_preset(DARK_THEME, PAGE, EXPLORER_PRESET, 0);
    let chat = StorybookVisual.render_preset(DARK_THEME, PAGE, CHAT_HISTORY_PRESET, 0);
    let toc = StorybookVisual.render_preset(DARK_THEME, PAGE, TOC_PRESET, 0);
    let floating = StorybookVisual.render_preset(DARK_THEME, PAGE, FLOATING_PRESET, 0);
    let icon_only = StorybookVisual.render_preset(DARK_THEME, PAGE, ICON_ONLY_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &explorer, &chat) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &chat, &toc) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &toc, &floating) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &floating, &icon_only) > BODY_DIFF_THRESHOLD);
}

#[test]
fn collapsible_panel_setting_option_updates_width_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn collapsible_panel_preview_action_updates_panel_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn collapsible_panel_light_and_dark_panel_uses_theme_surface() {
    assert_panel_token(DARK_THEME, ThemeSnapshot::dark());
    assert_panel_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_panel_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, EXPLORER_PRESET, 0);
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
