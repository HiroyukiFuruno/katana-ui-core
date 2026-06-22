use super::visual_interaction_test_support::{
    assert_inspector_option_contract_state, component_body_pixel_diff,
};
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{layout_metrics, render, storybook_ui_option_contract};

const DARK_THEME: &str = "dark";
const PAGE: &str = "search-control-strip";

#[test]
fn search_control_inspector_options_mutate_match_replace_and_active_result_semantic_state()
-> Result<(), String> {
    for &(setting, expected_state) in expected_states() {
        let mut state = page_state();
        let before = render_state(&state);
        click_option(&mut state, setting)?;
        let after = render_state(&state);

        assert_inspector_option_contract_state(&state, PAGE, setting, expected_state)?;
        assert!(component_body_pixel_diff(PAGE, &before, &after) > 0);
    }
    Ok(())
}

fn expected_states() -> &'static [(&'static str, &'static str)] {
    &[
        ("search_control.query", "search_control.query=heading"),
        (
            "search_control.match_case",
            "search_control.match_case=true",
        ),
        (
            "search_control.whole_word",
            "search_control.whole_word=true",
        ),
        ("search_control.use_regex", "search_control.regex=true"),
        (
            "search_control.replace_mode",
            "search_control.replace=disabled",
        ),
        (
            "search_control.result_count",
            "search_control.result_count=0",
        ),
        (
            "search_control.active_index",
            "search_control.active_index=none",
        ),
    ]
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
        .ok_or_else(|| format!("missing search-control option `{setting}`"))
}

fn render_state(state: &StorybookWindowState) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
        DARK_THEME,
        PAGE,
        0,
        state.screen_state.clone(),
    )
}

fn page_state() -> StorybookWindowState {
    StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    }
}
