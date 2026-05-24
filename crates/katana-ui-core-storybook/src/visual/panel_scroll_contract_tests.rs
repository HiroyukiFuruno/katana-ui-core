use super::{
    Canvas, layout_metrics, palette, panel_layout, panel_scroll_state, panel_scrollbars, preview,
    render,
};
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const BUTTON_PAGE: &str = "button";
const PANEL_PAGE: &str = "panel";
const DEFAULT_PRESET: usize = 0;
const PANEL_DIFF_THRESHOLD: usize = 80;
const SUMMARY_SETTING_INDEX: usize = 2;

#[test]
fn panel_story_hides_outer_preview_scrollbars_without_preview_overflow() {
    let canvas = render_panel_with_offsets(Default::default());
    let accent = palette::VisualPalette::from_theme(&ThemeSnapshot::dark()).accent;

    assert_eq!(
        Some(accent),
        pixel_at_rect(
            &canvas,
            panel_scrollbars::thumb_rect_for(
                panel_scroll_state::PanelScrollRegion::Navigation,
                Default::default(),
            ),
        )
    );
    assert_ne!(
        Some(accent),
        pixel_at_rect(
            &canvas,
            panel_scrollbars::thumb_rect_for(
                panel_scroll_state::PanelScrollRegion::Preview,
                Default::default(),
            ),
        )
    );
    assert_ne!(
        Some(accent),
        pixel_at_rect(
            &canvas,
            panel_scrollbars::horizontal_thumb_rect_for(
                panel_scroll_state::PanelScrollRegion::Preview,
                Default::default(),
            ),
        )
    );
}

#[test]
fn preview_horizontal_scroll_offset_is_ignored_without_preview_overflow() {
    let baseline = render_panel_with_offsets(Default::default());
    let mut offsets = panel_scroll_state::PanelScrollOffsets::default();
    offsets.set_drag_offset_x(
        panel_scroll_state::PanelScrollRegion::Preview,
        panel_scroll_state::max_scroll_x_for(
            panel_scroll_state::PanelScrollRegion::Preview,
            "button",
            Default::default(),
        ),
    );
    let scrolled = render_panel_with_offsets(offsets);

    assert_eq!(
        0,
        region_pixel_diff(
            &baseline,
            &scrolled,
            0,
            layout_metrics::PRESET_ACTIVE_Y,
            layout_metrics::PREVIEW_X,
            render::HEIGHT - layout_metrics::PRESET_ACTIVE_Y,
        )
    );
    assert_eq!(0, preview_panel_pixel_diff(&baseline, &scrolled));
}

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

fn render_panel_with_offsets(offsets: panel_scroll_state::PanelScrollOffsets) -> Canvas {
    render::render_storybook_canvas_with_options(render::StorybookRenderOptions {
        theme_id: DARK_THEME,
        selected_page: PANEL_PAGE,
        preset_index: DEFAULT_PRESET,
        scroll_y: 0,
        scrollbar_visible: true,
        panel_scroll: offsets,
        tree_expansion: Default::default(),
        show_navigation_lines: true,
        show_navigation_text_connectors: false,
        screen_state: Default::default(),
    })
}

fn preview_panel_pixel_diff(before: &Canvas, after: &Canvas) -> usize {
    let frame = panel_layout::region_frame(panel_scroll_state::PanelScrollRegion::Preview);
    region_pixel_diff(
        before,
        after,
        frame.x,
        layout_metrics::PRESET_ACTIVE_Y,
        frame.width,
        render::HEIGHT - layout_metrics::PRESET_ACTIVE_Y,
    )
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

fn pixel_at(canvas: &Canvas, x: usize, y: usize) -> Option<u32> {
    canvas.pixels().get(y * canvas.width() + x).copied()
}

fn pixel_at_rect(canvas: &Canvas, rect: layout_metrics::LayoutRect) -> Option<u32> {
    pixel_at(canvas, rect.x + rect.width / 2, rect.y + rect.height / 2)
}
