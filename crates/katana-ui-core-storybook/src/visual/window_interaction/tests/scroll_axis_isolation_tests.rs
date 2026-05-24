use super::super::{
    StorybookWindowState, apply_horizontal_scrollbar_drag, apply_scroll_delta,
    apply_scroll_delta_at, apply_scroll_delta_x_at, apply_scrollbar_drag,
};
use crate::visual::layout_metrics;
use crate::visual::panel_scroll_state::{PanelScrollOffsets, PanelScrollRegion};
use crate::visual::panel_scrollbars;

#[test]
fn axis_isolated_scroll_input_changes_only_horizontal_offset_in_inspector() {
    let mut state = StorybookWindowState {
        selected_page: "panel",
        panel_scroll: PanelScrollOffsets {
            inspector_x: layout_metrics::SCROLL_STEP,
            ..PanelScrollOffsets::default()
        },
        scrollbar_visible: true,
        ..StorybookWindowState::default()
    };

    assert!(apply_scroll_delta_x_at(
        &mut state,
        layout_metrics::INSPECTOR_X + 8,
        layout_metrics::INSPECTOR_Y + 90,
        -1.0,
    ));
    assert_eq!(0, state.panel_scroll.inspector_y);
    assert_eq!(0, state.panel_scroll.inspector_x);
}

#[test]
fn axis_isolated_scroll_input_changes_only_vertical_offset_in_inspector() {
    let mut state = StorybookWindowState {
        selected_page: "panel",
        panel_scroll: PanelScrollOffsets {
            inspector_x: layout_metrics::SCROLL_STEP,
            ..PanelScrollOffsets::default()
        },
        scrollbar_visible: true,
        ..StorybookWindowState::default()
    };

    assert!(apply_scroll_delta_at(
        &mut state,
        layout_metrics::INSPECTOR_X + 8,
        layout_metrics::INSPECTOR_Y + 90,
        -1.0,
    ));
    assert_eq!(layout_metrics::SCROLL_STEP, state.panel_scroll.inspector_x);
    assert_eq!(layout_metrics::SCROLL_STEP, state.panel_scroll.inspector_y);
}

#[test]
fn axis_isolated_drag_changes_only_horizontal_offset_in_inspector() {
    let mut state = StorybookWindowState {
        selected_page: "panel",
        scrollbar_visible: true,
        panel_scroll: PanelScrollOffsets {
            inspector_x: layout_metrics::SCROLL_STEP,
            ..PanelScrollOffsets::default()
        },
        ..StorybookWindowState::default()
    };
    let track = panel_scrollbars::horizontal_track_rect_for(PanelScrollRegion::Inspector);

    assert!(apply_horizontal_scrollbar_drag(
        &mut state,
        PanelScrollRegion::Inspector,
        track.right() - 1,
    ));
    assert_eq!(0, state.panel_scroll.inspector_y);
    assert_eq!(0, state.panel_scroll.inspector_x);
}

#[test]
fn axis_isolated_drag_changes_only_vertical_offset_in_inspector() {
    let mut state = StorybookWindowState {
        selected_page: "panel",
        panel_scroll: PanelScrollOffsets {
            inspector_x: layout_metrics::SCROLL_STEP,
            ..PanelScrollOffsets::default()
        },
        scrollbar_visible: true,
        ..StorybookWindowState::default()
    };
    let track = panel_scrollbars::track_rect_for(PanelScrollRegion::Inspector);

    assert!(apply_scrollbar_drag(
        &mut state,
        PanelScrollRegion::Inspector,
        track.bottom() - 1,
    ));
    assert_eq!(layout_metrics::SCROLL_STEP, state.panel_scroll.inspector_x);
    assert!(state.panel_scroll.inspector_y > 0);
}

#[test]
fn drag_reverse_mapping_reclamps_with_current_preview_max() {
    let max_for_preview = crate::visual::panel_scroll_state::max_scroll_x_for(
        PanelScrollRegion::Preview,
        "button",
        Default::default(),
    );
    let mut state = StorybookWindowState {
        panel_scroll: PanelScrollOffsets {
            preview_x: max_for_preview + layout_metrics::SCROLL_STEP,
            ..PanelScrollOffsets::default()
        },
        scrollbar_visible: true,
        ..StorybookWindowState::default()
    };

    assert!(max_for_preview == 0);
    assert!(apply_horizontal_scrollbar_drag(
        &mut state,
        PanelScrollRegion::Preview,
        layout_metrics::PREVIEW_X,
    ));
    assert_eq!(0, state.panel_scroll.preview_x);
    assert_eq!(0, state.panel_scroll.preview_y);
}

#[test]
fn scroll_delta_updates_vertical_viewport() {
    let mut state = StorybookWindowState::default();

    assert!(apply_scroll_delta(&mut state, -1.0));
    assert_eq!(layout_metrics::SCROLL_STEP, state.scroll_y);
    assert!(apply_scroll_delta(&mut state, 1.0));
    assert_eq!(0, state.scroll_y);
}
