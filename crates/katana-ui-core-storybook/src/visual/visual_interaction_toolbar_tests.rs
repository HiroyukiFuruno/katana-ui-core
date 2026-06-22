use super::interaction_spec::StorybookInteractionSpec;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at,
};
use super::window_interaction::{
    StorybookCursorStyle, StorybookWindowState, apply_click,
    apply_clickable_keyboard_activation_for_audit, apply_hover_at, cursor_style_at_for_test,
    focus_clickable_at_for_audit,
};
use super::{
    StorybookVisual, dedicated_toolbar, palette, preview_detail, render,
    storybook_ui_option_contract,
};
use crate::StoryCatalog;
use crate::catalog::StoryPresetLabels;
use crate::test_assert::KucTestExpect;
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "toolbar";
const SPLIT_PRESET: usize = 1;
const REQUIRED_PRESET_COUNT: usize = 18;
const REQUIRED_OPTION_COUNT: usize = 18;
const BODY_DIFF_THRESHOLD: usize = 80;
const BAR_X: usize = 44;
const BAR_Y: usize = 42;
const BAR_SAMPLE_X_OFFSET: usize = 382;
const BAR_SAMPLE_Y_OFFSET: usize = 8;
const SAVE_ACTION_INDEX: usize = 0;
const HOVER_BORDER_SAMPLE_X_OFFSET: usize = 8;

#[test]
fn toolbar_exposes_leaf_presets_options_and_action_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    for option in [
        "toolbar.display_mode",
        "toolbar.density",
        "toolbar.overflow_strategy",
        "toolbar.actions",
        "toolbar.groups",
        "toolbar.context_menu_anchor",
        "toolbar.action_priority",
        "toolbar.action_accelerator",
        "toolbar.action_split",
        "toolbar.action_group",
        "toolbar.action_tooltip",
        "toolbar.action_a11y",
        "toolbar.action_disabled",
        "toolbar.group_label",
        "toolbar.group_divider",
        "toolbar.split_disabled",
        "toolbar.split_tooltip",
        "toolbar.split_a11y",
    ] {
        assert!(
            options.iter().any(|it| it.setting == option),
            "toolbar option is not exposed: {option}"
        );
    }
    assert_eq!("tool_toggle", spec.action);
    assert_eq!("tool_changed", spec.event);
    assert_eq!("interaction.active", spec.option);
    assert_eq!("true", spec.after);
    assert_eq!("active=true", spec.state);
}

#[test]
fn toolbar_presets_render_distinct_overflow_split_display_density_and_accelerator_states() {
    let canvases = (0..REQUIRED_PRESET_COUNT)
        .map(|preset| StorybookVisual.render_preset(DARK_THEME, PAGE, preset, 0))
        .collect::<Vec<_>>();

    for index in 1..canvases.len() {
        assert!(
            component_body_pixel_diff(PAGE, &canvases[index - 1], &canvases[index])
                > BODY_DIFF_THRESHOLD,
            "toolbar preset {index} did not repaint the component body"
        );
    }
}

#[test]
fn toolbar_setting_option_updates_toolbar_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn toolbar_preview_action_updates_toolbar_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn toolbar_pointer_focus_and_keyboard_use_core_toolbar_contract() -> Result<(), String> {
    let component = preview_detail::component_action_hit_rect(PAGE);
    let action = dedicated_toolbar::action_rect_for_test(SAVE_ACTION_INDEX)
        .ok_or_else(|| "toolbar action rect is missing".to_string())?;
    let action_x = component.x + action.x + action.width / 2;
    let action_y = component.y + action.y + action.height / 2;

    let mut click_state = page_state();
    let before_click = render_state(&click_state);
    assert!(apply_click(&mut click_state, action_x, action_y));
    let after_click = render_state(&click_state);
    assert_eq!("tool_toggle", click_state.screen_state.last_action);
    assert_eq!("tool_changed", click_state.screen_state.last_event);
    assert_eq!("active=true", click_state.screen_state.state_label);
    assert!(component_body_pixel_diff(PAGE, &before_click, &after_click) > 0);

    let mut focus_state = page_state();
    let before_focus = render_state(&focus_state);
    assert!(focus_clickable_at_for_audit(
        &mut focus_state,
        component.x + 1,
        component.y + 1
    ));
    let after_focus = render_state(&focus_state);
    assert_eq!("toolbar_focus", focus_state.screen_state.last_action);
    assert_eq!("toolbar_focused", focus_state.screen_state.last_event);
    assert_eq!("focus=save", focus_state.screen_state.state_label);
    assert!(focus_state.screen_state.is_button_focused());
    assert!(component_body_pixel_diff(PAGE, &before_focus, &after_focus) > 0);

    let before_keyboard = render_state(&focus_state);
    assert!(apply_clickable_keyboard_activation_for_audit(
        &mut focus_state
    ));
    let after_keyboard = render_state(&focus_state);
    assert_eq!("tool_toggle", focus_state.screen_state.last_action);
    assert_eq!("tool_changed", focus_state.screen_state.last_event);
    assert_eq!("active=true", focus_state.screen_state.state_label);
    assert!(component_body_pixel_diff(PAGE, &before_keyboard, &after_keyboard) > 0);

    Ok(())
}

#[test]
fn toolbar_story_connects_core_overflow_and_split_logs() {
    let examples = StoryCatalog.examples();
    let story = examples
        .iter()
        .find(|example| example.page == PAGE)
        .kuc_expect("toolbar story exists");

    assert!(story.callback_logs.iter().any(|callback| {
        callback.action == "toolbar_overflow_plan" && callback.after.contains("search")
    }));
    assert!(story.callback_logs.iter().any(|callback| {
        callback.action == "toolbar_split_open" && callback.after.contains("split_dropdown_opened")
    }));
}

#[test]
fn toolbar_action_hover_uses_button_family_feedback() -> Result<(), String> {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    let component = preview_detail::component_action_hit_rect(PAGE);
    let action = dedicated_toolbar::action_rect_for_test(SAVE_ACTION_INDEX)
        .ok_or_else(|| "toolbar action rect is missing".to_string())?;
    let hover_x = component.x + action.x + action.width / 2;
    let hover_y = component.y + action.y + action.height / 2;
    let before = StorybookVisual.render_preset(DARK_THEME, PAGE, SPLIT_PRESET, 0);

    assert_eq!(
        StorybookCursorStyle::PointingHand,
        cursor_style_at_for_test(&state, hover_x, hover_y)
    );
    assert!(apply_hover_at(&mut state, hover_x, hover_y));
    let after = render::render_storybook_canvas_with_screen_state(
        DARK_THEME,
        PAGE,
        SPLIT_PRESET,
        state.screen_state.clone(),
    );
    let hover_border = pixel_at(
        &after,
        component.x + action.x + HOVER_BORDER_SAMPLE_X_OFFSET,
        component.y + action.y,
    );

    assert_ne!(
        pixel_at(
            &before,
            component.x + action.x + HOVER_BORDER_SAMPLE_X_OFFSET,
            component.y + action.y
        ),
        hover_border
    );
    assert_eq!(
        Some(palette::VisualPalette::from_theme(&ThemeSnapshot::dark()).hover_border),
        hover_border
    );
    Ok(())
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

#[test]
fn toolbar_light_and_dark_bar_uses_theme_surface() {
    assert_bar_token(DARK_THEME, ThemeSnapshot::dark());
    assert_bar_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_bar_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, SPLIT_PRESET, 0);
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
