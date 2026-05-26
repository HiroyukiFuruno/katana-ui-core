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
const PAGE: &str = "toast-stack-manager";
const POSITION_PRESET: usize = 0;
const DEDUP_PRESET: usize = 1;
const PAUSE_PRESET: usize = 2;
const QUEUE_PRESET: usize = 3;
const ACTION_PRESET: usize = 4;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const TOP_TOAST_X: usize = 232;
const TOP_TOAST_Y: usize = 32;
const TOAST_SAMPLE_X_OFFSET: usize = 120;
const TOAST_SAMPLE_Y_OFFSET: usize = 8;

#[test]
fn toast_stack_manager_exposes_leaf_presets_options_and_enqueue_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("toast_enqueue_visible", spec.action);
    assert_eq!("toast_shown", spec.event);
    assert_eq!("toast_stack.position", spec.option);
    assert_eq!("BottomEnd", spec.after);
    assert_eq!("visible=1", spec.state);
}

#[test]
fn toast_stack_manager_presets_render_distinct_stack_queue_pause_and_action_states() {
    let position = StorybookVisual.render_preset(DARK_THEME, PAGE, POSITION_PRESET, 0);
    let dedup = StorybookVisual.render_preset(DARK_THEME, PAGE, DEDUP_PRESET, 0);
    let pause = StorybookVisual.render_preset(DARK_THEME, PAGE, PAUSE_PRESET, 0);
    let queue = StorybookVisual.render_preset(DARK_THEME, PAGE, QUEUE_PRESET, 0);
    let action = StorybookVisual.render_preset(DARK_THEME, PAGE, ACTION_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &position, &dedup) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &dedup, &pause) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &pause, &queue) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &queue, &action) > BODY_DIFF_THRESHOLD);
}

#[test]
fn toast_stack_manager_setting_option_updates_stack_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn toast_stack_manager_preview_action_updates_visible_stack_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn toast_stack_manager_light_and_dark_top_toast_uses_theme_surface() {
    assert_top_toast_token(DARK_THEME, ThemeSnapshot::dark());
    assert_top_toast_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_top_toast_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, POSITION_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert_eq!(
        Some(colors.surface),
        pixel_at(
            &canvas,
            component.x + TOP_TOAST_X + TOAST_SAMPLE_X_OFFSET,
            component.y + TOP_TOAST_Y + TOAST_SAMPLE_Y_OFFSET
        )
    );
}
