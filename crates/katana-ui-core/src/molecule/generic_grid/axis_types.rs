use super::GridIndexRange;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GridTrackSizeProvider {
    Fixed {
        size: u32,
    },
    Variable {
        sizes: Vec<u32>,
        fallback_size: u32,
    },
    VariableWithHidden {
        sizes: Vec<u32>,
        fallback_size: u32,
        hidden_indices: Vec<usize>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GridAxisConfig {
    pub total_count: usize,
    pub track_sizes: GridTrackSizeProvider,
    pub viewport_extent: u32,
    pub scroll_offset: u32,
    pub overscan: usize,
    pub frozen_count: usize,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GridAxisPlan {
    pub total_extent: u32,
    pub frozen_extent: u32,
    pub viewport_extent: u32,
    pub scroll_offset: u32,
    pub visible_range: GridIndexRange,
    pub materialized_indices: Vec<usize>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct GridAxisPlanner;
