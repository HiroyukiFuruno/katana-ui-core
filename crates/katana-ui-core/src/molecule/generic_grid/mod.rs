mod axis;
mod axis_metrics;
mod axis_types;
mod component;
mod component_render;
mod component_span;
mod component_types;
mod geometry;
mod selection;

use crate::render_model::UiRect;
pub use crate::render_model::{
    UiGridCellAppearance as GridCellAppearance, UiGridCellSpan as GridCellSpan,
    UiGridCoordinate as GridCoordinate, UiGridDataBar as GridDataBar,
    UiGridHorizontalAlignment as GridHorizontalAlignment, UiGridIcon as GridIcon,
    UiGridIndexRange as GridIndexRange, UiGridRating as GridRating,
    UiGridSelection as GridSelection, UiGridVerticalAlignment as GridVerticalAlignment,
    UiGridViewport as GridViewport,
};
pub use axis_types::{GridAxisConfig, GridAxisPlan, GridAxisPlanner, GridTrackSizeProvider};
pub use component_types::GenericGrid;
pub use geometry::{GridCellLayout, GridLayout};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GridCellContent {
    pub coordinate: GridCoordinate,
    pub text: String,
    pub appearance: GridCellAppearance,
}

impl GridCellContent {
    #[must_use]
    pub fn new(coordinate: GridCoordinate, text: impl Into<String>) -> Self {
        Self {
            coordinate,
            text: text.into(),
            appearance: GridCellAppearance::default(),
        }
    }

    #[must_use]
    pub fn appearance(mut self, value: GridCellAppearance) -> Self {
        self.appearance = value;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GridNavigationIntent {
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GridAction {
    Select {
        coordinate: GridCoordinate,
        extend: bool,
    },
    Navigate {
        intent: GridNavigationIntent,
        extend: bool,
    },
    ScrollTo {
        x: u32,
        y: u32,
    },
    ClearSelection,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GridEvent {
    #[default]
    None,
    SelectionChanged(Option<GridSelection>),
    Scrolled(GridViewport),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct GridHitTest {
    pub coordinate: GridCoordinate,
    pub bounds: UiRect,
    pub frozen_row: bool,
    pub frozen_column: bool,
}

impl From<GridCellLayout> for GridHitTest {
    fn from(value: GridCellLayout) -> Self {
        Self {
            coordinate: value.coordinate,
            bounds: value.bounds,
            frozen_row: value.frozen_row,
            frozen_column: value.frozen_column,
        }
    }
}

pub(super) fn grid_coordinate_in_bounds(
    coordinate: GridCoordinate,
    row_count: usize,
    column_count: usize,
) -> bool {
    coordinate.row < row_count && coordinate.column < column_count
}
