use katana_ui_core::molecule::{
    GenericGrid, GridAction, GridCellAppearance, GridCellContent, GridCellSpan, GridCoordinate,
    GridDataBar, GridNavigationIntent, GridTrackSizeProvider, GridViewport,
};
use katana_ui_core::render_model::{UiGridValidationError, UiNode, UiNodeKind};

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn missing(message: &'static str) -> std::io::Error {
    std::io::Error::other(message)
}

fn large_grid() -> GenericGrid {
    GenericGrid::new("Data", 1_000, 100)
        .row_tracks(GridTrackSizeProvider::fixed(20))
        .column_tracks(GridTrackSizeProvider::fixed(80))
        .viewport(GridViewport::new(320, 100).scroll(160, 200))
        .overscan(1, 1)
        .frozen(1, 1)
}

#[test]
fn large_grid_materializes_only_bounded_visible_coordinates() {
    let grid = large_grid();
    let coordinates = grid.visible_coordinates();

    assert!(coordinates.len() < 100);
    assert!(coordinates.contains(&GridCoordinate::new(0, 0)));
    assert_eq!(
        coordinates.len(),
        coordinates
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len()
    );
}

#[test]
fn visible_content_becomes_typed_grid_props_with_accessibility_indexes() -> TestResult {
    let base = large_grid();
    let coordinate = base.visible_coordinates()[0];
    let grid = base.with_visible_cells(vec![GridCellContent::new(coordinate, "Header")])?;

    let node = UiNode::from(grid);
    let cell = node
        .props()
        .grid
        .cells
        .iter()
        .find(|cell| cell.coordinate == coordinate)
        .ok_or_else(|| missing("planned coordinate must be rendered"))?;

    assert_eq!(UiNodeKind::Grid, node.kind());
    assert_eq!(1_000, node.props().grid.row_count);
    assert_eq!(100, node.props().grid.column_count);
    assert!(node.props().grid.show_grid_lines);
    assert_eq!("Header", cell.text);
    assert_eq!(coordinate.row + 1, cell.accessibility_row_index);
    assert_eq!(coordinate.column + 1, cell.accessibility_column_index);
    Ok(())
}

#[test]
fn grid_line_visibility_is_preserved_in_typed_render_props() {
    let node = UiNode::from(GenericGrid::new("Grid", 1, 1).show_grid_lines(false));

    assert!(!node.props().grid.show_grid_lines);
}

#[test]
fn legacy_generic_grid_defaults_to_visible_grid_lines() -> TestResult {
    let mut value = serde_json::to_value(GenericGrid::new("Legacy", 1, 1).show_grid_lines(false))?;
    let removed = value
        .as_object_mut()
        .and_then(|object| object.remove("show_grid_lines"));
    assert_eq!(Some(serde_json::Value::Bool(false)), removed);

    let grid: GenericGrid = serde_json::from_value(value)?;
    let node = UiNode::from(grid);

    assert!(node.props().grid.show_grid_lines);
    Ok(())
}

#[test]
fn merged_cell_span_and_appearance_drive_render_geometry_and_hit_testing() -> TestResult {
    let appearance = GridCellAppearance {
        font_family: "Inter".to_owned(),
        font_size_px: 16,
        bold: true,
        fill_color: Some("#183B66".to_owned()),
        data_bar: Some(GridDataBar {
            positive_color: Some("#63BE7B".to_owned()),
            fill_ratio_basis_points: 7_500,
            show_value: true,
            ..GridDataBar::default()
        }),
        ..GridCellAppearance::default()
    };
    let span = GridCellSpan::new(GridCoordinate::new(0, 0), 1, 3);
    let grid = GenericGrid::new("Grid", 4, 4)
        .row_tracks(GridTrackSizeProvider::fixed(20))
        .column_tracks(GridTrackSizeProvider::fixed(20))
        .viewport(GridViewport::new(80, 80))
        .with_cell_spans(vec![span])?
        .with_visible_cells(vec![
            GridCellContent::new(span.anchor, "Merged").appearance(appearance.clone()),
        ])?;

    assert!(
        !grid
            .visible_coordinates()
            .contains(&GridCoordinate::new(0, 1))
    );
    assert_eq!(
        span.anchor,
        grid.hit_test(45, 10)
            .ok_or_else(|| missing("span must hit"))?
            .coordinate
    );
    let node = UiNode::from(grid);
    let cell = &node.props().grid.cells[0];
    assert_eq!((60, 20), (cell.bounds.width, cell.bounds.height));
    assert_eq!((1, 3), (cell.row_span, cell.column_span));
    assert_eq!(appearance, cell.appearance);
    Ok(())
}

