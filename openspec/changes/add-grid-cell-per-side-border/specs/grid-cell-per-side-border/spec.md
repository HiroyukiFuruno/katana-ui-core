## ADDED Requirements

### Requirement: Grid cell stores per-side border metadata

The public `UiGridCellAppearance` SHALL expose a serializable `UiGridCellBorders` value with independent left, right, top, and bottom `UiGridBorderSide` values. A side SHALL represent no border, a typed line style, and an optional explicit color without requiring a consumer-specific rendering type.

#### Scenario: Legacy serialized cell appearance omits borders

- **WHEN** a serialized `UiGridCellAppearance` has no `borders` field
- **THEN** it SHALL deserialize to an empty `UiGridCellBorders` value and SHALL not add a custom border

#### Scenario: Four sides differ

- **WHEN** a cell assigns different line styles or colors to its four sides
- **THEN** the model SHALL retain each side independently

### Requirement: Raster host renders per-side grid borders

The `raster-host` feature SHALL render every visible `UiNodeKind::Grid` cell using its `UiGridCellAppearance.borders` metadata. Rendering SHALL respect cell clipped bounds, viewport clipping, scroll offsets, and merged-cell anchor bounds.

#### Scenario: Cell has differently colored left and top borders

- **WHEN** a visible grid cell has an explicit left border and an explicit top border with different colors
- **THEN** the raster output SHALL contain the respective colors on the corresponding cell edges

#### Scenario: Border is clipped by the viewport

- **WHEN** a grid cell edge extends beyond the grid viewport
- **THEN** the raster host SHALL draw only the visible clipped segment and SHALL not write outside the viewport

### Requirement: Existing grid rendering remains unchanged without explicit borders

The raster host SHALL preserve existing grid output for cell appearances whose `borders` value is empty.

#### Scenario: Default cell appearance

- **WHEN** a grid cell uses `UiGridCellAppearance::default()`
- **THEN** rendering SHALL not introduce a custom per-side border
