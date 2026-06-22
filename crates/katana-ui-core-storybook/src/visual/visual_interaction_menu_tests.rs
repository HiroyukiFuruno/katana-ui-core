use super::interaction_spec::StorybookInteractionSpec;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at,
};
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{
    StorybookVisual, dedicated_dod_molecule_menu, palette, preview_detail, render,
    storybook_ui_option_contract,
};
use crate::StoryCatalog;
use crate::catalog::StoryPresetLabels;
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "menu";
const MENU_ITEMS_PRESET: usize = 0;
const SHORTCUT_PRESET: usize = 1;
const DISABLED_PRESET: usize = 2;
const THEME_PRESET: usize = 3;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const ROW_X: usize = 36;
const FIRST_ROW_Y: usize = 38;
const ROW_SAMPLE_OFFSET: usize = 4;
const PRIMARY_INSTANCE: &str = "menu.primary";
const SECONDARY_INSTANCE: &str = "menu.secondary";

#[test]
fn menu_exposes_leaf_presets_options_and_menu_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("menu_open", spec.action);
    assert_eq!("menu_opened", spec.event);
    assert_eq!("interaction.open", spec.option);
    assert_eq!("true", spec.after);
    assert_eq!("open=true", spec.state);
    assert!(
        options
            .iter()
            .any(|option| option.setting == "menu.panel_placement")
    );
    assert!(!options.iter().any(|option| option.setting == "overflow"));
    assert!(
        !options
            .iter()
            .any(|option| option.setting.contains("submenu"))
    );
    assert!(
        !options
            .iter()
            .any(|option| option.setting.contains("separator"))
    );
}

#[test]
fn menu_presets_render_distinct_menu_bodies() {
    let items = StorybookVisual.render_preset(DARK_THEME, PAGE, MENU_ITEMS_PRESET, 0);
    let shortcut = StorybookVisual.render_preset(DARK_THEME, PAGE, SHORTCUT_PRESET, 0);
    let disabled = StorybookVisual.render_preset(DARK_THEME, PAGE, DISABLED_PRESET, 0);
    let themed = StorybookVisual.render_preset(DARK_THEME, PAGE, THEME_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &items, &shortcut) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &shortcut, &disabled) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &items, &themed) > BODY_DIFF_THRESHOLD);
}

#[test]
fn menu_setting_option_updates_menu_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn menu_preview_action_updates_menu_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn menu_story_connects_action_event_and_callback_contract() {
    let examples = StoryCatalog.examples();
    let story = examples.iter().find(|example| example.page == PAGE);

    assert!(story.is_some(), "menu story exists");
    let Some(story) = story else {
        return;
    };

    let actions: Vec<&str> = story
        .callback_logs
        .iter()
        .map(|callback| callback.action.as_str())
        .collect();

    assert!(actions.contains(&"menu_open"));
    assert!(actions.contains(&"menu_close"));
    assert!(actions.contains(&"menu_select"));
    assert!(actions.contains(&"menu_shortcut_activate"));
    assert!(!actions.contains(&"menu_disabled"));
}

#[test]
fn menu_open_close_clicks_update_action_event_state() {
    let mut state = menu_state(MENU_ITEMS_PRESET);
    let component = preview_detail::component_action_hit_rect(PAGE);
    let open = dedicated_dod_molecule_menu::first_row_rect(component);
    let close = dedicated_dod_molecule_menu::second_row_rect(component);

    assert!(apply_click(&mut state, open.x + 1, open.y + 1));
    assert_eq!(1, state.screen_state.action_count);
    assert!(state.screen_state.selection.select_open);
    assert_eq!("menu_open", state.screen_state.last_action);
    assert_eq!("menu_opened", state.screen_state.last_event);

    assert!(apply_click(&mut state, close.x + 1, close.y + 1));
    assert_eq!(2, state.screen_state.action_count);
    assert!(!state.screen_state.selection.select_open);
    assert_eq!("menu_select", state.screen_state.last_action);
    assert_eq!("menu_item_selected", state.screen_state.last_event);
    assert_eq!(Some(1), state.screen_state.selection.select_selected_index);
}

#[test]
fn menu_disabled_item_click_does_not_select_or_callback() {
    let mut state = menu_state(DISABLED_PRESET);
    state.screen_state.selection.select_open = true;
    let component = preview_detail::component_action_hit_rect(PAGE);
    let disabled = dedicated_dod_molecule_menu::second_row_rect(component);

    assert!(apply_click(&mut state, disabled.x + 1, disabled.y + 1));
    assert_eq!(0, state.screen_state.action_count);
    assert_eq!("none", state.screen_state.last_action);
    assert_eq!("none", state.screen_state.last_event);
    assert_eq!(None, state.screen_state.selection.select_selected_index);
}