#[test]
fn cell_span_validation_rejects_invalid_overlap_and_frozen_crossing() {
    let anchor = GridCoordinate::new(0, 0);
    let invalid = GridCellSpan::new(anchor, 0, 1);
    let outside = GridCellSpan::new(GridCoordinate::new(3, 3), 2, 1);
    let first = GridCellSpan::new(anchor, 2, 2);
    let second = GridCellSpan::new(GridCoordinate::new(1, 1), 2, 2);
    let crossing = GridCellSpan::new(anchor, 2, 1);

    assert_eq!(
        Err(UiGridValidationError::InvalidCellSpan { anchor }),
        GenericGrid::new("Grid", 4, 4).with_cell_spans(vec![invalid])
    );
    assert_eq!(
        Err(UiGridValidationError::InvalidCellSpan {
            anchor: outside.anchor
        }),
        GenericGrid::new("Grid", 4, 4).with_cell_spans(vec![outside])
    );
    assert_eq!(
        Err(UiGridValidationError::OverlappingCellSpans {
            first: first.anchor,
            second: second.anchor,
        }),
        GenericGrid::new("Grid", 4, 4).with_cell_spans(vec![first, second])
    );
    assert_eq!(
        Err(UiGridValidationError::CellSpanCrossesFrozenBoundary { anchor }),
        GenericGrid::new("Grid", 4, 4)
            .frozen(1, 0)
            .with_cell_spans(vec![crossing])
    );
}

#[test]
fn selecting_a_coordinate_covered_by_a_span_selects_its_anchor() -> TestResult {
    let span = GridCellSpan::new(GridCoordinate::new(1, 1), 2, 2);
    let mut grid = GenericGrid::new("Grid", 4, 4).with_cell_spans(vec![span])?;

    grid.apply_action(GridAction::Select {
        coordinate: GridCoordinate::new(2, 2),
        extend: false,
    });

    assert_eq!(Some(span.anchor), grid.active_coordinate());
    Ok(())
}

#[test]
fn navigation_skips_covered_span_coordinates_in_each_forward_direction() -> TestResult {
    let horizontal = GridCellSpan::new(GridCoordinate::new(1, 1), 1, 2);
    let vertical = GridCellSpan::new(GridCoordinate::new(2, 3), 2, 1);
    let mut grid = GenericGrid::new("Grid", 6, 6)
        .with_cell_spans(vec![horizontal, vertical])?
        .active_cell(horizontal.anchor);

    grid.apply_action(GridAction::Navigate {
        intent: GridNavigationIntent::Right,
        extend: false,
    });
    assert_eq!(Some(GridCoordinate::new(1, 3)), grid.active_coordinate());

    grid.apply_action(GridAction::Select {
        coordinate: vertical.anchor,
        extend: false,
    });
    grid.apply_action(GridAction::Navigate {
        intent: GridNavigationIntent::Down,
        extend: false,
    });
    assert_eq!(Some(GridCoordinate::new(4, 3)), grid.active_coordinate());
    Ok(())
}

#[test]
fn adding_spans_normalizes_existing_selection_to_span_anchor() -> TestResult {
    let span = GridCellSpan::new(GridCoordinate::new(1, 1), 2, 2);
    let grid = GenericGrid::new("Grid", 4, 4)
        .active_cell(GridCoordinate::new(2, 2))
        .with_cell_spans(vec![span])?;

    assert_eq!(Some(span.anchor), grid.active_coordinate());
    assert_eq!(
        Some(span.anchor),
        grid.selection().map(|selection| selection.anchor)
    );
    Ok(())
}

