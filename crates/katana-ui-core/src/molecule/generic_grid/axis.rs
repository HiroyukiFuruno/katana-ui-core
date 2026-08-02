use super::axis_metrics::TrackMetrics;
use super::{GridAxisConfig, GridAxisPlan, GridAxisPlanner, GridIndexRange, GridTrackSizeProvider};

const MIN_TRACK_SIZE: u32 = 1;

impl Default for GridTrackSizeProvider {
    fn default() -> Self {
        Self::fixed(MIN_TRACK_SIZE)
    }
}

impl GridTrackSizeProvider {
    #[must_use]
    pub const fn fixed(size: u32) -> Self {
        Self::Fixed { size }
    }

    #[must_use]
    pub fn variable(sizes: Vec<u32>, fallback_size: u32) -> Self {
        Self::Variable {
            sizes,
            fallback_size,
        }
    }

    #[must_use]
    pub fn variable_with_hidden(
        sizes: Vec<u32>,
        fallback_size: u32,
        mut hidden_indices: Vec<usize>,
    ) -> Self {
        hidden_indices.sort_unstable();
        hidden_indices.dedup();
        Self::VariableWithHidden {
            sizes,
            fallback_size,
            hidden_indices,
        }
    }

    #[must_use]
    pub fn is_hidden(&self, index: usize) -> bool {
        match self {
            Self::VariableWithHidden {
                sizes,
                hidden_indices,
                ..
            } => index < sizes.len() && hidden_indices.contains(&index),
            Self::Fixed { .. } | Self::Variable { .. } => false,
        }
    }

    #[must_use]
    pub fn track_size(&self, index: usize) -> u32 {
        match self {
            Self::Fixed { size } => normalized_size(*size),
            Self::Variable {
                sizes,
                fallback_size,
            } => sizes
                .get(index)
                .copied()
                .map(normalized_size)
                .unwrap_or_else(|| normalized_size(*fallback_size)),
            Self::VariableWithHidden {
                sizes,
                fallback_size,
                hidden_indices,
            } => {
                if hidden_indices.contains(&index) {
                    0
                } else {
                    sizes
                        .get(index)
                        .copied()
                        .map(normalized_size)
                        .unwrap_or_else(|| normalized_size(*fallback_size))
                }
            }
        }
    }

    #[must_use]
    pub fn track_offset(&self, index: usize) -> u32 {
        match self {
            Self::Fixed { size } => normalized_size(*size).saturating_mul(usize_to_u32(index)),
            Self::Variable {
                sizes,
                fallback_size,
            } => variable_offset(sizes, *fallback_size, index),
            Self::VariableWithHidden {
                sizes,
                fallback_size,
                hidden_indices,
            } => variable_offset_with_hidden(sizes, *fallback_size, hidden_indices, index),
        }
    }
}

impl GridAxisConfig {
    #[must_use]
    pub const fn new(
        total_count: usize,
        track_sizes: GridTrackSizeProvider,
        viewport_extent: u32,
    ) -> Self {
        Self {
            total_count,
            track_sizes,
            viewport_extent,
            scroll_offset: 0,
            overscan: 0,
            frozen_count: 0,
        }
    }

    #[must_use]
    pub const fn scroll_offset(mut self, value: u32) -> Self {
        self.scroll_offset = value;
        self
    }

    #[must_use]
    pub const fn overscan(mut self, value: usize) -> Self {
        self.overscan = value;
        self
    }

    #[must_use]
    pub const fn frozen_count(mut self, value: usize) -> Self {
        self.frozen_count = value;
        self
    }
}

