mod height;

use height::{cumulative_height, normalized_height, row_height, upsert_override};
use serde::{Deserialize, Serialize};

const MIN_ROW_HEIGHT: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VirtualizationConfig {
    pub enabled: bool,
    pub total_count: usize,
    pub viewport_offset: u32,
    pub viewport_height: u32,
    pub overscan: usize,
    pub row_height_provider: RowHeightProvider,
    pub keep_focused_in_window: bool,
    pub focused_index: Option<usize>,
}

impl Default for VirtualizationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            total_count: 0,
            viewport_offset: 0,
            viewport_height: 0,
            overscan: 0,
            row_height_provider: RowHeightProvider::Fixed {
                height: MIN_ROW_HEIGHT,
            },
            keep_focused_in_window: false,
            focused_index: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RowHeightProvider {
    Fixed {
        height: u32,
    },
    Variable {
        row_heights: Vec<u32>,
        fallback_height: u32,
    },
    Estimated {
        estimated_height: u32,
        measured_overrides: Vec<RowHeightOverride>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RowHeightOverride {
    pub index: usize,
    pub height: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VirtualRange {
    pub start: usize,
    pub end: usize,
    pub total_count: usize,
    pub aria_set_size: usize,
    pub rows: Vec<VirtualRow>,
    pub focused_row: Option<VirtualRow>,
}

impl VirtualRange {
    #[must_use]
    pub fn announce_row(&self, label: &str, index: usize) -> String {
        format!("{label}, {} of {}", index + 1, self.aria_set_size)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct VirtualRow {
    pub index: usize,
    pub aria_pos_in_set: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScrollOffsetCorrection {
    pub before_offset: u32,
    pub after_offset: u32,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct VirtualizationPlanner;

impl VirtualizationPlanner {
    #[must_use]
    pub fn compute_visible_range(config: &VirtualizationConfig) -> VirtualRange {
        if config.total_count == 0 {
            return range_with_rows(0, 0, 0, None);
        }
        if !config.enabled {
            return range_with_rows(0, config.total_count, config.total_count, None);
        }
        let (visible_start, visible_end) = visible_bounds(config);
        let start = visible_start.saturating_sub(config.overscan);
        let end = visible_end
            .saturating_add(config.overscan)
            .min(config.total_count);
        let focused_row = focused_row(config, start, end);
        range_with_rows(start, end, config.total_count, focused_row)
    }

    #[must_use]
    pub fn merge_measured_overrides(
        provider: &RowHeightProvider,
        measurements: &[RowHeightOverride],
    ) -> RowHeightProvider {
        let mut overrides = match provider {
            RowHeightProvider::Estimated {
                measured_overrides, ..
            } => measured_overrides.clone(),
            _ => Vec::new(),
        };
        for measurement in measurements {
            upsert_override(&mut overrides, *measurement);
        }
        match provider {
            RowHeightProvider::Estimated {
                estimated_height, ..
            } => RowHeightProvider::Estimated {
                estimated_height: normalized_height(*estimated_height),
                measured_overrides: overrides,
            },
            _ => provider.clone(),
        }
    }

    #[must_use]
    pub fn correct_scroll_offset_after_measurement(
        before: &RowHeightProvider,
        after: &RowHeightProvider,
        viewport_offset: u32,
        anchor_index: usize,
    ) -> ScrollOffsetCorrection {
        let before_offset = cumulative_height(before, anchor_index);
        let after_offset = cumulative_height(after, anchor_index);
        let corrected = viewport_offset
            .saturating_add(after_offset)
            .saturating_sub(before_offset);
        ScrollOffsetCorrection {
            before_offset: viewport_offset,
            after_offset: corrected,
        }
    }
}

fn visible_bounds(config: &VirtualizationConfig) -> (usize, usize) {
    let mut top: u32 = 0;
    let mut start = 0;
    while start < config.total_count {
        let bottom = top.saturating_add(row_height(&config.row_height_provider, start));
        if bottom > config.viewport_offset {
            break;
        }
        top = bottom;
        start += 1;
    }
    let viewport_bottom = config
        .viewport_offset
        .saturating_add(config.viewport_height);
    let mut end = start;
    while end < config.total_count && top < viewport_bottom {
        top = top.saturating_add(row_height(&config.row_height_provider, end));
        end += 1;
    }
    (start, end.max(start + 1).min(config.total_count))
}

fn focused_row(config: &VirtualizationConfig, start: usize, end: usize) -> Option<VirtualRow> {
    if !config.keep_focused_in_window {
        return None;
    }
    let focused = config.focused_index?;
    if focused >= config.total_count || (start..end).contains(&focused) {
        return None;
    }
    Some(row_accessibility(focused))
}

fn range_with_rows(
    start: usize,
    end: usize,
    total_count: usize,
    focused_row: Option<VirtualRow>,
) -> VirtualRange {
    let rows = (start..end).map(row_accessibility).collect();
    VirtualRange {
        start,
        end,
        total_count,
        aria_set_size: total_count,
        rows,
        focused_row,
    }
}

fn row_accessibility(index: usize) -> VirtualRow {
    VirtualRow {
        index,
        aria_pos_in_set: index + 1,
    }
}
