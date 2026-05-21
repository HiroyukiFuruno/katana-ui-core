use super::super::{
    StorybookWindowState, apply_scroll_delta, apply_scroll_delta_at, apply_scroll_delta_x_at,
    apply_scrollbar_drag,
};
use crate::requirements::StoryRequirements;
use crate::visual::layout_metrics;
use crate::visual::navigation_tree;
use crate::visual::panel_scroll_state::{PanelScrollOffsets, PanelScrollRegion};
use crate::visual::panel_scrollbars;

const PREVIEW_POINTER_X_OFFSET: usize = 8;
const PREVIEW_POINTER_Y_OFFSET: usize = 40;

#[test]
fn scroll_delta_updates_vertical_viewport() {
    let mut state = StorybookWindowState::default();

    assert!(apply_scroll_delta(&mut state, -1.0));
    assert_eq!(layout_metrics::SCROLL_STEP, state.scroll_y);
    assert!(apply_scroll_delta(&mut state, 1.0));
    assert_eq!(0, state.scroll_y);
}

#[test]
fn panel_scrollbar_thumbs_move_only_for_scrolled_panel() {
    let mut state = StorybookWindowState::default();
    let thumb = panel_scrollbars::thumb_rect_for(PanelScrollRegion::Preview, state.panel_scroll);

    assert!(!apply_scrollbar_drag(
        &mut state,
        PanelScrollRegion::Preview,
        thumb.y + 180,
    ));
    assert_eq!(0, state.panel_scroll.preview_y);
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

    assert!(!apply_scroll_delta_at(
        &mut state,
        layout_metrics::PREVIEW_X + 8,
        layout_metrics::PRESET_ACTIVE_Y + 40,
        -1.0,
    ));
    assert_eq!(layout_metrics::SCROLL_STEP, state.panel_scroll.navigation_y);
    assert_eq!(0, state.panel_scroll.preview_y);
    assert_eq!(0, state.panel_scroll.inspector_y);
    assert_eq!(0, state.scroll_y);

    assert!(apply_scroll_delta_at(
        &mut state,
        layout_metrics::INSPECTOR_X + 8,
        layout_metrics::INSPECTOR_Y + 90,
        -1.0,
    ));
    assert_eq!(layout_metrics::SCROLL_STEP, state.panel_scroll.navigation_y);
    assert_eq!(0, state.panel_scroll.preview_y);
    assert_eq!(layout_metrics::SCROLL_STEP, state.panel_scroll.inspector_y);
    assert_eq!(0, state.scroll_y);

    assert!(apply_scroll_delta_at(&mut state, 288, 22, -1.0));
    assert_eq!(layout_metrics::SCROLL_STEP, state.panel_scroll.root_y);
    assert_eq!(layout_metrics::SCROLL_STEP, state.scroll_y);
}

#[test]
fn scroll_delta_region_uses_root_scrolled_content_position() {
    let mut state = StorybookWindowState {
        scroll_y: layout_metrics::SCROLL_STEP,
        panel_scroll: PanelScrollOffsets {
            root_y: layout_metrics::SCROLL_STEP,
            ..PanelScrollOffsets::default()
        },
        ..StorybookWindowState::default()
    };
    let visible_preview_y = layout_metrics::PRESET_ACTIVE_Y + 40 - state.panel_scroll.root_y;

    assert!(!apply_scroll_delta_at(
        &mut state,
        layout_metrics::PREVIEW_X + 8,
        visible_preview_y,
        -1.0,
    ));
    assert_eq!(layout_metrics::SCROLL_STEP, state.panel_scroll.root_y);
    assert_eq!(0, state.panel_scroll.preview_y);
    assert_eq!(layout_metrics::SCROLL_STEP, state.scroll_y);
}

#[test]
fn navigation_wheel_scroll_reaches_last_tree_row_without_fixed_cap() {
    let mut state = StorybookWindowState::default();
    let max_scroll = navigation_tree::max_scroll_y(state.tree_expansion);

    for _ in 0..100 {
        apply_scroll_delta_at(
            &mut state,
            layout_metrics::NAV_ROW_X,
            layout_metrics::NAV_FIRST_ROW_Y,
            -1.0,
        );
    }

    assert_eq!(max_scroll, state.panel_scroll.navigation_y);
    assert_eq!(
        layout_metrics::navigation_menu_panel_rect().bottom(),
        navigation_tree::last_row_bottom_at_scroll(state.tree_expansion, max_scroll)
    );
}

#[test]
fn hidden_preview_scrollbar_does_not_accept_wheel_input_on_button_page() {
    let mut state = StorybookWindowState::default();

    assert!(!apply_scroll_delta_at(
        &mut state,
        layout_metrics::PREVIEW_X + 8,
        layout_metrics::PRESET_ACTIVE_Y + 40,
        -1.0,
    ));
    assert!(!apply_scroll_delta_x_at(
        &mut state,
        layout_metrics::PREVIEW_X + 8,
        layout_metrics::PRESET_ACTIVE_Y + 40,
        -1.0,
    ));
    assert_eq!(0, state.panel_scroll.preview_y);
    assert_eq!(0, state.panel_scroll.preview_x);
}

#[test]
fn panel_preview_wheel_input_is_inert_when_preview_content_does_not_overflow() {
    let mut state = StorybookWindowState {
        selected_page: "panel",
        scrollbar_visible: true,
        ..StorybookWindowState::default()
    };

    assert!(!apply_scroll_delta_at(
        &mut state,
        layout_metrics::PREVIEW_X + 8,
        layout_metrics::PRESET_ACTIVE_Y + 40,
        -1.0,
    ));
    assert!(!apply_scroll_delta_x_at(
        &mut state,
        layout_metrics::PREVIEW_X + 8,
        layout_metrics::PRESET_ACTIVE_Y + 40,
        -1.0,
    ));
    assert_eq!(0, state.panel_scroll.preview_y);
    assert_eq!(0, state.panel_scroll.preview_x);
}

#[test]
fn non_overflowing_required_pages_ignore_preview_wheel_input() {
    for &page in StoryRequirements::required_pages() {
        let mut state = StorybookWindowState {
            selected_page: page,
            scrollbar_visible: true,
            ..StorybookWindowState::default()
        };
        let pointer_x = layout_metrics::PREVIEW_X + PREVIEW_POINTER_X_OFFSET;
        let pointer_y = layout_metrics::PRESET_ACTIVE_Y + PREVIEW_POINTER_Y_OFFSET;

        assert!(
            !apply_scroll_delta_at(&mut state, pointer_x, pointer_y, -1.0),
            "{page} accepted vertical preview wheel without overflow"
        );
        assert!(
            !apply_scroll_delta_x_at(&mut state, pointer_x, pointer_y, -1.0),
            "{page} accepted horizontal preview wheel without overflow"
        );
        assert_eq!(0, state.panel_scroll.preview_y, "{page} preview y");
        assert_eq!(0, state.panel_scroll.preview_x, "{page} preview x");
    }
}
