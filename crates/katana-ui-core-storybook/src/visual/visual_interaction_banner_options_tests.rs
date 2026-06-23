use super::visual_interaction_test_support::{
    assert_inspector_option_state, component_body_pixel_diff,
};
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{layout_metrics, render, storybook_ui_option_contract};

const DARK_THEME: &str = "dark";
const PAGE: &str = "banner";

#[test]
fn banner_inspector_options_mutate_feedback_details_icon_and_placement_semantic_state()
-> Result<(), String> {
    for &(setting, expected_value, expected_state) in expected_states() {
        let mut state = page_state();
        let before = render_state(&state);
        click_option(&mut state, setting)?;
        let after = render_state(&state);

        assert_inspector_option_state(&state, PAGE, setting, expected_value, expected_state);
        assert!(
            component_body_pixel_diff(PAGE, &before, &after) > 0,
            "banner option `{setting}` must repaint the live component"
        );
    }
    Ok(())
}

fn expected_states() -> &'static [(&'static str, &'static str, &'static str)] {
    &[
        ("severity", "warning", "banner.severity=warning"),
        ("density", "compact", "banner.density=compact"),
        ("action", "visible", "banner.action=visible"),
        ("dismiss", "true", "banner.dismiss=true"),
        ("banner.details", "expanded", "banner.details=expanded"),
        ("banner.title", "visible", "banner.title=visible"),
        (
            "banner.leading_icon",
            "custom",
            "banner.leading_icon=custom",
        ),
        ("banner.placement", "sticky", "banner.placement=sticky"),
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
        .ok_or_else(|| format!("missing banner option `{setting}`"))
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
