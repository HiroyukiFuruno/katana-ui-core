use super::GridTrackSizeProvider;

const MIN_TRACK_SIZE: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum TrackMetrics {
    Fixed {
        size: u32,
        total_count: usize,
    },
    Variable {
        prefix_extents: Vec<u32>,
        explicit_count: usize,
        fallback_size: u32,
        total_count: usize,
    },
}

impl TrackMetrics {
    pub(super) fn new(provider: &GridTrackSizeProvider, total_count: usize) -> Self {
        match provider {
            GridTrackSizeProvider::Fixed { size } => Self::Fixed {
                size: normalized_size(*size),
                total_count,
            },
            GridTrackSizeProvider::Variable {
                sizes,
                fallback_size,
            } => Self::variable(sizes, *fallback_size, &[], total_count),
            GridTrackSizeProvider::VariableWithHidden {
                sizes,
                fallback_size,
                hidden_indices,
            } => Self::variable(sizes, *fallback_size, hidden_indices, total_count),
        }
    }

    pub(super) fn total_extent(&self) -> u32 {
        self.offset(self.total_count())
    }

    pub(super) fn offset(&self, index: usize) -> u32 {
        let index = index.min(self.total_count());
        match self {
            Self::Fixed { size, .. } => size.saturating_mul(usize_to_u32(index)),
            Self::Variable {
                prefix_extents,
                explicit_count,
                fallback_size,
                ..
            } => variable_metric_offset(prefix_extents, *explicit_count, *fallback_size, index),
        }
    }

    pub(super) fn index_at_offset(&self, offset: u32) -> usize {
        let index = match self {
            Self::Fixed { size, .. } => usize::try_from(offset / *size).unwrap_or(usize::MAX),
            Self::Variable {
                prefix_extents,
                explicit_count,
                fallback_size,
                ..
            } => variable_index_at_offset(prefix_extents, *explicit_count, *fallback_size, offset),
        };
        index.min(self.total_count().saturating_sub(1))
    }

    fn variable(
        sizes: &[u32],
        fallback_size: u32,
        hidden_indices: &[usize],
        total_count: usize,
    ) -> Self {
        let explicit_count = sizes.len().min(total_count);
        let mut prefix_extents: Vec<u32> = Vec::with_capacity(explicit_count.saturating_add(1));
        prefix_extents.push(0);
        for (index, size) in sizes.iter().take(explicit_count).enumerate() {
            let extent = if hidden_indices.contains(&index) {
                0
            } else {
                normalized_size(*size)
            };
            let next = prefix_extents
                .last()
                .copied()
                .unwrap_or_default()
                .saturating_add(extent);
            prefix_extents.push(next);
        }
        Self::Variable {
            prefix_extents,
            explicit_count,
            fallback_size: normalized_size(fallback_size),
            total_count,
        }
    }

    fn total_count(&self) -> usize {
        match self {
            Self::Fixed { total_count, .. } | Self::Variable { total_count, .. } => *total_count,
        }
    }
}

fn variable_metric_offset(
    prefix_extents: &[u32],
    explicit_count: usize,
    fallback_size: u32,
    index: usize,
) -> u32 {
    if index <= explicit_count {
        return prefix_extents.get(index).copied().unwrap_or(u32::MAX);
    }
    let explicit_extent = prefix_extents.last().copied().unwrap_or_default();
    explicit_extent.saturating_add(
        fallback_size.saturating_mul(usize_to_u32(index.saturating_sub(explicit_count))),
    )
}

fn variable_index_at_offset(
    prefix_extents: &[u32],
    explicit_count: usize,
    fallback_size: u32,
    offset: u32,
) -> usize {
    let explicit_extent = prefix_extents.last().copied().unwrap_or_default();
    if offset < explicit_extent {
        return prefix_extents[1..].partition_point(|end| *end <= offset);
    }
    explicit_count.saturating_add(
        usize::try_from(offset.saturating_sub(explicit_extent) / fallback_size)
            .unwrap_or(usize::MAX),
    )
}

const fn normalized_size(size: u32) -> u32 {
    if size == 0 { MIN_TRACK_SIZE } else { size }
}

fn usize_to_u32(value: usize) -> u32 {
    u32::try_from(value).unwrap_or(u32::MAX)
}
