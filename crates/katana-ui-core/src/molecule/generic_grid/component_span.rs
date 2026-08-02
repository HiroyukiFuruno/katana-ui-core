use super::{
    GenericGrid, GridCellSpan, GridCoordinate, GridNavigationIntent, GridSelection,
    GridTrackSizeProvider,
};
use crate::render_model::UiGridValidationError;

impl GenericGrid {
    pub fn with_cell_spans(
        mut self,
        spans: Vec<GridCellSpan>,
    ) -> Result<Self, UiGridValidationError> {
        self.validate_spans(&spans)?;
        self.cell_spans = spans;
        if let Some(selection) = self.selection {
            let active = self.span_anchor(selection.active);
            let anchor = self.span_anchor(selection.anchor);
            self.selection = Some(GridSelection::new(anchor, active));
        }
        Ok(self)
    }

    pub(super) fn normalize_navigation(
        &mut self,
        previous: Option<GridCoordinate>,
        intent: GridNavigationIntent,
        extend: bool,
    ) {
        let Some(selection) = self.selection else {
            return;
        };
        let active = self.navigation_coordinate(previous, selection.active, intent);
        let active = self.span_anchor(active);
        let anchor = if extend {
            self.span_anchor(selection.anchor)
        } else {
            active
        };
        self.selection = Some(GridSelection::new(anchor, active));
    }

    pub(super) fn span_anchor(&self, coordinate: GridCoordinate) -> GridCoordinate {
        self.span_at(coordinate)
            .map_or(coordinate, |span| span.anchor)
    }

    fn navigation_coordinate(
        &self,
        previous: Option<GridCoordinate>,
        candidate: GridCoordinate,
        intent: GridNavigationIntent,
    ) -> GridCoordinate {
        let previous = previous.unwrap_or(candidate);
        let candidate = self.skip_current_span(previous, candidate, intent);
        self.skip_hidden_tracks(previous, candidate, intent)
    }

    fn skip_current_span(
        &self,
        previous: GridCoordinate,
        candidate: GridCoordinate,
        intent: GridNavigationIntent,
    ) -> GridCoordinate {
        let Some(span) = self.span_at(previous) else {
            return candidate;
        };
        if !span_contains(span, candidate) {
            return candidate;
        }
        match intent {
            GridNavigationIntent::Right => GridCoordinate::new(
                candidate.row,
                span.anchor
                    .column
                    .saturating_add(span.column_span)
                    .min(self.column_count.saturating_sub(1)),
            ),
            GridNavigationIntent::Down | GridNavigationIntent::PageDown => GridCoordinate::new(
                span.anchor
                    .row
                    .saturating_add(span.row_span)
                    .min(self.row_count.saturating_sub(1)),
                candidate.column,
            ),
            _ => candidate,
        }
    }

    fn skip_hidden_tracks(
        &self,
        previous: GridCoordinate,
        candidate: GridCoordinate,
        intent: GridNavigationIntent,
    ) -> GridCoordinate {
        match intent {
            GridNavigationIntent::Left => GridCoordinate::new(
                candidate.row,
                visible_backward(&self.column_tracks, candidate.column).unwrap_or(previous.column),
            ),
            GridNavigationIntent::Right => GridCoordinate::new(
                candidate.row,
                visible_forward(&self.column_tracks, candidate.column, self.column_count)
                    .unwrap_or(previous.column),
            ),
            GridNavigationIntent::Home => GridCoordinate::new(
                candidate.row,
                visible_forward(&self.column_tracks, 0, self.column_count)
                    .unwrap_or(previous.column),
            ),
            GridNavigationIntent::End => GridCoordinate::new(
                candidate.row,
                visible_backward(&self.column_tracks, self.column_count.saturating_sub(1))
                    .unwrap_or(previous.column),
            ),
            GridNavigationIntent::Up | GridNavigationIntent::PageUp => GridCoordinate::new(
                visible_backward(&self.row_tracks, candidate.row).unwrap_or(previous.row),
                candidate.column,
            ),
            GridNavigationIntent::Down | GridNavigationIntent::PageDown => GridCoordinate::new(
                visible_forward(&self.row_tracks, candidate.row, self.row_count)
                    .unwrap_or(previous.row),
                candidate.column,
            ),
        }
    }

    fn span_at(&self, coordinate: GridCoordinate) -> Option<&GridCellSpan> {
        self.cell_spans
            .iter()
            .find(|span| span_contains(span, coordinate))
    }

    fn validate_spans(&self, spans: &[GridCellSpan]) -> Result<(), UiGridValidationError> {
        for (index, span) in spans.iter().enumerate() {
            self.validate_span(span)?;
            if let Some(overlapping) = spans[..index]
                .iter()
                .find(|other| spans_overlap(span, other))
            {
                return Err(UiGridValidationError::OverlappingCellSpans {
                    first: overlapping.anchor,
                    second: span.anchor,
                });
            }
        }
        Ok(())
    }

    fn validate_span(&self, span: &GridCellSpan) -> Result<(), UiGridValidationError> {
        let row_end = span.anchor.row.saturating_add(span.row_span);
        let column_end = span.anchor.column.saturating_add(span.column_span);
        if span.row_span == 0
            || span.column_span == 0
            || row_end > self.row_count
            || column_end > self.column_count
        {
            return Err(UiGridValidationError::InvalidCellSpan {
                anchor: span.anchor,
            });
        }
        if crosses_boundary(span.anchor.row, row_end, self.frozen_rows)
            || crosses_boundary(span.anchor.column, column_end, self.frozen_columns)
        {
            return Err(UiGridValidationError::CellSpanCrossesFrozenBoundary {
                anchor: span.anchor,
            });
        }
        Ok(())
    }
}

fn visible_forward(tracks: &GridTrackSizeProvider, start: usize, count: usize) -> Option<usize> {
    (start..count).find(|index| !tracks.is_hidden(*index))
}

fn visible_backward(tracks: &GridTrackSizeProvider, start: usize) -> Option<usize> {
    (0..=start).rev().find(|index| !tracks.is_hidden(*index))
}

fn span_contains(span: &GridCellSpan, coordinate: GridCoordinate) -> bool {
    let row_end = span.anchor.row.saturating_add(span.row_span);
    let column_end = span.anchor.column.saturating_add(span.column_span);
    span.anchor.row <= coordinate.row
        && coordinate.row < row_end
        && span.anchor.column <= coordinate.column
        && coordinate.column < column_end
}

fn spans_overlap(first: &GridCellSpan, second: &GridCellSpan) -> bool {
    let first_row_end = first.anchor.row.saturating_add(first.row_span);
    let second_row_end = second.anchor.row.saturating_add(second.row_span);
    let first_column_end = first.anchor.column.saturating_add(first.column_span);
    let second_column_end = second.anchor.column.saturating_add(second.column_span);
    first.anchor.row < second_row_end
        && second.anchor.row < first_row_end
        && first.anchor.column < second_column_end
        && second.anchor.column < first_column_end
}

const fn crosses_boundary(start: usize, end: usize, boundary: usize) -> bool {
    start < boundary && boundary < end
}
