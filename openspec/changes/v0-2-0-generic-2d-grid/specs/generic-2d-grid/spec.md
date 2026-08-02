## ADDED Requirements

### Requirement: The generic 2D grid must be format-neutral

The system MUST provide public row, column, and cell contracts that do not
depend on a document format or spreadsheet engine.

#### Scenario: A consumer constructs a grid model

- **WHEN** a consumer supplies row count, column count, track sizes, viewport,
  visible-cell content, appearance, decoration, and optional spans
- **THEN** KUC returns generic coordinates, geometry, selection, appearance,
  spans, and render props
- **THEN** the public API and dependencies contain no document parser,
  document semantic engine, or KDV type

### Requirement: Rows and columns must virtualize independently

The system MUST calculate visible ranges and overscan independently for both
row and column axes, and MUST NOT materialize every cell.

#### Scenario: Display a 100,000-cell grid

- **WHEN** a finite viewport and overscan are assigned to a 1,000-row by
  100-column grid
- **THEN** the visible-cell DTO count is no greater than the product of
  visible, overscan, and frozen tracks
- **THEN** node and cell DTO counts do not scale to 100,000

#### Scenario: Scroll fixed tracks

- **WHEN** fixed row height, fixed column width, and two-dimensional scroll
  offsets are assigned
- **THEN** half-open row and column visible ranges include tracks intersecting
  the viewport
- **THEN** overscan clamps to each axis's total count

#### Scenario: Scroll variable tracks

- **WHEN** an axis with per-track sizes and a fallback scrolls to a large offset
- **THEN** KUC determines the first visible track with prefix extents and
  binary search
- **THEN** zero sizes become one logical pixel and do not overflow

### Requirement: Frozen and scroll tracks must use consistent geometry

The system MUST pin leading frozen rows and columns to the viewport origin and
lay them out without duplicating scrollable cells.

#### Scenario: Display frozen rows and columns

- **WHEN** frozen rows and columns are combined with scroll offsets
- **THEN** frozen cells are not affected by scroll offsets
- **THEN** coordinates shared with a scroll range materialize once
- **THEN** frozen cells take hit-test precedence over overlapping scroll cells

#### Scenario: A frozen extent fills the viewport

- **WHEN** a frozen-track extent reaches or exceeds viewport width or height
- **THEN** that axis's scrollable range is empty
- **THEN** bounds do not overflow

### Requirement: Selection and keyboard navigation must be provided

The system MUST retain the active cell, selection anchor, inclusive range, and
navigation intent and clamp them to grid bounds.

#### Scenario: Move the active cell with navigation intents

- **WHEN** a consumer applies Arrow, Home, End, PageUp, or PageDown intents
- **THEN** the active coordinate clamps to valid row and column bounds
- **THEN** `extend = false` collapses anchor and selection to the active cell
- **THEN** `extend = true` preserves the anchor and updates the selection range

#### Scenario: Operate on an empty grid

- **WHEN** selection or navigation is applied to a grid with zero rows or
  columns
- **THEN** no active cell or selection exists
- **THEN** no panic or out-of-range coordinate is produced

### Requirement: Pointer hit-testing must use rendered bounds

The system MUST reuse visible-cell DTO bounds for pointer hit-testing and MUST
NOT require an adapter to calculate separate cell geometry.

#### Scenario: Hit-test cell boundaries

- **WHEN** a pointer is on a cell's left/top edge, interior, or right/bottom
  edge
- **THEN** left/top edges hit that cell
- **THEN** right/bottom edges hit an adjacent cell or miss
- **THEN** the result includes coordinate, cell bounds, and frozen state

### Requirement: Typed grid render props must be public

The system MUST expose typed props for `UiNodeKind::Grid` that retain total
geometry, viewport, visible range, selection, active cell, and visible cells.

#### Scenario: Convert GenericGrid into UiNode

- **WHEN** a consumer converts `GenericGrid` with valid visible cells into
  `UiNode`
- **THEN** state id, row and column counts, viewport, effective scroll, and
  frozen counts are retained
- **THEN** visible-cell coordinates, bounds, text, selected state, and active
  state are retained
- **THEN** accessibility row and column indexes are one-based

#### Scenario: Supply a cell outside the materialized range

- **WHEN** a consumer adds a cell outside visible, overscan, and frozen ranges
- **THEN** KUC returns a typed validation error
- **THEN** the out-of-range cell is not retained silently

#### Scenario: Supply cell appearance and spans

- **WHEN** a consumer supplies format-neutral font, color, alignment,
  data-bar/icon/rating decoration, and row/column span values
- **THEN** KUC retains those values in the visible-cell render props
- **THEN** KUC expands the anchor geometry and does not materialize covered
  coordinates separately
- **THEN** invalid, overlapping, or frozen-boundary-crossing spans return a
  typed validation error
- **THEN** KUC does not interpret formulas, conditional rules, or document
  merge syntax

### Requirement: Numeric grid contracts must be fixed in the release gate

The system MUST verify virtualization quantity, bounds, alignment, selection
transitions, hit targets, and the public boundary through automated tests and
guards.

#### Scenario: Release KUC v0.2.0

- **WHEN** the KUC release gate verifies the generic 2D grid
- **THEN** strict coverage reaches 100% with zero uncovered lines and no
  threshold relaxation or added exclusions
- **THEN** the consumer app displays a bounded 100,000-cell model using only
  public APIs
- **THEN** the AST guard rejects format-specific types and framework-specific
  dependencies
- **THEN** the same pure-Rust core contract runs on macOS, Linux, and Windows
