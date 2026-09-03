pub use super::typed_grid_border::{UiGridBorderLineStyle, UiGridBorderSide, UiGridCellBorders};
pub use super::typed_grid_types::{
    UiGridCell, UiGridCellAppearance, UiGridCellSpan, UiGridCoordinate, UiGridDataBar,
    UiGridHorizontalAlignment, UiGridIcon, UiGridIndexRange, UiGridProps, UiGridRating,
    UiGridSelection, UiGridValidationError, UiGridVerticalAlignment, UiGridViewport,
    UiGridVisibleRange,
};
use std::collections::HashSet;
use std::fmt;

impl UiGridCoordinate {
    #[must_use]
    pub const fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }
}

impl UiGridCellSpan {
    #[must_use]
    pub const fn new(anchor: UiGridCoordinate, row_span: usize, column_span: usize) -> Self {
        Self {
            anchor,
            row_span,
            column_span,
        }
    }
}

impl UiGridIndexRange {
    #[must_use]
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    #[must_use]
    pub const fn contains(self, index: usize) -> bool {
        self.start <= index && index < self.end
    }

    #[must_use]
    pub const fn len(self) -> usize {
        self.end.saturating_sub(self.start)
    }

    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.start >= self.end
    }
}

impl UiGridViewport {
    #[must_use]
    pub const fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            scroll_x: 0,
            scroll_y: 0,
        }
    }

    #[must_use]
    pub const fn scroll(mut self, scroll_x: u32, scroll_y: u32) -> Self {
        self.scroll_x = scroll_x;
        self.scroll_y = scroll_y;
        self
    }
}

impl UiGridSelection {
    #[must_use]
    pub const fn new(anchor: UiGridCoordinate, active: UiGridCoordinate) -> Self {
        Self {
            anchor,
            active,
            start: UiGridCoordinate::new(
                if anchor.row < active.row {
                    anchor.row
                } else {
                    active.row
                },
                if anchor.column < active.column {
                    anchor.column
                } else {
                    active.column
                },
            ),
            end: UiGridCoordinate::new(
                if anchor.row > active.row {
                    anchor.row
                } else {
                    active.row
                },
                if anchor.column > active.column {
                    anchor.column
                } else {
                    active.column
                },
            ),
        }
    }

    #[must_use]
    pub const fn contains(self, coordinate: UiGridCoordinate) -> bool {
        self.start.row <= coordinate.row
            && coordinate.row <= self.end.row
            && self.start.column <= coordinate.column
            && coordinate.column <= self.end.column
    }
}

impl UiGridProps {
    pub fn validate(&self) -> Result<(), UiGridValidationError> {
        self.validate_ranges()?;
        if let Some(active) = self.active_cell {
            self.validate_coordinate(active, UiGridValidationError::ActiveCellOutsideGrid)?;
        }
        if let Some(selection) = self.selection {
            for coordinate in [
                selection.anchor,
                selection.active,
                selection.start,
                selection.end,
            ] {
                self.validate_coordinate(coordinate, UiGridValidationError::SelectionOutsideGrid)?;
            }
        }
        self.validate_cells()
    }

    fn validate_ranges(&self) -> Result<(), UiGridValidationError> {
        let range = self.visible_range;
        if range.rows.start > range.rows.end
            || range.rows.end > self.row_count
            || range.columns.start > range.columns.end
            || range.columns.end > self.column_count
            || range.frozen_rows > self.row_count
            || range.frozen_columns > self.column_count
        {
            return Err(UiGridValidationError::VisibleRangeOutsideGrid);
        }
        Ok(())
    }

    fn validate_cells(&self) -> Result<(), UiGridValidationError> {
        self.validate_cell_spans()?;
        let mut seen = HashSet::with_capacity(self.cells.len());
        for cell in &self.cells {
            if !self.coordinate_is_materialized(cell.coordinate) {
                return Err(UiGridValidationError::CellOutsideMaterializedRange {
                    coordinate: cell.coordinate,
                });
            }
            if !seen.insert(cell.coordinate) {
                return Err(UiGridValidationError::DuplicateCell {
                    coordinate: cell.coordinate,
                });
            }
            let expected_row = cell.coordinate.row.saturating_add(1);
            let expected_column = cell.coordinate.column.saturating_add(1);
            if cell.accessibility_row_index != expected_row
                || cell.accessibility_column_index != expected_column
            {
                return Err(UiGridValidationError::AccessibilityIndexMismatch {
                    coordinate: cell.coordinate,
                });
            }
        }
        Ok(())
    }