#[test]
fn navigation_handles_candidates_outside_and_clamped_inside_current_span() -> TestResult {
    let middle_span = GridCellSpan::new(GridCoordinate::new(1, 1), 1, 2);
    let mut outside = GenericGrid::new("Grid", 4, 4)
        .with_cell_spans(vec![middle_span])?
        .active_cell(middle_span.anchor);
    outside.apply_action(GridAction::Navigate {
        intent: GridNavigationIntent::End,
        extend: false,
    });
    assert_eq!(Some(GridCoordinate::new(1, 3)), outside.active_coordinate());

    let leading_span = GridCellSpan::new(GridCoordinate::new(0, 0), 1, 2);
    let mut clamped = GenericGrid::new("Grid", 4, 4)
        .with_cell_spans(vec![leading_span])?
        .active_cell(leading_span.anchor);
    clamped.apply_action(GridAction::Navigate {
        intent: GridNavigationIntent::Left,
        extend: false,
    });
    assert_eq!(Some(leading_span.anchor), clamped.active_coordinate());
    Ok(())
}

#[test]
fn navigation_skips_hidden_rows_and_columns() {
    let tracks = GridTrackSizeProvider::variable_with_hidden(vec![10, 10, 10, 10], 10, vec![1, 2]);
    let mut grid = GenericGrid::new("Grid", 4, 4)
        .row_tracks(tracks.clone())
        .column_tracks(tracks)
        .active_cell(GridCoordinate::new(0, 0));

    grid.apply_action(GridAction::Navigate {
        intent: GridNavigationIntent::Right,
        extend: false,
    });
    assert_eq!(Some(GridCoordinate::new(0, 3)), grid.active_coordinate());

    grid.apply_action(GridAction::Navigate {
        intent: GridNavigationIntent::Down,
        extend: false,
    });
    assert_eq!(Some(GridCoordinate::new(3, 3)), grid.active_coordinate());

    grid.apply_action(GridAction::Navigate {
        intent: GridNavigationIntent::Home,
        extend: false,
    });
    assert_eq!(Some(GridCoordinate::new(3, 0)), grid.active_coordinate());
}

#[test]
fn ui_node_uses_effective_scroll_offsets_in_grid_and_interaction_props() {
    let node = UiNode::from(
        GenericGrid::new("Grid", 2, 2)
            .row_tracks(GridTrackSizeProvider::fixed(10))
            .column_tracks(GridTrackSizeProvider::fixed(10))
            .viewport(GridViewport::new(10, 10).scroll(u32::MAX, u32::MAX)),
    );

    assert_eq!(10, node.props().grid.viewport.scroll_x);
    assert_eq!(10, node.props().grid.viewport.scroll_y);
    assert_eq!("10,10", node.props().interaction.value);
}

#[test]
fn content_outside_materialized_ranges_is_rejected() {
    let result = large_grid().with_visible_cells(vec![GridCellContent::new(
        GridCoordinate::new(999, 99),
        "outside",
    )]);

    assert!(matches!(
        result,
        Err(UiGridValidationError::CellOutsideMaterializedRange {
            coordinate: GridCoordinate {
                row: 999,
                column: 99
            }
        })
    ));
}

#[test]
fn content_outside_grid_dimensions_is_rejected() {
    assert_eq!(
        Err(UiGridValidationError::CellOutsideGrid),
        large_grid().with_visible_cells(vec![GridCellContent::new(
            GridCoordinate::new(1_000, 0),
            "outside-grid",
        )])
    );
}

#[test]
fn duplicate_visible_content_is_rejected() {
    let base = large_grid();
    let coordinate = base.visible_coordinates()[0];
    let result = base.with_visible_cells(vec![
        GridCellContent::new(coordinate, "first"),
        GridCellContent::new(coordinate, "second"),
    ]);

    assert_eq!(
        Err(UiGridValidationError::DuplicateCell { coordinate }),
        result
    );
}

