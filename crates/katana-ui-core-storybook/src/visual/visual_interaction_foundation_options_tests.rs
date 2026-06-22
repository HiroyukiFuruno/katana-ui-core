use super::visual_interaction_test_support::{
    assert_inspector_option_state, component_body_pixel_diff,
};
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{layout_metrics, render, storybook_ui_option_contract};

const DARK_THEME: &str = "dark";

#[test]
fn text_inspector_options_mutate_role_script_metrics_and_wrap_semantic_state() -> Result<(), String>
{
    assert_options(
        "text",
        &[
            ("text.role", "heading", "text.role=heading"),
            ("text.content", "empty", "text.content=empty"),
            ("text.script", "jp+emoji", "text.script=jp+emoji"),
            ("text.color", "accent", "text.color=accent"),
            ("text.color_token", "accent", "text.color_token=accent"),
            ("text.line_metrics", "compact", "text.line_metrics=compact"),
            (
                "text.vertical_centered",
                "true",
                "text.vertical_centered=true",
            ),
            ("text.spans", "rich", "text.spans=rich"),
            ("text.wrap", "multi", "text.wrap=multi"),
        ],
    )
}

#[test]
fn progress_bar_inspector_options_mutate_progress_loading_tone_and_size_semantic_state()
-> Result<(), String> {
    assert_options(
        "progress-bar",
        &[
            ("variant", "alternate", "progress_bar.variant=alternate"),
            ("progress.percent", "82", "progress_bar.percent=82"),
            (
                "loading.animation_state",
                "Paused",
                "progress_bar.animation_state=Paused",
            ),
            ("loading.label", "Syncing", "progress_bar.label=Syncing"),
            ("loading.speed_ms", "96", "progress_bar.speed_ms=96"),
            ("loading.dot_count", "5", "progress_bar.dot_count=5"),
            (
                "loading.reduced_motion",
                "true",
                "progress_bar.reduced_motion=true",
            ),
            ("tone", "accent", "progress_bar.tone=accent"),
            ("size", "large", "progress_bar.size=large"),
        ],
    )
}

#[test]
fn loading_indicator_inspector_options_mutate_animation_label_tone_and_size_semantic_state()
-> Result<(), String> {
    assert_options(
        "loading-dots",
        &[
            ("variant", "alternate", "loading_dots.variant=alternate"),
            (
                "loading.animation_state",
                "Paused",
                "loading_dots.animation_state=Paused",
            ),
            (
                "loading.reduced_motion",
                "true",
                "loading_dots.reduced_motion=true",
            ),
            ("loading.label", "Saving", "loading_dots.label=Saving"),
            ("loading.speed_ms", "96", "loading_dots.speed_ms=96"),
            ("loading.dot_count", "5", "loading_dots.dot_count=5"),
            ("tone", "accent", "loading_dots.tone=accent"),
            ("size", "large", "loading_dots.size=large"),
        ],
    )?;
    assert_options(
        "spinner",
        &[
            ("variant", "alternate", "spinner.variant=alternate"),
            (
                "loading.animation_state",
                "Paused",
                "spinner.animation_state=Paused",
            ),
            (
                "loading.reduced_motion",
                "true",
                "spinner.reduced_motion=true",
            ),
            ("loading.label", "Saving", "spinner.label=Saving"),
            ("loading.speed_ms", "96", "spinner.speed_ms=96"),
            ("loading.dot_count", "5", "spinner.dot_count=5"),
            ("tone", "accent", "spinner.tone=accent"),
            ("size", "large", "spinner.size=large"),
        ],
    )
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
        assert!(
            component_body_pixel_diff(page, &before, &after) > 0,
            "{page} option `{setting}` must repaint the live component"
        );
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
