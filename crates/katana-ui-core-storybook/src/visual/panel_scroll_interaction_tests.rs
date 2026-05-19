use super::{
    Canvas, StorybookVisual, layout_metrics, palette, panel_scroll_state, panel_scrollbars, render,
};
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const BUTTON_PAGE: &str = "button";
const DEFAULT_PRESET: usize = 0;
const PANEL_DIFF_THRESHOLD: usize = 80;

#[test]
fn storybook_draws_independent_panel_scrollbars() {
    let canvas = StorybookVisual.render_preset(DARK_THEME, BUTTON_PAGE, DEFAULT_PRESET, 0);
    let accent = palette::VisualPalette::from_theme(&ThemeSnapshot::dark()).accent;

    assert_eq!(
        Some(accent),
        pixel_at_rect(
            &canvas,
            panel_scrollbars::thumb_rect_for(
                panel_scroll_state::PanelScrollRegion::Navigation,
                Default::default()
            )
        )
    );
    assert_eq!(
        Some(accent),
        pixel_at_rect(
            &canvas,
            panel_scrollbars::thumb_rect_for(
                panel_scroll_state::PanelScrollRegion::Preview,
                Default::default()
            )
        )
    );
    assert_eq!(
        Some(accent),
        pixel_at_rect(
            &canvas,
            panel_scrollbars::thumb_rect_for(
                panel_scroll_state::PanelScrollRegion::Inspector,
                Default::default()
            )
        )
    );
}

#[test]
fn panel_scrollbar_thumbs_move_only_for_scrolled_panel() {
    let accent = palette::VisualPalette::from_theme(&ThemeSnapshot::dark()).accent;
    let mut offsets = panel_scroll_state::PanelScrollOffsets::default();

    assert!(offsets.scroll_delta(panel_scroll_state::PanelScrollRegion::Navigation, -1.0));
    let canvas = render_with_offsets(offsets);
    let nav_thumb = panel_scrollbars::thumb_rect_for(
        panel_scroll_state::PanelScrollRegion::Navigation,
        offsets,
    );

    assert_eq!(Some(accent), pixel_at_rect(&canvas, nav_thumb));
    assert_eq!(
        Some(accent),
        pixel_at_rect(
            &canvas,
            panel_scrollbars::thumb_rect_for(
                panel_scroll_state::PanelScrollRegion::Preview,
                offsets
            )
        )
    );
    assert_eq!(
        Some(accent),
        pixel_at_rect(
            &canvas,
            panel_scrollbars::thumb_rect_for(
                panel_scroll_state::PanelScrollRegion::Inspector,
                offsets
            )
        )
    );
}

#[test]
fn panel_scroll_offsets_move_only_target_panel_content() {
    let baseline = render_with_offsets(Default::default());
    let mut preview_offsets = panel_scroll_state::PanelScrollOffsets::default();
    assert!(preview_offsets.scroll_delta(panel_scroll_state::PanelScrollRegion::Preview, -1.0));
    let preview_scrolled = render_with_offsets(preview_offsets);
    let mut inspector_offsets = panel_scroll_state::PanelScrollOffsets::default();
    assert!(inspector_offsets.scroll_delta(panel_scroll_state::PanelScrollRegion::Inspector, -1.0));
    let inspector_scrolled = render_with_offsets(inspector_offsets);

    let preview_diff = preview_panel_pixel_diff(&baseline, &preview_scrolled);
    let inspector_diff_after_preview_scroll =
        inspector_panel_pixel_diff(&baseline, &preview_scrolled);
    assert!(preview_diff > PANEL_DIFF_THRESHOLD);
    assert!(preview_diff > inspector_diff_after_preview_scroll * 4);
    assert!(inspector_panel_pixel_diff(&baseline, &inspector_scrolled) > PANEL_DIFF_THRESHOLD);
}

fn render_with_offsets(offsets: panel_scroll_state::PanelScrollOffsets) -> Canvas {
    render::render_storybook_canvas_with_options(render::StorybookRenderOptions {
        theme_id: DARK_THEME,
        selected_page: BUTTON_PAGE,
        preset_index: DEFAULT_PRESET,
        scroll_y: 0,
        scrollbar_visible: true,
        panel_scroll: offsets,
        tree_expansion: Default::default(),
        screen_state: Default::default(),
    })
}

fn preview_panel_pixel_diff(before: &Canvas, after: &Canvas) -> usize {
    region_pixel_diff(
        before,
        after,
        layout_metrics::PREVIEW_X,
        layout_metrics::PRESET_ACTIVE_Y,
        panel_scrollbars::PREVIEW_SCROLL_X - layout_metrics::PREVIEW_X,
        render::HEIGHT - layout_metrics::PRESET_ACTIVE_Y,
    )
}

fn inspector_panel_pixel_diff(before: &Canvas, after: &Canvas) -> usize {
    region_pixel_diff(
        before,
        after,
        layout_metrics::INSPECTOR_X,
        layout_metrics::INSPECTOR_Y,
        layout_metrics::INSPECTOR_WIDTH,
        layout_metrics::INSPECTOR_HEIGHT,
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