#[test]
fn keyboard_navigation_clamps_and_preserves_extended_anchor() -> TestResult {
    let mut grid = GenericGrid::new("Grid", 10, 10).active_cell(GridCoordinate::new(5, 5));

    grid.apply_action(GridAction::Navigate {
        intent: GridNavigationIntent::Right,
        extend: false,
    });
    grid.apply_action(GridAction::Navigate {
        intent: GridNavigationIntent::Up,
        extend: true,
    });

    let selection = grid
        .selection()
        .ok_or_else(|| missing("selection must exist"))?;
    assert_eq!(GridCoordinate::new(5, 6), selection.anchor);
    assert_eq!(GridCoordinate::new(4, 6), selection.active);
    assert_eq!(GridCoordinate::new(4, 6), selection.start);
    assert_eq!(GridCoordinate::new(5, 6), selection.end);

    grid.apply_action(GridAction::Navigate {
        intent: GridNavigationIntent::Home,
        extend: false,
    });
    assert_eq!(Some(GridCoordinate::new(4, 0)), grid.active_coordinate());
    Ok(())
}

#[test]
fn empty_grid_never_creates_selection_or_active_cell() {
    let mut grid = GenericGrid::new("Empty", 0, 10).active_cell(GridCoordinate::new(4, 4));

    grid.apply_action(GridAction::Select {
        coordinate: GridCoordinate::new(1, 1),
        extend: true,
    });
    grid.apply_action(GridAction::Navigate {
        intent: GridNavigationIntent::PageDown,
        extend: false,
    });

    assert_eq!(None, grid.active_coordinate());
    assert_eq!(None, grid.selection());
    assert!(grid.visible_coordinates().is_empty());
}

#[test]
fn all_grid_actions_emit_typed_clamped_state_transitions() {
    let mut grid = GenericGrid::new("Grid", 10, 10)
        .row_tracks(GridTrackSizeProvider::fixed(20))
        .column_tracks(GridTrackSizeProvider::fixed(20))
        .viewport(GridViewport::new(40, 40))
        .active_cell(GridCoordinate::new(5, 5));

    assert!(grid.state_id().as_str().starts_with("state:Grid:"));
    for intent in [
        GridNavigationIntent::Left,
        GridNavigationIntent::Down,
        GridNavigationIntent::End,
        GridNavigationIntent::PageUp,
        GridNavigationIntent::PageDown,
    ] {
        grid.apply_action(GridAction::Navigate {
            intent,
            extend: false,
        });
    }
    grid.apply_action(GridAction::Select {
        coordinate: GridCoordinate::new(usize::MAX, usize::MAX),
        extend: true,
    });
    assert_eq!(Some(GridCoordinate::new(9, 9)), grid.active_coordinate());

    let scrolled = grid.apply_action(GridAction::ScrollTo { x: 40, y: 60 });
    assert_eq!(
        katana_ui_core::molecule::GridEvent::Scrolled(GridViewport::new(40, 40).scroll(40, 60)),
        scrolled
    );
    assert_eq!(
        katana_ui_core::molecule::GridEvent::None,
        grid.apply_action(GridAction::ScrollTo { x: 40, y: 60 })
    );
    assert_eq!(
        katana_ui_core::molecule::GridEvent::SelectionChanged(None),
        grid.apply_action(GridAction::ClearSelection)
    );
    assert_eq!(
        &katana_ui_core::molecule::GridEvent::SelectionChanged(None),
        grid.last_event()
    );
}

