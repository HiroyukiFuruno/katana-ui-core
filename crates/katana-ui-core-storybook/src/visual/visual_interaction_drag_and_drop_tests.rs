use super::interaction_spec::StorybookInteractionSpec;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at,
};
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{
    StorybookVisual, dedicated_drag_and_drop, palette, preview_detail, render,
    storybook_ui_option_contract,
};
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
const DROP_INDICATOR_OPTION_ROW: usize = 3;
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
fn drag_and_drop_component_source_and_target_use_core_drag_contract() {
    let mut state = drag_and_drop_state();
    let before = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    );
    let component = preview_detail::component_action_hit_rect(PAGE);
    let source = dedicated_drag_and_drop::source_rect(component.x, component.y);
    let target = dedicated_drag_and_drop::target_rect(component.x, component.y);

    assert!(apply_click(&mut state, source.x + 1, source.y + 1));
    assert_eq!("drag_start", state.screen_state.last_action);
    assert_eq!("drag_start", state.screen_state.last_event);
    assert_eq!("dragging=true", state.screen_state.state_label);
    assert!(state.screen_state.drag_and_drop.is_dragging());

    assert!(apply_click(&mut state, target.x + 1, target.y + 1));
    assert_eq!("drop", state.screen_state.last_action);
    assert_eq!("drag_end(committed=true)", state.screen_state.last_event);
    assert_eq!("committed=true", state.screen_state.state_label);
    assert!(state.screen_state.drag_and_drop.committed());

    let after = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    );
    assert!(component_body_pixel_diff(PAGE, &before, &after) > BODY_DIFF_THRESHOLD);
}

#[test]
fn drag_and_drop_keyboard_cancel_routes_through_keyboard_drag_state() {
    let mut state = drag_and_drop_state();
    let component = preview_detail::component_action_hit_rect(PAGE);
    let rail = dedicated_drag_and_drop::rail_rect(component.x, component.y);

    assert!(apply_click(&mut state, rail.x + rail.width - 1, rail.y + 1));
    assert_eq!("drag_keyboard_cancel", state.screen_state.last_action);
    assert_eq!("drag_end(committed=false)", state.screen_state.last_event);
    assert_eq!("committed=false", state.screen_state.state_label);
    assert!(!state.screen_state.drag_and_drop.committed());
}

#[test]
fn drag_and_drop_visible_edge_and_target_corner_route_to_core_actions() {
    let mut scroll_state = drag_and_drop_state();
    let component = preview_detail::component_action_hit_rect(PAGE);
    let edge = dedicated_drag_and_drop::autoscroll_edge_rect(component.x, component.y);

    assert!(apply_click(&mut scroll_state, edge.x + 1, edge.y + 1));
    assert_eq!("drag_autoscroll", scroll_state.screen_state.last_action);
    assert_eq!(
        "drag_autoscroll_requested",
        scroll_state.screen_state.last_event
    );
    assert_eq!("scroll=edge", scroll_state.screen_state.state_label);

    let mut resize_state = drag_and_drop_state();
    let resize = dedicated_drag_and_drop::resize_target_rect(component.x, component.y);

    assert!(apply_click(&mut resize_state, resize.x + 1, resize.y + 1));
    assert_eq!("drag_resize_target", resize_state.screen_state.last_action);
    assert_eq!("drag_target_resized", resize_state.screen_state.last_event);
    assert_eq!("resize=target", resize_state.screen_state.state_label);
}

#[test]
fn drag_and_drop_inspector_options_mutate_drag_state() {
    let mut state = drag_and_drop_state();
    let setting = super::layout_metrics::inspector_setting_row_hit_rect(DROP_INDICATOR_OPTION_ROW);

    assert!(apply_click(&mut state, setting.x + 1, setting.y + 1));
    assert_eq!(1, state.screen_state.settings_revision);
    assert_eq!(DROP_INDICATOR_OPTION_ROW, state.preset_index);
    assert_eq!("drag_indicator_option", state.screen_state.last_action);
    assert_eq!("drag_indicator_changed", state.screen_state.last_event);
    assert_eq!("drag.drop_indicator", state.screen_state.last_setting);
    assert_eq!("after", state.screen_state.last_setting_value);
    assert_eq!("drag.drop_indicator=after", state.screen_state.state_label);
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

fn drag_and_drop_state() -> StorybookWindowState {
    StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    }
}