#[test]
fn menu_shortcut_activation_uses_action_path_callback() {
    let mut state = menu_state(SHORTCUT_PRESET);
    let component = preview_detail::component_action_hit_rect(PAGE);
    let shortcut = dedicated_dod_molecule_menu::first_row_rect(component);

    assert!(apply_click(&mut state, shortcut.x + 1, shortcut.y + 1));
    assert_eq!(1, state.screen_state.action_count);
    assert_eq!("menu_shortcut_activate", state.screen_state.last_action);
    assert_eq!("menu_item_selected", state.screen_state.last_event);
    assert_eq!(
        "shortcut=Cmd+O selected=open",
        state.screen_state.state_label
    );
    assert_eq!(Some(0), state.screen_state.selection.select_selected_index);
}

#[test]
fn menu_click_actions_change_rendered_component_body() {
    let mut state = menu_state(MENU_ITEMS_PRESET);
    let before = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    );
    let component = preview_detail::component_action_hit_rect(PAGE);
    let open = dedicated_dod_molecule_menu::first_row_rect(component);

    assert!(apply_click(&mut state, open.x + 1, open.y + 1));
    let after = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    );

    assert!(component_body_pixel_diff(PAGE, &before, &after) > BODY_DIFF_THRESHOLD);
}

#[test]
fn menu_window_interaction_keeps_open_select_shortcut_instance_isolated() {
    let mut state = menu_state(MENU_ITEMS_PRESET);

    state.select_instance(PRIMARY_INSTANCE);
    click_first_row(&mut state);
    click_second_row(&mut state);
    let primary = state.screen_state.clone();
    let primary_canvas = render_state(&state);
    assert_eq!("menu_select", primary.last_action);
    assert_eq!(Some(1), primary.selection.select_selected_index);

    state.select_instance(SECONDARY_INSTANCE);
    assert_eq!("none", state.screen_state.last_action);
    click_first_row(&mut state);
    let secondary = state.screen_state.clone();
    let secondary_canvas = render_state(&state);
    assert_eq!("menu_open", secondary.last_action);
    assert!(secondary.selection.select_open);
    assert_eq!(None, secondary.selection.select_selected_index);

    state.select_instance(PRIMARY_INSTANCE);
    assert_eq!(primary.last_action, state.screen_state.last_action);
    assert_eq!(
        primary.selection.select_selected_index,
        state.screen_state.selection.select_selected_index
    );
    assert!(
        component_body_pixel_diff(PAGE, &primary_canvas, &secondary_canvas) > BODY_DIFF_THRESHOLD
    );
}

#[test]
fn menu_shortcut_activation_keeps_instance_state_isolated() {
    let mut state = menu_state(SHORTCUT_PRESET);

    state.select_instance(PRIMARY_INSTANCE);
    click_first_row(&mut state);
    let primary = state.screen_state.clone();
    assert_eq!("menu_shortcut_activate", primary.last_action);
    assert_eq!(Some(0), primary.selection.select_selected_index);

    state.select_instance(SECONDARY_INSTANCE);
    assert_eq!("none", state.screen_state.last_action);
    assert_eq!(None, state.screen_state.selection.select_selected_index);

    state.select_instance(PRIMARY_INSTANCE);
    assert_eq!(primary.last_action, state.screen_state.last_action);
    assert_eq!(
        primary.selection.select_selected_index,
        state.screen_state.selection.select_selected_index
    );
}

#[test]
fn menu_light_and_dark_row_uses_theme_surface() {
    assert_row_token(DARK_THEME, ThemeSnapshot::dark());
    assert_row_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn menu_state(preset_index: usize) -> StorybookWindowState {
    StorybookWindowState {
        selected_page: PAGE,
        preset_index,
        ..StorybookWindowState::default()
    }
}

fn click_first_row(state: &mut StorybookWindowState) {
    let component = preview_detail::component_action_hit_rect(PAGE);
    let row = dedicated_dod_molecule_menu::first_row_rect(component);

    assert!(apply_click(state, row.x + 1, row.y + 1));
}

fn click_second_row(state: &mut StorybookWindowState) {
    let component = preview_detail::component_action_hit_rect(PAGE);
    let row = dedicated_dod_molecule_menu::second_row_rect(component);

    assert!(apply_click(state, row.x + 1, row.y + 1));
}

fn render_state(state: &StorybookWindowState) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    )
}

fn assert_row_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, MENU_ITEMS_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert_eq!(
        Some(colors.surface),
        pixel_at(
            &canvas,
            component.x + ROW_X + ROW_SAMPLE_OFFSET,
            component.y + FIRST_ROW_Y + ROW_SAMPLE_OFFSET
        )
    );
}
