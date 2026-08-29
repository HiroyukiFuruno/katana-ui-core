## Context

KUC `v0.1.4` provides `VirtualizedList` and a row-oriented
`VirtualizationPlanner`, but it has no public contract combining column
virtualization, frozen tracks, cell selection, and two-dimensional
hit-testing. KDV `v0.4.0` must display and operate on XLSX `interactive-grid`
models with at least 100,000 cells without moving format semantics into KUC.

## Goals / Non-Goals

**Goals:**

- Materialize only visible and overscan ranges on both axes.
- Support fixed and variable track sizes, two-dimensional scrolling, and
  frozen rows and columns.
- Combine active cells, range selection, keyboard navigation, and pointer
  hit-testing in one state contract.
- Expose typed props, visible-cell bounds, appearance, generic decorations,
  and spans that an adapter can render directly.
- Prohibit node creation proportional to all cells in a 100,000-cell fixture.

**Non-Goals:**

- Add document parsing, formulas, charts, pivots, or conditional formatting to
  KUC.
- Add data editing, a formula bar, column resizing, sorting, or filtering.
- Add framework-specific widgets, a GPU renderer, or a font rasterizer to the
  core.
- Interpret document-format merge semantics in KUC. A consumer remains
  responsible for supplying format-neutral cell spans.

## Decisions

### D1. Separate axis planning from grid state

`GridAxisConfig` and `GridAxisPlanner` calculate one axis's visible range,
track offsets, and extent, and the same implementation is used for rows and
columns. `GenericGrid` combines both planners with selection, active cell,
viewport, and frozen-track state. Nesting two existing `VirtualizedList`
instances is rejected because it would separate selection and hit-test
coordinates.

### D2. Make fixed and variable track sizing explicit

`GridTrackSizeProvider` stores either a fixed size or per-track sizes with a
fallback. Zero sizes normalize to one logical pixel, and extent arithmetic
saturates. Variable-track lookup uses prefix extents and binary search instead
of scanning from the first track to the viewport.

### D3. Keep only visible cells in render DTOs

`GridVisibleRange` carries half-open row and column ranges and frozen-track
counts. `UiGridProps` retains total counts, visible range, scroll position,
viewport, selection, and visible `UiGridCell` values only. Cell appearance,
data-bar/icon/rating decoration, and span are generic render data; KUC does not
calculate document rules. A consumer-provided cell outside the materialized
range produces a typed validation error instead of being retained silently.

### D4. Separate frozen tracks from scroll ranges

Leading `frozen_rows` and `frozen_columns` remain pinned to the viewport
origin. An index overlapping a scroll range is materialized only once. When a
frozen extent fills the viewport, the scrollable range is empty and geometry
must not overflow.

### D5. Retain selection and navigation by coordinate

`GridCoordinate`, inclusive `GridSelection`, and `GridNavigationIntent` are
public contracts. Arrow, Home, End, PageUp, and PageDown navigation clamps to
row and column counts. Extending a selection preserves its anchor. Empty grids
do not create a selection.

### D6. Use render bounds for hit-testing

Planner-generated `UiGridCell.bounds` are the source of truth for pointer
hit-testing. Frozen cells take precedence over overlapping scroll cells.
Left/top edges are inclusive and right/bottom edges are exclusive. Adapters
must not calculate separate geometry.

### D7. Add typed grid props to KUC's public render model

The existing `UiNodeKind::Grid` is retained, and `UiGridProps` is added to
`UiProps.grid`. `GenericGrid -> UiNode` conversion preserves the stable state
id, accessibility row and column indexes, active and selected state,
visible-cell bounds, appearance, and span. Covered span coordinates are not
materialized separately. No format-specific type is exposed.

### D8. Fix the release order

Publish KUC `v0.2.0` only after strict gates pass. KDV must then consume the
registry release or a public Git tag. KDV and KatanA must not use local path
dependencies, and KDV must not copy a private grid implementation before KUC
is published.

## Risks / Trade-offs

- [Variable-track prefixes are large] Retain row and column counts, never
  cell-count-sized state.
- [Frozen and scroll ranges overlap] Deduplicate their deterministic index
  union.
- [A consumer supplies an out-of-range cell] Reject it with a typed validation
  error.
- [Adapter bounds diverge] Use KUC-generated bounds as the only hit-test and
  render input.
- [The public API becomes format-specific] Reject format names and semantic
  engine types with an AST guard.

## Migration Plan

1. Add the API in KUC `v0.2.0` without breaking existing `Grid` or
   `VirtualizedList` consumers.
2. Prove bounded materialization with a 100,000-cell consumer fixture.
3. Publish KUC, then make KDV `v0.4.0` consume the published version.
4. If integration fails, disable the KDV feature without affecting KUC
   `v0.1.4` consumers.

## Open Questions

None. The rendering profile, typed unsupported semantics, and release order
are approved.
