use super::visual_interaction_test_support::{
    assert_inspector_option_state, component_body_pixel_diff,
};
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{layout_metrics, render, storybook_ui_option_contract};

const DARK_THEME: &str = "dark";
const PAGE: &str = "split-pane";

#[test]
fn split_pane_inspector_options_mutate_axis_ratio_bounds_and_resize_semantic_state()
-> Result<(), String> {
    for &(setting, expected_value, expected_state) in expected_states() {
        let mut state = page_state();
        let before = render_state(&state);
        click_option(&mut state, setting)?;
        let after = render_state(&state);

        assert_inspector_option_state(&state, PAGE, setting, expected_value, expected_state);
        assert!(
            component_body_pixel_diff(PAGE, &before, &after) > 0,
            "split-pane option `{setting}` must repaint the live component"
        );
    }
    Ok(())
}

fn expected_states() -> &'static [(&'static str, &'static str, &'static str)] {
    &[
        ("axis", "Vertical", "split_pane.axis=Vertical"),
        ("gap", "12", "split_pane.gap=12"),
        ("alignment", "Center", "split_pane.alignment=Center"),
        ("overflow", "Scroll", "split_pane.overflow=Scroll"),
        (
            "split_pane.ratio_percent",
            "64",
            "split_pane.ratio_percent=64",
        ),
        ("split_pane.min_percent", "24", "split_pane.min_percent=24"),
        ("split_pane.max_percent", "76", "split_pane.max_percent=76"),
        (
            "split_pane.reset_percent",
            "55",
            "split_pane.reset_percent=55",
        ),
        (
            "split_pane.handle_width_px",
            "10",
            "split_pane.handle_width_px=10",
        ),
        (
            "split_pane.resize_mode",
            "KeyboardOnly",
            "split_pane.resize_mode=KeyboardOnly",
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
        .ok_or_else(|| format!("missing split-pane option `{setting}`"))
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
