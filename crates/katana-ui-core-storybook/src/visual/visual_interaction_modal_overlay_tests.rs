use super::interaction_spec::StorybookInteractionSpec;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at,
};
use super::window_interaction::{
    StorybookWindowState, apply_click, apply_clickable_keyboard_activation_for_audit,
    apply_context_click, apply_hover_at, focus_clickable_at_for_audit,
};
use super::{StorybookVisual, palette, preview_detail, storybook_ui_option_contract};
use crate::catalog::StoryPresetLabels;
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "modal-overlay";
const OVERLAY_PRESET: usize = 0;
const BACKDROP_PRESET: usize = 1;
const ESCAPE_PRESET: usize = 2;
const FOCUS_PRESET: usize = 3;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const COMPONENT_HIT_INSET: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const DIALOG_X: usize = 38;
const DIALOG_Y: usize = 42;
const DIALOG_SAMPLE_OFFSET: usize = 8;

#[test]
fn modal_overlay_exposes_leaf_presets_options_and_close_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("overlay_close", spec.action);
    assert_eq!("overlay_closed", spec.event);
    assert_eq!("interaction.open", spec.option);
    assert_eq!("false", spec.after);
    assert_eq!("open=false", spec.state);
}

#[test]
fn modal_overlay_presets_render_distinct_overlay_backdrop_escape_and_focus_states() {
    let overlay = StorybookVisual.render_preset(DARK_THEME, PAGE, OVERLAY_PRESET, 0);
    let backdrop = StorybookVisual.render_preset(DARK_THEME, PAGE, BACKDROP_PRESET, 0);
    let escape = StorybookVisual.render_preset(DARK_THEME, PAGE, ESCAPE_PRESET, 0);
    let focus = StorybookVisual.render_preset(DARK_THEME, PAGE, FOCUS_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &overlay, &backdrop) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &backdrop, &escape) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &escape, &focus) > BODY_DIFF_THRESHOLD);
}

#[test]
fn modal_overlay_setting_option_updates_dialog_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn modal_overlay_preview_action_updates_dialog_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn modal_overlay_live_backdrop_focus_keyboard_hover_and_context_block_use_core_actions() {
    let mut pointer_state = page_state();
    let pointer_before = render_state(&pointer_state);
    assert!(apply_click(
        &mut pointer_state,
        component_x(),
        component_y()
    ));
    let pointer_after = render_state(&pointer_state);
    assert_eq!("overlay_close", pointer_state.screen_state.last_action);
    assert_eq!("overlay_closed", pointer_state.screen_state.last_event);
    assert_eq!("open=false", pointer_state.screen_state.state_label);
    assert!(component_body_pixel_diff(PAGE, &pointer_before, &pointer_after) > 0);

    let mut hover_state = page_state();
    let hover_before = render_state(&hover_state);
    assert!(apply_hover_at(
        &mut hover_state,
        component_x(),
        component_y()
    ));
    let hover_after = render_state(&hover_state);
    assert_eq!("modal_overlay_hover", hover_state.screen_state.last_action);
    assert_eq!("modal_overlay_hovered", hover_state.screen_state.last_event);
    assert_eq!("hover=true", hover_state.screen_state.state_label);
    assert!(hover_state.screen_state.preview_hovered);
    assert!(component_body_pixel_diff(PAGE, &hover_before, &hover_after) > 0);
    let hover_action_count = hover_state.screen_state.action_count;
    assert!(apply_hover_at(
        &mut hover_state,
        component_x(),
        component_y()
    ));
    assert_eq!(hover_action_count, hover_state.screen_state.action_count);

    let mut keyboard_state = page_state();
    let focus_before = render_state(&keyboard_state);
    assert!(focus_clickable_at_for_audit(
        &mut keyboard_state,
        component_x(),
        component_y()
    ));
    let focus_after = render_state(&keyboard_state);
    assert_eq!(
        "modal_overlay_focus",
        keyboard_state.screen_state.last_action
    );
    assert_eq!(
        "modal_overlay_focused",
        keyboard_state.screen_state.last_event
    );
    assert_eq!("focus=trapped", keyboard_state.screen_state.state_label);
    assert!(keyboard_state.screen_state.is_button_focused());
    assert!(component_body_pixel_diff(PAGE, &focus_before, &focus_after) > 0);

    let keyboard_before = render_state(&keyboard_state);
    assert!(apply_clickable_keyboard_activation_for_audit(
        &mut keyboard_state
    ));
    let keyboard_after = render_state(&keyboard_state);
    assert_eq!(
        "modal_overlay_escape",
        keyboard_state.screen_state.last_action
    );
    assert_eq!(
        "modal_overlay_closed",
        keyboard_state.screen_state.last_event
    );
    assert_eq!("open=false", keyboard_state.screen_state.state_label);
    assert!(component_body_pixel_diff(PAGE, &keyboard_before, &keyboard_after) > 0);

    let mut context_state = page_state();
    let context_before = render_state(&context_state);
    assert!(apply_context_click(
        &mut context_state,
        component_x(),
        component_y()
    ));
    let context_after = render_state(&context_state);
    assert_eq!(
        "modal_overlay_context_block",
        context_state.screen_state.last_action
    );
    assert_eq!(
        "modal_overlay_context_ignored",
        context_state.screen_state.last_event
    );
    assert_eq!(
        "interaction=blocked",
        context_state.screen_state.state_label
    );
    assert!(component_body_pixel_diff(PAGE, &context_before, &context_after) > 0);
}

#[test]
fn modal_overlay_light_and_dark_dialog_uses_theme_surface() {
    assert_dialog_token(DARK_THEME, ThemeSnapshot::dark());
    assert_dialog_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn page_state() -> StorybookWindowState {
    StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    }
}

fn render_state(state: &StorybookWindowState) -> super::Canvas {
    super::render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    )
}

fn component_x() -> usize {
    preview_detail::component_action_hit_rect(PAGE).x + COMPONENT_HIT_INSET
}

fn component_y() -> usize {
    preview_detail::component_action_hit_rect(PAGE).y + COMPONENT_HIT_INSET
}

fn assert_dialog_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, OVERLAY_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert_eq!(
        Some(colors.surface),
        pixel_at(
            &canvas,
            component.x + DIALOG_X + DIALOG_SAMPLE_OFFSET,
            component.y + DIALOG_Y + DIALOG_SAMPLE_OFFSET
        )
    );
}
