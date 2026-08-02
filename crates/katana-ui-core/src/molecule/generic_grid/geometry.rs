use super::{
    GridAxisConfig, GridAxisPlan, GridAxisPlanner, GridCellSpan, GridCoordinate,
    GridTrackSizeProvider, GridViewport,
};
use crate::render_model::UiRect;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct GridCellLayout {
    pub coordinate: GridCoordinate,
    pub bounds: UiRect,
    pub clipped_bounds: UiRect,
    pub frozen_row: bool,
    pub frozen_column: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GridLayout {
    pub rows: GridAxisPlan,
    pub columns: GridAxisPlan,
    pub cells: Vec<GridCellLayout>,
}

pub(super) fn plan_grid_layout(
    rows: &GridAxisConfig,
    columns: &GridAxisConfig,
    viewport: GridViewport,
    spans: &[GridCellSpan],
) -> GridLayout {
    let row_plan = GridAxisPlanner::plan(rows);
    let column_plan = GridAxisPlanner::plan(columns);
    let mut cells = Vec::new();
    for row in &row_plan.materialized_indices {
        for column in &column_plan.materialized_indices {
            let coordinate = GridCoordinate::new(*row, *column);
            if span_covering(spans, coordinate).is_some_and(|span| span.anchor != coordinate) {
                continue;
            }
            let span = span_covering(spans, coordinate)
                .copied()
                .unwrap_or_default();
            cells.push(cell_layout(
                *row,
                *column,
                span.row_span,
                span.column_span,
                rows,
                columns,
                &row_plan,
                &column_plan,
                viewport,
            ));
        }
    }
    GridLayout {
        rows: row_plan,
        columns: column_plan,
        cells,
    }
}

fn cell_layout(
    row: usize,
    column: usize,
    row_span: usize,
    column_span: usize,
    rows: &GridAxisConfig,
    columns: &GridAxisConfig,
    row_plan: &GridAxisPlan,
    column_plan: &GridAxisPlan,
    viewport: GridViewport,
) -> GridCellLayout {
    let frozen_row = row < rows.frozen_count.min(rows.total_count);
    let frozen_column = column < columns.frozen_count.min(columns.total_count);
    let row_geometry = track_geometry(
        row,
        row_span,
        rows.total_count,
        &rows.track_sizes,
        row_plan,
        viewport.height,
        frozen_row,
    );
    let column_geometry = track_geometry(
        column,
        column_span,
        columns.total_count,
        &columns.track_sizes,
        column_plan,
        viewport.width,
        frozen_column,
    );
    let bounds = UiRect::new(
        i64_to_i32(column_geometry.position),
        i64_to_i32(row_geometry.position),
        column_geometry.size,
        row_geometry.size,
    );
    let clipped_bounds = clip_rect(
        bounds,
        column_geometry.clip_start,
        column_geometry.clip_end,
        row_geometry.clip_start,
        row_geometry.clip_end,
    );
    GridCellLayout {
        coordinate: GridCoordinate::new(row, column),
        bounds,
        clipped_bounds,
        frozen_row,
        frozen_column,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TrackGeometry {
    position: i64,
    size: u32,
    clip_start: u32,
    clip_end: u32,
}

fn track_geometry(
    index: usize,
    span: usize,
    total_count: usize,
    sizes: &GridTrackSizeProvider,
    plan: &GridAxisPlan,
    viewport_extent: u32,
    frozen: bool,
) -> TrackGeometry {
    let size = span_size(sizes, index, span, total_count);
    let frozen_viewport_extent = plan.frozen_extent.min(viewport_extent);
    if frozen {
        return TrackGeometry {
            position: i64::from(sizes.track_offset(index)),
            size,
            clip_start: 0,
            clip_end: frozen_viewport_extent,
        };
    }
    let scrollable_offset = sizes.track_offset(index).saturating_sub(plan.frozen_extent);
    TrackGeometry {
        position: i64::from(frozen_viewport_extent)
            .saturating_add(i64::from(scrollable_offset))
            .saturating_sub(i64::from(plan.scroll_offset)),
        size,
        clip_start: frozen_viewport_extent,
        clip_end: viewport_extent,
    }
}

fn span_size(sizes: &GridTrackSizeProvider, index: usize, span: usize, total_count: usize) -> u32 {
    let count = span.min(total_count.saturating_sub(index));
    (index..index.saturating_add(count)).fold(0_u32, |extent, track| {
        extent.saturating_add(sizes.track_size(track))
    })
}

fn span_covering(spans: &[GridCellSpan], coordinate: GridCoordinate) -> Option<&GridCellSpan> {
    spans.iter().find(|span| {
        let row_end = span.anchor.row.saturating_add(span.row_span);
        let column_end = span.anchor.column.saturating_add(span.column_span);
        span.anchor.row <= coordinate.row
            && coordinate.row < row_end
            && span.anchor.column <= coordinate.column
            && coordinate.column < column_end
    })
}

fn clip_rect(
    bounds: UiRect,
    clip_left: u32,
    clip_right: u32,
    clip_top: u32,
    clip_bottom: u32,
) -> UiRect {
    let left = i64::from(bounds.x).max(i64::from(clip_left));
    let top = i64::from(bounds.y).max(i64::from(clip_top));
    let right = i64::from(bounds.x)
        .saturating_add(i64::from(bounds.width))
        .min(i64::from(clip_right));
    let bottom = i64::from(bounds.y)
        .saturating_add(i64::from(bounds.height))
        .min(i64::from(clip_bottom));
    if right <= left || bottom <= top {
        return UiRect::new(i64_to_i32(left), i64_to_i32(top), 0, 0);
    }
    UiRect::new(
        i64_to_i32(left),
        i64_to_i32(top),
        i64_to_u32(right.saturating_sub(left)),
        i64_to_u32(bottom.saturating_sub(top)),
    )
}

pub(super) fn point_in_rect(x: i32, y: i32, rect: UiRect) -> bool {
    let x = i64::from(x);
    let y = i64::from(y);
    let left = i64::from(rect.x);
    let top = i64::from(rect.y);
    let right = left.saturating_add(i64::from(rect.width));
    let bottom = top.saturating_add(i64::from(rect.height));
    rect.width > 0 && rect.height > 0 && left <= x && x < right && top <= y && y < bottom
}

fn i64_to_i32(value: i64) -> i32 {
    value.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32
}

fn i64_to_u32(value: i64) -> u32 {
    value as u32
}
