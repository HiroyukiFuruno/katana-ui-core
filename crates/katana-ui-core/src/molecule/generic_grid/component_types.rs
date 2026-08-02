use super::{
    GridCellContent, GridCellSpan, GridEvent, GridSelection, GridTrackSizeProvider, GridViewport,
};
use crate::render_model::UiStateId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GenericGrid {
    pub(super) label: String,
    pub(super) state_id: UiStateId,
    pub(super) row_count: usize,
    pub(super) column_count: usize,
    pub(super) row_tracks: GridTrackSizeProvider,
    pub(super) column_tracks: GridTrackSizeProvider,
    pub(super) viewport: GridViewport,
    pub(super) row_overscan: usize,
    pub(super) column_overscan: usize,
    pub(super) frozen_rows: usize,
    pub(super) frozen_columns: usize,
    #[serde(default = "default_show_grid_lines")]
    pub(super) show_grid_lines: bool,
    pub(super) selection: Option<GridSelection>,
    #[serde(default)]
    pub(super) cell_spans: Vec<GridCellSpan>,
    pub(super) visible_cells: Vec<GridCellContent>,
    pub(super) last_event: GridEvent,
}

const fn default_show_grid_lines() -> bool {
    true
}
