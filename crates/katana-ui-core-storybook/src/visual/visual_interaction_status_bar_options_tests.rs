use super::visual_interaction_test_support::{
    assert_inspector_option_contract_state, component_body_pixel_diff,
};
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{layout_metrics, render, storybook_ui_option_contract};

const DARK_THEME: &str = "dark";
const PAGE: &str = "status-bar";

#[test]
fn status_bar_inspector_options_mutate_segment_and_message_semantic_state() -> Result<(), String> {
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
        ("status_bar.mode", "status_bar.mode=MultiSegment"),
        ("status_bar.segments", "status_bar.segments=4"),
        ("status_bar.density", "status_bar.density=Compact"),
        (
            "status_bar.progress_popover",
            "status_bar.progress_popover=true",
        ),
        ("status_bar.message", "status_bar.message=Ready"),
        ("status_bar.severity", "status_bar.severity=Warning"),
        ("status_bar.dismiss", "status_bar.dismiss=available"),
        ("status_bar.segment_a11y", "status_bar.segment_a11y=custom"),
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
        .ok_or_else(|| format!("missing status-bar option `{setting}`"))
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
