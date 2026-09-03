use super::super::UiRect;
use super::typed_grid_border::UiGridCellBorders;
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
    #[serde(default)]
    pub borders: UiGridCellBorders,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiGridProps {
    pub row_count: usize,
    pub column_count: usize,
    pub total_width: u32,
    pub total_height: u32,
    pub viewport: UiGridViewport,
    pub visible_range: UiGridVisibleRange,
    pub selection: Option<UiGridSelection>,
    pub active_cell: Option<UiGridCoordinate>,
    #[serde(default = "default_show_grid_lines")]
    pub show_grid_lines: bool,
    pub cells: Vec<UiGridCell>,
}

impl Default for UiGridProps {
    fn default() -> Self {
        Self {
            row_count: 0,
            column_count: 0,
            total_width: 0,
            total_height: 0,
            viewport: UiGridViewport::default(),
            visible_range: UiGridVisibleRange::default(),
            selection: None,
            active_cell: None,
            show_grid_lines: true,
            cells: Vec::new(),
        }
    }
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

const fn default_show_grid_lines() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::super::typed_grid_border::{UiGridBorderLineStyle, UiGridBorderSide};
    use super::{UiGridCellAppearance, UiGridCellBorders};
    use crate::test_assert::KucTestExpect;

    #[test]
    fn legacy_grid_cell_appearance_deserializes_with_empty_borders() {
        let mut legacy = serde_json::to_value(UiGridCellAppearance::default())
            .kuc_expect("grid cell appearance must serialize");
        legacy
            .as_object_mut()
            .kuc_expect("serialized grid cell appearance must be an object")
            .remove("borders");
        let appearance: UiGridCellAppearance =
            serde_json::from_value(legacy).kuc_expect("legacy cell appearance must deserialize");

        assert_eq!(UiGridCellBorders::default(), appearance.borders);
    }

    #[test]
    fn grid_cell_appearance_retains_independent_border_sides() {
        let borders = UiGridCellBorders {
            left: UiGridBorderSide::solid("#B7C4CE"),
            right: UiGridBorderSide {
                line_style: UiGridBorderLineStyle::Double,
                color: Some("#113355".to_owned()),
            },
            top: UiGridBorderSide {
                line_style: UiGridBorderLineStyle::Dotted,
                color: Some("#AA5500".to_owned()),
            },
            bottom: UiGridBorderSide::default(),
        };
        let appearance = UiGridCellAppearance {
            borders: borders.clone(),
            ..UiGridCellAppearance::default()
        };

        assert!(appearance.borders.left.is_visible());
        assert_eq!(
            UiGridBorderLineStyle::Double,
            appearance.borders.right.line_style
        );
        assert_eq!(Some("#AA5500"), appearance.borders.top.color.as_deref());
        assert!(!appearance.borders.bottom.is_visible());
        assert_eq!(borders, appearance.borders);
    }
}
