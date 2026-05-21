use super::layout_metrics::LayoutRect;
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{Canvas, StorybookVisual, preview_detail, render};

const DARK_THEME: &str = "dark";
const DEFAULT_PRESET: usize = 0;
const COMPONENT_BODY_DIFF_THRESHOLD: usize = 80;

pub(super) fn pixel_at(canvas: &Canvas, x: usize, y: usize) -> Option<u32> {
    canvas.pixels().get(y * canvas.width() + x).copied()
}

pub(super) fn rect_pixel_diff(rect: LayoutRect, before: &Canvas, after: &Canvas) -> usize {
    let mut diff = 0;
    for current_y in rect.y..rect.bottom() {
        for current_x in rect.x..rect.right() {
            let index = current_y * before.width() + current_x;
            if before.pixels()[index] != after.pixels()[index] {
                diff += 1;
            }
        }
    }
    diff
}

pub(super) fn rect_non_background_pixels(
    rect: LayoutRect,
    canvas: &Canvas,
    background: u32,
) -> usize {
    let mut count = 0;
    for current_y in rect.y..rect.bottom() {
        for current_x in rect.x..rect.right() {
            let index = current_y * canvas.width() + current_x;
            if canvas.pixels()[index] != background {
                count += 1;
            }
        }
    }
    count
}

pub(super) fn component_body_pixel_diff(page: &str, before: &Canvas, after: &Canvas) -> usize {
    rect_pixel_diff(
        preview_detail::component_action_hit_rect(page),
        before,
        after,
    )
}

pub(super) fn assert_clicked_page_changes_body(page: &str) {
    let before = StorybookVisual.render_preset(DARK_THEME, page, DEFAULT_PRESET, 0);
    let after = StorybookVisual.render_clicked_preset_with_scrollbar(
        DARK_THEME,
        page,
        DEFAULT_PRESET,
        0,
        true,
    );

    assert!(component_body_pixel_diff(page, &before, &after) > COMPONENT_BODY_DIFF_THRESHOLD);
}

pub(super) fn assert_settings_page_changes_body(page: &'static str) {
    let mut state = StorybookWindowState {
        selected_page: page,
        ..StorybookWindowState::default()
    };
    let before = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state,
    );
    let setting = super::layout_metrics::button_setting_hit_rect();

    assert!(apply_click(&mut state, setting.x + 1, setting.y + 1));
    let after = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state,
    );

    assert!(component_body_pixel_diff(page, &before, &after) > COMPONENT_BODY_DIFF_THRESHOLD);
}
