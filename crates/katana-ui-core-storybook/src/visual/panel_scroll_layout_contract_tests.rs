use super::{
    Canvas, button_options, layout_metrics, palette, panel_layout, panel_scroll_state,
    panel_scrollbars, render,
};
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const BUTTON_PAGE: &str = "button";
const DEFAULT_PRESET: usize = 0;

#[test]
fn panel_scrollbar_tracks_stay_inside_own_panel_frames() {
    let navigation_frame = layout_metrics::navigation_menu_panel_rect();
    let navigation_track =
        panel_scrollbars::track_rect_for(panel_scroll_state::PanelScrollRegion::Navigation);
    let inspector_frame = layout_metrics::inspector_rect();
    let inspector_track =
        panel_scrollbars::track_rect_for(panel_scroll_state::PanelScrollRegion::Inspector);
    let inspector_horizontal_track = panel_scrollbars::horizontal_track_rect_for(
        panel_scroll_state::PanelScrollRegion::Inspector,
    );

    assert!(rect_inside(navigation_track, navigation_frame));
    assert!(rect_inside(inspector_track, inspector_frame));
    assert!(rect_inside(inspector_horizontal_track, inspector_frame));
}

#[test]
fn inspector_controls_stay_left_of_scrollbar_gutter() {
    let scrollbar =
        panel_scrollbars::track_rect_for(panel_scroll_state::PanelScrollRegion::Inspector);

    for control in button_options::StorybookButtonOptionControl::all() {
        let rect = button_options::control_rect(control);
        assert!(rect.right() <= scrollbar.x);
    }
}

#[test]
fn panel_content_viewports_reserve_scrollbar_gutters() {
    for region in [
        panel_scroll_state::PanelScrollRegion::Navigation,
        panel_scroll_state::PanelScrollRegion::Preview,
        panel_scroll_state::PanelScrollRegion::Inspector,
    ] {
        let layout = panel_layout::region_layout(region);

        assert!(rect_inside(layout.content_viewport, layout.frame));
        assert!(layout.content_viewport.right() <= layout.vertical_track.x);
        assert!(layout.content_viewport.bottom() <= layout.horizontal_track.y);
    }
}

#[test]
fn non_overflowing_preview_has_no_scroll_offset() {
    let overflow = panel_scroll_state::PanelScrollOverflowModel::overflow_for(
        panel_scroll_state::PanelScrollRegion::Preview,
        BUTTON_PAGE,
        Default::default(),
    );
    let mut offsets = panel_scroll_state::PanelScrollOffsets::default();

    assert_eq!(0, overflow.max_x());
    assert_eq!(0, overflow.max_y());
    assert!(!offsets.scroll_delta(panel_scroll_state::PanelScrollRegion::Preview, -1.0));
    assert!(!offsets.scroll_delta_x(panel_scroll_state::PanelScrollRegion::Preview, -1.0));
    assert_eq!(0, offsets.preview_x);
    assert_eq!(0, offsets.preview_y);
}

#[test]
fn inspector_horizontal_thumb_reaches_track_end_at_max_offset() {
    let mut offsets = panel_scroll_state::PanelScrollOffsets::default();
    offsets.set_drag_offset_x(
        panel_scroll_state::PanelScrollRegion::Inspector,
        panel_scroll_state::PanelScrollOverflowModel::max_scroll_x_for(
            panel_scroll_state::PanelScrollRegion::Inspector,
            "button",
            Default::default(),
        ),
    );

    let track = panel_scrollbars::horizontal_track_rect_for(
        panel_scroll_state::PanelScrollRegion::Inspector,
    );
    let thumb = panel_scrollbars::horizontal_thumb_rect_for(
        panel_scroll_state::PanelScrollRegion::Inspector,
        offsets,
    );

    assert_eq!(track.right(), thumb.right());
}

#[test]
fn rendered_panel_content_does_not_paint_reserved_scrollbar_gutter() {
    let canvas = render::render_storybook_canvas_with_options(render::StorybookRenderOptions {
        theme_id: DARK_THEME,
        selected_page: BUTTON_PAGE,
        preset_index: DEFAULT_PRESET,
        preset_tab_scroll_x: 0,
        scroll_y: 0,
        scrollbar_visible: true,
        panel_scroll: Default::default(),
        tree_expansion: Default::default(),
        show_navigation_lines: true,
        show_navigation_text_connectors: false,
        screen_state: Default::default(),
    });
    let palette = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());

    for region in [
        panel_scroll_state::PanelScrollRegion::Navigation,
        panel_scroll_state::PanelScrollRegion::Inspector,
    ] {
        let layout = panel_layout::region_layout(region);
        let gutter = reserved_vertical_gutter(layout);

        assert!(gutter.width > 0);
        assert_eq!(0, color_count(&canvas, gutter, palette.accent));
        assert_eq!(0, color_count(&canvas, gutter, palette.selection));
    }
}

fn rect_inside(inner: layout_metrics::LayoutRect, outer: layout_metrics::LayoutRect) -> bool {
    inner.x >= outer.x
        && inner.y >= outer.y
        && inner.right() <= outer.right()
        && inner.bottom() <= outer.bottom()
}

fn reserved_vertical_gutter(layout: panel_layout::PanelRegionLayout) -> layout_metrics::LayoutRect {
    layout_metrics::LayoutRect::new(
        layout.content_viewport.right(),
        layout.content_viewport.y,
        layout
            .vertical_track
            .x
            .saturating_sub(layout.content_viewport.right()),
        layout.content_viewport.height,
    )
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

fn pixel_at(canvas: &Canvas, x: usize, y: usize) -> Option<u32> {
    canvas.pixels().get(y * canvas.width() + x).copied()
}
