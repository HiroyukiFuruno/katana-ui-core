use super::{panel_scroll_state, panel_scrollbars};
use crate::catalog::story_map::STORY_GROUPS;
use crate::visual::navigation_tree::{self, TreeExpansionState};

const BUTTON_PAGE: &str = "button";

#[test]
fn visible_panel_scrollbar_thumbs_follow_content_metrics() {
    for region in [
        panel_scroll_state::PanelScrollRegion::Navigation,
        panel_scroll_state::PanelScrollRegion::Inspector,
    ] {
        let track = panel_scrollbars::track_rect_for(region);
        let overflow = panel_scroll_state::overflow_for(region, BUTTON_PAGE, Default::default());
        let thumb = panel_scrollbars::thumb_rect_for(region, Default::default());
        let expected = expected_thumb_len(
            track.height,
            overflow.viewport_height,
            overflow.content_height,
        );

        assert_eq!(expected, thumb.height, "{region:?} vertical thumb height");
    }

    let navigation_thumb = panel_scrollbars::thumb_rect_for(
        panel_scroll_state::PanelScrollRegion::Navigation,
        Default::default(),
    );
    let inspector_thumb = panel_scrollbars::thumb_rect_for(
        panel_scroll_state::PanelScrollRegion::Inspector,
        Default::default(),
    );

    assert_ne!(navigation_thumb.height, inspector_thumb.height);
}

#[test]
fn visible_panel_horizontal_scrollbar_thumb_follows_content_metrics() {
    let region = panel_scroll_state::PanelScrollRegion::Inspector;
    let track = panel_scrollbars::horizontal_track_rect_for(region);
    let overflow = panel_scroll_state::overflow_for(region, BUTTON_PAGE, Default::default());
    let thumb = panel_scrollbars::horizontal_thumb_rect_for(region, Default::default());
    let expected = expected_thumb_len(track.width, overflow.viewport_width, overflow.content_width);

    assert_eq!(expected, thumb.width);
}

#[test]
fn visible_preview_horizontal_scrollbar_thumb_is_full_track_without_overflow() {
    let region = panel_scroll_state::PanelScrollRegion::Preview;
    let track = panel_scrollbars::horizontal_track_rect_for(region);
    let thumb = panel_scrollbars::horizontal_thumb_rect_for(region, Default::default());

    assert_eq!(track.width, thumb.width);
}

#[test]
fn visible_navigation_scrollbar_metrics_follow_current_tree_expansion() {
    let region = panel_scroll_state::PanelScrollRegion::Navigation;
    let expanded = TreeExpansionState::default();
    let collapsed = collapsed_navigation_expansion();
    let track = panel_scrollbars::track_rect_for(region);
    let expanded_thumb =
        panel_scrollbars::thumb_rect_for_state(region, Default::default(), BUTTON_PAGE, expanded);
    let collapsed_thumb =
        panel_scrollbars::thumb_rect_for_state(region, Default::default(), BUTTON_PAGE, collapsed);

    assert!(panel_scrollbars::vertical_bar_visible_for(
        region,
        BUTTON_PAGE,
        expanded,
        true
    ));
    assert!(!panel_scrollbars::vertical_bar_visible_for(
        region,
        BUTTON_PAGE,
        collapsed,
        true
    ));
    assert!(!panel_scrollbars::vertical_bar_visible_for(
        region,
        BUTTON_PAGE,
        expanded,
        false
    ));
    assert!(collapsed_thumb.height > expanded_thumb.height);
    assert_eq!(track.height, collapsed_thumb.height);
}

#[test]
fn visible_navigation_scrollbar_drag_reverse_follows_current_tree_expansion() {
    let region = panel_scroll_state::PanelScrollRegion::Navigation;
    let expanded = TreeExpansionState::default();
    let collapsed = collapsed_navigation_expansion();
    let expanded_max = navigation_tree::max_scroll_y(expanded);
    let expanded_offsets = panel_scroll_state::PanelScrollOffsets {
        navigation_y: expanded_max,
        ..Default::default()
    };
    let expanded_thumb =
        panel_scrollbars::thumb_rect_for_state(region, expanded_offsets, BUTTON_PAGE, expanded);
    let track = panel_scrollbars::track_rect_for(region);

    assert!(expanded_max > 0);
    assert_eq!(
        expanded_max,
        panel_scrollbars::offset_from_drag_for(region, expanded_thumb.y, BUTTON_PAGE, expanded)
    );
    assert_eq!(
        0,
        panel_scrollbars::offset_from_drag_for(region, track.bottom(), BUTTON_PAGE, collapsed)
    );
}

#[test]
fn visible_panel_scrollbar_thumbs_reach_track_end_at_max_offset() {
    let mut offsets = panel_scroll_state::PanelScrollOffsets::default();
    offsets.set_drag_offset(
        panel_scroll_state::PanelScrollRegion::Navigation,
        panel_scroll_state::max_scroll_y(panel_scroll_state::PanelScrollRegion::Navigation),
    );
    offsets.set_drag_offset(
        panel_scroll_state::PanelScrollRegion::Inspector,
        panel_scroll_state::max_scroll_y(panel_scroll_state::PanelScrollRegion::Inspector),
    );

    let vertical_track =
        panel_scrollbars::track_rect_for(panel_scroll_state::PanelScrollRegion::Navigation);
    let vertical_thumb = panel_scrollbars::thumb_rect_for(
        panel_scroll_state::PanelScrollRegion::Navigation,
        offsets,
    );
    let inspector_track =
        panel_scrollbars::track_rect_for(panel_scroll_state::PanelScrollRegion::Inspector);
    let inspector_thumb =
        panel_scrollbars::thumb_rect_for(panel_scroll_state::PanelScrollRegion::Inspector, offsets);

    assert_eq!(vertical_track.bottom(), vertical_thumb.bottom());
    assert_eq!(inspector_track.bottom(), inspector_thumb.bottom());
}

#[test]
fn visible_panel_horizontal_scrollbar_thumb_reaches_track_end_at_max_offset() {
    let mut offsets = panel_scroll_state::PanelScrollOffsets::default();
    offsets.set_drag_offset_x(
        panel_scroll_state::PanelScrollRegion::Inspector,
        panel_scroll_state::max_scroll_x(panel_scroll_state::PanelScrollRegion::Inspector),
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

fn expected_thumb_len(track_len: usize, viewport_len: usize, content_len: usize) -> usize {
    if content_len <= viewport_len {
        return track_len;
    }
    (track_len * viewport_len / content_len)
        .max(panel_scrollbars::PANEL_SCROLLBAR_THUMB_MIN_LENGTH)
        .min(track_len)
}

fn collapsed_navigation_expansion() -> TreeExpansionState {
    let mut expansion = TreeExpansionState::default();
    for group in STORY_GROUPS.iter().copied() {
        expansion.toggle(group);
    }
    expansion
}
