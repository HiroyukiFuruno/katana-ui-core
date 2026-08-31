use super::visual_interaction_test_support::component_body_pixel_diff;
use super::window_interaction::diagnostics_list_operation::DiagnosticsListStoryAction;
use super::window_interaction::{
    StorybookWindowState, apply_click, apply_clickable_keyboard_activation_for_audit,
    apply_diagnostics_list_scroll_for_audit, apply_hover_at, focus_clickable_at_for_audit,
};
use super::{layout_metrics, preview_detail, render, storybook_ui_option_contract};

const PAGE: &str = "diagnostics-list";
const PRIMARY_INSTANCE: &str = "diagnostics-list.primary";
const SECONDARY_INSTANCE: &str = "diagnostics-list.secondary";

#[test]
fn diagnostics_list_inspector_options_mutate_filter_bulk_and_fix_preview_semantic_state()
-> Result<(), String> {
    for &(setting, expected_state, expected_value) in expected_states() {
        let mut state = page_state();
        let before = render_state(&state);
        click_option(&mut state, setting)?;
        let after = render_state(&state);

        assert_eq!(setting, state.screen_state.last_setting);
        assert_eq!(
            "settings_diagnostics_option",
            state.screen_state.last_action
        );
        assert_eq!("molecule_settings_changed", state.screen_state.last_event);
        assert_eq!(expected_value, state.screen_state.last_setting_value);
        assert_eq!(expected_state, state.screen_state.state_label);
        assert_diagnostics_list_runtime(setting, &state);
        assert!(component_body_pixel_diff(PAGE, &before, &after) > 0);
    }
    Ok(())
}

#[test]
fn diagnostics_list_window_interaction_keeps_filter_bulk_fix_preview_instance_isolated()
-> Result<(), String> {
    let mut state = page_state();
    state.select_instance(PRIMARY_INSTANCE);
    let before = render_state(&state);

    click_component(&mut state);

    assert_eq!(1, state.screen_state.action_count);
    assert_eq!("diagnostic_fix_preview", state.screen_state.last_action);
    assert_eq!(
        "diagnostic_fix_preview_toggled",
        state.screen_state.last_event
    );
    assert_eq!("preview=true", state.screen_state.state_label);
    assert!(state.screen_state.diagnostics_list.has_fix_preview());
    assert_eq!(
        "diagnostic_fix_preview",
        state.screen_state.diagnostics_list.callback_action()
    );

    click_option(&mut state, "diagnostics.severity_filter")?;
    assert!(state.screen_state.diagnostics_list.has_error_filter());
    let _ = state
        .screen_state
        .diagnostics_list
        .apply_action(DiagnosticsListStoryAction::OpenBulkPreview);
    assert!(state.screen_state.diagnostics_list.has_bulk_preview_open());
    click_option(&mut state, "diagnostics.bulk_action")?;
    assert!(state.screen_state.diagnostics_list.has_bulk_applied());
    assert!(!state.screen_state.diagnostics_list.has_bulk_preview_open());
    assert_eq!(
        "diagnostic_bulk_apply",
        state.screen_state.diagnostics_list.callback_action()
    );
    click_option(&mut state, "diagnostics.fix_preview")?;
    assert!(!state.screen_state.diagnostics_list.has_fix_preview());
    let primary = state.screen_state.clone();
    assert!(component_body_pixel_diff(PAGE, &before, &render_state(&state)) > 0);

    state.select_instance(SECONDARY_INSTANCE);
    assert_eq!(0, state.screen_state.action_count);
    assert_eq!("idle", state.screen_state.state_label);
    assert!(!state.screen_state.diagnostics_list.has_error_filter());
    assert!(!state.screen_state.diagnostics_list.has_bulk_applied());
    assert!(!state.screen_state.diagnostics_list.has_bulk_preview_open());
    assert!(!state.screen_state.diagnostics_list.has_fix_preview());

    state.select_instance(PRIMARY_INSTANCE);
    assert_eq!(primary, state.screen_state);
    Ok(())
}

