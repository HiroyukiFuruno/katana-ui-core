use katana_ui_core::molecule::{
    GridAxisConfig, GridAxisPlanner, GridIndexRange, GridTrackSizeProvider,
};

#[test]
fn fixed_axis_clamps_half_open_range_and_overscan() {
    let config = GridAxisConfig::new(100, GridTrackSizeProvider::fixed(20), 40)
        .scroll_offset(20)
        .overscan(1);

    let plan = GridAxisPlanner::plan(&config);

    assert_eq!(GridIndexRange::new(0, 4), plan.visible_range);
    assert_eq!(vec![0, 1, 2, 3], plan.materialized_indices);
    assert_eq!(2_000, plan.total_extent);
    assert_eq!(20, plan.scroll_offset);
}

#[test]
fn variable_axis_normalizes_zero_and_uses_fallback_sizes() {
    let provider = GridTrackSizeProvider::variable(vec![0, 30], 10);
    let config = GridAxisConfig::new(3, provider.clone(), 30).scroll_offset(1);

    let plan = GridAxisPlanner::plan(&config);

    assert_eq!(1, provider.track_size(0));
    assert_eq!(30, provider.track_size(1));
    assert_eq!(10, provider.track_size(2));
    assert_eq!(31, provider.track_offset(2));
    assert_eq!(41, provider.track_offset(3));
    assert_eq!(41, plan.total_extent);
    assert_eq!(GridIndexRange::new(1, 2), plan.visible_range);
}

#[test]
fn hidden_variable_tracks_have_zero_extent_and_are_not_materialized() {
    let provider =
        GridTrackSizeProvider::variable_with_hidden(vec![10, 20, 30, 40], 12, vec![1, 3, 3]);
    let plan = GridAxisPlanner::plan(&GridAxisConfig::new(4, provider.clone(), 100));

    assert_eq!(10, provider.track_size(0));
    assert_eq!(0, provider.track_size(1));
    assert_eq!(30, provider.track_size(2));
    assert_eq!(0, provider.track_size(3));
    assert_eq!(10, provider.track_offset(2));
    assert_eq!(40, provider.track_offset(4));
    assert_eq!(40, plan.total_extent);
    assert_eq!(vec![0, 2], plan.materialized_indices);
}

#[test]
fn visible_variable_track_beyond_explicit_sizes_uses_normalized_fallback() {
    let provider = GridTrackSizeProvider::variable_with_hidden(vec![12], 0, vec![0]);

    assert_eq!(1, provider.track_size(4));
}

#[test]
fn all_hidden_variable_tracks_produce_an_empty_visible_plan() {
    let provider =
        GridTrackSizeProvider::variable_with_hidden(vec![10, 20], 12, vec![0, 1, usize::MAX]);
    let plan = GridAxisPlanner::plan(&GridAxisConfig::new(2, provider, 100));

    assert_eq!(0, plan.total_extent);
    assert_eq!(GridIndexRange::new(0, 0), plan.visible_range);
    assert!(plan.materialized_indices.is_empty());
}

#[test]
fn variable_axis_binary_searches_explicit_and_fallback_tracks() {
    let config = GridAxisConfig::new(6, GridTrackSizeProvider::variable(vec![5, 10], 20), 10)
        .scroll_offset(35);

    let plan = GridAxisPlanner::plan(&config);

    assert_eq!(GridIndexRange::new(3, 4), plan.visible_range);
    assert_eq!(35, plan.scroll_offset);
}

#[test]
fn zero_viewport_and_all_frozen_tracks_have_empty_scroll_ranges() {
    let zero_viewport =
        GridAxisPlanner::plan(&GridAxisConfig::new(4, GridTrackSizeProvider::fixed(10), 0));
    let all_frozen = GridAxisPlanner::plan(
        &GridAxisConfig::new(2, GridTrackSizeProvider::fixed(10), 100).frozen_count(9),
    );

    assert!(zero_viewport.visible_range.is_empty());
    assert_eq!(GridIndexRange::new(2, 2), all_frozen.visible_range);
    assert_eq!(vec![0, 1], all_frozen.materialized_indices);
}

#[test]
fn frozen_extent_larger_than_viewport_removes_scrollable_range() {
    let config = GridAxisConfig::new(10, GridTrackSizeProvider::fixed(20), 30)
        .frozen_count(2)
        .scroll_offset(u32::MAX)
        .overscan(5);

    let plan = GridAxisPlanner::plan(&config);

    assert_eq!(40, plan.frozen_extent);
    assert_eq!(GridIndexRange::new(2, 2), plan.visible_range);
    assert_eq!(vec![0, 1], plan.materialized_indices);
    assert_eq!(0, plan.scroll_offset);
}

#[test]
fn axis_extents_and_offsets_saturate_without_panicking() {
    let provider = GridTrackSizeProvider::fixed(u32::MAX);
    let config = GridAxisConfig::new(usize::MAX, provider.clone(), 100);

    let plan = GridAxisPlanner::plan(&config);

    assert_eq!(u32::MAX, provider.track_offset(usize::MAX));
    assert_eq!(u32::MAX, plan.total_extent);
    assert_eq!(GridIndexRange::new(0, 1), plan.visible_range);
}

#[test]
fn empty_axis_has_no_materialized_tracks() {
    let plan = GridAxisPlanner::plan(&GridAxisConfig::new(
        0,
        GridTrackSizeProvider::fixed(20),
        100,
    ));

    assert_eq!(GridIndexRange::new(0, 0), plan.visible_range);
    assert!(plan.materialized_indices.is_empty());
    assert_eq!(0, plan.total_extent);
}
