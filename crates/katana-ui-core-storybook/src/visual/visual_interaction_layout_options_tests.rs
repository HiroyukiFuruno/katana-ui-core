use super::visual_interaction_test_support::{
    assert_inspector_option_state, assert_inspector_option_state_with_event,
    component_body_pixel_diff,
};
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{layout_metrics, render, storybook_ui_option_contract};

const DARK_THEME: &str = "dark";

#[test]
fn layout_inspector_options_mutate_axis_gap_alignment_and_overflow_semantic_state()
-> Result<(), String> {
    for &(page, prefix) in pages() {
        assert_layout_options(page, prefix)?;
    }
    Ok(())
}

fn pages() -> &'static [(&'static str, &'static str)] {
    &[
        ("row", "row"),
        ("column", "column"),
        ("stack", "stack"),
        ("grid", "grid"),
        ("scroll-area", "scroll_area"),
        ("align-center", "align_center"),
    ]
}

fn assert_layout_options(page: &'static str, prefix: &'static str) -> Result<(), String> {
    for &(setting, expected_value, suffix) in expected_states() {
        let mut state = page_state(page);
        let before = render_state(&state, page);
        click_option(&mut state, page, setting)?;
        let after = render_state(&state, page);

        assert_layout_option_state(
            &state,
            page,
            setting,
            expected_value,
            state_label(prefix, suffix),
        );
        assert!(
            component_body_pixel_diff(page, &before, &after) > 0,
            "{page} option `{setting}` must repaint the live component"
        );
    }
    Ok(())
}

fn assert_layout_option_state(
    state: &StorybookWindowState,
    page: &str,
    setting: &str,
    expected_value: &str,
    expected_state: &str,
) {
    if matches!(page, "row" | "column" | "stack" | "grid" | "align-center") {
        assert_inspector_option_state_with_event(
            state,
            setting,
            expected_value,
            expected_state,
            "layout_option_changed",
            "layout_option_changed",
        );
        return;
    }
    assert_inspector_option_state(state, page, setting, expected_value, expected_state);
}

fn expected_states() -> &'static [(&'static str, &'static str, &'static str)] {
    &[
        ("axis", "y", "axis=y"),
        ("gap", "large", "gap=large"),
        ("alignment", "center", "alignment=center"),
        ("overflow", "scroll", "overflow=scroll"),
    ]
}

fn state_label(prefix: &str, suffix: &str) -> &'static str {
    match (prefix, suffix) {
        ("row", "axis=y") => "row.axis=y",
        ("row", "gap=large") => "row.gap=large",
        ("row", "alignment=center") => "row.alignment=center",
        ("row", "overflow=scroll") => "row.overflow=scroll",
        ("column", "axis=y") => "column.axis=y",
        ("column", "gap=large") => "column.gap=large",
        ("column", "alignment=center") => "column.alignment=center",
        ("column", "overflow=scroll") => "column.overflow=scroll",
        ("stack", "axis=y") => "stack.axis=y",
        ("stack", "gap=large") => "stack.gap=large",
        ("stack", "alignment=center") => "stack.alignment=center",
        ("stack", "overflow=scroll") => "stack.overflow=scroll",
        ("grid", "axis=y") => "grid.axis=y",
        ("grid", "gap=large") => "grid.gap=large",
        ("grid", "alignment=center") => "grid.alignment=center",
        ("grid", "overflow=scroll") => "grid.overflow=scroll",
        ("scroll_area", "axis=y") => "scroll_area.axis=y",
        ("scroll_area", "gap=large") => "scroll_area.gap=large",
        ("scroll_area", "alignment=center") => "scroll_area.alignment=center",
        ("scroll_area", "overflow=scroll") => "scroll_area.overflow=scroll",
        ("align_center", "axis=y") => "align_center.axis=y",
        ("align_center", "gap=large") => "align_center.gap=large",
        ("align_center", "alignment=center") => "align_center.alignment=center",
        ("align_center", "overflow=scroll") => "align_center.overflow=scroll",
        _ => "",
    }
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
