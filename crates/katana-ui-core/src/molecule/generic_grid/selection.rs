use super::{GridCoordinate, GridNavigationIntent, GridSelection, GridViewport};

pub(super) fn normalized_coordinate(
    coordinate: GridCoordinate,
    row_count: usize,
    column_count: usize,
) -> Option<GridCoordinate> {
    if row_count == 0 || column_count == 0 {
        return None;
    }
    Some(GridCoordinate::new(
        coordinate.row.min(row_count.saturating_sub(1)),
        coordinate.column.min(column_count.saturating_sub(1)),
    ))
}

pub(super) fn select_coordinate(
    current: Option<GridSelection>,
    coordinate: GridCoordinate,
    extend: bool,
    row_count: usize,
    column_count: usize,
) -> Option<GridSelection> {
    let active = normalized_coordinate(coordinate, row_count, column_count)?;
    let anchor = if extend {
        current.map_or(active, |selection| selection.anchor)
    } else {
        active
    };
    Some(GridSelection::new(anchor, active))
}

pub(super) fn navigate_selection(
    current: Option<GridSelection>,
    intent: GridNavigationIntent,
    extend: bool,
    row_count: usize,
    column_count: usize,
    viewport: GridViewport,
    row_size: u32,
) -> Option<GridSelection> {
    if row_count == 0 || column_count == 0 {
        return None;
    }
    let active = current.map_or(GridCoordinate::new(0, 0), |selection| selection.active);
    let page_rows = page_track_count(viewport.height, row_size);
    let next = match intent {
        GridNavigationIntent::Left => {
            GridCoordinate::new(active.row, active.column.saturating_sub(1))
        }
        GridNavigationIntent::Right => GridCoordinate::new(
            active.row,
            active
                .column
                .saturating_add(1)
                .min(column_count.saturating_sub(1)),
        ),
        GridNavigationIntent::Up => {
            GridCoordinate::new(active.row.saturating_sub(1), active.column)
        }
        GridNavigationIntent::Down => GridCoordinate::new(
            active
                .row
                .saturating_add(1)
                .min(row_count.saturating_sub(1)),
            active.column,
        ),
        GridNavigationIntent::Home => GridCoordinate::new(active.row, 0),
        GridNavigationIntent::End => {
            GridCoordinate::new(active.row, column_count.saturating_sub(1))
        }
        GridNavigationIntent::PageUp => {
            GridCoordinate::new(active.row.saturating_sub(page_rows), active.column)
        }
        GridNavigationIntent::PageDown => GridCoordinate::new(
            active
                .row
                .saturating_add(page_rows)
                .min(row_count.saturating_sub(1)),
            active.column,
        ),
    };
    select_coordinate(current, next, extend, row_count, column_count)
}

fn page_track_count(viewport_extent: u32, track_size: u32) -> usize {
    let normalized_size = track_size.max(1);
    usize::try_from(viewport_extent / normalized_size)
        .unwrap_or(usize::MAX)
        .max(1)
}