#[test]
fn hit_test_uses_rendered_half_open_bounds_and_frozen_geometry() -> TestResult {
    let grid = GenericGrid::new("Grid", 4, 4)
        .row_tracks(GridTrackSizeProvider::fixed(20))
        .column_tracks(GridTrackSizeProvider::fixed(20))
        .viewport(GridViewport::new(60, 60).scroll(20, 20))
        .frozen(1, 1);

    let frozen = grid
        .hit_test(0, 0)
        .ok_or_else(|| missing("frozen corner must hit"))?;
    let adjacent = grid
        .hit_test(20, 20)
        .ok_or_else(|| missing("left/top edge must hit"))?;

    assert_eq!(GridCoordinate::new(0, 0), frozen.coordinate);
    assert!(frozen.frozen_row);
    assert!(frozen.frozen_column);
    assert_eq!(GridCoordinate::new(2, 2), adjacent.coordinate);
    assert_eq!(None, grid.hit_test(60, 60));
    assert_eq!(
        None,
        GenericGrid::new("Zero", 1, 1)
            .viewport(GridViewport::new(0, 0))
            .hit_test(0, 0)
    );
    Ok(())
}

#[test]
fn extreme_track_geometry_clamps_signed_bounds_without_overflow() {
    let positive_grid = GenericGrid::new("Positive", 2, 2)
        .row_tracks(GridTrackSizeProvider::variable(vec![u32::MAX, 1], 1))
        .column_tracks(GridTrackSizeProvider::variable(vec![u32::MAX, 1], 1))
        .viewport(GridViewport::new(u32::MAX, u32::MAX))
        .frozen(2, 2);
    let positive = positive_grid.layout();
    let negative = GenericGrid::new("Negative", 2, 2)
        .row_tracks(GridTrackSizeProvider::variable(vec![u32::MAX, 1], 1))
        .column_tracks(GridTrackSizeProvider::variable(vec![u32::MAX, 1], 1))
        .viewport(GridViewport::new(100, 100).scroll(u32::MAX, u32::MAX))
        .overscan(1, 1)
        .layout();

    assert!(positive.cells.iter().any(|cell| cell.bounds.x == i32::MAX));
    assert!(positive.cells.iter().any(|cell| cell.bounds.y == i32::MAX));
    assert!(negative.cells.iter().any(|cell| cell.bounds.x == i32::MIN));
    assert!(negative.cells.iter().any(|cell| cell.bounds.y == i32::MIN));
    assert_eq!(
        Some(GridCoordinate::new(1, 1)),
        positive_grid
            .hit_test(i32::MAX, i32::MAX)
            .map(|hit| hit.coordinate)
    );
}

#[test]
fn serialized_ui_node_without_grid_props_uses_backward_compatible_default() -> TestResult {
    let node = UiNode::from(GenericGrid::new("Grid", 2, 2));
    let mut value = serde_json::to_value(node)?;
    value["props"]
        .as_object_mut()
        .ok_or_else(|| missing("props must be an object"))?
        .remove("grid");

    let decoded: UiNode = serde_json::from_value(value)?;

    assert_eq!(UiNodeKind::Grid, decoded.kind());
    assert_eq!(0, decoded.props().grid.row_count);
    assert!(decoded.props().grid.cells.is_empty());
    Ok(())
}

#[test]
fn generic_grid_and_layout_serde_round_trip_keeps_public_state() -> TestResult {
    let grid = large_grid().active_cell(GridCoordinate::new(10, 2));
    let encoded_grid = serde_json::to_string(&grid)?;
    let decoded_grid: GenericGrid = serde_json::from_str(&encoded_grid)?;
    let layout = decoded_grid.layout();
    let encoded_layout = serde_json::to_string(&layout)?;
    let decoded_layout: katana_ui_core::molecule::GridLayout =
        serde_json::from_str(&encoded_layout)?;

    assert_eq!(
        grid.visible_coordinates(),
        decoded_grid.visible_coordinates()
    );
    assert_eq!(layout, decoded_layout);
    assert_eq!(grid.selection(), decoded_grid.selection());

    let selected_node = UiNode::from(decoded_grid);
    assert!(
        selected_node
            .props()
            .grid
            .cells
            .iter()
            .any(|cell| cell.selected)
    );

    let owned_label = GenericGrid::new("Owned", 1, 1)
        .viewport(GridViewport::new(1, 1))
        .with_visible_cells(Vec::<GridCellContent>::new())?;
    assert_eq!(1, owned_label.visible_coordinates().len());
    Ok(())
}
