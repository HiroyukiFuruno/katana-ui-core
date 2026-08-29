## Why

KDV's XLSX `interactive-grid` profile needs a framework-neutral rendering
contract that virtualizes both rows and columns. KUC currently provides only
the one-dimensional `VirtualizedList`. Requiring each consumer to implement
its own grid, hit-testing, selection, and scroll state would mix UI
responsibilities with document semantics again.

## What Changes

- Add a public state, action, and event contract for a generic two-dimensional
  virtualized grid in KUC `v0.2.0`.
- Define row and column geometry, frozen headers, visible ranges, overscan,
  scrolling, selection, active cells, keyboard navigation, and hit targets in
  a framework-neutral way.
- Add format-neutral cell appearance, conditional decoration, and span DTOs
  without adding document-format-specific types or interpretation.
- Materialize only visible and overscan cells, rather than every cell in a
  large data set.
- Verify bounds, alignment, selection transitions, navigation, and
  virtualization quantity with numeric contracts.

## Capabilities

### New Capabilities

- `generic-2d-grid`: Defines format-neutral two-dimensional virtualization,
  geometry, selection, navigation, scrolling, and a visible-cell render model.

### Modified Capabilities

None.

## Impact

- `crates/katana-ui-core/src/molecule/`: Public grid component and
  state/action/event contracts.
- `crates/katana-ui-core/src/render_model/`: Grid viewport and visible-cell
  models.
- `crates/katana-ui-core/tests/`: Consumer, virtualization, input, and layout
  regression contracts.
- `examples/kuc-consumer-app/`: Grid integration using only public APIs.
- KDV `v0.4.0`: Consumes the neutral grid contract from published KUC
  `v0.2.0`.
