use super::interaction_spec::StorybookInteractionSpec;
use super::render_context::ScenarioContext;
use super::screen_state::StorybookScreenState;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at,
};
use super::window_interaction::{
    StorybookWindowState, apply_click, apply_clickable_keyboard_activation_for_audit,
    focus_clickable_at_for_audit,
};
use super::{StorybookVisual, palette, preview_detail, render, storybook_ui_option_contract};
use crate::StoryCatalog;
use crate::catalog::StoryPresetLabels;
use crate::test_assert::KucTestExpect;
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "modal";
const NATIVE_PRESET: usize = 0;
const ESCAPE_PRESET: usize = 1;
const FOCUS_PRESET: usize = 2;
const PARENT_BLOCK_PRESET: usize = 3;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const MODAL_BLOCK_COUNT: usize = 4;
const BACKDROP_INDEX: usize = 0;
const DIALOG_INDEX: usize = 1;
const NATIVE_INDEX: usize = 2;
const CLOSE_INDEX: usize = 3;
const DIALOG_X: usize = 38;
const DIALOG_Y: usize = 42;
const DIALOG_SAMPLE_OFFSET: usize = 8;

#[test]
fn modal_exposes_leaf_presets_options_and_escape_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("modal_escape", spec.action);
    assert_eq!("modal_closed", spec.event);
    assert_eq!("interaction.open", spec.option);
    assert_eq!("false", spec.after);
    assert_eq!("open=false", spec.state);
}

#[test]
fn modal_presets_render_distinct_native_escape_focus_and_parent_states() {
    let native = StorybookVisual.render_preset(DARK_THEME, PAGE, NATIVE_PRESET, 0);
    let escape = StorybookVisual.render_preset(DARK_THEME, PAGE, ESCAPE_PRESET, 0);
    let focus = StorybookVisual.render_preset(DARK_THEME, PAGE, FOCUS_PRESET, 0);
    let parent = StorybookVisual.render_preset(DARK_THEME, PAGE, PARENT_BLOCK_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &native, &escape) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &escape, &focus) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &focus, &parent) > BODY_DIFF_THRESHOLD);
}

#[test]
fn modal_setting_option_updates_dialog_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn modal_preview_action_updates_dialog_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn modal_pointer_focus_and_keyboard_use_core_modal_contract() {
    let component = preview_detail::component_action_hit_rect(PAGE);

    let mut click_state = page_state();
    let before_click = render_state(&click_state);
    assert!(apply_click(
        &mut click_state,
        component.x + 1,
        component.y + 1
    ));
    let after_click = render_state(&click_state);
    assert_eq!("modal_escape", click_state.screen_state.last_action);
    assert_eq!("modal_closed", click_state.screen_state.last_event);
    assert_eq!("open=false", click_state.screen_state.state_label);
    assert!(component_body_pixel_diff(PAGE, &before_click, &after_click) > 0);

    let mut focus_state = page_state();
    let before_focus = render_state(&focus_state);
    assert!(focus_clickable_at_for_audit(
        &mut focus_state,
        component.x + 1,
        component.y + 1
    ));
    let after_focus = render_state(&focus_state);
    assert_eq!("modal_focus_trap", focus_state.screen_state.last_action);
    assert_eq!("modal_focused", focus_state.screen_state.last_event);
    assert_eq!("focus=trapped", focus_state.screen_state.state_label);
    assert!(focus_state.screen_state.is_button_focused());
    assert!(component_body_pixel_diff(PAGE, &before_focus, &after_focus) > 0);

    let before_keyboard = render_state(&focus_state);
    assert!(apply_clickable_keyboard_activation_for_audit(
        &mut focus_state
    ));
    let after_keyboard = render_state(&focus_state);
    assert_eq!("modal_escape", focus_state.screen_state.last_action);
    assert_eq!("modal_closed", focus_state.screen_state.last_event);
    assert_eq!("open=false", focus_state.screen_state.state_label);
    assert!(component_body_pixel_diff(PAGE, &before_keyboard, &after_keyboard) > 0);
}

