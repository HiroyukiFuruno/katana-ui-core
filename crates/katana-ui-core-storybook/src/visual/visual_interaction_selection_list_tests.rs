use super::interaction_spec::StorybookInteractionSpec;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at,
};
use super::window_interaction::{
    StorybookWindowState, apply_clickable_keyboard_activation_for_audit, apply_hover_at,
    apply_selection_list_scroll_for_audit, focus_clickable_at_for_audit,
};
use super::{
    StorybookVisual, palette, preview_detail, selection_control_metrics as sm,
    storybook_ui_option_contract,
};
use crate::catalog::StoryPresetLabels;
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "selection-list";
const ITEMS_PRESET: usize = 0;
const SELECT_PRESET: usize = 1;
const MULTI_PRESET: usize = 2;
const THEME_PRESET: usize = 3;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const LIST_FILL_SAMPLE_X_OFFSET: usize = 4;
const LIST_FILL_SAMPLE_Y_OFFSET: usize = 4;

#[test]
fn selection_list_exposes_leaf_presets_options_and_selection_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("selection_list_select_row", spec.action);
    assert_eq!("select_box_selected", spec.event);
    assert_eq!("interaction.selected_index", spec.option);
    assert_eq!("2", spec.after);
    assert_eq!("single=2 multi=none focus=2", spec.state);
    assert!(
        options
            .iter()
            .any(|option| option.setting == "selection_list.marker")
    );
    assert!(
        options
            .iter()
            .any(|option| option.setting == "selection_list.more_row")
    );
    assert!(
        !options
            .iter()
            .any(|option| option.setting == "theme.marker")
    );
}

#[test]
fn selection_list_presets_render_distinct_list_bodies() {
    let items = StorybookVisual.render_preset(DARK_THEME, PAGE, ITEMS_PRESET, 0);
    let selected = StorybookVisual.render_preset(DARK_THEME, PAGE, SELECT_PRESET, 0);
    let multi = StorybookVisual.render_preset(DARK_THEME, PAGE, MULTI_PRESET, 0);
    let themed = StorybookVisual.render_preset(DARK_THEME, PAGE, THEME_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &items, &selected) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &selected, &multi) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &items, &themed) > BODY_DIFF_THRESHOLD);
}

#[test]
fn selection_list_setting_option_updates_list_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn selection_list_preview_action_updates_list_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn selection_list_live_hover_focus_keyboard_and_scroll_use_core_paths() {
    let mut state = page_state();
    let target = preview_detail::component_action_hit_rect(PAGE);
    let before_hover = render_state(&state);
    assert!(apply_hover_at(&mut state, target.x + 1, target.y + 1));
    let after_hover = render_state(&state);

    assert_eq!("selection_list_hover", state.screen_state.last_action);
    assert_eq!("hover_start", state.screen_state.last_event);
    assert_eq!("hover=true", state.screen_state.state_label);
    assert!(state.screen_state.selection.selection_list_hovered);
    assert!(component_body_pixel_diff(PAGE, &before_hover, &after_hover) > 0);

    let before_focus = render_state(&state);
    assert!(focus_clickable_at_for_audit(
        &mut state,
        target.x + 1,
        target.y + 1
    ));
    let after_focus = render_state(&state);

    assert_eq!("selection_list_focus", state.screen_state.last_action);
    assert_eq!("focus", state.screen_state.last_event);
    assert_eq!(
        "single=none multi=none focus=0",
        state.screen_state.state_label
    );
    assert!(state.screen_state.is_button_focused());
    assert_eq!(
        Some(0),
        state.screen_state.selection.selection_list_focus_index
    );
    assert!(component_body_pixel_diff(PAGE, &before_focus, &after_focus) > 0);

    let before_keyboard = render_state(&state);
    assert!(apply_clickable_keyboard_activation_for_audit(&mut state));
    let after_keyboard = render_state(&state);

    assert_eq!(
        "selection_list_keyboard_next",
        state.screen_state.last_action
    );
    assert_eq!("set_selected_index", state.screen_state.last_event);
    assert_eq!(
        "single=1 multi=none focus=1",
        state.screen_state.state_label
    );
    assert_eq!(
        Some(1),
        state.screen_state.selection.selection_list_focus_index
    );
    assert!(component_body_pixel_diff(PAGE, &before_keyboard, &after_keyboard) > 0);

    let before_scroll = render_state(&state);
    assert!(apply_selection_list_scroll_for_audit(
        &mut state,
        target.x + 1,
        target.y + 1
    ));
    let after_scroll = render_state(&state);

    assert_eq!("selection_list_scroll", state.screen_state.last_action);
    assert_eq!("scroll_by", state.screen_state.last_event);
    assert_eq!("scroll=1", state.screen_state.state_label);
    assert_eq!(1, state.screen_state.selection.selection_list_scroll_offset);
    assert!(component_body_pixel_diff(PAGE, &before_scroll, &after_scroll) > 0);
}

#[test]
fn selection_list_light_and_dark_rows_use_theme_tokens() {
    assert_list_tokens(DARK_THEME, ThemeSnapshot::dark());
    assert_list_tokens(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_list_tokens(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, ITEMS_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let component = preview_detail::component_action_hit_rect(PAGE);
    let list_x = component.x + sm::TRIGGER_X;
    let list_y = component.y + sm::SELECTION_LIST_Y;
    let border_y = list_y + sm::SELECTION_LIST_ROW_HEIGHT - 1;

    assert_eq!(Some(colors.border), pixel_at(&canvas, list_x, border_y));
    assert_eq!(
        Some(colors.surface),
        pixel_at(
            &canvas,
            list_x + LIST_FILL_SAMPLE_X_OFFSET,
            list_y + LIST_FILL_SAMPLE_Y_OFFSET
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
