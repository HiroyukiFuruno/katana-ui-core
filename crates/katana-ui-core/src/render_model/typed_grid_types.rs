use super::super::UiRect;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UiGridCoordinate {
    pub row: usize,
    pub column: usize,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiGridIndexRange {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiGridViewport {
    pub width: u32,
    pub height: u32,
    pub scroll_x: u32,
    pub scroll_y: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiGridSelection {
    pub anchor: UiGridCoordinate,
    pub active: UiGridCoordinate,
    pub start: UiGridCoordinate,
    pub end: UiGridCoordinate,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiGridVisibleRange {
    pub rows: UiGridIndexRange,
    pub columns: UiGridIndexRange,
    pub frozen_rows: usize,
    pub frozen_columns: usize,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiGridHorizontalAlignment {
    #[default]
    General,
    Left,
    Center,
    Right,
    Fill,
    Justify,
    Distributed,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiGridVerticalAlignment {
    #[default]
    Bottom,
    Center,
    Top,
    Justify,
    Distributed,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiGridDataBar {
    pub positive_color: Option<String>,
    pub negative_color: Option<String>,
    pub fill_ratio_basis_points: u16,
    pub axis_ratio_basis_points: u16,
    pub gradient: bool,
    pub show_value: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiGridIcon {
    pub name: String,
    pub color: Option<String>,
    pub show_value: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiGridRating {
    pub icon_name: String,
    pub count: u32,
    pub maximum: u32,
    pub color: Option<String>,
    pub show_value: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiGridCellAppearance {
    pub font_family: String,
    pub font_size_px: u16,
    pub text_color: Option<String>,
    pub fill_color: Option<String>,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strike: bool,
    pub horizontal_alignment: UiGridHorizontalAlignment,
    pub vertical_alignment: UiGridVerticalAlignment,
    pub wrap_text: bool,
    pub data_bar: Option<UiGridDataBar>,
    pub icon: Option<UiGridIcon>,
    pub rating: Option<UiGridRating>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiGridCellSpan {
    pub anchor: UiGridCoordinate,
    pub row_span: usize,
    pub column_span: usize,
}

impl Default for UiGridCellSpan {
    fn default() -> Self {
        Self {
            anchor: UiGridCoordinate::default(),
            row_span: 1,
            column_span: 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiGridCell {
    pub coordinate: UiGridCoordinate,
    pub bounds: UiRect,
    pub clipped_bounds: UiRect,
    pub text: String,
    #[serde(default)]
    pub appearance: UiGridCellAppearance,
    #[serde(default = "default_grid_span")]
    pub row_span: usize,
    #[serde(default = "default_grid_span")]
    pub column_span: usize,
    pub selected: bool,
    pub active: bool,
    pub frozen_row: bool,
    pub frozen_column: bool,
    pub accessibility_row_index: usize,
    pub accessibility_column_index: usize,
}

impl Default for UiGridCell {
    fn default() -> Self {
        Self {
            coordinate: UiGridCoordinate::default(),
            bounds: UiRect::default(),
            clipped_bounds: UiRect::default(),
            text: String::new(),
            appearance: UiGridCellAppearance::default(),
            row_span: 1,
            column_span: 1,
            selected: false,
            active: false,
            frozen_row: false,
            frozen_column: false,
            accessibility_row_index: 0,
            accessibility_column_index: 0,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiGridProps {
    pub row_count: usize,
    pub column_count: usize,
    pub total_width: u32,
    pub total_height: u32,
    pub viewport: UiGridViewport,
    pub visible_range: UiGridVisibleRange,
    pub selection: Option<UiGridSelection>,
    pub active_cell: Option<UiGridCoordinate>,
    pub cells: Vec<UiGridCell>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiGridValidationError {
    VisibleRangeOutsideGrid,
    ActiveCellOutsideGrid,
    SelectionOutsideGrid,
    CellOutsideGrid,
    CellOutsideMaterializedRange {
        coordinate: UiGridCoordinate,
    },
    DuplicateCell {
        coordinate: UiGridCoordinate,
    },
    AccessibilityIndexMismatch {
        coordinate: UiGridCoordinate,
    },
    InvalidCellSpan {
        anchor: UiGridCoordinate,
    },
    OverlappingCellSpans {
        first: UiGridCoordinate,
        second: UiGridCoordinate,
    },
    CellSpanCrossesFrozenBoundary {
        anchor: UiGridCoordinate,
    },
}

const fn default_grid_span() -> usize {
    1
}
