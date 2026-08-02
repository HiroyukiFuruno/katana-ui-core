use super::{
    GenericGrid, GridAxisConfig, GridCellContent, GridCoordinate, GridTrackSizeProvider,
    GridViewport,
};
use crate::render_model::{
    UiGridCell, UiGridProps, UiGridVisibleRange, UiInteractionState, UiNode, UiNodeKind,
};
use std::collections::HashMap;

impl GenericGrid {
    fn axis_configs(&self) -> (GridAxisConfig, GridAxisConfig) {
        (
            self.axis_config(
                self.row_count,
                self.row_tracks.clone(),
                self.viewport.height,
                self.viewport.scroll_y,
                self.row_overscan,
                self.frozen_rows,
            ),
            self.axis_config(
                self.column_count,
                self.column_tracks.clone(),
                self.viewport.width,
                self.viewport.scroll_x,
                self.column_overscan,
                self.frozen_columns,
            ),
        )
    }

    fn axis_config(
        &self,
        count: usize,
        tracks: GridTrackSizeProvider,
        viewport: u32,
        scroll: u32,
        overscan: usize,
        frozen: usize,
    ) -> GridAxisConfig {
        GridAxisConfig::new(count, tracks, viewport)
            .scroll_offset(scroll)
            .overscan(overscan)
            .frozen_count(frozen)
    }

    pub(super) fn planned_axis_configs(&self) -> (GridAxisConfig, GridAxisConfig) {
        self.axis_configs()
    }

    fn render_props(&self) -> UiGridProps {
        let layout = self.layout();
        let content = self
            .visible_cells
            .iter()
            .map(|cell| (cell.coordinate, cell))
            .collect::<HashMap<_, _>>();
        let cells = layout
            .cells
            .iter()
            .map(|cell| self.render_cell(cell, &content))
            .collect();
        UiGridProps {
            row_count: self.row_count,
            column_count: self.column_count,
            total_width: layout.columns.total_extent,
            total_height: layout.rows.total_extent,
            viewport: GridViewport {
                scroll_x: layout.columns.scroll_offset,
                scroll_y: layout.rows.scroll_offset,
                ..self.viewport
            },
            visible_range: UiGridVisibleRange {
                rows: layout.rows.visible_range,
                columns: layout.columns.visible_range,
                frozen_rows: self.frozen_rows.min(self.row_count),
                frozen_columns: self.frozen_columns.min(self.column_count),
            },
            selection: self.selection,
            active_cell: self.active_coordinate(),
            cells,
        }
    }

    fn render_cell(
        &self,
        cell: &super::GridCellLayout,
        content: &HashMap<GridCoordinate, &GridCellContent>,
    ) -> UiGridCell {
        let value = content.get(&cell.coordinate).copied();
        let span = self
            .cell_spans
            .iter()
            .find(|span| span.anchor == cell.coordinate);
        UiGridCell {
            coordinate: cell.coordinate,
            bounds: cell.bounds,
            clipped_bounds: cell.clipped_bounds,
            text: value.map_or_else(String::new, |value| value.text.clone()),
            appearance: value.map_or_else(Default::default, |value| value.appearance.clone()),
            row_span: span.map_or(1, |span| span.row_span),
            column_span: span.map_or(1, |span| span.column_span),
            selected: self
                .selection
                .is_some_and(|selection| selection.contains(cell.coordinate)),
            active: self.active_coordinate() == Some(cell.coordinate),
            frozen_row: cell.frozen_row,
            frozen_column: cell.frozen_column,
            accessibility_row_index: cell.coordinate.row.saturating_add(1),
            accessibility_column_index: cell.coordinate.column.saturating_add(1),
        }
    }
}

impl From<GenericGrid> for UiNode {
    fn from(mut value: GenericGrid) -> Self {
        let layout = value.layout();
        value.viewport.scroll_x = layout.columns.scroll_offset;
        value.viewport.scroll_y = layout.rows.scroll_offset;
        let interaction = UiInteractionState {
            has_selection: value.selection.is_some(),
            item_count: layout.cells.len(),
            value: format!("{},{}", value.viewport.scroll_x, value.viewport.scroll_y),
            ..UiInteractionState::default()
        };
        let props = value.render_props();
        UiNode::from_state(UiNodeKind::Grid, value.label, value.state_id)
            .grid(props)
            .interaction(interaction)
    }
}
