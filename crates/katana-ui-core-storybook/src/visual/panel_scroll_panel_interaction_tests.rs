use super::{
    Canvas, layout_metrics, palette, panel_scroll_state, panel_scrollbars, preview_detail, render,
};
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const PANEL_PAGE: &str = "panel";
const DEFAULT_PRESET: usize = 0;

#[test]
fn storybook_outer_preview_scrollbars_stay_hidden_for_panel_page() {
    let accent = palette::VisualPalette::from_theme(&ThemeSnapshot::dark()).accent;
    let mut offsets = panel_scroll_state::PanelScrollOffsets::default();

    assert!(offsets.scroll_delta(panel_scroll_state::PanelScrollRegion::Navigation, -1.0));
    let canvas = render_panel_with_offsets(offsets);
    let nav_thumb = panel_scrollbars::thumb_rect_for(
        panel_scroll_state::PanelScrollRegion::Navigation,
        offsets,
    );
    let preview_thumb = panel_scrollbars::thumb_rect_for_state(
        panel_scroll_state::PanelScrollRegion::Preview,
        offsets,
        PANEL_PAGE,
        Default::default(),
    );
    let inspector_thumb =
        panel_scrollbars::thumb_rect_for(panel_scroll_state::PanelScrollRegion::Inspector, offsets);

    assert_eq!(Some(accent), pixel_at_rect(&canvas, nav_thumb));
    assert_ne!(Some(accent), pixel_at_rect(&canvas, preview_thumb));
    assert_eq!(Some(accent), pixel_at_rect(&canvas, inspector_thumb));
}

#[test]
fn panel_preview_outer_scroll_offsets_do_not_move_panel_foundation() {
    let baseline = render_panel_with_offsets(Default::default());
    let mut preview_offsets = panel_scroll_state::PanelScrollOffsets::default();
    assert!(!preview_offsets.scroll_delta_with_max(
        panel_scroll_state::PanelScrollRegion::Preview,
        panel_scroll_state::PanelScrollOverflowModel::max_scroll_y_for(
            panel_scroll_state::PanelScrollRegion::Preview,
            PANEL_PAGE,
            Default::default(),
        ),
        -1.0,
    ));
    let preview_scrolled = render_panel_with_offsets(preview_offsets);
    let mut inspector_offsets = panel_scroll_state::PanelScrollOffsets::default();
    assert!(inspector_offsets.scroll_delta(panel_scroll_state::PanelScrollRegion::Inspector, -1.0));
    let inspector_scrolled = render_panel_with_offsets(inspector_offsets);

    assert_eq!(
        0,
        preview_component_pixel_diff(&baseline, &preview_scrolled)
    );
    assert!(inspector_panel_pixel_diff(&baseline, &inspector_scrolled) > 80);
}

#[test]
fn storybook_hides_panel_preview_horizontal_scrollbar_on_panel_page() {
    let accent = palette::VisualPalette::from_theme(&ThemeSnapshot::dark()).accent;
    let mut offsets = panel_scroll_state::PanelScrollOffsets::default();
    assert!(!offsets.scroll_delta_x_with_max(
        panel_scroll_state::PanelScrollRegion::Preview,
        panel_scroll_state::PanelScrollOverflowModel::max_scroll_x_for(
            panel_scroll_state::PanelScrollRegion::Preview,
            PANEL_PAGE,
            Default::default(),
        ),
        -1.0,
    ));
    let canvas = render_panel_with_offsets(offsets);
    let thumb = panel_scrollbars::horizontal_thumb_rect_for_state(
        panel_scroll_state::PanelScrollRegion::Preview,
        offsets,
        PANEL_PAGE,
        Default::default(),
    );

    assert_ne!(Some(accent), pixel_at_rect(&canvas, thumb));
}

fn render_panel_with_offsets(offsets: panel_scroll_state::PanelScrollOffsets) -> Canvas {
    render::render_storybook_canvas_with_options(render::StorybookRenderOptions {
        theme_id: DARK_THEME,
        selected_page: PANEL_PAGE,
        preset_index: DEFAULT_PRESET,
        preset_tab_scroll_x: 0,
        scroll_y: 0,
        scrollbar_visible: true,
        panel_scroll: offsets,
        tree_expansion: Default::default(),
        show_navigation_lines: true,
        show_navigation_text_connectors: false,
        screen_state: Default::default(),
    })
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

fn preview_component_pixel_diff(before: &Canvas, after: &Canvas) -> usize {
    let rect = preview_detail::component_action_hit_rect(PANEL_PAGE);
    region_pixel_diff(before, after, rect.x, rect.y, rect.width, rect.height)
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
