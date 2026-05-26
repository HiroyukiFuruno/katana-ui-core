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
const PAGE: &str = "popover";
const ANCHOR_PRESET: usize = 0;
const PLACEMENT_PRESET: usize = 1;
const AUTO_FLIP_PRESET: usize = 2;
const OFFSET_WIDTH_PRESET: usize = 3;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const PANEL_X: usize = 116;
const PANEL_Y: usize = 34;
const PANEL_SAMPLE_OFFSET: usize = 8;

#[test]
fn popover_exposes_leaf_presets_options_and_open_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("popover_open", spec.action);
    assert_eq!("popover_opened", spec.event);
    assert_eq!("interaction.open", spec.option);
    assert_eq!("true", spec.after);
    assert_eq!("open=true", spec.state);
}

#[test]
fn popover_presets_render_distinct_anchor_placement_flip_and_width_bodies() {
    let anchor = StorybookVisual.render_preset(DARK_THEME, PAGE, ANCHOR_PRESET, 0);
    let placement = StorybookVisual.render_preset(DARK_THEME, PAGE, PLACEMENT_PRESET, 0);
    let auto_flip = StorybookVisual.render_preset(DARK_THEME, PAGE, AUTO_FLIP_PRESET, 0);
    let offset_width = StorybookVisual.render_preset(DARK_THEME, PAGE, OFFSET_WIDTH_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &anchor, &placement) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &placement, &auto_flip) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &anchor, &offset_width) > BODY_DIFF_THRESHOLD);
}

#[test]
fn popover_setting_option_updates_panel_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn popover_preview_action_opens_panel_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn popover_light_and_dark_panel_uses_theme_surface() {
    assert_panel_token(DARK_THEME, ThemeSnapshot::dark());
    assert_panel_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_panel_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, ANCHOR_PRESET, 0);
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
