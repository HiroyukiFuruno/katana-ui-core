use super::interaction_spec::StorybookInteractionSpec;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at,
};
use super::window_interaction::{
    StorybookWindowState, apply_click, apply_clickable_keyboard_activation_for_audit,
    apply_hover_at, apply_list_scroll_for_audit, focus_clickable_at_for_audit,
};
use super::{StorybookVisual, palette, preview_detail, storybook_ui_option_contract};
use crate::catalog::StoryPresetLabels;
use crate::visual::render;
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "list";
const ROWS_PRESET: usize = 0;
const SELECTION_PRESET: usize = 1;
const EMPTY_PRESET: usize = 2;
const THEME_PRESET: usize = 3;
const VIRTUALIZATION_PRESET: usize = 4;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const LIST_X: usize = 34;
const LIST_Y: usize = 30;
const LIST_SAMPLE_X_OFFSET: usize = 250;
const LIST_SAMPLE_Y_OFFSET: usize = 78;
const CLICK_OFFSET: usize = 4;

#[test]
fn list_exposes_leaf_presets_options_and_selection_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert!(rows.iter().any(|row| row.starts_with("list.rows:")));
    assert!(
        rows.iter()
            .any(|row| row.starts_with("list.virtualization:"))
    );
    assert_eq!("list_select", spec.action);
    assert_eq!("selection_changed", spec.event);
    assert_eq!("interaction.selected_index", spec.option);
    assert_eq!("1", spec.after);
    assert_eq!("selected=1", spec.state);
}

#[test]
fn list_presets_render_distinct_rows_selection_empty_theme_and_virtual_states() {
    let rows = StorybookVisual.render_preset(DARK_THEME, PAGE, ROWS_PRESET, 0);
    let selection = StorybookVisual.render_preset(DARK_THEME, PAGE, SELECTION_PRESET, 0);
    let empty = StorybookVisual.render_preset(DARK_THEME, PAGE, EMPTY_PRESET, 0);
    let themed = StorybookVisual.render_preset(DARK_THEME, PAGE, THEME_PRESET, 0);
    let virtualization = StorybookVisual.render_preset(DARK_THEME, PAGE, VIRTUALIZATION_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &rows, &selection) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &selection, &empty) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &empty, &themed) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &themed, &virtualization) > BODY_DIFF_THRESHOLD);
}

#[test]
fn list_setting_option_updates_collection_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn list_preview_action_updates_selection_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn list_pointer_hover_focus_keyboard_and_scroll_update_body_and_state() {
    let target = preview_detail::component_action_hit_rect(PAGE);
    let mut pointer = page_state();
    let before_pointer = render_state(&pointer);

    assert!(apply_click(
        &mut pointer,
        target.x + CLICK_OFFSET,
        target.y + CLICK_OFFSET
    ));
    assert_eq!("list_select", pointer.screen_state.last_action);
    assert_eq!("selection_changed", pointer.screen_state.last_event);
    assert_eq!("selected=1", pointer.screen_state.state_label);
    assert_eq!(Some(1), pointer.screen_state.list.selected_index);
    assert!(component_body_pixel_diff(PAGE, &before_pointer, &render_state(&pointer)) > 0);

    let mut hover = page_state();
    let before_hover = render_state(&hover);
    assert!(apply_hover_at(
        &mut hover,
        target.x + CLICK_OFFSET,
        target.y + CLICK_OFFSET
    ));
    assert!(hover.screen_state.preview_hovered);
    assert!(component_body_pixel_diff(PAGE, &before_hover, &render_state(&hover)) > 0);

    let mut focus = page_state();
    let before_focus = render_state(&focus);
    assert!(focus_clickable_at_for_audit(
        &mut focus,
        target.x + CLICK_OFFSET,
        target.y + CLICK_OFFSET
    ));
    assert_eq!("list_focus", focus.screen_state.last_action);
    assert_eq!("list_focused", focus.screen_state.last_event);
    assert_eq!("focused=1", focus.screen_state.state_label);
    assert!(focus.screen_state.is_button_focused());
    assert!(component_body_pixel_diff(PAGE, &before_focus, &render_state(&focus)) > 0);

    let before_keyboard = render_state(&focus);
    assert!(apply_clickable_keyboard_activation_for_audit(&mut focus));
    assert_eq!("list_keyboard_next", focus.screen_state.last_action);
    assert_eq!("selection_changed", focus.screen_state.last_event);
    assert_eq!("selected=2", focus.screen_state.state_label);
    assert_eq!(Some(2), focus.screen_state.list.selected_index);
    assert!(component_body_pixel_diff(PAGE, &before_keyboard, &render_state(&focus)) > 0);

    let mut scroll = page_state();
    let before_scroll = render_state(&scroll);
    assert!(apply_list_scroll_for_audit(
        &mut scroll,
        target.x + CLICK_OFFSET,
        target.y + CLICK_OFFSET
    ));
    assert_eq!("list_scroll", scroll.screen_state.last_action);
    assert_eq!("list_virtual_range_changed", scroll.screen_state.last_event);
    assert_eq!("virtual=48/200", scroll.screen_state.state_label);
    assert!(scroll.screen_state.list.scrolled);
    assert!(component_body_pixel_diff(PAGE, &before_scroll, &render_state(&scroll)) > 0);
}

#[test]
fn list_light_and_dark_surface_uses_theme_surface() {
    assert_list_token(DARK_THEME, ThemeSnapshot::dark());
    assert_list_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_list_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, ROWS_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert_eq!(
        Some(colors.surface),
        pixel_at(
            &canvas,
            component.x + LIST_X + LIST_SAMPLE_X_OFFSET,
            component.y + LIST_Y + LIST_SAMPLE_Y_OFFSET
        )
    );
}

fn page_state() -> StorybookWindowState {
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
