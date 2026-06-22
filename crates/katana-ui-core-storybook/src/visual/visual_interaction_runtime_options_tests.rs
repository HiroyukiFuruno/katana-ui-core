use super::visual_interaction_test_support::{
    assert_inspector_option_contract_state, component_body_pixel_diff,
};
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{layout_metrics, render, storybook_ui_option_contract};

const DARK_THEME: &str = "dark";
const CONTEXT_MENU_PAGE: &str = "context-menu";
const STARTUP_STATE_PAGE: &str = "startup-state-panel";
const CODE_DIFF_PAGE: &str = "code-diff";

#[test]
fn context_menu_inspector_options_mutate_anchor_placement_and_size_semantic_state()
-> Result<(), String> {
    for &(setting, expected_state) in context_menu_expected_states() {
        assert_option_state(CONTEXT_MENU_PAGE, setting, expected_state)?;
    }
    Ok(())
}

#[test]
fn startup_state_inspector_options_mutate_error_progress_and_action_semantic_state()
-> Result<(), String> {
    for &(setting, expected_state) in startup_state_expected_states() {
        assert_option_state(STARTUP_STATE_PAGE, setting, expected_state)?;
    }
    Ok(())
}

#[test]
fn code_diff_inspector_options_mutate_mode_layout_and_sync_semantic_state() -> Result<(), String> {
    for &(setting, expected_state) in code_diff_expected_states() {
        assert_option_state(CODE_DIFF_PAGE, setting, expected_state)?;
    }
    Ok(())
}

fn context_menu_expected_states() -> &'static [(&'static str, &'static str)] {
    &[
        ("context_menu.anchor", "context_menu.anchor=Pointer(0,0)"),
        (
            "context_menu.placement_priority",
            "context_menu.placement_priority=AboveEnd>BelowStart",
        ),
        (
            "context_menu.placement_used",
            "context_menu.placement_used=AboveEnd",
        ),
        ("context_menu.min_width", "context_menu.min_width=280"),
        ("context_menu.max_height", "context_menu.max_height=320"),
    ]
}

fn startup_state_expected_states() -> &'static [(&'static str, &'static str)] {
    &[
        ("startup_state.state", "startup_state.state=Error"),
        ("startup_state.progress", "startup_state.progress=64"),
        (
            "startup_state.label",
            "startup_state.label=Loading workspace",
        ),
        ("startup_state.retry", "startup_state.retry=true"),
        ("startup_state.cancel", "startup_state.cancel=true"),
    ]
}

fn code_diff_expected_states() -> &'static [(&'static str, &'static str)] {
    &[
        ("code_diff.mode", "code_diff.mode=Split"),
        ("code_diff.whitespace", "code_diff.whitespace=Visible"),
        ("code_diff.direction", "code_diff.direction=Vertical"),
        ("code_diff.context_lines", "code_diff.context_lines=0"),
        ("code_diff.item_count", "code_diff.item_count=3"),
        ("code_diff.scroll_sync", "code_diff.scroll_sync=false"),
        ("code_diff.language", "code_diff.language=markdown"),
    ]
}

fn assert_option_state(
    page: &'static str,
    setting: &str,
    expected_state: &'static str,
) -> Result<(), String> {
    let mut state = page_state(page);
    let before = render_state(page, &state);
    click_option(&mut state, page, setting)?;
    let after = render_state(page, &state);

    assert_inspector_option_contract_state(&state, page, setting, expected_state)?;
    assert!(component_body_pixel_diff(page, &before, &after) > 0);
    Ok(())
}

fn click_option(
    state: &mut StorybookWindowState,
    page: &'static str,
    setting: &str,
) -> Result<(), String> {
    let index = option_index(page, setting)?;
    let row = layout_metrics::inspector_setting_row_hit_rect(index);

    assert!(apply_click(state, row.x + 1, row.y + 1));
    Ok(())
}

fn option_index(page: &'static str, setting: &str) -> Result<usize, String> {
    storybook_ui_option_contract::options_for_page(page)
        .iter()
        .position(|option| option.setting == setting)
        .ok_or_else(|| format!("missing {page} option `{setting}`"))
}

fn render_state(page: &'static str, state: &StorybookWindowState) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
        DARK_THEME,
        page,
        state.preset_index,
        state.screen_state.clone(),
    )
}

fn page_state(page: &'static str) -> StorybookWindowState {
    StorybookWindowState {
        selected_page: page,
        ..StorybookWindowState::default()
    }
}
