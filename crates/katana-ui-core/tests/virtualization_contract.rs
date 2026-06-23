use katana_ui_core::interaction::virtualization::{
    RowHeightOverride, RowHeightProvider, VirtualizationConfig, VirtualizationPlanner,
};

const TOTAL_ROWS: usize = 100;
const LARGE_TOTAL_ROWS: usize = 10_000;
const ROW_HEIGHT: u32 = 24;
const COMPACT_ROW_HEIGHT: u32 = 20;
const VIEWPORT_HEIGHT: u32 = 48;
const OVERSCAN_ROWS: usize = 1;
const LARGE_OVERSCAN_ROWS: usize = 3;
const FIRST_SCROLLED_ROW_OFFSET: u32 = ROW_HEIGHT;
const LARGE_VIEWPORT_HEIGHT: u32 = 200;
const LARGE_VIEWPORT_OFFSET: u32 = 1_000;
const FOCUSED_INDEX: usize = 5;
const ANCHOR_INDEX: usize = 2;
const BASE_OFFSET: u32 = 100;

#[test]
fn fixed_provider_clamps_range_and_reports_global_aria_positions() {
    let config = VirtualizationConfig {
        enabled: true,
        total_count: TOTAL_ROWS,
        viewport_offset: FIRST_SCROLLED_ROW_OFFSET,
        viewport_height: VIEWPORT_HEIGHT,
        overscan: OVERSCAN_ROWS,
        row_height_provider: RowHeightProvider::Fixed { height: ROW_HEIGHT },
        keep_focused_in_window: false,
        focused_index: None,
    };

    let range = VirtualizationPlanner::compute_visible_range(&config);

    assert_eq!(0, range.start);
    assert_eq!(4, range.end);
    assert_eq!(TOTAL_ROWS, range.total_count);
    assert_eq!(TOTAL_ROWS, range.aria_set_size);
    assert_eq!(1, range.rows[0].aria_pos_in_set);
    assert_eq!(4, range.rows[3].aria_pos_in_set);
}

#[test]
fn variable_provider_uses_per_row_heights_without_shared_state() {
    let config = VirtualizationConfig {
        enabled: true,
        total_count: 3,
        viewport_offset: 10,
        viewport_height: 30,
        overscan: 0,
        row_height_provider: RowHeightProvider::Variable {
            row_heights: vec![10, 30, 10],
            fallback_height: 10,
        },
        keep_focused_in_window: false,
        focused_index: None,
    };

    let first = VirtualizationPlanner::compute_visible_range(&config);
    let second = VirtualizationPlanner::compute_visible_range(&config);

    assert_eq!(first, second);
    assert_eq!(1, first.start);
    assert_eq!(2, first.end);
}

#[test]
fn estimated_provider_uses_measured_overrides() {
    let config = VirtualizationConfig {
        enabled: true,
        total_count: TOTAL_ROWS,
        viewport_offset: 49,
        viewport_height: 2,
        overscan: 0,
        row_height_provider: RowHeightProvider::Estimated {
            estimated_height: 10,
            measured_overrides: vec![RowHeightOverride {
                index: 0,
                height: 50,
            }],
        },
        keep_focused_in_window: false,
        focused_index: None,
    };

    let range = VirtualizationPlanner::compute_visible_range(&config);

    assert_eq!(0, range.start);
    assert_eq!(2, range.end);
}

#[test]
fn focused_row_outside_range_is_returned_as_sentinel() {
    let config = VirtualizationConfig {
        enabled: true,
        total_count: TOTAL_ROWS,
        viewport_offset: COMPACT_ROW_HEIGHT * 50,
        viewport_height: COMPACT_ROW_HEIGHT * 2,
        overscan: OVERSCAN_ROWS,
        row_height_provider: RowHeightProvider::Fixed {
            height: COMPACT_ROW_HEIGHT,
        },
        keep_focused_in_window: true,
        focused_index: Some(FOCUSED_INDEX),
    };

    let range = VirtualizationPlanner::compute_visible_range(&config);

    assert_eq!(49, range.start);
    assert_eq!(53, range.end);
    assert_eq!(Some(FOCUSED_INDEX), range.focused_row.map(|it| it.index));
    assert!(range.rows.iter().all(|it| it.index != FOCUSED_INDEX));
}

#[test]
fn measured_override_merge_and_scroll_correction_are_pure() {
    let before = RowHeightProvider::Estimated {
        estimated_height: 10,
        measured_overrides: vec![RowHeightOverride {
            index: 1,
            height: 10,
        }],
    };
    let after = VirtualizationPlanner::merge_measured_overrides(
        &before,
        &[
            RowHeightOverride {
                index: 0,
                height: 20,
            },
            RowHeightOverride {
                index: 1,
                height: 30,
            },
        ],
    );

    let correction = VirtualizationPlanner::correct_scroll_offset_after_measurement(
        &before,
        &after,
        BASE_OFFSET,
        ANCHOR_INDEX,
    );

    assert_eq!(BASE_OFFSET, correction.before_offset);
    assert_eq!(BASE_OFFSET + 30, correction.after_offset);
}

#[test]
fn ten_thousand_rows_render_only_viewport_plus_overscan() {
    let config = VirtualizationConfig {
        enabled: true,
        total_count: LARGE_TOTAL_ROWS,
        viewport_offset: LARGE_VIEWPORT_OFFSET,
        viewport_height: LARGE_VIEWPORT_HEIGHT,
        overscan: LARGE_OVERSCAN_ROWS,
        row_height_provider: RowHeightProvider::Fixed {
            height: COMPACT_ROW_HEIGHT,
        },
        keep_focused_in_window: false,
        focused_index: None,
    };
    let viewport_rows = (LARGE_VIEWPORT_HEIGHT / COMPACT_ROW_HEIGHT) as usize;

    let range = VirtualizationPlanner::compute_visible_range(&config);

    assert_eq!(LARGE_TOTAL_ROWS, range.aria_set_size);
    assert!(range.rows.len() <= viewport_rows + (LARGE_OVERSCAN_ROWS * 2));
}

#[test]
fn disabled_config_returns_full_range_for_backward_compatibility() {
    let config = VirtualizationConfig {
        enabled: false,
        total_count: TOTAL_ROWS,
        viewport_offset: LARGE_VIEWPORT_OFFSET,
        viewport_height: VIEWPORT_HEIGHT,
        overscan: LARGE_OVERSCAN_ROWS,
        row_height_provider: RowHeightProvider::Fixed { height: ROW_HEIGHT },
        keep_focused_in_window: true,
        focused_index: Some(FOCUSED_INDEX),
    };

    let range = VirtualizationPlanner::compute_visible_range(&config);

    assert_eq!(0, range.start);
    assert_eq!(TOTAL_ROWS, range.end);
    assert_eq!(TOTAL_ROWS, range.rows.len());
    assert_eq!(None, range.focused_row);
}
