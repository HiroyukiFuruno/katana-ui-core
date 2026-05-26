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
const PAGE: &str = "closeable-tab-strip";
const DEFAULT_PRESET: usize = 0;
const OVERFLOW_PRESET: usize = 1;
const PINNED_PRESET: usize = 2;
const GROUPS_PRESET: usize = 3;
const DIRTY_PRESET: usize = 4;
const DRAGGING_PRESET: usize = 5;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const STRIP_X: usize = 30;
const STRIP_Y: usize = 42;
const STRIP_SAMPLE_X_OFFSET: usize = 430;
const STRIP_SAMPLE_Y_OFFSET: usize = 10;

#[test]
fn closeable_tab_strip_exposes_leaf_presets_options_and_tab_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("select_tab", spec.action);
    assert_eq!("closeable_tab_selected", spec.event);
    assert_eq!("active_tab_id", spec.option);
    assert_eq!("settings", spec.after);
    assert_eq!("active=settings", spec.state);
}

#[test]
fn closeable_tab_strip_presets_render_distinct_tab_lifecycle_states() {
    let default = StorybookVisual.render_preset(DARK_THEME, PAGE, DEFAULT_PRESET, 0);
    let overflow = StorybookVisual.render_preset(DARK_THEME, PAGE, OVERFLOW_PRESET, 0);
    let pinned = StorybookVisual.render_preset(DARK_THEME, PAGE, PINNED_PRESET, 0);
    let groups = StorybookVisual.render_preset(DARK_THEME, PAGE, GROUPS_PRESET, 0);
    let dirty = StorybookVisual.render_preset(DARK_THEME, PAGE, DIRTY_PRESET, 0);
    let dragging = StorybookVisual.render_preset(DARK_THEME, PAGE, DRAGGING_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &default, &overflow) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &overflow, &pinned) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &pinned, &groups) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &groups, &dirty) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &dirty, &dragging) > BODY_DIFF_THRESHOLD);
}

#[test]
fn closeable_tab_strip_setting_option_updates_tab_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn closeable_tab_strip_preview_action_updates_active_tab_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn closeable_tab_strip_light_and_dark_strip_uses_theme_surface() {
    assert_strip_token(DARK_THEME, ThemeSnapshot::dark());
    assert_strip_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_strip_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, DEFAULT_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert_eq!(
        Some(colors.surface),
        pixel_at(
            &canvas,
            component.x + STRIP_X + STRIP_SAMPLE_X_OFFSET,
            component.y + STRIP_Y + STRIP_SAMPLE_Y_OFFSET
        )
    );
}
