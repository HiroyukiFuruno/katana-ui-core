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
const PAGE: &str = "search-control-strip";
const WORKSPACE_PRESET: usize = 0;
const FIND_PRESET: usize = 1;
const REPLACE_PRESET: usize = 2;
const VIEWER_PRESET: usize = 3;
const HISTORY_PRESET: usize = 4;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const SURFACE_TOKEN_X: usize = 338;
const SURFACE_TOKEN_Y: usize = 34;
const SURFACE_SAMPLE_X_OFFSET: usize = 120;
const SURFACE_SAMPLE_Y_OFFSET: usize = 12;

#[test]
fn search_control_strip_exposes_leaf_presets_options_and_query_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert!(
        rows.iter()
            .any(|row| row.starts_with("search_control.query:"))
    );
    assert!(
        rows.iter()
            .any(|row| row.starts_with("search_control.use_regex:"))
    );
    assert_eq!("search_query_changed", spec.action);
    assert_eq!("search_query_changed", spec.event);
    assert_eq!("search_control.query", spec.option);
    assert_eq!("heading", spec.after);
    assert_eq!("query=heading", spec.state);
}

#[test]
fn search_control_strip_presets_render_distinct_query_replace_viewer_and_history_states() {
    let workspace = StorybookVisual.render_preset(DARK_THEME, PAGE, WORKSPACE_PRESET, 0);
    let find = StorybookVisual.render_preset(DARK_THEME, PAGE, FIND_PRESET, 0);
    let replace = StorybookVisual.render_preset(DARK_THEME, PAGE, REPLACE_PRESET, 0);
    let viewer = StorybookVisual.render_preset(DARK_THEME, PAGE, VIEWER_PRESET, 0);
    let history = StorybookVisual.render_preset(DARK_THEME, PAGE, HISTORY_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &workspace, &find) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &find, &replace) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &replace, &viewer) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &viewer, &history) > BODY_DIFF_THRESHOLD);
}

#[test]
fn search_control_strip_setting_option_updates_query_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn search_control_strip_preview_action_updates_query_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn search_control_strip_live_focus_hover_and_keyboard_use_core_actions() {
    let mut state = page_state();
    let target = preview_detail::component_action_hit_rect(PAGE);
    let before_hover = render_state(&state);
    assert!(apply_hover_at(&mut state, target.x + 1, target.y + 1));
    let after_hover = render_state(&state);

    assert_eq!("search_control_hover", state.screen_state.last_action);
    assert_eq!("hover_start", state.screen_state.last_event);
    assert_eq!("hover=true", state.screen_state.state_label);
    assert!(state.screen_state.search_control.hovered);
    assert!(component_body_pixel_diff(PAGE, &before_hover, &after_hover) > 0);

    let before_focus = render_state(&state);
    assert!(focus_clickable_at_for_audit(
        &mut state,
        target.x + 1,
        target.y + 1
    ));
    let after_focus = render_state(&state);

    assert_eq!("search_control_focus", state.screen_state.last_action);
    assert_eq!("focus", state.screen_state.last_event);
    assert_eq!("focus=true", state.screen_state.state_label);
    assert!(state.screen_state.is_button_focused());
    assert!(state.screen_state.search_control.focused);
    assert!(component_body_pixel_diff(PAGE, &before_focus, &after_focus) > 0);

    let before_keyboard = render_state(&state);
    assert!(apply_clickable_keyboard_activation_for_audit(&mut state));
    let after_keyboard = render_state(&state);

    assert_eq!(
        "search_control_keyboard_next",
        state.screen_state.last_action
    );
    assert_eq!("search_navigation_requested", state.screen_state.last_event);
    assert_eq!("navigation=next", state.screen_state.state_label);
    assert!(state.screen_state.search_control.navigated_next);
    assert!(component_body_pixel_diff(PAGE, &before_keyboard, &after_keyboard) > 0);
}

#[test]
fn search_control_strip_light_and_dark_surface_uses_theme_surface() {
    assert_search_control_surface_token(DARK_THEME, ThemeSnapshot::dark());
    assert_search_control_surface_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_search_control_surface_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, WORKSPACE_PRESET, 0);
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
