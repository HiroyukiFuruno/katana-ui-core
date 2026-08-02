use super::geometry::{plan_grid_layout, point_in_rect};
use super::selection::{navigate_selection, select_coordinate};
use super::{
    GenericGrid, GridAction, GridCellContent, GridCoordinate, GridEvent, GridHitTest, GridLayout,
    GridSelection, GridTrackSizeProvider, GridViewport, grid_coordinate_in_bounds,
};
use crate::render_model::{UiGridValidationError, UiNodeKind, UiStateId};
use std::collections::HashSet;

impl GenericGrid {
    #[must_use]
    pub fn new(label: &str, row_count: usize, column_count: usize) -> Self {
        Self {
            label: label.to_string(),
            state_id: UiStateId::next_for(UiNodeKind::Grid),
            row_count,
            column_count,
            row_tracks: GridTrackSizeProvider::default(),
            column_tracks: GridTrackSizeProvider::default(),
            viewport: GridViewport::default(),
            row_overscan: 0,
            column_overscan: 0,
            frozen_rows: 0,
            frozen_columns: 0,
            selection: None,
            cell_spans: Vec::new(),
            visible_cells: Vec::new(),
            last_event: GridEvent::None,
        }
    }

    #[must_use]
    pub fn state_id(&self) -> &UiStateId {
        &self.state_id
    }

    #[must_use]
    pub fn row_tracks(mut self, value: GridTrackSizeProvider) -> Self {
        self.row_tracks = value;
        self
    }

    #[must_use]
    pub fn column_tracks(mut self, value: GridTrackSizeProvider) -> Self {
        self.column_tracks = value;
        self
    }

    #[must_use]
    pub const fn viewport(mut self, value: GridViewport) -> Self {
        self.viewport = value;
        self
    }

    #[must_use]
    pub const fn overscan(mut self, rows: usize, columns: usize) -> Self {
        self.row_overscan = rows;
        self.column_overscan = columns;
        self
    }

    #[must_use]
    pub const fn frozen(mut self, rows: usize, columns: usize) -> Self {
        self.frozen_rows = rows;
        self.frozen_columns = columns;
        self
    }

    #[must_use]
    pub fn active_cell(mut self, coordinate: GridCoordinate) -> Self {
        let coordinate = self.span_anchor(coordinate);
        self.selection =
            select_coordinate(None, coordinate, false, self.row_count, self.column_count);
        self
    }

    pub fn with_visible_cells(
        mut self,
        cells: Vec<GridCellContent>,
    ) -> Result<Self, UiGridValidationError> {
        let materialized = self
            .visible_coordinates()
            .into_iter()
            .collect::<HashSet<_>>();
        let mut seen = HashSet::new();
        let mut validated = Vec::new();
        for cell in cells {
            if !grid_coordinate_in_bounds(cell.coordinate, self.row_count, self.column_count) {
                return Err(UiGridValidationError::CellOutsideGrid);
            }
            if !materialized.contains(&cell.coordinate) {
                return Err(UiGridValidationError::CellOutsideMaterializedRange {
                    coordinate: cell.coordinate,
                });
            }
            if !seen.insert(cell.coordinate) {
                return Err(UiGridValidationError::DuplicateCell {
                    coordinate: cell.coordinate,
                });
            }
            validated.push(cell);
        }
        self.visible_cells = validated;
        Ok(self)
    }

    #[must_use]
    pub fn layout(&self) -> GridLayout {
        let (rows, columns) = self.planned_axis_configs();
        plan_grid_layout(&rows, &columns, self.viewport, &self.cell_spans)
    }

    #[must_use]
    pub fn visible_coordinates(&self) -> Vec<GridCoordinate> {
        self.layout()
            .cells
            .into_iter()
            .map(|cell| cell.coordinate)
            .collect()
    }

    #[must_use]
    pub const fn active_coordinate(&self) -> Option<GridCoordinate> {
        match self.selection {
            Some(selection) => Some(selection.active),
            None => None,
        }
    }

    #[must_use]
    pub const fn selection(&self) -> Option<GridSelection> {
        self.selection
    }

    #[must_use]
    pub const fn last_event(&self) -> &GridEvent {
        &self.last_event
    }

    pub fn apply_action(&mut self, action: GridAction) -> GridEvent {
        let event = match action {
            GridAction::Select { coordinate, extend } => {
                let coordinate = self.span_anchor(coordinate);
                self.selection = select_coordinate(
                    self.selection,
                    coordinate,
                    extend,
                    self.row_count,
                    self.column_count,
                );
                self.selection_event()
            }
            GridAction::Navigate { intent, extend } => {
                let previous_active = self.active_coordinate();
                self.selection = navigate_selection(
                    self.selection,
                    intent,
                    extend,
                    self.row_count,
                    self.column_count,
                    self.viewport,
                    self.row_tracks.track_size(0),
                );
                self.normalize_navigation(previous_active, intent, extend);
                self.selection_event()
            }
            GridAction::ScrollTo { x, y } => {
                let before = self.viewport;
                self.viewport.scroll_x = x;
                self.viewport.scroll_y = y;
                let layout = self.layout();
                self.viewport.scroll_x = layout.columns.scroll_offset;
                self.viewport.scroll_y = layout.rows.scroll_offset;
                if self.viewport == before {
                    GridEvent::None
                } else {
                    GridEvent::Scrolled(self.viewport)
                }
            }
            GridAction::ClearSelection => {
                self.selection = None;
                GridEvent::SelectionChanged(None)
            }
        };
        self.last_event = event.clone();
        event
    }

    #[must_use]
    pub fn hit_test(&self, x: i32, y: i32) -> Option<GridHitTest> {
        self.layout()
            .cells
            .into_iter()
            .filter(|cell| point_in_rect(x, y, cell.clipped_bounds))
            .max_by_key(|cell| {
                (
                    u8::from(cell.frozen_row).saturating_add(u8::from(cell.frozen_column)),
                    cell.frozen_row,
                    cell.frozen_column,
                )
            })
            .map(GridHitTest::from)
    }

    fn selection_event(&mut self) -> GridEvent {
        GridEvent::SelectionChanged(self.selection)
    }
}
