use super::interaction_spec::StorybookInteractionSpec;
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
const PAGE: &str = "dynamic-array-editor";
const ROWS_PRESET: usize = 0;
const ADD_REMOVE_PRESET: usize = 1;
const REORDER_PRESET: usize = 2;
const THEME_PRESET: usize = 3;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const SURFACE_TOKEN_X: usize = 338;
const SURFACE_TOKEN_Y: usize = 34;
const COMPONENT_HIT_INSET: usize = 4;
const SURFACE_SAMPLE_X_OFFSET: usize = 120;
const SURFACE_SAMPLE_Y_OFFSET: usize = 12;
const ADD_X: usize = 246;
const ADD_Y: usize = 54;
const REMOVE_Y: usize = 78;
const REORDER_Y: usize = 102;

#[test]
fn dynamic_array_editor_exposes_leaf_presets_options_and_add_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert_eq!(options.len(), REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert!(rows.iter().any(|row| row.starts_with("array.rows:")));
    assert!(rows.iter().any(|row| row.starts_with("array.reorder:")));
    assert_eq!("array_add", spec.action);
    assert_eq!("array_changed", spec.event);
    assert_eq!("interaction.value", spec.option);
    assert_eq!("3 rows", spec.after);
    assert_eq!("rows=3", spec.state);
}

#[test]
fn dynamic_array_editor_presets_render_distinct_rows_add_reorder_and_theme_states() {
    let rows = StorybookVisual.render_preset(DARK_THEME, PAGE, ROWS_PRESET, 0);
    let add_remove = StorybookVisual.render_preset(DARK_THEME, PAGE, ADD_REMOVE_PRESET, 0);
    let reorder = StorybookVisual.render_preset(DARK_THEME, PAGE, REORDER_PRESET, 0);
    let theme = StorybookVisual.render_preset(DARK_THEME, PAGE, THEME_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &rows, &add_remove) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &add_remove, &reorder) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &reorder, &theme) > BODY_DIFF_THRESHOLD);
}

#[test]
fn dynamic_array_editor_setting_option_updates_row_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn dynamic_array_editor_preview_action_updates_row_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn dynamic_array_editor_component_buttons_mutate_state_action_event_and_rendering() {
    let mut state = dynamic_array_state();
    let before = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    );

    assert!(click_component_control(&mut state, ADD_Y));
    assert_eq!(1, state.screen_state.action_count);
    assert_eq!("array_add", state.screen_state.last_action);
    assert_eq!("array_changed", state.screen_state.last_event);
    assert_eq!("rows=4", state.screen_state.state_label);
    assert_eq!(4, state.screen_state.dynamic_array_editor.item_count());
    assert_eq!("callback=add", state.screen_state.last_setting_value);

    let after = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    );
    assert!(component_body_pixel_diff(PAGE, &before, &after) > BODY_DIFF_THRESHOLD);
}

#[test]
fn dynamic_array_editor_remove_and_reorder_are_real_component_operations() {
    let mut state = dynamic_array_state();

    assert!(click_component_control(&mut state, REMOVE_Y));
    assert_eq!("array_remove", state.screen_state.last_action);
    assert_eq!("rows=2", state.screen_state.state_label);
    assert_eq!(2, state.screen_state.dynamic_array_editor.item_count());

    assert!(click_component_control(&mut state, REORDER_Y));
    assert_eq!("array_reorder", state.screen_state.last_action);
    assert_eq!("array_changed", state.screen_state.last_event);
    assert_eq!("order=2,1,3", state.screen_state.state_label);
    assert_eq!(
        "order=2,1,3",
        state.screen_state.dynamic_array_editor.order_label()
    );
}

#[test]
fn dynamic_array_editor_live_hover_focus_keyboard_and_validation_use_core_actions() {
    let mut hover_state = dynamic_array_state();
    let hover_before = render_state(&hover_state);
    assert!(apply_hover_at(
        &mut hover_state,
        component_x(),
        component_y()
    ));
    let hover_after = render_state(&hover_state);
    assert_eq!("array_hover", hover_state.screen_state.last_action);
    assert_eq!("array_hovered", hover_state.screen_state.last_event);
    assert_eq!("hover=true", hover_state.screen_state.state_label);
    assert!(hover_state.screen_state.preview_hovered);
    assert!(component_body_pixel_diff(PAGE, &hover_before, &hover_after) > 0);

    let mut keyboard_state = dynamic_array_state();
    let focus_before = render_state(&keyboard_state);
    assert!(focus_clickable_at_for_audit(
        &mut keyboard_state,
        component_x(),
        component_y()
    ));
    let focus_after = render_state(&keyboard_state);
    assert_eq!("array_focus", keyboard_state.screen_state.last_action);
    assert_eq!("array_focused", keyboard_state.screen_state.last_event);
    assert_eq!("focus=true", keyboard_state.screen_state.state_label);
    assert!(keyboard_state.screen_state.is_button_focused());
    assert!(component_body_pixel_diff(PAGE, &focus_before, &focus_after) > 0);

    let keyboard_before = render_state(&keyboard_state);
    assert!(apply_clickable_keyboard_activation_for_audit(
        &mut keyboard_state
    ));
    let keyboard_after = render_state(&keyboard_state);
    assert_eq!(
        "array_keyboard_edit",
        keyboard_state.screen_state.last_action
    );
    assert_eq!("array_changed", keyboard_state.screen_state.last_event);
    assert_eq!("edited=row-1", keyboard_state.screen_state.state_label);
    assert_eq!(
        "callback=edit",
        keyboard_state.screen_state.last_setting_value
    );
    assert!(component_body_pixel_diff(PAGE, &keyboard_before, &keyboard_after) > 0);
}

#[test]
fn dynamic_array_editor_inspector_options_mutate_component_contract() {
    let mut state = dynamic_array_state();
    let setting = super::layout_metrics::inspector_setting_row_hit_rect(REORDER_PRESET);

    assert!(apply_click(&mut state, setting.x + 1, setting.y + 1));
    assert_eq!(1, state.screen_state.settings_revision);
    assert_eq!(REORDER_PRESET, state.preset_index);
    assert_eq!("array_reorder_option", state.screen_state.last_action);
    assert_eq!("array_changed", state.screen_state.last_event);
    assert_eq!("array.reorder", state.screen_state.last_setting);
    assert_eq!("true", state.screen_state.last_setting_value);
    assert_eq!("array.order=2,1,3", state.screen_state.state_label);
    assert_eq!(
        "order=2,1,3",
        state.screen_state.dynamic_array_editor.order_label()
    );
}

#[test]
fn dynamic_array_editor_light_and_dark_surface_uses_theme_surface() {
    assert_dynamic_array_surface_token(DARK_THEME, ThemeSnapshot::dark());
    assert_dynamic_array_surface_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_dynamic_array_surface_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, ROWS_PRESET, 0);
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

fn dynamic_array_state() -> StorybookWindowState {
    StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    }
}

fn render_state(state: &StorybookWindowState) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
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

fn click_component_control(state: &mut StorybookWindowState, y: usize) -> bool {
    let component = preview_detail::component_action_hit_rect(PAGE);
    apply_click(state, component.x + ADD_X + 1, component.y + y + 1)
}