impl GridAxisPlanner {
    #[must_use]
    pub fn plan(config: &GridAxisConfig) -> GridAxisPlan {
        if config.total_count == 0 {
            return GridAxisPlan {
                viewport_extent: config.viewport_extent,
                ..GridAxisPlan::default()
            };
        }
        let metrics = TrackMetrics::new(&config.track_sizes, config.total_count);
        let frozen_count = config.frozen_count.min(config.total_count);
        let frozen_extent = metrics.offset(frozen_count);
        let scrollable_viewport = config.viewport_extent.saturating_sub(frozen_extent);
        let scrollable_extent = metrics.total_extent().saturating_sub(frozen_extent);
        let max_scroll = scrollable_extent.saturating_sub(scrollable_viewport);
        let scroll_offset = if scrollable_viewport == 0 {
            0
        } else {
            config.scroll_offset.min(max_scroll)
        };
        let visible_range = if scrollable_extent == 0 {
            GridIndexRange::new(frozen_count, frozen_count)
        } else {
            visible_range(
                config,
                &metrics,
                frozen_count,
                frozen_extent,
                scrollable_viewport,
                scroll_offset,
            )
        };
        let mut materialized_indices =
            Vec::with_capacity(frozen_count.saturating_add(visible_range.len()));
        materialized_indices
            .extend((0..frozen_count).filter(|index| !config.track_sizes.is_hidden(*index)));
        materialized_indices.extend(
            (visible_range.start..visible_range.end)
                .filter(|index| !config.track_sizes.is_hidden(*index)),
        );
        GridAxisPlan {
            total_extent: metrics.total_extent(),
            frozen_extent,
            viewport_extent: config.viewport_extent,
            scroll_offset,
            visible_range,
            materialized_indices,
        }
    }
}

fn visible_range(
    config: &GridAxisConfig,
    metrics: &TrackMetrics,
    frozen_count: usize,
    frozen_extent: u32,
    scrollable_viewport: u32,
    scroll_offset: u32,
) -> GridIndexRange {
    if frozen_count >= config.total_count || scrollable_viewport == 0 {
        return GridIndexRange::new(frozen_count, frozen_count);
    }
    let viewport_start = frozen_extent.saturating_add(scroll_offset);
    let viewport_end = viewport_start
        .saturating_add(scrollable_viewport)
        .min(metrics.total_extent());
    let visible_start = metrics
        .index_at_offset(viewport_start)
        .max(frozen_count)
        .min(config.total_count);
    let visible_end = metrics
        .index_at_offset(viewport_end.saturating_sub(1))
        .saturating_add(1)
        .min(config.total_count);
    GridIndexRange::new(
        visible_start
            .saturating_sub(config.overscan)
            .max(frozen_count),
        visible_end
            .saturating_add(config.overscan)
            .min(config.total_count),
    )
}

fn variable_offset(sizes: &[u32], fallback_size: u32, index: usize) -> u32 {
    let explicit_count = sizes.len().min(index);
    let explicit_extent = sizes
        .iter()
        .take(explicit_count)
        .fold(0_u32, |total, size| {
            total.saturating_add(normalized_size(*size))
        });
    explicit_extent.saturating_add(
        normalized_size(fallback_size)
            .saturating_mul(usize_to_u32(index.saturating_sub(explicit_count))),
    )
}

fn variable_offset_with_hidden(
    sizes: &[u32],
    fallback_size: u32,
    hidden_indices: &[usize],
    index: usize,
) -> u32 {
    let explicit_count = sizes.len().min(index);
    let explicit_extent =
        sizes
            .iter()
            .take(explicit_count)
            .enumerate()
            .fold(0_u32, |total, (track_index, size)| {
                if hidden_indices.contains(&track_index) {
                    total
                } else {
                    total.saturating_add(normalized_size(*size))
                }
            });
    explicit_extent.saturating_add(
        normalized_size(fallback_size)
            .saturating_mul(usize_to_u32(index.saturating_sub(explicit_count))),
    )
}

const fn normalized_size(size: u32) -> u32 {
    if size == 0 { MIN_TRACK_SIZE } else { size }
}

fn usize_to_u32(value: usize) -> u32 {
    u32::try_from(value).unwrap_or(u32::MAX)
}
