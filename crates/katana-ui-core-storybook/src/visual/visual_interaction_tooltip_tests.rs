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
const PAGE: &str = "tooltip";
const ANCHOR_PRESET: usize = 0;
const HOVER_PRESET: usize = 1;
const EDGE_PRESET: usize = 2;
const THEME_PRESET: usize = 3;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const BUBBLE_X: usize = 112;
const BUBBLE_Y: usize = 34;
const BUBBLE_SAMPLE_OFFSET: usize = 8;

#[test]
fn tooltip_exposes_leaf_presets_options_and_hover_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("tooltip_hover", spec.action);
    assert_eq!("tooltip_opened", spec.event);
    assert_eq!("interaction.hovered", spec.option);
    assert_eq!("true", spec.after);
    assert_eq!("hover=true", spec.state);
}

#[test]
fn tooltip_presets_render_distinct_anchor_hover_edge_and_theme_bodies() {
    let anchor = StorybookVisual.render_preset(DARK_THEME, PAGE, ANCHOR_PRESET, 0);
    let hover = StorybookVisual.render_preset(DARK_THEME, PAGE, HOVER_PRESET, 0);
    let edge = StorybookVisual.render_preset(DARK_THEME, PAGE, EDGE_PRESET, 0);
    let themed = StorybookVisual.render_preset(DARK_THEME, PAGE, THEME_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &anchor, &hover) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &hover, &edge) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &anchor, &themed) > BODY_DIFF_THRESHOLD);
}

#[test]
fn tooltip_setting_option_updates_overlay_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn tooltip_preview_action_opens_hover_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn tooltip_light_and_dark_bubble_uses_theme_surface() {
    assert_bubble_token(DARK_THEME, ThemeSnapshot::dark());
    assert_bubble_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_bubble_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, ANCHOR_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert_eq!(
        Some(colors.surface),
        pixel_at(
            &canvas,
            component.x + BUBBLE_X + BUBBLE_SAMPLE_OFFSET,
            component.y + BUBBLE_Y + BUBBLE_SAMPLE_OFFSET
        )
    );
}