#[test]
fn modal_escape_close_removes_modal_surfaces_from_preview() {
    let component = preview_detail::component_action_hit_rect(PAGE);
    let mut state = page_state();
    let before = modal_blocks_for_screen_state(&state.screen_state);

    assert!(apply_click(&mut state, component.x + 1, component.y + 1));

    let after = modal_blocks_for_screen_state(&state.screen_state);
    assert_eq!("open=false", state.screen_state.state_label);
    for index in [BACKDROP_INDEX, DIALOG_INDEX, NATIVE_INDEX, CLOSE_INDEX] {
        assert!(before[index].rect.width > 0);
        assert_eq!(
            0, after[index].rect.width,
            "closed modal must not leave surface block {index} visible"
        );
    }
}

#[test]
fn modal_escape_after_close_is_idempotent_and_does_not_reclose() {
    let component = preview_detail::component_action_hit_rect(PAGE);
    let mut state = page_state();

    assert!(focus_clickable_at_for_audit(
        &mut state,
        component.x + 1,
        component.y + 1
    ));
    assert!(apply_clickable_keyboard_activation_for_audit(&mut state));
    let closed = render_state(&state);
    let action_count = state.screen_state.action_count;

    assert!(!apply_clickable_keyboard_activation_for_audit(&mut state));
    let after_second_escape = render_state(&state);

    assert_eq!(action_count, state.screen_state.action_count);
    assert_eq!("modal_escape", state.screen_state.last_action);
    assert_eq!("modal_closed", state.screen_state.last_event);
    assert_eq!("open=false", state.screen_state.state_label);
    assert_eq!(
        0,
        component_body_pixel_diff(PAGE, &closed, &after_second_escape)
    );
}

#[test]
fn modal_story_connects_core_escape_and_focus_logs() {
    let examples = StoryCatalog.examples();
    let story = examples
        .iter()
        .find(|example| example.page == PAGE)
        .kuc_expect("modal story exists");

    assert!(story.callback_logs.iter().any(|callback| {
        callback.action == "modal_escape" && callback.after.contains("ModalEscaped")
    }));
    assert!(story.callback_logs.iter().any(|callback| {
        callback.action == "modal_focus_return" && callback.after.contains("FocusReturned")
    }));
}

#[test]
fn modal_light_and_dark_dialog_uses_theme_surface() {
    assert_dialog_token(DARK_THEME, ThemeSnapshot::dark());
    assert_dialog_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn page_state() -> StorybookWindowState {
    StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    }
}

fn render_state(state: &StorybookWindowState) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
        DARK_THEME,
        PAGE,
        state.preset_index,
        state.screen_state.clone(),
    )
}

fn modal_blocks_for_screen_state(
    screen_state: &StorybookScreenState,
) -> [super::dedicated_modal::ModalBlockSnapshot; MODAL_BLOCK_COUNT] {
    let colors = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    super::dedicated_modal::modal_blocks_for_test(&colors, scenario(screen_state))
}

fn scenario(screen_state: &StorybookScreenState) -> ScenarioContext<'_> {
    ScenarioContext {
        selected_page: PAGE,
        selected_instance_id: crate::visual::window_interaction::DEFAULT_INSTANCE_ID,
        preset_index: NATIVE_PRESET,
        preset_tab_scroll_x: 0,
        tree_expansion: Default::default(),
        scrollbar_visible: true,
        panel_scroll: Default::default(),
        screen_state,
        show_navigation_lines: true,
        show_navigation_text_connectors: false,
    }
}

fn assert_dialog_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, NATIVE_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert_eq!(
        Some(colors.surface),
        pixel_at(
            &canvas,
            component.x + DIALOG_X + DIALOG_SAMPLE_OFFSET,
            component.y + DIALOG_Y + DIALOG_SAMPLE_OFFSET
        )
    );
}
