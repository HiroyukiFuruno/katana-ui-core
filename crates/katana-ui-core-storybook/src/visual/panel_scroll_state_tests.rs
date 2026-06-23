use super::{layout_metrics, panel_scroll_state};

#[test]
fn visible_scrollbar_offset_step_stays_moderate_for_viewport_size() {
    let mut offsets = panel_scroll_state::PanelScrollOffsets::default();
    let max_reasonable_step = super::render::VIEWPORT_HEIGHT / 16;

    assert!(layout_metrics::SCROLL_STEP <= max_reasonable_step);
    assert!(offsets.scroll_delta(panel_scroll_state::PanelScrollRegion::Root, -1.0));
    assert_eq!(layout_metrics::SCROLL_STEP, offsets.root_y);
}

#[test]
fn visible_scrollbar_small_overflow_reaches_max_with_existing_step_limit() {
    let mut offsets = panel_scroll_state::PanelScrollOffsets::default();
    let small_max_offset = layout_metrics::SCROLL_STEP + layout_metrics::SCROLL_STEP / 2;

    assert!(offsets.scroll_delta_with_max(
        panel_scroll_state::PanelScrollRegion::Root,
        small_max_offset,
        -1.0,
    ));
    assert_eq!(layout_metrics::SCROLL_STEP, offsets.root_y);
    assert!(offsets.scroll_delta_with_max(
        panel_scroll_state::PanelScrollRegion::Root,
        small_max_offset,
        -1.0,
    ));
    assert_eq!(small_max_offset, offsets.root_y);
    assert!(!offsets.scroll_delta_with_max(
        panel_scroll_state::PanelScrollRegion::Root,
        small_max_offset,
        -1.0,
    ));
    assert_eq!(small_max_offset, offsets.root_y);
}
