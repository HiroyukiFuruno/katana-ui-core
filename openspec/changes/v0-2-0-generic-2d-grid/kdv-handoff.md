# KDV v0.4.0 Handoff

## Version Contract

KDV must consume published `katana-ui-core` version `0.2.0` from the registry.
A local path dependency, copied private grid implementation, or framework
adapter in KUC is not permitted.

## Public API

KDV may import the following types from
`katana_ui_core::widget::molecules`:

- `GenericGrid`
- `GridAction`
- `GridAxisConfig`
- `GridAxisPlan`
- `GridAxisPlanner`
- `GridCellContent`
- `GridCellAppearance`
- `GridCellLayout`
- `GridCellSpan`
- `GridCoordinate`
- `GridDataBar`
- `GridEvent`
- `GridHitTest`
- `GridHorizontalAlignment`
- `GridIcon`
- `GridIndexRange`
- `GridLayout`
- `GridNavigationIntent`
- `GridRating`
- `GridSelection`
- `GridTrackSizeProvider`
- `GridVerticalAlignment`
- `GridViewport`

KDV may inspect the following typed render-model values from
`katana_ui_core::render_model`:

- `UiGridCell`
- `UiGridCellAppearance`
- `UiGridCellSpan`
- `UiGridCoordinate`
- `UiGridDataBar`
- `UiGridHorizontalAlignment`
- `UiGridIcon`
- `UiGridIndexRange`
- `UiGridProps`
- `UiGridRating`
- `UiGridSelection`
- `UiGridValidationError`
- `UiGridVerticalAlignment`
- `UiGridViewport`
- `UiGridVisibleRange`

## Adapter Responsibilities

KDV owns document semantics and supplies only materialized visible-cell
content. KUC owns axis planning, frozen-track geometry, effective scroll
clamping, selection, navigation, hit-testing, and conversion to
`UiNodeKind::Grid`.

KDV must:

1. Construct `GenericGrid` from neutral row/column counts and track sizes.
2. Apply viewport, overscan, frozen-track, and active-cell state.
3. Convert neutral merged ranges into `GridCellSpan` values.
4. Call `with_cell_spans(...)` before requesting visible coordinates.
5. Request `visible_coordinates()`.
6. Resolve content and appearance for only those coordinates.
7. Call `with_visible_cells(...)` and handle `UiGridValidationError`.
8. Route `GridAction` values and consume typed `GridEvent` values.

KDV must not:

1. Recalculate cell geometry or hit targets.
2. Materialize the complete document cell matrix.
3. Add document-format terminology or semantic engine types to KUC.
4. Use a local path dependency or copy the grid implementation.

## Verification Contract

The KDV integration test must use at least a 1,000-row by 100-column fixture,
assert bounded visible-cell materialization, exercise two-dimensional scroll,
frozen rows and columns, pointer hit-testing, keyboard navigation, and range
selection, and verify that no KDV-private grid implementation exists.
