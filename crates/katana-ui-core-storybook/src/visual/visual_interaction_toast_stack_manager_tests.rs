use super::interaction_spec::StorybookInteractionSpec;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at,
};
use super::window_interaction::{
    StorybookWindowState, apply_hover_at, focus_clickable_at_for_audit,
};
use super::{StorybookVisual, palette, preview_detail, render, storybook_ui_option_contract};
use crate::StoryCatalog;
use crate::catalog::StoryPresetLabels;
use crate::test_assert::KucTestExpect;
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
fn toast_stack_manager_hover_and_focus_pause_use_core_events() {
    let mut hover_state = page_state();
    let before_hover = render_state(&hover_state);
    let component = preview_detail::component_action_hit_rect(PAGE);
    assert!(apply_hover_at(
        &mut hover_state,
        component.x + 1,
        component.y + 1
    ));
    let after_hover = render_state(&hover_state);
    assert_eq!(
        "toast_stack_hover_pause",
        hover_state.screen_state.last_action
    );
    assert_eq!("toast_paused", hover_state.screen_state.last_event);
    assert_eq!(
        "toast_stack.paused=true",
        hover_state.screen_state.state_label
    );
    assert!(component_body_pixel_diff(PAGE, &before_hover, &after_hover) > 0);

    let mut focus_state = page_state();
    let before_focus = render_state(&focus_state);
    assert!(focus_clickable_at_for_audit(
        &mut focus_state,
        component.x + 1,
        component.y + 1
    ));
    let after_focus = render_state(&focus_state);
    assert_eq!(
        "toast_stack_focus_pause",
        focus_state.screen_state.last_action
    );
    assert_eq!("toast_paused", focus_state.screen_state.last_event);
    assert_eq!(
        "toast_stack.paused=true",
        focus_state.screen_state.state_label
    );
    assert!(focus_state.screen_state.is_button_focused());
    assert!(component_body_pixel_diff(PAGE, &before_focus, &after_focus) > 0);
}

#[test]
fn toast_stack_manager_story_connects_core_queue_timer_pause_and_action_events() {
    let examples = StoryCatalog.examples();
    let story = examples
        .iter()
        .find(|example| example.page == PAGE)
        .kuc_expect("toast-stack-manager story exists");

    for (action, event) in [
        ("toast_enqueue_visible", "ToastShown"),
        ("toast_queue_and_overflow", "ToastQueueOverflow"),
        ("toast_pause_hover", "ToastPaused"),
        ("toast_action_dismiss", "ToastDismissed"),
        ("toast_timeout", "ToastTimedOut"),
    ] {
        assert!(
            story
                .callback_logs
                .iter()
                .any(|callback| callback.action == action && callback.after.contains(event)),
            "toast-stack-manager callback log lacks {action}/{event}"
        );
    }
}

#[test]
fn toast_stack_manager_light_and_dark_top_toast_uses_theme_surface() {
    assert_top_toast_token(DARK_THEME, ThemeSnapshot::dark());
    assert_top_toast_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn page_state() -> StorybookWindowState {
    StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    }
}

fn render_state(state: &StorybookWindowState) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
        DARK_THEME,
        PAGE,
        state.preset_index,
        state.screen_state.clone(),
    )
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
