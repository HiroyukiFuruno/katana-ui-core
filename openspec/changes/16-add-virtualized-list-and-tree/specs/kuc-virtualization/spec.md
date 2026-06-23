## ADDED Requirements

### Requirement: Virtualization config is shared by list-like molecules

KUC MUST expose a shared `VirtualizationConfig` consumed by `List`, `SelectionList`, `TreeView`, `CommandPalette`, and `DiagnosticsList`.
The config MUST include `enabled`, `estimated_row_height`, `overscan`, `row_height_provider`, and `keep_focused_in_window`.

#### Scenario: shared config governs visible range

- **WHEN** the same `VirtualizationConfig` is applied to two molecules with the same row layout
- **THEN** their computed visible ranges match for the same viewport
- **AND** their state ids remain distinct per molecule

#### Scenario: disabled config preserves existing behavior

- **WHEN** `enabled = false`
- **THEN** all rows are rendered as before this change
- **AND** consumers do not see API changes affecting them

### Requirement: compute_visible_range is a pure deterministic function

`compute_visible_range(viewport_offset, viewport_height, row_heights, total_count, overscan)` MUST be a pure function returning `VirtualRange { start, end, total }`.
The function MUST handle Fixed, Variable, and Estimated row height providers without panicking.

#### Scenario: Fixed provider returns simple division

- **WHEN** rows are `Fixed(24px)` and viewport is `480px` at offset `0`
- **THEN** start = 0 and end ≈ 20 + overscan
- **AND** total equals the input total_count

#### Scenario: Estimated provider uses measured overrides

- **WHEN** some rows have measured overrides differing from the estimate
- **THEN** the visible range is computed using the cumulative actual heights for measured rows and the estimate for the rest
- **AND** the function is stable for identical inputs

### Requirement: keep_focused_in_window renders focused row regardless of range

When `keep_focused_in_window = true`, list-like molecules MUST render the focused row even if its index falls outside the computed virtual range.

#### Scenario: focused row outside range still renders

- **WHEN** the focused row is two pages above the current viewport and `keep_focused_in_window = true`
- **THEN** the focused row is rendered as a sentinel inside the rendered nodes (e.g., at the closest edge of the virtual_range or as a separately-positioned overlay)
- **AND** accessibility focus continues to map to the correct row id

#### Scenario: focused row inside range behaves normally

- **WHEN** the focused row is within the visible range
- **THEN** no special sentinel is rendered
- **AND** focus and selection behavior matches the non-virtualized case

### Requirement: Virtualization preserves accessibility set size and position

List-like molecules MUST report `aria-setsize = total_count` and per-rendered-row `aria-posinset = index + 1` even when virtualization hides rows.
Screen reader announcements MUST reflect total count, not the rendered count.

#### Scenario: aria-setsize equals total_count

- **WHEN** the molecule renders 30 rows out of 10000 total
- **THEN** `aria-setsize` (or KUC's accessibility analogue) reports `10000`
- **AND** each rendered row reports its global `aria-posinset`

#### Scenario: announce uses total count

- **WHEN** a screen reader announces the focused row
- **THEN** the announcement uses `"<label>, <index> of <total>"` using the global index and total_count
- **AND** the announcement does not say "<rendered_index> of <rendered_count>"

### Requirement: Virtualization is opt-in and backward compatible

When `VirtualizationConfig::enabled = false` (default), the existing rendering behavior of all affected molecules MUST be unchanged.
Existing Storybook presets, contract tests, and numeric layout / rendering contracts MUST continue to pass without modification.

#### Scenario: existing preset stays valid

- **WHEN** a Storybook preset relies on default options
- **THEN** turning the virtualization-aware code path off renders identically to before
- **AND** numeric rendering contracts for that preset remain stable

#### Scenario: opt-in does not leak to disabled consumers

- **WHEN** a consumer never sets `VirtualizationConfig`
- **THEN** the molecule emits no virtualization-related events
- **AND** state shape changes are limited to opt-in code paths