    fn validate_cell_spans(&self) -> Result<(), UiGridValidationError> {
        for (index, cell) in self.cells.iter().enumerate() {
            self.validate_coordinate(cell.coordinate, UiGridValidationError::CellOutsideGrid)?;
            self.validate_cell_span(cell)?;
            if let Some(overlapping) = self.cells[..index]
                .iter()
                .find(|other| other.coordinate != cell.coordinate && cells_overlap(cell, other))
            {
                return Err(UiGridValidationError::OverlappingCellSpans {
                    first: overlapping.coordinate,
                    second: cell.coordinate,
                });
            }
        }
        Ok(())
    }

    fn validate_cell_span(&self, cell: &UiGridCell) -> Result<(), UiGridValidationError> {
        let row_end = cell.coordinate.row.saturating_add(cell.row_span);
        let column_end = cell.coordinate.column.saturating_add(cell.column_span);
        if cell.row_span == 0
            || cell.column_span == 0
            || row_end > self.row_count
            || column_end > self.column_count
        {
            return Err(UiGridValidationError::InvalidCellSpan {
                anchor: cell.coordinate,
            });
        }
        if crosses_boundary(cell.coordinate.row, row_end, self.visible_range.frozen_rows)
            || crosses_boundary(
                cell.coordinate.column,
                column_end,
                self.visible_range.frozen_columns,
            )
        {
            return Err(UiGridValidationError::CellSpanCrossesFrozenBoundary {
                anchor: cell.coordinate,
            });
        }
        Ok(())
    }

    fn validate_coordinate(
        &self,
        coordinate: UiGridCoordinate,
        error: UiGridValidationError,
    ) -> Result<(), UiGridValidationError> {
        if coordinate.row >= self.row_count || coordinate.column >= self.column_count {
            return Err(error);
        }
        Ok(())
    }

    fn coordinate_is_materialized(&self, coordinate: UiGridCoordinate) -> bool {
        let range = self.visible_range;
        let row_allowed = coordinate.row < range.frozen_rows || range.rows.contains(coordinate.row);
        let column_allowed =
            coordinate.column < range.frozen_columns || range.columns.contains(coordinate.column);
        row_allowed && column_allowed
    }
}

fn cells_overlap(first: &UiGridCell, second: &UiGridCell) -> bool {
    let first_row_end = first.coordinate.row.saturating_add(first.row_span);
    let second_row_end = second.coordinate.row.saturating_add(second.row_span);
    let first_column_end = first.coordinate.column.saturating_add(first.column_span);
    let second_column_end = second.coordinate.column.saturating_add(second.column_span);
    first.coordinate.row < second_row_end
        && second.coordinate.row < first_row_end
        && first.coordinate.column < second_column_end
        && second.coordinate.column < first_column_end
}

const fn crosses_boundary(start: usize, end: usize, boundary: usize) -> bool {
    start < boundary && boundary < end
}

impl fmt::Display for UiGridValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::VisibleRangeOutsideGrid => write!(formatter, "grid visible range is invalid"),
            Self::ActiveCellOutsideGrid => {
                write!(formatter, "grid active cell is outside the grid")
            }
            Self::SelectionOutsideGrid => write!(formatter, "grid selection is outside the grid"),
            Self::CellOutsideGrid => write!(formatter, "grid cell is outside the grid"),
            Self::CellOutsideMaterializedRange { coordinate } => write!(
                formatter,
                "grid cell {},{} is outside the materialized range",
                coordinate.row, coordinate.column
            ),
            Self::DuplicateCell { coordinate } => write!(
                formatter,
                "grid cell {},{} is duplicated",
                coordinate.row, coordinate.column
            ),
            Self::AccessibilityIndexMismatch { coordinate } => write!(
                formatter,
                "grid cell {},{} has invalid accessibility indexes",
                coordinate.row, coordinate.column
            ),
            Self::InvalidCellSpan { anchor } => write!(
                formatter,
                "grid cell {},{} has an invalid span",
                anchor.row, anchor.column
            ),
            Self::OverlappingCellSpans { first, second } => write!(
                formatter,
                "grid spans {},{} and {},{} overlap",
                first.row, first.column, second.row, second.column
            ),
            Self::CellSpanCrossesFrozenBoundary { anchor } => write!(
                formatter,
                "grid span {},{} crosses a frozen boundary",
                anchor.row, anchor.column
            ),
        }
    }
}

impl std::error::Error for UiGridValidationError {}
