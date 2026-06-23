use super::{MIN_ROW_HEIGHT, RowHeightOverride, RowHeightProvider};

pub(super) fn upsert_override(
    overrides: &mut Vec<RowHeightOverride>,
    measurement: RowHeightOverride,
) {
    if let Some(existing) = overrides
        .iter_mut()
        .find(|it| it.index == measurement.index)
    {
        existing.height = normalized_height(measurement.height);
        return;
    }
    overrides.push(RowHeightOverride {
        index: measurement.index,
        height: normalized_height(measurement.height),
    });
    overrides.sort_by_key(|it| it.index);
}

pub(super) fn cumulative_height(provider: &RowHeightProvider, end: usize) -> u32 {
    (0..end).fold(0_u32, |height, index| {
        height.saturating_add(row_height(provider, index))
    })
}

pub(super) fn row_height(provider: &RowHeightProvider, index: usize) -> u32 {
    match provider {
        RowHeightProvider::Fixed { height } => normalized_height(*height),
        RowHeightProvider::Variable {
            row_heights,
            fallback_height,
        } => normalized_height(row_heights.get(index).copied().unwrap_or(*fallback_height)),
        RowHeightProvider::Estimated {
            estimated_height,
            measured_overrides,
        } => measured_overrides
            .iter()
            .find(|it| it.index == index)
            .map_or(normalized_height(*estimated_height), |it| {
                normalized_height(it.height)
            }),
    }
}

pub(super) fn normalized_height(height: u32) -> u32 {
    height.max(MIN_ROW_HEIGHT)
}
