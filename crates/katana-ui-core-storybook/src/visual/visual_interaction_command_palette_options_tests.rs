use super::visual_interaction_test_support::component_body_pixel_diff;
use super::window_interaction::{
    StorybookWindowState, apply_click, apply_clickable_keyboard_activation_for_audit,
    apply_command_palette_escape_for_audit, apply_hover_at, focus_clickable_at_for_audit,
};
use super::{layout_metrics, render, storybook_ui_option_contract};

const DARK_THEME: &str = "dark";
const PAGE: &str = "command-palette";
const PRIMARY_INSTANCE: &str = "command-palette.primary";
const SECONDARY_INSTANCE: &str = "command-palette.secondary";
const COMPONENT_HIT_INSET: usize = 4;

#[test]
fn command_palette_inspector_options_mutate_query_highlight_provider_semantic_state()
-> Result<(), String> {
    for &(setting, expected_state, expected_value) in expected_states() {
        let mut state = page_state();
        let before = render_state(&state);
        click_option(&mut state, setting)?;
        let after = render_state(&state);

        assert_eq!(setting, state.screen_state.last_setting);
        assert_eq!(
            "settings_command_palette_option",
            state.screen_state.last_action
        );
        assert_eq!("molecule_settings_changed", state.screen_state.last_event);
        assert_eq!(expected_value, state.screen_state.last_setting_value);
        assert_eq!(expected_state, state.screen_state.state_label);
        assert_command_palette_runtime(setting, &state);
        assert!(component_body_pixel_diff(PAGE, &before, &after) > 0);
    }
    Ok(())
}

#[test]
fn command_palette_window_interaction_keeps_query_and_highlight_instance_isolated()
-> Result<(), String> {
    let mut state = page_state();

    state.select_instance(PRIMARY_INSTANCE);
    click_option(&mut state, "command_palette.query")?;
    let primary_preset = state.preset_index;
    let primary = state.screen_state.clone();
    let primary_canvas = render_state(&state);
    assert_eq!("command_palette.query=theme", primary.state_label);

    state.select_instance(SECONDARY_INSTANCE);
    assert_eq!("idle", state.screen_state.state_label);
    click_option(&mut state, "command_palette.highlight")?;
    let secondary_preset = state.preset_index;
    let secondary = state.screen_state.clone();
    let secondary_canvas = render_state(&state);
    assert_eq!("command_palette.highlight=2", secondary.state_label);

    state.select_preset(primary_preset);
    state.select_instance(PRIMARY_INSTANCE);
    assert_eq!(primary.state_label, state.screen_state.state_label);

    state.select_preset(secondary_preset);
    state.select_instance(SECONDARY_INSTANCE);
    assert_eq!(secondary.state_label, state.screen_state.state_label);
    assert!(
        component_body_pixel_diff(PAGE, &primary_canvas, &secondary_canvas) > 0,
        "command-palette query/highlight state must be instance-local"
    );
    Ok(())
}

