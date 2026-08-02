use katana_ui_core::render_model::{
    UiGridCell, UiGridCoordinate, UiGridIndexRange, UiGridProps, UiGridSelection,
    UiGridValidationError, UiGridVisibleRange, UiRect,
};

fn cell(row: usize, column: usize) -> UiGridCell {
    UiGridCell {
        coordinate: UiGridCoordinate::new(row, column),
        bounds: UiRect::new(0, 0, 10, 10),
        clipped_bounds: UiRect::new(0, 0, 10, 10),
        text: "value".to_string(),
        appearance: Default::default(),
        row_span: 1,
        column_span: 1,
        selected: false,
        active: false,
        frozen_row: row == 0,
        frozen_column: column == 0,
        accessibility_row_index: row.saturating_add(1),
        accessibility_column_index: column.saturating_add(1),
    }
}

fn valid_props() -> UiGridProps {
    UiGridProps {
        row_count: 3,
        column_count: 3,
        visible_range: UiGridVisibleRange {
            rows: UiGridIndexRange::new(1, 3),
            columns: UiGridIndexRange::new(1, 3),
            frozen_rows: 1,
            frozen_columns: 1,
        },
        active_cell: Some(UiGridCoordinate::new(1, 1)),
        selection: Some(UiGridSelection::new(
            UiGridCoordinate::new(1, 1),
            UiGridCoordinate::new(2, 2),
        )),
        cells: vec![cell(0, 0), cell(1, 1)],
        ..UiGridProps::default()
    }
}

#[test]
fn grid_range_and_selection_helpers_keep_half_open_and_inclusive_semantics() {
    let range = UiGridIndexRange::new(2, 5);
    let empty = UiGridIndexRange::new(4, 4);
    let forward = UiGridSelection::new(UiGridCoordinate::new(3, 1), UiGridCoordinate::new(1, 4));
    let reverse = UiGridSelection::new(UiGridCoordinate::new(1, 4), UiGridCoordinate::new(3, 1));

    assert!(range.contains(2));
    assert!(!range.contains(5));
    assert_eq!(3, range.len());
    assert!(!range.is_empty());
    assert!(empty.is_empty());
    assert_eq!(UiGridCoordinate::new(1, 1), forward.start);
    assert_eq!(UiGridCoordinate::new(3, 4), forward.end);
    assert_eq!(forward.start, reverse.start);
    assert_eq!(forward.end, reverse.end);
    assert!(forward.contains(UiGridCoordinate::new(2, 3)));
    assert!(!forward.contains(UiGridCoordinate::new(0, 3)));
}

#[test]
fn valid_grid_props_accept_frozen_and_scrolled_cells() {
    assert_eq!(Ok(()), valid_props().validate());

    let mut without_active_or_selection = valid_props();
    without_active_or_selection.active_cell = None;
    without_active_or_selection.selection = None;
    assert_eq!(Ok(()), without_active_or_selection.validate());
}

#[test]
fn grid_props_reject_invalid_ranges_active_and_selection() {
    let mut range = valid_props();
    range.visible_range.rows = UiGridIndexRange::new(3, 2);
    assert_eq!(
        Err(UiGridValidationError::VisibleRangeOutsideGrid),
        range.validate()
    );

    let mut active = valid_props();
    active.active_cell = Some(UiGridCoordinate::new(3, 0));
    assert_eq!(
        Err(UiGridValidationError::ActiveCellOutsideGrid),
        active.validate()
    );

    let mut selection = valid_props();
    selection.selection = Some(UiGridSelection::new(
        UiGridCoordinate::new(0, 0),
        UiGridCoordinate::new(0, 3),
    ));
    assert_eq!(
        Err(UiGridValidationError::SelectionOutsideGrid),
        selection.validate()
    );
}

