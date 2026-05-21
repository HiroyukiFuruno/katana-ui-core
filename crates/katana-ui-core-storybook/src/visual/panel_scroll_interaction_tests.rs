use super::{
    Canvas, StorybookVisual, layout_metrics, palette, panel_scroll_state, panel_scrollbars, render,
    scrollbar,
};
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const BUTTON_PAGE: &str = "button";
const PANEL_PAGE: &str = "panel";
const DEFAULT_PRESET: usize = 0;
const PANEL_DIFF_THRESHOLD: usize = 80;
const MARKER_COLORS: &[u32] = &[0xd7ba7d, 0xf44747, 0x9cdcfe, 0x6a9955, 0xc586c0];

#[test]
fn storybook_hides_preview_scrollbars_when_selected_component_does_not_overflow() {
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
                panel_scroll_state::PanelScrollRegion::Inspector,
                Default::default()
            )
        )
    );
    assert_ne!(
        Some(accent),
        pixel_at_rect(
            &canvas,
            panel_scrollbars::thumb_rect_for(
                panel_scroll_state::PanelScrollRegion::Preview,
                Default::default()
            )
        )
    );
}

#[test]
fn hidden_preview_scroll_offsets_do_not_shift_button_page_rendering() {
    let baseline = render_with_offsets(Default::default());
    let shifted = render_with_offsets(panel_scroll_state::PanelScrollOffsets {
        preview_x: layout_metrics::SCROLL_STEP * 3,
        preview_y: layout_metrics::SCROLL_STEP * 2,
        ..Default::default()
    });

    assert_eq!(0, preview_panel_pixel_diff(&baseline, &shifted));
}

#[test]
fn storybook_outer_scrollbars_are_shown_only_for_overflowing_regions() {
    let accent = palette::VisualPalette::from_theme(&ThemeSnapshot::dark()).accent;
    let mut offsets = panel_scroll_state::PanelScrollOffsets::default();

    assert!(offsets.scroll_delta(panel_scroll_state::PanelScrollRegion::Navigation, -1.0));
    let canvas = render_panel_with_offsets(offsets);
    let nav_thumb = panel_scrollbars::thumb_rect_for(
        panel_scroll_state::PanelScrollRegion::Navigation,
        offsets,
    );

    assert_eq!(Some(accent), pixel_at_rect(&canvas, nav_thumb));
    assert_ne!(
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
fn hidden_preview_scroll_offsets_do_not_move_panel_foundation_preview() {
    let baseline = render_panel_with_offsets(Default::default());
    let mut preview_offsets = panel_scroll_state::PanelScrollOffsets::default();
    assert!(preview_offsets.scroll_delta(panel_scroll_state::PanelScrollRegion::Preview, -1.0));
    let preview_scrolled = render_panel_with_offsets(preview_offsets);
    let mut inspector_offsets = panel_scroll_state::PanelScrollOffsets::default();
    assert!(inspector_offsets.scroll_delta(panel_scroll_state::PanelScrollRegion::Inspector, -1.0));
    let inspector_scrolled = render_panel_with_offsets(inspector_offsets);

    let preview_diff = preview_panel_pixel_diff(&baseline, &preview_scrolled);
    assert_eq!(0, preview_diff);
    assert!(inspector_panel_pixel_diff(&baseline, &inspector_scrolled) > PANEL_DIFF_THRESHOLD);
}

#[test]
fn root_panel_scroll_reaches_bottom_and_root_thumb_reaches_track_end() {
    let mut offsets = panel_scroll_state::PanelScrollOffsets::default();

    for _ in 0..100 {
        offsets.scroll_delta(panel_scroll_state::PanelScrollRegion::Root, -1.0);
    }
    let track = scrollbar::track_rect();
    let thumb = scrollbar::thumb_rect(offsets.root_y);

    assert_eq!(layout_metrics::MAX_SCROLL_Y, offsets.root_y);
    assert_eq!(track.bottom(), thumb.bottom());
}

#[test]
fn storybook_hides_preview_horizontal_scrollbar_without_preview_overflow() {
    let accent = palette::VisualPalette::from_theme(&ThemeSnapshot::dark()).accent;
    let mut offsets = panel_scroll_state::PanelScrollOffsets::default();
    offsets.scroll_delta_x(panel_scroll_state::PanelScrollRegion::Preview, -1.0);
    let canvas = render_panel_with_offsets(offsets);
    let thumb = panel_scrollbars::horizontal_thumb_rect_for(
        panel_scroll_state::PanelScrollRegion::Preview,
        offsets,
    );

    assert_ne!(Some(accent), pixel_at_rect(&canvas, thumb));
}

#[test]
fn storybook_panel_scrollbars_do_not_draw_position_highlight_markers() {
    let canvas = render_with_offsets(Default::default());

    for region in [
        panel_scroll_state::PanelScrollRegion::Navigation,
        panel_scroll_state::PanelScrollRegion::Preview,
        panel_scroll_state::PanelScrollRegion::Inspector,
    ] {
        let track = panel_scrollbars::track_rect_for(region);
        for marker_color in MARKER_COLORS {
            assert_eq!(0, color_count(&canvas, track, *marker_color));
        }
    }
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

fn render_panel_with_offsets(offsets: panel_scroll_state::PanelScrollOffsets) -> Canvas {
    render::render_storybook_canvas_with_options(render::StorybookRenderOptions {
        theme_id: DARK_THEME,
        selected_page: PANEL_PAGE,
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

fn color_count(canvas: &Canvas, rect: layout_metrics::LayoutRect, color: u32) -> usize {
    let mut count = 0;
    for current_y in rect.y..rect.bottom() {
        for current_x in rect.x..rect.right() {
            if pixel_at(canvas, current_x, current_y) == Some(color) {
                count += 1;
            }
        }
    }
    count
}
