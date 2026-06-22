use super::interaction_spec::StorybookInteractionSpec;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at,
};
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{
    StorybookVisual, dedicated_context_menu_popup, palette, preview_detail, render,
    storybook_ui_option_contract,
};
use crate::catalog::StoryPresetLabels;
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "context-menu";
const EDITOR_PRESET: usize = 0;
const EXPLORER_PRESET: usize = 1;
const TAB_BAR_PRESET: usize = 2;
const ICON_SHORTCUT_PRESET: usize = 4;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const MENU_X: usize = 128;
const MENU_Y: usize = 30;
const MENU_SAMPLE_OFFSET: usize = 4;
const PRIMARY_INSTANCE: &str = "context-menu.primary";
const SECONDARY_INSTANCE: &str = "context-menu.secondary";

#[test]
fn context_menu_exposes_leaf_presets_options_and_context_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("context_menu_open", spec.action);
    assert_eq!("context_menu_opened", spec.event);
    assert_eq!("context_menu.anchor", spec.option);
    assert_eq!("Pointer(192,128)", spec.after);
    assert_eq!("context_menu=open", spec.state);
}

#[test]
fn context_menu_presets_render_distinct_anchor_and_menu_bodies() {
    let editor = StorybookVisual.render_preset(DARK_THEME, PAGE, EDITOR_PRESET, 0);
    let explorer = StorybookVisual.render_preset(DARK_THEME, PAGE, EXPLORER_PRESET, 0);
    let tab_bar = StorybookVisual.render_preset(DARK_THEME, PAGE, TAB_BAR_PRESET, 0);
    let shortcut = StorybookVisual.render_preset(DARK_THEME, PAGE, ICON_SHORTCUT_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &editor, &explorer) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &explorer, &tab_bar) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &tab_bar, &shortcut) > BODY_DIFF_THRESHOLD);
}

#[test]
fn context_menu_setting_option_updates_context_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn context_menu_preview_action_updates_context_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn context_menu_preview_submenu_and_item_selection_use_real_core_actions() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert!(super::window_interaction::apply_context_click_for_test(
        &mut state,
        component.x + 1,
        component.y + 1,
    ));
    assert_eq!("context_menu_open", state.screen_state.last_action);
    assert_eq!("context_menu_opened", state.screen_state.last_event);

    let closed = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    );
    let insert = dedicated_context_menu_popup::insert_row_rect(component.x, component.y);
    assert!(apply_click(&mut state, insert.x + 1, insert.y + 1));
    assert_eq!("context_menu_open_submenu", state.screen_state.last_action);
    assert_eq!("context_menu_submenu_opened", state.screen_state.last_event);
    assert_eq!("context_menu.submenu=[2]", state.screen_state.state_label);

    let submenu = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    );
    assert!(component_body_pixel_diff(PAGE, &closed, &submenu) > BODY_DIFF_THRESHOLD);

    let link = dedicated_context_menu_popup::submenu_link_rect(component.x, component.y);
    assert!(apply_click(&mut state, link.x + 1, link.y + 1));
    assert_eq!("context_menu_select_item", state.screen_state.last_action);
    assert_eq!("context_menu_item_selected", state.screen_state.last_event);
    assert_eq!("context_menu.command", state.screen_state.last_setting);
    assert_eq!("link", state.screen_state.last_setting_value);
    assert_eq!(
        "context_menu.selected=[2,1]",
        state.screen_state.state_label
    );

    let selected = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    );
    assert!(component_body_pixel_diff(PAGE, &submenu, &selected) > BODY_DIFF_THRESHOLD);
}

#[test]
fn context_menu_window_interaction_keeps_context_action_instance_isolated() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };

    state.select_instance(PRIMARY_INSTANCE);
    context_click_component(&mut state);
    click_insert(&mut state);
    click_link(&mut state);
    let primary = state.screen_state.clone();
    let primary_canvas = render_state(&state);
    assert_eq!("context_menu_select_item", primary.last_action);
    assert_eq!("context_menu.selected=[2,1]", primary.state_label);

    state.select_instance(SECONDARY_INSTANCE);
    assert_eq!("none", state.screen_state.last_action);
    context_click_component(&mut state);
    let secondary = state.screen_state.clone();
    let secondary_canvas = render_state(&state);
    assert_eq!("context_menu_open", secondary.last_action);
    assert_eq!("context_menu=open", secondary.state_label);

    state.select_instance(PRIMARY_INSTANCE);
    assert_eq!(primary.last_action, state.screen_state.last_action);
    assert_eq!(primary.state_label, state.screen_state.state_label);
    assert!(
        component_body_pixel_diff(PAGE, &primary_canvas, &secondary_canvas) > BODY_DIFF_THRESHOLD
    );
}

#[test]
fn context_menu_light_and_dark_panel_uses_theme_panel() {
    assert_menu_panel_token(DARK_THEME, ThemeSnapshot::dark());
    assert_menu_panel_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn context_click_component(state: &mut StorybookWindowState) {
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert!(super::window_interaction::apply_context_click_for_test(
        state,
        component.x + 1,
        component.y + 1,
    ));
}

fn click_insert(state: &mut StorybookWindowState) {
    let component = preview_detail::component_action_hit_rect(PAGE);
    let insert = dedicated_context_menu_popup::insert_row_rect(component.x, component.y);

    assert!(apply_click(state, insert.x + 1, insert.y + 1));
}

fn click_link(state: &mut StorybookWindowState) {
    let component = preview_detail::component_action_hit_rect(PAGE);
    let link = dedicated_context_menu_popup::submenu_link_rect(component.x, component.y);

    assert!(apply_click(state, link.x + 1, link.y + 1));
}

fn render_state(state: &StorybookWindowState) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    )
}

fn assert_menu_panel_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, EDITOR_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert_eq!(
        Some(colors.panel),
        pixel_at(
            &canvas,
            component.x + MENU_X + MENU_SAMPLE_OFFSET,
            component.y + MENU_Y + MENU_SAMPLE_OFFSET
        )
    );
}
