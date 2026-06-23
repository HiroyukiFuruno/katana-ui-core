use super::visual_interaction_test_support::{
    assert_inspector_option_state, component_body_pixel_diff,
};
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{layout_metrics, render, storybook_ui_option_contract};

const DARK_THEME: &str = "dark";
const PAGE: &str = "skeleton";

#[test]
fn skeleton_inspector_options_mutate_shape_motion_size_and_a11y_semantic_state()
-> Result<(), String> {
    for &(setting, expected_value, expected_state) in expected_states() {
        let mut state = page_state();
        let before = render_state(&state);
        click_option(&mut state, setting)?;
        let after = render_state(&state);

        assert_inspector_option_state(&state, PAGE, setting, expected_value, expected_state);
        assert!(
            component_body_pixel_diff(PAGE, &before, &after) > 0,
            "skeleton option `{setting}` must repaint the live component"
        );
    }
    Ok(())
}

fn expected_states() -> &'static [(&'static str, &'static str, &'static str)] {
    &[
        ("skeleton.shape", "Line", "skeleton.shape=Line"),
        ("skeleton.text_lines", "2", "skeleton.text_lines=2"),
        (
            "skeleton.last_line_ratio",
            "0.62",
            "skeleton.last_line_ratio=0.62",
        ),
        (
            "skeleton.line_thickness",
            "12",
            "skeleton.line_thickness=12",
        ),
        ("size", "Fill", "skeleton.size=Fill"),
        ("skeleton.animation", "Wave", "skeleton.animation=Wave"),
        ("tone", "Accent", "skeleton.tone=Accent"),
        ("skeleton.radius_px", "14", "skeleton.radius_px=14"),
        (
            "skeleton.reduced_motion",
            "true",
            "skeleton.reduced_motion=true",
        ),
        (
            "a11y.label",
            "Loading profile",
            "skeleton.a11y.label=Loading profile",
        ),
        (
            "skeleton.aspect_ratio",
            "16:9",
            "skeleton.aspect_ratio=16:9",
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
        .ok_or_else(|| format!("missing skeleton option `{setting}`"))
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
