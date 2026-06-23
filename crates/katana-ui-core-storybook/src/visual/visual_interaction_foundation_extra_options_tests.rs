use super::visual_interaction_test_support::{
    assert_inspector_option_state, component_body_pixel_diff,
};
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{layout_metrics, render, storybook_ui_option_contract};

const DARK_THEME: &str = "dark";

#[test]
fn foundation_extra_inspector_options_mutate_theme_key_cap_and_motion_semantic_state()
-> Result<(), String> {
    assert_options("theme-tokens", theme_states())?;
    assert_options("key-cap", key_cap_states())?;
    assert_options("motion", motion_states())
}

fn theme_states() -> &'static [(&'static str, &'static str, &'static str)] {
    &[
        ("theme.id", "light", "theme.id=light"),
        ("color.background", "light", "theme.color.background=light"),
        ("color.surface", "contrast", "theme.color.surface=contrast"),
        ("color.accent", "green", "theme.color.accent=green"),
    ]
}

fn key_cap_states() -> &'static [(&'static str, &'static str, &'static str)] {
    &[
        ("content.value", "custom", "key_cap.content.value=custom"),
        ("visual.role", "icon", "key_cap.visual.role=icon"),
        ("a11y.label", "changed", "key_cap.a11y.label=changed"),
        ("theme.color", "accent", "key_cap.theme.color=accent"),
    ]
}

fn motion_states() -> &'static [(&'static str, &'static str, &'static str)] {
    &[
        ("motion.primitive", "Shimmer", "motion.primitive=Shimmer"),
        ("motion.duration", "Fast", "motion.duration=Fast"),
        ("motion.distance", "Compact", "motion.distance=Compact"),
        (
            "motion.reduced_policy",
            "ForceReduced",
            "motion.reduced_policy=ForceReduced",
        ),
    ]
}

fn assert_options(
    page: &'static str,
    expected_states: &'static [(&'static str, &'static str, &'static str)],
) -> Result<(), String> {
    for &(setting, expected_value, expected_state) in expected_states {
        let mut state = page_state(page);
        let before = render_state(&state, page);
        click_option(&mut state, page, setting)?;
        let after = render_state(&state, page);

        assert_inspector_option_state(&state, page, setting, expected_value, expected_state);
        assert!(component_body_pixel_diff(page, &before, &after) > 0);
    }
    Ok(())
}

fn click_option(state: &mut StorybookWindowState, page: &str, setting: &str) -> Result<(), String> {
    let index = option_index(page, setting)?;
    let row = layout_metrics::inspector_setting_row_hit_rect(index);

    assert!(apply_click(state, row.x + 1, row.y + 1));
    Ok(())
}

fn option_index(page: &str, setting: &str) -> Result<usize, String> {
    storybook_ui_option_contract::options_for_page(page)
        .iter()
        .position(|option| option.setting == setting)
        .ok_or_else(|| format!("missing {page} option `{setting}`"))
}

fn render_state(state: &StorybookWindowState, page: &str) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        page,
        state.preset_index,
        state.screen_state.clone(),
    )
}

fn page_state(page: &'static str) -> StorybookWindowState {
    StorybookWindowState {
        theme_id: DARK_THEME,
        selected_page: page,
        ..StorybookWindowState::default()
    }
}
