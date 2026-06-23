use super::interaction_spec::StorybookInteractionSpec;
use super::screen_state::StorybookScreenState;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at,
};
use super::window_interaction::{self, StorybookWindowState, apply_click};
use super::{
    StorybookVisual, dedicated_dod_molecule_tree_parts as tree_parts, layout_metrics, palette,
    preview_detail, render, storybook_ui_option_contract,
};
use crate::catalog::StoryPresetLabels;
use crate::visual::canvas::Canvas;
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "tree-view";
const FOLDERS_PRESET: usize = 0;
const TOGGLE_PRESET: usize = 1;
const CONTEXT_PRESET: usize = 2;
const THEME_TREE_PRESET: usize = 3;
const VIRTUALIZATION_PRESET: usize = 4;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const TREE_PANEL_X: usize = 14;
const TREE_PANEL_Y: usize = 30;
const TREE_SAMPLE_X_OFFSET: usize = 168;
const TREE_SAMPLE_Y_OFFSET: usize = 62;
const TREE_ROW_LABEL_CLICK_OFFSET: usize = 8;
const SCROLLED_TREE_OFFSET: u32 = 96;
const PRIMARY_INSTANCE: &str = "tree-view.primary";
const SECONDARY_INSTANCE: &str = "tree-view.secondary";
const CONTEXT_MENU_OPTION_INDEX: usize = 3;

#[test]
fn tree_view_exposes_leaf_presets_options_and_toggle_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert!(rows.iter().any(|row| row.starts_with("line:")));
    assert!(rows.iter().any(|row| row.starts_with("trigger:")));
    assert_eq!("tree_click_toggle", spec.action);
    assert_eq!("tree_toggled", spec.event);
    assert_eq!("interaction.open", spec.option);
    assert_eq!("false", spec.after);
    assert_eq!("open=false", spec.state);
}

#[test]
fn tree_view_presets_render_distinct_folder_toggle_context_theme_and_virtual_states() {
    let folders = StorybookVisual.render_preset(DARK_THEME, PAGE, FOLDERS_PRESET, 0);
    let toggle = StorybookVisual.render_preset(DARK_THEME, PAGE, TOGGLE_PRESET, 0);
    let context = StorybookVisual.render_preset(DARK_THEME, PAGE, CONTEXT_PRESET, 0);
    let themed = StorybookVisual.render_preset(DARK_THEME, PAGE, THEME_TREE_PRESET, 0);
    let virtualized = StorybookVisual.render_preset(DARK_THEME, PAGE, VIRTUALIZATION_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &folders, &toggle) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &toggle, &context) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &context, &themed) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &themed, &virtualized) > BODY_DIFF_THRESHOLD);
}

#[test]
fn tree_view_setting_option_updates_tree_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn tree_view_preview_action_updates_toggle_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn tree_view_click_after_scroll_keeps_visible_offset() {
    let mut state = tree_state(FOLDERS_PRESET);
    state.screen_state.tree_view_scroll_offset = SCROLLED_TREE_OFFSET;

    click_first_tree_row(&mut state);

    assert_eq!(
        SCROLLED_TREE_OFFSET, state.screen_state.tree_view_scroll_offset,
        "TreeView click must not reset the scrolled viewport to the top"
    );
    assert_eq!("tree_select_file", state.screen_state.last_action);
    assert_eq!("tree_selected", state.screen_state.last_event);
    assert_eq!("katana/nested/b.md", state.screen_state.last_setting_value);
}

#[test]
fn tree_view_preview_actions_expose_hover_focus_keyboard_and_scroll_ports() {
    assert_tree_view_preview_action(
        "tree-view-hover",
        "tree_hover_item",
        "hover_start",
        "hover=katana/a.md",
    );
    assert_tree_view_preview_action(
        "tree-view-focus",
        "tree_focus_item",
        "tree_item_focused",
        "focus=katana/a.md",
    );
    assert_tree_view_preview_action(
        "tree-view-keyboard",
        "tree_keyboard_select",
        "tree_selected",
        "selected=katana/a.md",
    );
    assert_tree_view_preview_action(
        "tree-view-scroll",
        "tree_scroll_retained",
        "tree_scroll_offset_kept",
        "scroll=retained",
    );
}

