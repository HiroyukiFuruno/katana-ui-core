use super::interaction_spec::StorybookInteractionSpec;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at,
};
use super::window_interaction::{
    StorybookWindowState, apply_clickable_keyboard_activation_for_audit, apply_hover_at,
    focus_clickable_at_for_audit,
};
use super::{StorybookVisual, palette, preview_detail, storybook_ui_option_contract};
use crate::catalog::StoryPresetLabels;
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "segmented-toggle";
const SEGMENTS_PRESET: usize = 0;
const SELECT_PRESET: usize = 1;
const DISABLED_PRESET: usize = 2;
const THEME_PRESET: usize = 3;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const SEGMENT_X_OFFSET: usize = 18;
const SEGMENT_Y_OFFSET: usize = 44;
const SEGMENT_WIDTH: usize = 92;
const SEGMENT_FILL_SAMPLE_X_OFFSET: usize = 4;
const SEGMENT_FILL_SAMPLE_Y_OFFSET: usize = 4;

#[test]
fn segmented_toggle_exposes_leaf_presets_options_and_selection_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("segment_select", spec.action);
    assert_eq!("segmented_toggle_selected", spec.event);
    assert_eq!("interaction.selected_index", spec.option);
    assert_eq!("1", spec.after);
    assert_eq!("segment=1", spec.state);
}

#[test]
fn segmented_toggle_presets_render_distinct_segment_bodies() {
    let segments = StorybookVisual.render_preset(DARK_THEME, PAGE, SEGMENTS_PRESET, 0);
    let selected = StorybookVisual.render_preset(DARK_THEME, PAGE, SELECT_PRESET, 0);
    let disabled = StorybookVisual.render_preset(DARK_THEME, PAGE, DISABLED_PRESET, 0);
    let themed = StorybookVisual.render_preset(DARK_THEME, PAGE, THEME_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &segments, &selected) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &segments, &disabled) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &segments, &themed) > BODY_DIFF_THRESHOLD);
}

#[test]
fn segmented_toggle_setting_option_updates_selection_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn segmented_toggle_preview_action_updates_selection_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn segmented_toggle_live_hover_focus_keyboard_and_disabled_guard_use_core_actions() {
    let mut state = page_state();
    let target = preview_detail::component_action_hit_rect(PAGE);
    let before_hover = render_state(&state);
    assert!(apply_hover_at(&mut state, target.x + 1, target.y + 1));
    let after_hover = render_state(&state);

    assert_eq!("segment_hover", state.screen_state.last_action);
    assert_eq!("hover_start", state.screen_state.last_event);
    assert_eq!("hover=true", state.screen_state.state_label);
    assert!(state.screen_state.segmented_toggle.hovered);
    assert!(component_body_pixel_diff(PAGE, &before_hover, &after_hover) > 0);

    let before_focus = render_state(&state);
    assert!(focus_clickable_at_for_audit(
        &mut state,
        target.x + 1,
        target.y + 1
    ));
    let after_focus = render_state(&state);

    assert_eq!("segment_focus", state.screen_state.last_action);
    assert_eq!("focus", state.screen_state.last_event);
    assert_eq!("focus=true", state.screen_state.state_label);
    assert!(state.screen_state.is_button_focused());
    assert!(state.screen_state.segmented_toggle.focused);
    assert!(component_body_pixel_diff(PAGE, &before_focus, &after_focus) > 0);

    let before_keyboard = render_state(&state);
    assert!(apply_clickable_keyboard_activation_for_audit(&mut state));
    let after_keyboard = render_state(&state);

    assert_eq!("segment_keyboard_select", state.screen_state.last_action);
    assert_eq!("segmented_toggle_selected", state.screen_state.last_event);
    assert_eq!("segment=1", state.screen_state.state_label);
    assert_eq!(1, state.screen_state.segmented_toggle.selected_index);
    assert!(component_body_pixel_diff(PAGE, &before_keyboard, &after_keyboard) > 0);

    state.screen_state.register_segmented_toggle_action(
        super::screen_state_segmented_toggle::SegmentedToggleScreenAction::DisabledSelect,
    );
    assert_eq!("segment_disabled_select", state.screen_state.last_action);
    assert_eq!("segmented_toggle_ignored", state.screen_state.last_event);
    assert!(state.screen_state.segmented_toggle.disabled_blocked);
}

#[test]
fn segmented_toggle_light_and_dark_segments_use_theme_tokens() {
    assert_segment_tokens(DARK_THEME, ThemeSnapshot::dark());
    assert_segment_tokens(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_segment_tokens(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, SEGMENTS_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let rect = preview_detail::component_action_hit_rect(PAGE);
    let active_x = rect.x + SEGMENT_X_OFFSET;
    let inactive_x = active_x + SEGMENT_WIDTH;
    let segment_y = rect.y + SEGMENT_Y_OFFSET;

    assert_eq!(Some(colors.border), pixel_at(&canvas, active_x, segment_y));
    assert_eq!(
        Some(colors.accent),
        pixel_at(
            &canvas,
            active_x + SEGMENT_FILL_SAMPLE_X_OFFSET,
            segment_y + SEGMENT_FILL_SAMPLE_Y_OFFSET
        )
    );
    assert_eq!(
        Some(colors.panel),
        pixel_at(
            &canvas,
            inactive_x + SEGMENT_FILL_SAMPLE_X_OFFSET,
            segment_y + SEGMENT_FILL_SAMPLE_Y_OFFSET
        )
    );
}

fn render_state(state: &StorybookWindowState) -> super::Canvas {
    super::render::render_storybook_canvas_with_screen_state(
        DARK_THEME,
        PAGE,
        0,
        state.screen_state.clone(),
    )
}

fn page_state() -> StorybookWindowState {
    StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    }
}