#[test]
fn command_palette_live_focus_hover_keyboard_execute_and_close_use_core_actions() {
    let mut hover_state = page_state();
    let hover_before = render_state(&hover_state);
    assert!(apply_hover_at(&mut hover_state, command_x(), command_y()));
    let hover_after = render_state(&hover_state);
    assert_eq!(
        "command_palette_hover",
        hover_state.screen_state.last_action
    );
    assert_eq!(
        "command_palette_hovered",
        hover_state.screen_state.last_event
    );
    assert_eq!("hover=true", hover_state.screen_state.state_label);
    assert!(hover_state.screen_state.preview_hovered);
    assert!(component_body_pixel_diff(PAGE, &hover_before, &hover_after) > 0);

    let mut keyboard_state = page_state();
    let focus_before = render_state(&keyboard_state);
    assert!(focus_clickable_at_for_audit(
        &mut keyboard_state,
        command_x(),
        command_y()
    ));
    let focus_after = render_state(&keyboard_state);
    assert_eq!(
        "command_palette_focus",
        keyboard_state.screen_state.last_action
    );
    assert_eq!(
        "command_palette_focused",
        keyboard_state.screen_state.last_event
    );
    assert_eq!("focus=true", keyboard_state.screen_state.state_label);
    assert!(keyboard_state.screen_state.is_button_focused());
    assert!(component_body_pixel_diff(PAGE, &focus_before, &focus_after) > 0);

    let execute_before = render_state(&keyboard_state);
    assert!(apply_clickable_keyboard_activation_for_audit(
        &mut keyboard_state
    ));
    let execute_after = render_state(&keyboard_state);
    assert_eq!(
        "command_palette_keyboard_execute",
        keyboard_state.screen_state.last_action
    );
    assert_eq!(
        "command_palette_result_executed",
        keyboard_state.screen_state.last_event
    );
    assert_eq!("executed=format", keyboard_state.screen_state.state_label);
    assert!(component_body_pixel_diff(PAGE, &execute_before, &execute_after) > 0);

    let close_before = render_state(&keyboard_state);
    assert!(apply_command_palette_escape_for_audit(&mut keyboard_state));
    let close_after = render_state(&keyboard_state);
    assert_eq!(
        "command_palette_keyboard_close",
        keyboard_state.screen_state.last_action
    );
    assert_eq!(
        "command_palette_closed",
        keyboard_state.screen_state.last_event
    );
    assert_eq!("closed=true", keyboard_state.screen_state.state_label);
    assert!(component_body_pixel_diff(PAGE, &close_before, &close_after) > 0);
}

fn expected_states() -> &'static [(&'static str, &'static str, &'static str)] {
    &[
        (
            "command_palette.query",
            "command_palette.query=theme",
            "theme",
        ),
        (
            "command_palette.highlight",
            "command_palette.highlight=2",
            "2",
        ),
        (
            "command_palette.row_count",
            "command_palette.row_count=50",
            "50",
        ),
        (
            "command_palette.provider_group",
            "command_palette.provider_group=workspace/editor/app",
            "workspace/editor/app",
        ),
        (
            "command_palette.shortcut_display",
            "command_palette.shortcut_display=false",
            "false",
        ),
    ]
}

fn assert_command_palette_runtime(setting: &str, state: &StorybookWindowState) {
    let command_palette = &state.screen_state.command_palette;
    let options = command_palette.option_state();
    match setting {
        "command_palette.query" => {
            assert_eq!("theme", command_palette.query());
            assert_eq!(Some(2), command_palette.highlighted_index());
            assert_eq!("command_palette_query", command_palette.callback_action());
        }
        "command_palette.highlight" => {
            assert_eq!(Some(2), command_palette.highlighted_index());
            assert_eq!(
                "command_palette_highlight",
                command_palette.callback_action()
            );
        }
        "command_palette.row_count" => {
            assert_eq!(50, options.row_count);
            assert_eq!(50, command_palette.row_count());
        }
        "command_palette.provider_group" => {
            assert!(options.provider_group_workspace_editor_app);
            assert_eq!(
                "command_palette_provider_group",
                command_palette.callback_action()
            );
        }
        "command_palette.shortcut_display" => {
            assert!(!options.shortcut_display_visible);
            assert_eq!(
                "command_palette_shortcut_display",
                command_palette.callback_action()
            );
        }
        _ => {}
    }
}

fn click_option(state: &mut StorybookWindowState, setting: &str) -> Result<(), String> {
    let index = option_index(setting)?;
    let row = layout_metrics::inspector_setting_row_hit_rect(index);

    assert!(apply_click(state, row.x + 1, row.y + 1));
    Ok(())
}

fn option_index(setting: &str) -> Result<usize, String> {
    storybook_ui_option_contract::options_for_page(PAGE)
        .iter()
        .position(|option| option.setting == setting)
        .ok_or_else(|| format!("missing command-palette option `{setting}`"))
}

fn render_state(state: &StorybookWindowState) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
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

fn command_x() -> usize {
    super::preview_detail::component_action_hit_rect(PAGE).x + COMPONENT_HIT_INSET
}

fn command_y() -> usize {
    super::preview_detail::component_action_hit_rect(PAGE).y + COMPONENT_HIT_INSET
}
