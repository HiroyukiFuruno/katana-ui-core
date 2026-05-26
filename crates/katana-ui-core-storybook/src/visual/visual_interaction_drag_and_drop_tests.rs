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
const PAGE: &str = "drag-and-drop";
const REORDER_PRESET: usize = 0;
const FILE_PRESET: usize = 1;
const TAB_PRESET: usize = 2;
const ATTACHMENT_PRESET: usize = 3;
const KEYBOARD_PRESET: usize = 4;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const SURFACE_TOKEN_X: usize = 338;
const SURFACE_TOKEN_Y: usize = 20;
const SURFACE_SAMPLE_X_OFFSET: usize = 120;
const SURFACE_SAMPLE_Y_OFFSET: usize = 12;

#[test]
fn drag_and_drop_exposes_leaf_presets_options_and_drag_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert_eq!(options.len(), REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert!(
        rows.iter()
            .any(|row| row.starts_with("drag.accept_policy:"))
    );
    assert!(
        rows.iter()
            .any(|row| row.starts_with("drag.keyboard_draggable:"))
    );
    assert_eq!("drag_over", spec.action);
    assert_eq!("drag_over", spec.event);
    assert_eq!("drop_indicator.kind", spec.option);
    assert_eq!("after", spec.after);
    assert_eq!("dragging=true", spec.state);
}

#[test]
fn drag_and_drop_presets_render_distinct_payload_target_and_keyboard_states() {
    let reorder = StorybookVisual.render_preset(DARK_THEME, PAGE, REORDER_PRESET, 0);
    let file = StorybookVisual.render_preset(DARK_THEME, PAGE, FILE_PRESET, 0);
    let tab = StorybookVisual.render_preset(DARK_THEME, PAGE, TAB_PRESET, 0);
    let attachment = StorybookVisual.render_preset(DARK_THEME, PAGE, ATTACHMENT_PRESET, 0);
    let keyboard = StorybookVisual.render_preset(DARK_THEME, PAGE, KEYBOARD_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &reorder, &file) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &file, &tab) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &tab, &attachment) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &attachment, &keyboard) > BODY_DIFF_THRESHOLD);
}

#[test]
fn drag_and_drop_setting_option_updates_acceptance_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn drag_and_drop_preview_action_updates_dragging_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn drag_and_drop_light_and_dark_surface_uses_theme_surface() {
    assert_drag_surface_token(DARK_THEME, ThemeSnapshot::dark());
    assert_drag_surface_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_drag_surface_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, REORDER_PRESET, 0);
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
