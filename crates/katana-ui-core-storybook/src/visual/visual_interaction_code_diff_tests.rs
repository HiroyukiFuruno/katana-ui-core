use super::interaction_spec::StorybookInteractionSpec;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at,
};
use super::{StorybookVisual, palette, preview_detail, render, storybook_ui_option_contract};
use crate::catalog::{StoryCatalog, StoryPresetLabels};
use crate::visual::window_interaction::{
    StorybookWindowState, apply_clickable_keyboard_activation_for_audit,
    apply_code_diff_scroll_sync_for_audit, apply_hover_at, focus_clickable_at_for_audit,
};
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "code-diff";
const MODE_PRESET: usize = 0;
const WHITESPACE_PRESET: usize = 1;
const DIRECTION_PRESET: usize = 2;
const CONTEXT_PRESET: usize = 3;
const ITEM_COUNT_PRESET: usize = 4;
const SCROLL_SYNC_PRESET: usize = 5;
const LANGUAGE_PRESET: usize = 6;
const REQUIRED_PRESET_COUNT: usize = 7;
const REQUIRED_OPTION_COUNT: usize = 7;
const BODY_DIFF_THRESHOLD: usize = 80;
const SURFACE_TOKEN_X: usize = 338;
const SURFACE_TOKEN_Y: usize = 20;
const SURFACE_SAMPLE_X_OFFSET: usize = 120;
const SURFACE_SAMPLE_Y_OFFSET: usize = 12;

#[test]
fn code_diff_exposes_leaf_presets_options_and_mode_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert!(rows.iter().any(|row| row.starts_with("code_diff.mode:")));
    assert!(
        rows.iter()
            .any(|row| row.starts_with("code_diff.whitespace:"))
    );
    assert!(
        rows.iter()
            .any(|row| row.starts_with("code_diff.direction:"))
    );
    assert!(
        rows.iter()
            .any(|row| row.starts_with("code_diff.item_count:"))
    );
    assert_eq!("diff_mode_switch", spec.action);
    assert_eq!("diff_mode_changed", spec.event);
    assert_eq!("interaction.value", spec.option);
    assert_eq!("Split", spec.after);
    assert_eq!("mode=split", spec.state);
}

#[test]
fn code_diff_presets_render_distinct_split_inline_collapsed_and_whitespace_states() {
    let mode = StorybookVisual.render_preset(DARK_THEME, PAGE, MODE_PRESET, 0);
    let whitespace = StorybookVisual.render_preset(DARK_THEME, PAGE, WHITESPACE_PRESET, 0);
    let direction = StorybookVisual.render_preset(DARK_THEME, PAGE, DIRECTION_PRESET, 0);
    let context = StorybookVisual.render_preset(DARK_THEME, PAGE, CONTEXT_PRESET, 0);
    let item_count = StorybookVisual.render_preset(DARK_THEME, PAGE, ITEM_COUNT_PRESET, 0);
    let scroll_sync = StorybookVisual.render_preset(DARK_THEME, PAGE, SCROLL_SYNC_PRESET, 0);
    let language = StorybookVisual.render_preset(DARK_THEME, PAGE, LANGUAGE_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &mode, &whitespace) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &whitespace, &direction) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &direction, &context) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &context, &item_count) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &item_count, &scroll_sync) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &scroll_sync, &language) > BODY_DIFF_THRESHOLD);
}

#[test]
fn code_diff_setting_option_updates_mode_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn code_diff_preview_action_updates_mode_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn code_diff_story_materializes_lines_highlights_collapse_and_callback() -> Result<(), &'static str>
{
    let examples = StoryCatalog.examples();
    let story = examples
        .iter()
        .find(|it| it.page == PAGE)
        .ok_or("code-diff story missing")?;
    let root = story.tree.root();

    assert_eq!("Code diff", root.props().label);
    assert_eq!(2, root.props().interaction.item_count);
    assert!(
        story
            .callback_logs
            .iter()
            .any(|it| it.action == "code_diff_mode_changed" && it.after.contains("value=Split"))
    );
    assert!(story.tree.root().children().iter().any(|node| {
        node.props()
            .label
            .contains("日本語 highlight: 表示ずれなし")
    }));
    Ok(())
}

#[test]
fn code_diff_hover_focus_keyboard_and_scroll_sync_route_through_core_actions() {
    assert_code_diff_hover_updates_state();
    assert_code_diff_focus_updates_state();
    assert_code_diff_keyboard_after_focus_updates_state();
    assert_code_diff_scroll_sync_updates_state();
}

fn assert_code_diff_hover_updates_state() {
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

    assert_eq!("code_diff_hover", state.screen_state.last_action);
    assert_eq!("code_diff_hovered", state.screen_state.last_event);
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

fn assert_code_diff_focus_updates_state() {
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

    assert_eq!("code_diff_focus", state.screen_state.last_action);
    assert_eq!("code_diff_focused", state.screen_state.last_event);
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

fn assert_code_diff_keyboard_after_focus_updates_state() {
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

    assert_eq!("code_diff_expand", state.screen_state.last_action);
    assert_eq!("code_diff_block_expanded", state.screen_state.last_event);
    assert_eq!("code_diff.collapsed_block", state.screen_state.last_setting);
    assert_eq!("collapsed=false", state.screen_state.state_label);
    let after = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    );
    assert!(component_body_pixel_diff(PAGE, &before, &after) > BODY_DIFF_THRESHOLD);
}

fn assert_code_diff_scroll_sync_updates_state() {
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

    assert!(apply_code_diff_scroll_sync_for_audit(
        &mut state,
        target.x + 1,
        target.y + 1
    ));

    assert_eq!("code_diff_scroll_sync", state.screen_state.last_action);
    assert_eq!(
        "code_diff_scroll_sync_changed",
        state.screen_state.last_event
    );
    assert_eq!("code_diff.scroll_sync", state.screen_state.last_setting);
    assert_eq!("scroll_sync=true", state.screen_state.state_label);
    let after = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    );
    assert!(component_body_pixel_diff(PAGE, &before, &after) > BODY_DIFF_THRESHOLD);
}

#[test]
fn code_diff_light_and_dark_surface_token_uses_theme_surface() {
    assert_surface_token(DARK_THEME, ThemeSnapshot::dark());
    assert_surface_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_surface_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, MODE_PRESET, 0);
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
