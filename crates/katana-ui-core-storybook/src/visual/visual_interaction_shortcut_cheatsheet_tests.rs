use super::interaction_spec::StorybookInteractionSpec;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at,
};
use super::{StorybookVisual, palette, preview_detail, storybook_ui_option_contract};
use crate::catalog::StoryPresetLabels;
use crate::visual::window_interaction::{
    StorybookWindowState, apply_clickable_keyboard_activation_for_audit, apply_hover_at,
    apply_shortcut_cheatsheet_scroll_for_audit, focus_clickable_at_for_audit,
};
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "shortcut-cheatsheet";
const SAMPLE_PRESET: usize = 0;
const CATEGORY_PRESET: usize = 1;
const TWO_COLUMN_PRESET: usize = 2;
const ONE_COLUMN_PRESET: usize = 3;
const SELECT_PRESET: usize = 4;
const LABEL_PRESET: usize = 5;
const GROUPS_PRESET: usize = 6;
const GROUP_TITLE_PRESET: usize = 7;
const ITEMS_PRESET: usize = 8;
const ITEM_COMBO_PRESET: usize = 9;
const REQUIRED_PRESET_COUNT: usize = 10;
const REQUIRED_OPTION_COUNT: usize = 9;
const BODY_DIFF_THRESHOLD: usize = 80;
const SURFACE_TOKEN_X: usize = 338;
const SURFACE_TOKEN_Y: usize = 34;
const SURFACE_SAMPLE_X_OFFSET: usize = 120;
const SURFACE_SAMPLE_Y_OFFSET: usize = 12;

#[test]
fn shortcut_cheatsheet_exposes_leaf_presets_options_and_selection_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert_eq!(options.len(), REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    for option in [
        "shortcut_cheatsheet.label",
        "shortcut_cheatsheet.groups",
        "shortcut_cheatsheet.group_title",
        "shortcut_cheatsheet.items",
        "shortcut_cheatsheet.item_combo",
        "shortcut_cheatsheet.group_layout",
        "shortcut_cheatsheet.query",
        "shortcut_cheatsheet.selected",
        "shortcut_cheatsheet.result_count",
    ] {
        assert!(
            options.iter().any(|it| it.setting == option),
            "shortcut-cheatsheet option is not exposed: {option}"
        );
    }
    assert!(
        rows.iter()
            .any(|row| row.starts_with("shortcut_cheatsheet.group_layout:"))
    );
    assert!(
        rows.iter()
            .any(|row| row.starts_with("shortcut_cheatsheet.query:"))
    );
    assert_eq!("shortcut_filter_select", spec.action);
    assert_eq!("shortcut_selected", spec.event);
    assert_eq!("shortcut.query", spec.option);
    assert_eq!("format", spec.after);
    assert_eq!("selected=format", spec.state);
}

#[test]
fn shortcut_cheatsheet_presets_render_distinct_filter_layout_and_selection_states() {
    let sample = StorybookVisual.render_preset(DARK_THEME, PAGE, SAMPLE_PRESET, 0);
    let category = StorybookVisual.render_preset(DARK_THEME, PAGE, CATEGORY_PRESET, 0);
    let two_column = StorybookVisual.render_preset(DARK_THEME, PAGE, TWO_COLUMN_PRESET, 0);
    let one_column = StorybookVisual.render_preset(DARK_THEME, PAGE, ONE_COLUMN_PRESET, 0);
    let selected = StorybookVisual.render_preset(DARK_THEME, PAGE, SELECT_PRESET, 0);
    let label = StorybookVisual.render_preset(DARK_THEME, PAGE, LABEL_PRESET, 0);
    let groups = StorybookVisual.render_preset(DARK_THEME, PAGE, GROUPS_PRESET, 0);
    let group_title = StorybookVisual.render_preset(DARK_THEME, PAGE, GROUP_TITLE_PRESET, 0);
    let items = StorybookVisual.render_preset(DARK_THEME, PAGE, ITEMS_PRESET, 0);
    let item_combo = StorybookVisual.render_preset(DARK_THEME, PAGE, ITEM_COMBO_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &sample, &category) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &category, &two_column) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &two_column, &one_column) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &one_column, &selected) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &selected, &label) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &label, &groups) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &groups, &group_title) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &group_title, &items) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &items, &item_combo) > BODY_DIFF_THRESHOLD);
}

#[test]
fn shortcut_cheatsheet_setting_option_updates_filter_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn shortcut_cheatsheet_preview_action_updates_selected_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn shortcut_cheatsheet_live_hover_focus_keyboard_and_scroll_use_core_selection() {
    let target = preview_detail::component_action_hit_rect(PAGE);
    let mut hover = state_for();
    let before_hover = render_state(&hover);
    assert!(apply_hover_at(&mut hover, target.x + 1, target.y + 1));
    let after_hover = render_state(&hover);
    assert_eq!("shortcut_cheatsheet_hover", hover.screen_state.last_action);
    assert_eq!("hover_start", hover.screen_state.last_event);
    assert_eq!("hover=true", hover.screen_state.state_label);
    assert!(hover.screen_state.shortcut_cheatsheet.hovered);
    assert!(component_body_pixel_diff(PAGE, &before_hover, &after_hover) > 0);

    let mut keyboard = state_for();
    assert!(focus_clickable_at_for_audit(
        &mut keyboard,
        target.x + 1,
        target.y + 1
    ));
    assert_eq!(
        "shortcut_cheatsheet_focus",
        keyboard.screen_state.last_action
    );
    assert_eq!("focus", keyboard.screen_state.last_event);
    assert_eq!("focus=true", keyboard.screen_state.state_label);
    let before_key = render_state(&keyboard);
    assert!(apply_clickable_keyboard_activation_for_audit(&mut keyboard));
    let after_key = render_state(&keyboard);
    assert_eq!("shortcut_filter_select", keyboard.screen_state.last_action);
    assert_eq!("shortcut_selected", keyboard.screen_state.last_event);
    assert_eq!("selected=format", keyboard.screen_state.state_label);
    assert!(component_body_pixel_diff(PAGE, &before_key, &after_key) > 0);

    let mut scroll = state_for();
    let before_scroll = render_state(&scroll);
    assert!(apply_shortcut_cheatsheet_scroll_for_audit(
        &mut scroll,
        target.x + 1,
        target.y + 1
    ));
    let after_scroll = render_state(&scroll);
    assert_eq!(
        "shortcut_cheatsheet_scroll",
        scroll.screen_state.last_action
    );
    assert_eq!("scroll_by", scroll.screen_state.last_event);
    assert_eq!("scroll=1", scroll.screen_state.state_label);
    assert!(component_body_pixel_diff(PAGE, &before_scroll, &after_scroll) > 0);
}

#[test]
fn shortcut_cheatsheet_light_and_dark_surface_uses_theme_surface() {
    assert_cheatsheet_surface_token(DARK_THEME, ThemeSnapshot::dark());
    assert_cheatsheet_surface_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn state_for() -> StorybookWindowState {
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

fn assert_cheatsheet_surface_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, SAMPLE_PRESET, 0);
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