#[test]
fn grid_props_reject_invalid_cell_coordinates_and_metadata() {
    let mut outside_grid = valid_props();
    outside_grid.cells = vec![cell(3, 0)];
    assert_eq!(
        Err(UiGridValidationError::CellOutsideGrid),
        outside_grid.validate()
    );

    let mut outside_range = valid_props();
    outside_range.visible_range.rows = UiGridIndexRange::new(2, 3);
    outside_range.visible_range.columns = UiGridIndexRange::new(2, 3);
    outside_range.cells = vec![cell(1, 1)];
    assert_eq!(
        Err(UiGridValidationError::CellOutsideMaterializedRange {
            coordinate: UiGridCoordinate::new(1, 1)
        }),
        outside_range.validate()
    );

    let mut duplicate = valid_props();
    duplicate.cells = vec![cell(1, 1), cell(1, 1)];
    assert_eq!(
        Err(UiGridValidationError::DuplicateCell {
            coordinate: UiGridCoordinate::new(1, 1)
        }),
        duplicate.validate()
    );

    let mut accessibility = valid_props();
    accessibility.cells = vec![UiGridCell {
        accessibility_row_index: 0,
        ..cell(1, 1)
    }];
    assert_eq!(
        Err(UiGridValidationError::AccessibilityIndexMismatch {
            coordinate: UiGridCoordinate::new(1, 1)
        }),
        accessibility.validate()
    );
}

#[test]
fn grid_props_reject_invalid_overlapping_and_frozen_crossing_spans() {
    let mut invalid = valid_props();
    invalid.cells = vec![UiGridCell {
        row_span: 0,
        ..cell(1, 1)
    }];
    assert_eq!(
        Err(UiGridValidationError::InvalidCellSpan {
            anchor: UiGridCoordinate::new(1, 1)
        }),
        invalid.validate()
    );

    let mut overlapping = valid_props();
    overlapping.cells = vec![
        UiGridCell {
            row_span: 2,
            column_span: 2,
            ..cell(1, 1)
        },
        cell(2, 2),
    ];
    assert_eq!(
        Err(UiGridValidationError::OverlappingCellSpans {
            first: UiGridCoordinate::new(1, 1),
            second: UiGridCoordinate::new(2, 2),
        }),
        overlapping.validate()
    );

    let mut crossing = valid_props();
    crossing.cells = vec![UiGridCell {
        coordinate: UiGridCoordinate::new(0, 1),
        row_span: 2,
        accessibility_row_index: 1,
        accessibility_column_index: 2,
        ..cell(0, 1)
    }];
    assert_eq!(
        Err(UiGridValidationError::CellSpanCrossesFrozenBoundary {
            anchor: UiGridCoordinate::new(0, 1)
        }),
        crossing.validate()
    );
}

#[test]
fn legacy_grid_cell_json_defaults_appearance_and_spans() -> Result<(), serde_json::Error> {
    let value = serde_json::json!({
        "coordinate": {"row": 1, "column": 2},
        "bounds": {"x": 0, "y": 0, "width": 10, "height": 10},
        "clipped_bounds": {"x": 0, "y": 0, "width": 10, "height": 10},
        "text": "legacy",
        "selected": false,
        "active": false,
        "frozen_row": false,
        "frozen_column": false,
        "accessibility_row_index": 2,
        "accessibility_column_index": 3
    });

    let cell: UiGridCell = serde_json::from_value(value)?;

    assert_eq!(UiGridCell::default().appearance, cell.appearance);
    assert_eq!((1, 1), (cell.row_span, cell.column_span));
    Ok(())
}

#[test]
fn grid_validation_errors_explain_each_typed_failure() {
    let coordinate = UiGridCoordinate::new(2, 4);
    let errors = [
        UiGridValidationError::VisibleRangeOutsideGrid,
        UiGridValidationError::ActiveCellOutsideGrid,
        UiGridValidationError::SelectionOutsideGrid,
        UiGridValidationError::CellOutsideGrid,
        UiGridValidationError::CellOutsideMaterializedRange { coordinate },
        UiGridValidationError::DuplicateCell { coordinate },
        UiGridValidationError::AccessibilityIndexMismatch { coordinate },
        UiGridValidationError::InvalidCellSpan { anchor: coordinate },
        UiGridValidationError::OverlappingCellSpans {
            first: coordinate,
            second: UiGridCoordinate::new(3, 5),
        },
        UiGridValidationError::CellSpanCrossesFrozenBoundary { anchor: coordinate },
    ];

    for error in errors {
        assert!(!error.to_string().is_empty());
    }
}
