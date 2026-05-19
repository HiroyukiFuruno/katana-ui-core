use super::super::{
    StorybookWindowState, apply_scroll_delta, apply_scroll_delta_at, apply_scrollbar_drag,
};
use crate::visual::layout_metrics;
use crate::visual::panel_scroll_state::PanelScrollRegion;
use crate::visual::panel_scrollbars;

#[test]
fn scroll_delta_updates_vertical_viewport() {
    let mut state = StorybookWindowState::default();

    assert!(apply_scroll_delta(&mut state, -1.0));
    assert_eq!(layout_metrics::SCROLL_STEP, state.scroll_y);
    assert!(apply_scroll_delta(&mut state, 1.0));
    assert_eq!(0, state.scroll_y);
}

#[test]
fn dragging_panel_scrollbar_thumb_updates_only_that_panel_offset() {
    let mut state = StorybookWindowState::default();
    let thumb = panel_scrollbars::thumb_rect_for(PanelScrollRegion::Preview, state.panel_scroll);

    assert!(apply_scrollbar_drag(
        &mut state,
        PanelScrollRegion::Preview,
        thumb.y + 180,
    ));
    assert!(state.panel_scroll.preview_y > 0);
    assert_eq!(0, state.panel_scroll.navigation_y);
    assert_eq!(0, state.panel_scroll.inspector_y);
}

#[test]
fn scroll_delta_updates_only_the_panel_under_pointer() {
    let mut state = StorybookWindowState::default();

    assert!(apply_scroll_delta_at(
        &mut state,
        layout_metrics::NAV_ROW_X,
        layout_metrics::NAV_FIRST_ROW_Y,
        -1.0,
    ));
    assert_eq!(layout_metrics::SCROLL_STEP, state.panel_scroll.navigation_y);
    assert_eq!(0, state.panel_scroll.preview_y);
    assert_eq!(0, state.panel_scroll.inspector_y);
    assert_eq!(0, state.scroll_y);

    assert!(apply_scroll_delta_at(
        &mut state,
        layout_metrics::PREVIEW_X + 8,
        layout_metrics::PRESET_ACTIVE_Y + 40,
        -1.0,
    ));
    assert_eq!(layout_metrics::SCROLL_STEP, state.panel_scroll.navigation_y);
    assert_eq!(layout_metrics::SCROLL_STEP, state.panel_scroll.preview_y);
    assert_eq!(0, state.panel_scroll.inspector_y);
    assert_eq!(0, state.scroll_y);

    assert!(apply_scroll_delta_at(
        &mut state,
        layout_metrics::INSPECTOR_X + 8,
        layout_metrics::INSPECTOR_Y + 90,
        -1.0,
    ));
    assert_eq!(layout_metrics::SCROLL_STEP, state.panel_scroll.navigation_y);
    assert_eq!(layout_metrics::SCROLL_STEP, state.panel_scroll.preview_y);
    assert_eq!(layout_metrics::SCROLL_STEP, state.panel_scroll.inspector_y);
    assert_eq!(0, state.scroll_y);

    assert!(apply_scroll_delta_at(&mut state, 288, 22, -1.0));
    assert_eq!(layout_metrics::SCROLL_STEP, state.panel_scroll.root_y);
    assert_eq!(layout_metrics::SCROLL_STEP, state.scroll_y);
}
