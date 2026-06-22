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
const PAGE: &str = "notification-toast";
const TOAST_PRESET: usize = 0;
const DISMISS_PRESET: usize = 1;
const STACK_PRESET: usize = 2;
const THEME_PRESET: usize = 3;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const COMPONENT_HIT_INSET: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const TOAST_X: usize = 46;
const TOAST_Y: usize = 36;
const TOAST_SAMPLE_OFFSET: usize = 8;

#[test]
fn notification_toast_exposes_leaf_presets_options_and_dismiss_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("toast_dismiss", spec.action);
    assert_eq!("toast_dismissed", spec.event);
    assert_eq!("interaction.open", spec.option);
    assert_eq!("false", spec.after);
    assert_eq!("visible=false", spec.state);
}

#[test]
fn notification_toast_presets_render_distinct_toast_dismiss_stack_and_theme_states() {
    let toast = StorybookVisual.render_preset(DARK_THEME, PAGE, TOAST_PRESET, 0);
    let dismiss = StorybookVisual.render_preset(DARK_THEME, PAGE, DISMISS_PRESET, 0);
    let stack = StorybookVisual.render_preset(DARK_THEME, PAGE, STACK_PRESET, 0);
    let themed = StorybookVisual.render_preset(DARK_THEME, PAGE, THEME_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &toast, &dismiss) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &dismiss, &stack) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &toast, &themed) > BODY_DIFF_THRESHOLD);
}

#[test]
fn notification_toast_setting_option_updates_toast_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn notification_toast_preview_action_updates_dismiss_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn notification_toast_live_hover_focus_and_keyboard_dismiss_use_core_actions() {
    let mut hover_state = page_state();
    let hover_before = render_state(&hover_state);
    assert!(apply_hover_at(&mut hover_state, toast_x(), toast_y()));
    let hover_after = render_state(&hover_state);

    assert_eq!("toast_hover", hover_state.screen_state.last_action);
    assert_eq!("toast_hovered", hover_state.screen_state.last_event);
    assert_eq!("hover=true", hover_state.screen_state.state_label);
    assert!(hover_state.screen_state.preview_hovered);
    assert!(component_body_pixel_diff(PAGE, &hover_before, &hover_after) > 0);

    let mut keyboard_state = page_state();
    let focus_before = render_state(&keyboard_state);
    assert!(focus_clickable_at_for_audit(
        &mut keyboard_state,
        toast_x(),
        toast_y()
    ));
    let focus_after = render_state(&keyboard_state);

    assert_eq!("toast_focus", keyboard_state.screen_state.last_action);
    assert_eq!("toast_focused", keyboard_state.screen_state.last_event);
    assert_eq!("focus=true", keyboard_state.screen_state.state_label);
    assert!(keyboard_state.screen_state.is_button_focused());
    assert!(component_body_pixel_diff(PAGE, &focus_before, &focus_after) > 0);

    let keyboard_before = render_state(&keyboard_state);
    assert!(apply_clickable_keyboard_activation_for_audit(
        &mut keyboard_state
    ));
    let keyboard_after = render_state(&keyboard_state);

    assert_eq!(
        "toast_keyboard_dismiss",
        keyboard_state.screen_state.last_action
    );
    assert_eq!("toast_dismissed", keyboard_state.screen_state.last_event);
    assert_eq!("visible=false", keyboard_state.screen_state.state_label);
    assert!(component_body_pixel_diff(PAGE, &keyboard_before, &keyboard_after) > 0);
}

#[test]
fn notification_toast_light_and_dark_toast_uses_theme_surface() {
    assert_toast_token(DARK_THEME, ThemeSnapshot::dark());
    assert_toast_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_toast_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, TOAST_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert_eq!(
        Some(colors.surface),
        pixel_at(
            &canvas,
            component.x + TOAST_X + TOAST_SAMPLE_OFFSET,
            component.y + TOAST_Y + TOAST_SAMPLE_OFFSET
        )
    );
}

fn render_state(state: &StorybookWindowState) -> super::Canvas {
    super::render::render_storybook_canvas_with_screen_state(
        DARK_THEME,
        PAGE,
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

fn toast_x() -> usize {
    preview_detail::component_action_hit_rect(PAGE).x + COMPONENT_HIT_INSET
}

fn toast_y() -> usize {
    preview_detail::component_action_hit_rect(PAGE).y + COMPONENT_HIT_INSET
}