#[test]
fn tree_view_light_and_dark_panel_uses_theme_surface() {
    assert_tree_token(DARK_THEME, ThemeSnapshot::dark());
    assert_tree_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_tree_view_preview_action(
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

#[test]
fn tree_view_window_interaction_keeps_instance_state_isolated() {
    let mut state = tree_state(FOLDERS_PRESET);

    state.select_instance(PRIMARY_INSTANCE);
    click_first_tree_row(&mut state);
    let primary = state.screen_state.clone();
    let primary_canvas = render_state(&state);
    assert_eq!("tree_click_toggle", primary.last_action);
    assert_eq!("tree_toggled", primary.last_event);
    assert_eq!("open=false", primary.state_label);

    state.select_instance(SECONDARY_INSTANCE);
    assert_eq!("none", state.screen_state.last_action);
    assert_eq!("none", state.screen_state.last_event);
    assert_eq!("idle", state.screen_state.state_label);
    let secondary_canvas = render_state(&state);

    state.select_instance(PRIMARY_INSTANCE);
    assert_eq!(primary.last_action, state.screen_state.last_action);
    assert_eq!(primary.last_event, state.screen_state.last_event);
    assert_eq!(primary.state_label, state.screen_state.state_label);
    assert!(
        component_body_pixel_diff(PAGE, &primary_canvas, &secondary_canvas) > BODY_DIFF_THRESHOLD
    );
}

#[test]
fn tree_view_context_menu_keeps_instance_state_isolated() {
    let mut state = tree_state(CONTEXT_PRESET);

    state.select_instance(PRIMARY_INSTANCE);
    context_click_component(&mut state);
    let primary = state.screen_state.clone();
    let primary_canvas = render_state(&state);
    assert_eq!("tree_context_menu", primary.last_action);
    assert_eq!("tree_context_opened", primary.last_event);
    assert_eq!("context_menu=open", primary.state_label);

    state.select_instance(SECONDARY_INSTANCE);
    assert_eq!("none", state.screen_state.last_action);
    assert_eq!("none", state.screen_state.last_event);
    assert_eq!("idle", state.screen_state.state_label);
    let secondary_canvas = render_state(&state);

    state.select_instance(PRIMARY_INSTANCE);
    assert_eq!(primary.last_action, state.screen_state.last_action);
    assert_eq!(primary.last_event, state.screen_state.last_event);
    assert_eq!(primary.state_label, state.screen_state.state_label);
    assert!(
        component_body_pixel_diff(PAGE, &primary_canvas, &secondary_canvas) > BODY_DIFF_THRESHOLD
    );
}

#[test]
fn tree_view_setting_action_keeps_instance_setting_isolated() {
    let mut state = tree_state(FOLDERS_PRESET);

    state.select_instance(PRIMARY_INSTANCE);
    click_context_menu_option(&mut state);
    let primary = state.screen_state.clone();
    let primary_canvas = render_state(&state);
    assert_eq!("context_menu", primary.last_setting);
    assert_eq!("enabled", primary.last_setting_value);
    assert_eq!("tree.context_menu=enabled", primary.state_label);

    state.select_instance(SECONDARY_INSTANCE);
    assert_eq!("none", state.screen_state.last_action);
    assert_eq!("idle", state.screen_state.state_label);
    let secondary_canvas = render_state(&state);

    state.select_instance(PRIMARY_INSTANCE);
    assert_eq!(primary.last_setting, state.screen_state.last_setting);
    assert_eq!(
        primary.last_setting_value,
        state.screen_state.last_setting_value
    );
    assert_eq!(primary.state_label, state.screen_state.state_label);
    assert!(
        component_body_pixel_diff(PAGE, &primary_canvas, &secondary_canvas) > BODY_DIFF_THRESHOLD
    );
}

fn assert_tree_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, FOLDERS_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert_eq!(
        Some(colors.surface),
        pixel_at(
            &canvas,
            component.x + TREE_PANEL_X + TREE_SAMPLE_X_OFFSET,
            component.y + TREE_PANEL_Y + TREE_SAMPLE_Y_OFFSET
        )
    );
}

fn tree_state(preset_index: usize) -> StorybookWindowState {
    StorybookWindowState {
        selected_page: PAGE,
        preset_index,
        ..StorybookWindowState::default()
    }
}

fn click_first_tree_row(state: &mut StorybookWindowState) {
    let x =
        preview_detail::HERO_PREVIEW_X_FOR_TEST + tree_parts::LABEL_X + TREE_ROW_LABEL_CLICK_OFFSET;
    let y = preview_detail::HERO_PREVIEW_Y_FOR_TEST
        + tree_parts::TREE_PANEL_Y
        + tree_parts::ROW_HEIGHT / 2;

    assert!(apply_click(state, x, y));
}

fn context_click_component(state: &mut StorybookWindowState) {
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert!(window_interaction::apply_context_click_for_test(
        state,
        component.x + 1,
        component.y + 1,
    ));
}

fn click_context_menu_option(state: &mut StorybookWindowState) {
    let row = layout_metrics::inspector_setting_row_hit_rect(CONTEXT_MENU_OPTION_INDEX);

    assert!(apply_click(state, row.x + 1, row.y + 1));
}

fn render_state(state: &StorybookWindowState) -> Canvas {
    render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    )
}
