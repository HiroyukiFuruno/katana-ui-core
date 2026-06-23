use crate::interaction::{VirtualRange, VirtualizationConfig, VirtualizationPlanner};
use crate::render_model::UiInteractionState;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MoleculeVirtualization;

impl MoleculeVirtualization {
    pub(crate) fn range(
        config: &Option<VirtualizationConfig>,
        total_count: usize,
    ) -> Option<VirtualRange> {
        let mut config = config.clone()?;
        config.total_count = total_count;
        Some(VirtualizationPlanner::compute_visible_range(&config))
    }

    pub(crate) fn interaction(
        base: UiInteractionState,
        range: Option<&VirtualRange>,
    ) -> UiInteractionState {
        let Some(range) = range else {
            return base;
        };
        UiInteractionState {
            item_count: range.rows.len(),
            value: format!("{}..{}/{}", range.start, range.end, range.total_count),
            cursor: range.start,
            selection_start: range.start,
            selection_end: range.end,
            dismiss_reason: format!("aria-setsize={}", range.aria_set_size),
            ..base
        }
    }

    pub(crate) fn slice_by_range<T>(items: Vec<T>, range: Option<&VirtualRange>) -> Vec<T> {
        let Some(range) = range else {
            return items;
        };
        if range.rows.len() >= items.len() {
            return items;
        }
        items
            .into_iter()
            .enumerate()
            .filter_map(|(index, item)| {
                range
                    .rows
                    .iter()
                    .any(|row| row.index == index)
                    .then_some(item)
            })
            .collect()
    }
}
