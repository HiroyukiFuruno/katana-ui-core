use super::interaction_spec::StorybookInteractionSpec;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at,
};
use super::{StorybookVisual, palette, preview_detail, render, storybook_ui_option_contract};
use crate::catalog::StoryPresetLabels;
use crate::visual::window_interaction::{
    StorybookWindowState, apply_clickable_keyboard_activation_for_audit, apply_hover_at,
    focus_clickable_at_for_audit,
};
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "accordion";
const CLOSED_PRESET: usize = 0;
const OPEN_PRESET: usize = 1;
const DISABLED_PRESET: usize = 2;
const CONTROLLED_PRESET: usize = 3;
const MULTIPLE_PRESET: usize = 4;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const SURFACE_SAMPLE_X: usize = 210;
const SURFACE_SAMPLE_Y: usize = 46;

#[test]
fn accordion_exposes_leaf_presets_options_and_toggle_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert!(
        rows.iter()
            .any(|row| row.starts_with("accordion.expanded:"))
    );
    assert!(
        rows.iter()
            .any(|row| row.starts_with("accordion.trigger_area:"))
    );
    assert_eq!("accordion_toggle", spec.action);
    assert_eq!("accordion_changed", spec.event);
    assert_eq!("interaction.open", spec.option);
    assert_eq!("false", spec.after);
    assert_eq!("open=false", spec.state);
}

#[test]
fn accordion_presets_render_distinct_open_disabled_controlled_and_multiple_states() {
    let closed = StorybookVisual.render_preset(DARK_THEME, PAGE, CLOSED_PRESET, 0);
    let open = StorybookVisual.render_preset(DARK_THEME, PAGE, OPEN_PRESET, 0);
    let disabled = StorybookVisual.render_preset(DARK_THEME, PAGE, DISABLED_PRESET, 0);
    let controlled = StorybookVisual.render_preset(DARK_THEME, PAGE, CONTROLLED_PRESET, 0);
    let multiple = StorybookVisual.render_preset(DARK_THEME, PAGE, MULTIPLE_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &closed, &open) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &open, &disabled) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &disabled, &controlled) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &controlled, &multiple) > BODY_DIFF_THRESHOLD);
}

#[test]
fn accordion_setting_option_updates_controlled_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn accordion_preview_action_updates_open_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn accordion_hover_focus_keyboard_disabled_and_group_actions_update_state() {
    assert_accordion_hover_updates_state();
    assert_accordion_focus_updates_state();
    assert_accordion_keyboard_after_focus_updates_state();
    assert_accordion_disabled_toggle_is_blocked();
    assert_accordion_group_toggle_records_multiple_state();
}

fn assert_accordion_hover_updates_state() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    let target = preview_detail::component_action_hit_rect(PAGE);
    let before = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    );

    assert!(apply_hover_at(&mut state, target.x + 1, target.y + 1));

    assert_eq!("accordion_hover", state.screen_state.last_action);
    assert_eq!("accordion_hovered", state.screen_state.last_event);
    assert_eq!("interaction.hovered", state.screen_state.last_setting);
    assert_eq!("hover=true", state.screen_state.state_label);
    let after = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    );
    assert!(component_body_pixel_diff(PAGE, &before, &after) > BODY_DIFF_THRESHOLD);
}

fn assert_accordion_focus_updates_state() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    let target = preview_detail::component_action_hit_rect(PAGE);
    let before = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    );

    assert!(focus_clickable_at_for_audit(
        &mut state,
        target.x + 1,
        target.y + 1
    ));

    assert_eq!("accordion_focus", state.screen_state.last_action);
    assert_eq!("accordion_focused", state.screen_state.last_event);
    assert_eq!("interaction.focused", state.screen_state.last_setting);
    assert_eq!("focus=true", state.screen_state.state_label);
    assert!(state.screen_state.is_button_focused());
    let after = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    );
    assert!(component_body_pixel_diff(PAGE, &before, &after) > BODY_DIFF_THRESHOLD);
}

fn assert_accordion_keyboard_after_focus_updates_state() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    let target = preview_detail::component_action_hit_rect(PAGE);
    assert!(focus_clickable_at_for_audit(
        &mut state,
        target.x + 1,
        target.y + 1
    ));
    let before = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    );

    assert!(apply_clickable_keyboard_activation_for_audit(&mut state));

    assert_eq!("accordion_toggle", state.screen_state.last_action);
    assert_eq!("accordion_changed", state.screen_state.last_event);
    assert_eq!("interaction.open", state.screen_state.last_setting);
    assert_eq!("false", state.screen_state.last_setting_value);
    assert_eq!("open=false", state.screen_state.state_label);
    let after = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    );
    assert!(component_body_pixel_diff(PAGE, &before, &after) > BODY_DIFF_THRESHOLD);
}

fn assert_accordion_disabled_toggle_is_blocked() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };

    state.screen_state.register_accordion_disabled_block();

    assert_eq!("accordion_disabled_block", state.screen_state.last_action);
    assert_eq!("accordion_toggle_ignored", state.screen_state.last_event);
    assert_eq!("accordion.disabled", state.screen_state.last_setting);
    assert_eq!("disabled=true", state.screen_state.state_label);
}

fn assert_accordion_group_toggle_records_multiple_state() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };

    state.screen_state.register_accordion_group_toggle();

    assert_eq!("accordion_group_toggle", state.screen_state.last_action);
    assert_eq!("accordion_group_changed", state.screen_state.last_event);
    assert_eq!("accordion.multiple", state.screen_state.last_setting);
    assert_eq!("open=item-a,item-b", state.screen_state.state_label);
}

#[test]
fn accordion_light_and_dark_header_uses_theme_surface() {
    assert_accordion_surface_token(DARK_THEME, ThemeSnapshot::dark());
    assert_accordion_surface_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_accordion_surface_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, CLOSED_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert_eq!(
        Some(colors.surface),
        pixel_at(
            &canvas,
            component.x + SURFACE_SAMPLE_X,
            component.y + SURFACE_SAMPLE_Y
        )
    );
}
