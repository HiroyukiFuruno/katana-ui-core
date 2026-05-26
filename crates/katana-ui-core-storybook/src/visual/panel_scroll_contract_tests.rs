use super::{Canvas, layout_metrics, preview, render};

const DARK_THEME: &str = "dark";
const BUTTON_PAGE: &str = "button";
const DEFAULT_PRESET: usize = 0;
const PANEL_DIFF_THRESHOLD: usize = 80;
const SUMMARY_SETTING_INDEX: usize = 2;

#[test]
fn summary_tooltip_renders_above_tabs_and_inspector_scrollbars() {
    let screen_state = super::screen_state::StorybookScreenState {
        last_setting: "layout",
        last_setting_value: "basic-with-a-very-long-value",
        hovered_summary_index: Some(SUMMARY_SETTING_INDEX),
        ..Default::default()
    };
    let before = render::render_storybook_canvas_with_screen_state(
        DARK_THEME,
        BUTTON_PAGE,
        DEFAULT_PRESET,
        Default::default(),
    );
    let after = render::render_storybook_canvas_with_screen_state(
        DARK_THEME,
        BUTTON_PAGE,
        DEFAULT_PRESET,
        screen_state,
    );
    let rect = preview::summary_control_rect_for_test(SUMMARY_SETTING_INDEX);
    let tab = layout_metrics::preset_tab_rect(SUMMARY_SETTING_INDEX);
    let overlap = layout_metrics::LayoutRect::new(rect.x, tab.y, rect.width, tab.height);

    assert!(
        region_pixel_diff(
            &before,
            &after,
            overlap.x,
            overlap.y,
            overlap.width,
            overlap.height
        ) > PANEL_DIFF_THRESHOLD
    );
}

fn region_pixel_diff(
    before: &Canvas,
    after: &Canvas,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> usize {
    let mut diff = 0;
    for current_y in y..y + height {
        for current_x in x..x + width {
            let index = current_y * before.width() + current_x;
            if before.pixels()[index] != after.pixels()[index] {
                diff += 1;
            }
        }
    }
    diff
}
