use super::super::{StorybookWindowState, apply_scroll_delta_at, panel_scroll_drag};
use crate::catalog::story_map::STORY_GROUPS;
use crate::visual::layout_metrics;
use crate::visual::navigation_tree::TreeExpansionState;
use crate::visual::panel_scroll_state::{PanelScrollOffsets, PanelScrollRegion};
use crate::visual::panel_scrollbars;

#[test]
fn hidden_scrollbar_option_does_not_change_wheel_scroll_offsets() {
    let mut visible = StorybookWindowState {
        scrollbar_visible: true,
        ..StorybookWindowState::default()
    };
    let mut hidden = StorybookWindowState {
        scrollbar_visible: false,
        ..StorybookWindowState::default()
    };

    assert_eq!(
        apply_scroll_delta_at(
            &mut visible,
            layout_metrics::NAV_ROW_X,
            layout_metrics::NAV_FIRST_ROW_Y,
            -1.0,
        ),
        apply_scroll_delta_at(
            &mut hidden,
            layout_metrics::NAV_ROW_X,
            layout_metrics::NAV_FIRST_ROW_Y,
            -1.0,
        )
    );
    assert_eq!(
        visible.panel_scroll.navigation_y,
        hidden.panel_scroll.navigation_y
    );
    assert_eq!(visible.scroll_y, hidden.scroll_y);
}

#[test]
fn collapsed_navigation_visible_scrollbar_is_not_hit_test_target() {
    let expanded = TreeExpansionState::default();
    let collapsed = collapsed_navigation_expansion();
    let track = panel_scrollbars::track_rect_for(PanelScrollRegion::Navigation);
    let x = track.x + track.width / 2;
    let y = track.y + 1;

    assert_eq!(
        Some(PanelScrollRegion::Navigation),
        panel_scroll_drag::vertical_region_at(
            x,
            y,
            PanelScrollOffsets::default(),
            "button",
            expanded,
            true,
        )
    );
    assert_eq!(
        None,
        panel_scroll_drag::vertical_region_at(
            x,
            y,
            PanelScrollOffsets::default(),
            "button",
            collapsed,
            true,
        )
    );
}

fn collapsed_navigation_expansion() -> TreeExpansionState {
    let mut expansion = TreeExpansionState::default();
    for group in STORY_GROUPS.iter().copied() {
        expansion.toggle(group);
    }
    expansion
}
