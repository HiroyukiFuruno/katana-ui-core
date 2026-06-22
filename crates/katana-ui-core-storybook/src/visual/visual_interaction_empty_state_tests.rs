use super::interaction_spec::StorybookInteractionSpec;
use super::screen_state::StorybookScreenState;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at,
};
use super::window_interaction::{
    StorybookWindowState, apply_click, apply_clickable_keyboard_activation_for_audit,
    apply_hover_at, focus_clickable_at_for_audit,
};
use super::{StorybookVisual, palette, preview_detail, render, storybook_ui_option_contract};
use crate::catalog::StoryPresetLabels;
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "empty-state";
const EXPLORER_PRESET: usize = 0;
const SEARCH_PRESET: usize = 1;
const CLEAN_PRESET: usize = 2;
const HISTORY_PRESET: usize = 3;
const ERROR_PRESET: usize = 4;
const HEADING_PRESET: usize = 5;
const BODY_PRESET: usize = 6;
const ICON_PRESET: usize = 7;
const ILLUSTRATION_PRESET: usize = 8;
const REQUIRED_PRESET_COUNT: usize = 9;
const REQUIRED_OPTION_COUNT: usize = 8;
const BODY_DIFF_THRESHOLD: usize = 80;
const SURFACE_TOKEN_X: usize = 338;
const SURFACE_TOKEN_Y: usize = 34;
const SURFACE_SAMPLE_X_OFFSET: usize = 120;
const SURFACE_SAMPLE_Y_OFFSET: usize = 12;

#[test]
fn empty_state_exposes_leaf_presets_options_and_action_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    for option in [
        "empty_state.heading",
        "empty_state.body",
        "empty_state.icon",
        "empty_state.illustration",
        "empty_state.tone",
        "empty_state.size",
        "empty_state.alignment",
        "empty_state.actions",
    ] {
        assert!(
            options.iter().any(|it| it.setting == option),
            "empty-state option is not exposed: {option}"
        );
    }
    assert!(rows.iter().any(|row| row.starts_with("empty_state.tone:")));
    assert!(
        rows.iter()
            .any(|row| row.starts_with("empty_state.alignment:"))
    );
    assert_eq!("empty_state_primary", spec.action);
    assert_eq!("empty_state_actioned", spec.event);
    assert_eq!("empty_state.primary_action", spec.option);
    assert_eq!("reload", spec.after);
    assert_eq!("action=reload", spec.state);
}

#[test]
fn empty_state_presets_render_distinct_empty_clean_history_and_error_states() {
    let explorer = StorybookVisual.render_preset(DARK_THEME, PAGE, EXPLORER_PRESET, 0);
    let search = StorybookVisual.render_preset(DARK_THEME, PAGE, SEARCH_PRESET, 0);
    let clean = StorybookVisual.render_preset(DARK_THEME, PAGE, CLEAN_PRESET, 0);
    let history = StorybookVisual.render_preset(DARK_THEME, PAGE, HISTORY_PRESET, 0);
    let error = StorybookVisual.render_preset(DARK_THEME, PAGE, ERROR_PRESET, 0);
    let heading = StorybookVisual.render_preset(DARK_THEME, PAGE, HEADING_PRESET, 0);
    let body = StorybookVisual.render_preset(DARK_THEME, PAGE, BODY_PRESET, 0);
    let icon = StorybookVisual.render_preset(DARK_THEME, PAGE, ICON_PRESET, 0);
    let illustration = StorybookVisual.render_preset(DARK_THEME, PAGE, ILLUSTRATION_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &explorer, &search) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &search, &clean) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &clean, &history) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &history, &error) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &error, &heading) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &heading, &body) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &body, &icon) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &icon, &illustration) > BODY_DIFF_THRESHOLD);
}

#[test]
fn empty_state_setting_option_updates_alignment_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn empty_state_preview_action_updates_primary_action_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn empty_state_preview_actions_expose_hover_focus_and_keyboard_ports() {
    assert_empty_state_preview_action(
        "empty-state-hover",
        "empty_state_hover",
        "hover_start",
        "hover=primary",
    );
    assert_empty_state_preview_action(
        "empty-state-focus",
        "empty_state_focus",
        "focus",
        "focus=primary",
    );
    assert_empty_state_preview_action(
        "empty-state-keyboard",
        "empty_state_keyboard_primary",
        "empty_state_actioned",
        "keyboard=reload",
    );
}

#[test]
fn empty_state_live_operations_route_primary_action_focus_hover_and_keyboard() {
    let mut state = page_state();
    let target = preview_detail::component_action_hit_rect(PAGE);
    let before = render_state(&state);

    assert!(apply_click(&mut state, target.x + 4, target.y + 4));
    assert_eq!("empty_state_primary", state.screen_state.last_action);
    assert_eq!("empty_state_actioned", state.screen_state.last_event);
    assert_eq!("action=reload", state.screen_state.state_label);
    assert!(component_body_pixel_diff(PAGE, &before, &render_state(&state)) > 0);

    let hover_before = render_state(&state);
    assert!(apply_hover_at(
        &mut state,
        target.x + target.width / 2,
        target.y + target.height / 2
    ));
    assert_eq!("empty_state_hover", state.screen_state.last_action);
    assert_eq!("hover_start", state.screen_state.last_event);
    assert!(state.screen_state.preview_hovered);
    assert!(component_body_pixel_diff(PAGE, &hover_before, &render_state(&state)) > 0);

    let focus_before = render_state(&state);
    assert!(focus_clickable_at_for_audit(
        &mut state,
        target.x + 4,
        target.y + 4
    ));
    assert_eq!("empty_state_focus", state.screen_state.last_action);
    assert_eq!("focus", state.screen_state.last_event);
    assert_eq!("focus=primary", state.screen_state.state_label);
    assert!(state.screen_state.is_button_focused());
    assert!(component_body_pixel_diff(PAGE, &focus_before, &render_state(&state)) > 0);

    let keyboard_before = render_state(&state);
    assert!(apply_clickable_keyboard_activation_for_audit(&mut state));
    assert_eq!(
        "empty_state_keyboard_primary",
        state.screen_state.last_action
    );
    assert_eq!("empty_state_actioned", state.screen_state.last_event);
    assert_eq!("keyboard=reload", state.screen_state.state_label);
    assert!(component_body_pixel_diff(PAGE, &keyboard_before, &render_state(&state)) > 0);
}

#[test]
fn empty_state_light_and_dark_surface_uses_theme_surface() {
    assert_empty_state_surface_token(DARK_THEME, ThemeSnapshot::dark());
    assert_empty_state_surface_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn render_state(state: &StorybookWindowState) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    )
}

fn page_state() -> StorybookWindowState {
    StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    }
}

fn assert_empty_state_preview_action(
    page: &str,
    expected_action: &str,
    expected_event: &str,
    expected_state: &str,
) {
    let mut state = StorybookScreenState::default();

    state.register_preview_action(page);

    assert_eq!(expected_action, state.last_action);
    assert_eq!(expected_event, state.last_event);
    assert_eq!(expected_state, state.state_label);
}

fn assert_empty_state_surface_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, EXPLORER_PRESET, 0);
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