#[test]
fn diagnostics_list_live_operations_select_jump_focus_hover_and_keep_scroll() {
    let mut state = page_state();
    let target = preview_detail::component_action_hit_rect(PAGE);
    let before = render_state(&state);

    click_component(&mut state);

    assert_eq!("diagnostic_fix_preview", state.screen_state.last_action);
    assert!(state.screen_state.diagnostics_list.has_fix_preview());
    assert!(component_body_pixel_diff(PAGE, &before, &render_state(&state)) > 0);

    let hover_before = render_state(&state);
    assert!(apply_hover_at(
        &mut state,
        target.x + target.width / 2,
        target.y + target.height / 2
    ));
    assert_eq!("diagnostic_hover_item", state.screen_state.last_action);
    assert_eq!("hover_start", state.screen_state.last_event);
    assert!(state.screen_state.preview_hovered);
    assert!(state.screen_state.diagnostics_list.hovered);
    assert!(component_body_pixel_diff(PAGE, &hover_before, &render_state(&state)) > 0);

    let focus_before = render_state(&state);
    assert!(focus_clickable_at_for_audit(
        &mut state,
        target.x + 4,
        target.y + 4
    ));
    assert_eq!("diagnostic_focus_list", state.screen_state.last_action);
    assert_eq!("diagnostic_selected", state.screen_state.last_event);
    assert_eq!("focus=syntax-error", state.screen_state.state_label);
    assert!(state.screen_state.is_button_focused());
    assert!(state.screen_state.diagnostics_list.focused);
    assert!(state.screen_state.diagnostics_list.selected_item());
    assert!(component_body_pixel_diff(PAGE, &focus_before, &render_state(&state)) > 0);

    let keyboard_before = render_state(&state);
    assert!(apply_clickable_keyboard_activation_for_audit(&mut state));
    assert_eq!(
        "diagnostic_keyboard_navigate",
        state.screen_state.last_action
    );
    assert_eq!("diagnostic_jump_requested", state.screen_state.last_event);
    assert_eq!("jump=syntax-error", state.screen_state.state_label);
    assert!(state.screen_state.diagnostics_list.keyboard_navigated());
    assert!(component_body_pixel_diff(PAGE, &keyboard_before, &render_state(&state)) > 0);

    let scroll_before = render_state(&state);
    assert!(apply_diagnostics_list_scroll_for_audit(
        &mut state,
        target.x + 4,
        target.y + 4
    ));
    assert_eq!("diagnostic_scroll_retained", state.screen_state.last_action);
    assert_eq!(
        "diagnostic_visible_range_kept",
        state.screen_state.last_event
    );
    assert_eq!("scroll=selection-retained", state.screen_state.state_label);
    assert!(state.screen_state.diagnostics_list.scroll_retained());
    assert!(component_body_pixel_diff(PAGE, &scroll_before, &render_state(&state)) > 0);
}

fn expected_states() -> &'static [(&'static str, &'static str, &'static str)] {
    &[
        (
            "diagnostics.group_by",
            "diagnostics.group_by=Source",
            "Source",
        ),
        (
            "diagnostics.sort_by",
            "diagnostics.sort_by=Location",
            "Location",
        ),
        (
            "diagnostics.severity_filter",
            "diagnostics.severity_filter=Error",
            "Error",
        ),
        (
            "diagnostics.wrap_error_navigation",
            "diagnostics.wrap_error_navigation=false",
            "false",
        ),
        (
            "diagnostics.virtualization",
            "diagnostics.virtualization=Windowed",
            "Windowed",
        ),
        (
            "diagnostics.bulk_action",
            "diagnostics.bulk_action=Apply",
            "Apply",
        ),
        (
            "diagnostics.fix_preview",
            "diagnostics.fix_preview=Collapsed",
            "Collapsed",
        ),
    ]
}

fn assert_diagnostics_list_runtime(setting: &str, state: &StorybookWindowState) {
    let diagnostics = &state.screen_state.diagnostics_list;
    let options = diagnostics.option_state();
    match setting {
        "diagnostics.group_by" => {
            assert!(options.group_by_source);
            assert_eq!("diagnostic_group_by_source", diagnostics.callback_action());
        }
        "diagnostics.sort_by" => {
            assert!(options.sort_by_location);
            assert_eq!("diagnostic_sort_by_location", diagnostics.callback_action());
        }
        "diagnostics.severity_filter" => {
            assert!(options.severity_filter_error_only);
            assert!(diagnostics.has_error_filter());
            assert_eq!("diagnostic_filter_error", diagnostics.callback_action());
        }
        "diagnostics.wrap_error_navigation" => {
            assert!(options.wrap_error_navigation_disabled);
            assert_eq!(
                "diagnostic_wrap_navigation_disabled",
                diagnostics.callback_action()
            );
        }
        "diagnostics.virtualization" => {
            assert!(options.virtualization_windowed);
            assert_eq!(
                "diagnostic_virtualization_windowed",
                diagnostics.callback_action()
            );
        }
        "diagnostics.bulk_action" => {
            assert!(options.bulk_action_apply);
            assert!(diagnostics.has_bulk_applied());
            assert_eq!("diagnostic_bulk_apply", diagnostics.callback_action());
        }
        "diagnostics.fix_preview" => {
            assert!(options.fix_preview_collapsed);
            assert!(!diagnostics.has_fix_preview());
            assert_eq!("diagnostic_fix_preview", diagnostics.callback_action());
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

fn click_component(state: &mut StorybookWindowState) {
    let rect = preview_detail::component_action_hit_rect(PAGE);

    assert!(apply_click(state, rect.x + 1, rect.y + 1));
}

fn option_index(setting: &str) -> Result<usize, String> {
    storybook_ui_option_contract::options_for_page(PAGE)
        .iter()
        .position(|option| option.setting == setting)
        .ok_or_else(|| format!("missing diagnostics-list option `{setting}`"))
}

fn render_state(state: &StorybookWindowState) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
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
