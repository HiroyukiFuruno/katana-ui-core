## MODIFIED Requirements

### Requirement: Typed grid render props must be public

The system MUST expose typed props for `UiNodeKind::Grid` that retain total
geometry, viewport, visible range, selection, active cell, visible cells, and
the format-neutral grid-line visibility preference.

#### Scenario: Preserve grid-line visibility

- **WHEN** a consumer enables or disables grid lines on `GenericGrid`
- **THEN** KUC retains the exact value in `UiGridProps`
- **THEN** KUC does not infer document or spreadsheet semantics

#### Scenario: Read a legacy grid model

- **WHEN** serialized grid props do not contain a grid-line visibility field
- **THEN** KUC defaults to visible grid lines
- **THEN** the `v0.2.0` host rendering behavior is preserved
