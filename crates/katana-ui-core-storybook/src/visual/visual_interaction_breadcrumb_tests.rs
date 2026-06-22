use super::interaction_spec::StorybookInteractionSpec;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at,
};
use super::{StorybookVisual, palette, preview_detail, render, storybook_ui_option_contract};
use crate::catalog::{StoryCatalog, StoryPresetLabels};
use crate::visual::dedicated_breadcrumb;
use crate::visual::window_interaction::{
    StorybookWindowState, apply_click, apply_clickable_keyboard_activation_for_audit,
    apply_hover_at, focus_clickable_at_for_audit,
};
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "breadcrumb";
const TRAIL_PRESET: usize = 0;
const CLICK_PRESET: usize = 1;
const OVERFLOW_PRESET: usize = 2;
const THEME_PRESET: usize = 3;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const BAR_X: usize = 38;
const BAR_Y: usize = 46;
const BAR_SAMPLE_X_OFFSET: usize = 350;
const BAR_SAMPLE_Y_OFFSET: usize = 12;
const ROOT_INDEX: usize = 0;
const SRC_INDEX: usize = 1;
const FILE_INDEX: usize = 2;

#[test]
fn breadcrumb_exposes_leaf_presets_options_and_route_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("breadcrumb_click", spec.action);
    assert_eq!("route_changed", spec.event);
    assert_eq!("interaction.selected_index", spec.option);
    assert_eq!("2", spec.after);
    assert_eq!("route=2", spec.state);
    assert!(
        options
            .iter()
            .any(|option| option.setting == "breadcrumb.crumb_action")
    );
}

#[test]
fn breadcrumb_presets_render_distinct_trail_click_overflow_and_theme_states() {
    let trail = StorybookVisual.render_preset(DARK_THEME, PAGE, TRAIL_PRESET, 0);
    let click = StorybookVisual.render_preset(DARK_THEME, PAGE, CLICK_PRESET, 0);
    let overflow = StorybookVisual.render_preset(DARK_THEME, PAGE, OVERFLOW_PRESET, 0);
    let themed = StorybookVisual.render_preset(DARK_THEME, PAGE, THEME_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &trail, &click) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &click, &overflow) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &trail, &themed) > BODY_DIFF_THRESHOLD);
}

#[test]
fn breadcrumb_story_materializes_current_item_and_overflow_menu_props() -> Result<(), &'static str>
{
    let examples = StoryCatalog.examples();
    let story = examples
        .iter()
        .find(|it| it.page == PAGE)
        .ok_or("breadcrumb story missing")?;
    let root = story.tree.root();

    assert_eq!("Breadcrumb", root.props().label);
    assert!(root.props().interaction.open);
    assert_eq!(3, root.props().interaction.item_count);
    assert_eq!(0, root.props().interaction.selected_index);
    assert_eq!("root", root.props().interaction.value);
    assert!(
        story
            .callback_logs
            .iter()
            .any(|it| it.action == "breadcrumb_click" && it.after.contains("event=route_changed"))
    );
    Ok(())
}

#[test]
fn breadcrumb_setting_option_updates_route_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn breadcrumb_preview_action_updates_route_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn breadcrumb_clicking_each_crumb_updates_action_event_and_route_state() -> Result<(), String> {
    let component = preview_detail::component_action_hit_rect(PAGE);
    let root = dedicated_breadcrumb::root_crumb_rect(component.x, component.y);
    let src = dedicated_breadcrumb::src_crumb_rect(component.x, component.y);
    let file = dedicated_breadcrumb::file_crumb_rect(component.x, component.y);

    assert_crumb_click_updates_state(ROOT_INDEX, &root, "0", "route=0")?;
    assert_crumb_click_updates_state(SRC_INDEX, &src, "1", "route=1")?;
    assert_crumb_click_updates_state(FILE_INDEX, &file, "2", "route=2")?;

    Ok(())
}

#[test]
fn breadcrumb_hover_focus_and_keyboard_route_through_core_action_state() {
    assert_breadcrumb_hover_updates_state();
    assert_breadcrumb_focus_updates_state();
    assert_breadcrumb_keyboard_after_focus_updates_state();
}

fn assert_breadcrumb_hover_updates_state() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    let target = breadcrumb_file_target();
    let before = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    );

    assert!(apply_hover_at(&mut state, target.x + 1, target.y + 1));

    assert_eq!("breadcrumb_hover", state.screen_state.last_action);
    assert_eq!("breadcrumb_hovered", state.screen_state.last_event);
    assert_eq!("interaction.hovered", state.screen_state.last_setting);
    assert_eq!("route=2", state.screen_state.state_label);
    let after = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    );
    assert!(component_body_pixel_diff(PAGE, &before, &after) > BODY_DIFF_THRESHOLD);
}

fn assert_breadcrumb_focus_updates_state() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    let target = breadcrumb_file_target();
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

    assert_eq!("breadcrumb_focus", state.screen_state.last_action);
    assert_eq!("breadcrumb_focused", state.screen_state.last_event);
    assert_eq!("interaction.focused", state.screen_state.last_setting);
    assert_eq!("route=2", state.screen_state.state_label);
    assert!(state.screen_state.is_button_focused());
    let after = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    );
    assert!(component_body_pixel_diff(PAGE, &before, &after) > BODY_DIFF_THRESHOLD);
}

fn assert_breadcrumb_keyboard_after_focus_updates_state() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    let target = breadcrumb_file_target();
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

    assert_eq!("breadcrumb_click", state.screen_state.last_action);
    assert_eq!("route_changed", state.screen_state.last_event);
    assert_eq!(
        "interaction.selected_index",
        state.screen_state.last_setting
    );
    assert_eq!("1", state.screen_state.last_setting_value);
    assert_eq!("route=1", state.screen_state.state_label);
    let after = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    );
    assert!(component_body_pixel_diff(PAGE, &before, &after) > BODY_DIFF_THRESHOLD);
}

fn breadcrumb_file_target() -> crate::visual::layout_metrics::LayoutRect {
    let component = preview_detail::component_action_hit_rect(PAGE);
    dedicated_breadcrumb::file_crumb_rect(component.x, component.y)
}

fn assert_crumb_click_updates_state(
    index: usize,
    rect: &crate::visual::layout_metrics::LayoutRect,
    route_value: &str,
    route_state: &str,
) -> Result<(), String> {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    let clicked = apply_click(&mut state, rect.x + 1, rect.y + 1);
    assert!(
        clicked,
        "breadcrumb crumb {index} click should mutate state"
    );
    assert_eq!(
        "breadcrumb_click", state.screen_state.last_action,
        "index {index} action"
    );
    assert_eq!(
        "route_changed", state.screen_state.last_event,
        "index {index} event"
    );
    assert_eq!(
        "interaction.selected_index", state.screen_state.last_setting,
        "index {index} setting"
    );
    assert_eq!(
        route_value, state.screen_state.last_setting_value,
        "index {index} setting value"
    );
    assert_eq!(
        route_state, state.screen_state.state_label,
        "index {index} state"
    );
    assert_eq!(
        index, state.screen_state.breadcrumb_selected_index,
        "index {index} selected index"
    );
    Ok(())
}

#[test]
fn breadcrumb_light_and_dark_bar_uses_theme_surface() {
    assert_bar_token(DARK_THEME, ThemeSnapshot::dark());
    assert_bar_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_bar_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, TRAIL_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert_eq!(
        Some(colors.surface),
        pixel_at(
            &canvas,
            component.x + BAR_X + BAR_SAMPLE_X_OFFSET,
            component.y + BAR_Y + BAR_SAMPLE_Y_OFFSET
        )
    );
}
