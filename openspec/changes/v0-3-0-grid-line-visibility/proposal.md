# KUC v0.3.0 grid-line visibility

## Why

KDV retains whether a spreadsheet sheet shows grid lines, but KUC `v0.2.0`
cannot carry that format-neutral display preference to a host renderer.

## What Changes

- Add a grid-line visibility flag to `GenericGrid` and `UiGridProps`.
- Preserve the existing visible-grid-lines behavior for legacy models.
- Keep document and spreadsheet semantics outside KUC.
