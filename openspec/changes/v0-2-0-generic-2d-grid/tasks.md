# Tasks - v0-2-0-generic-2d-grid

## 1. Readiness and branch contract

- [x] 1.1 Record the approved XLSX `interactive-grid` profile and KUC ownership boundary in proposal, design, and specification artifacts.
- [x] 1.2 Create local `release/v0.2.0` and work on `feature/v0.2.0-task1` from the same release base.
- [x] 1.3 Pass strict OpenSpec validation before production implementation. Evidence: `rtk ./scripts/openspec validate v0-2-0-generic-2d-grid --strict`.

## 2. Axis virtualization

- [x] 2.1 Add fixed and variable `GridTrackSizeProvider` contracts with one logical pixel normalization and saturating extents. Evidence: `crates/katana-ui-core/src/molecule/generic_grid/axis.rs`.
- [x] 2.2 Add `GridAxisPlanner` prefix extents, binary-search lookup, fixed-track fast path, and clamped half-open ranges. Evidence: `crates/katana-ui-core/src/molecule/generic_grid/axis.rs`.
- [x] 2.3 Cover empty axes, zero sizes, maximum offsets, overscan, and frozen-extents-over-viewport behavior with focused tests. Evidence: `crates/katana-ui-core/tests/generic_grid_axis_contract.rs`.

## 3. Grid state and geometry

- [x] 3.1 Add public coordinate, viewport, bounds, visible-range, and frozen-state types without document-format terminology. Evidence: `crates/katana-ui-core/src/render_model/typed_grid.rs`, `crates/katana-ui-core/src/render_model/typed_grid_types.rs`, `crates/katana-ui-core/src/molecule/generic_grid/mod.rs`, `crates/katana-ui-core/src/molecule/generic_grid/axis_types.rs`, `crates/katana-ui-core/src/molecule/generic_grid/component_types.rs`.
- [x] 3.2 Add `GenericGrid` planning that unions frozen and scrollable row/column indexes without duplicate cells. Evidence: `crates/katana-ui-core/src/molecule/generic_grid/component.rs`.
- [x] 3.3 Add visible-cell geometry whose fixed cells ignore scroll offsets and whose scrollable cells remain clipped to the viewport. Evidence: `crates/katana-ui-core/src/molecule/generic_grid/geometry.rs`.
- [x] 3.4 Prove that a 1,000 x 100 grid materializes only visible, overscan, and frozen cells. Evidence: `crates/katana-ui-core/tests/generic_grid_component_contract.rs`.

## 4. Input and selection

- [x] 4.1 Add active-cell and inclusive range-selection state with empty-grid normalization. Evidence: `crates/katana-ui-core/src/molecule/generic_grid/selection.rs`.
- [x] 4.2 Add Arrow, Home, End, PageUp, and PageDown navigation with clamping and extend-selection semantics. Evidence: `crates/katana-ui-core/tests/generic_grid_component_contract.rs`.
- [x] 4.3 Add pointer hit-testing from rendered cell bounds with left/top inclusive and right/bottom exclusive edges. Evidence: `crates/katana-ui-core/src/molecule/generic_grid/geometry.rs`.
- [x] 4.4 Cover frozen-cell precedence, boundary coordinates, selection collapse, anchor preservation, and empty-grid input. Evidence: `crates/katana-ui-core/tests/generic_grid_component_contract.rs`.

## 5. Typed render model

- [x] 5.1 Add typed `UiGridProps`, visible-cell DTOs, accessibility indexes, and validation errors to the public render model. Evidence: `crates/katana-ui-core/src/render_model/typed_grid.rs`, `crates/katana-ui-core/src/render_model/typed_grid_types.rs`.
- [x] 5.2 Add backward-compatible `UiProps.grid` serialization and validation for `UiNodeKind::Grid`. Evidence: `crates/katana-ui-core/src/render_model/props.rs`, `crates/katana-ui-core/tests/generic_grid_render_contract.rs`.
- [x] 5.3 Add `GenericGrid` to `UiNode` conversion preserving state id, geometry, selection, active state, and visible text. Evidence: `crates/katana-ui-core/src/molecule/generic_grid/component.rs`.
- [x] 5.4 Reject cells outside visible, overscan, or frozen ranges instead of retaining them silently. Evidence: `crates/katana-ui-core/tests/generic_grid_component_contract.rs`, `crates/katana-ui-core/tests/generic_grid_render_contract.rs`.
- [x] 5.5 Add format-neutral cell appearance, conditional decoration, and span DTOs without document semantics. Evidence: `crates/katana-ui-core/src/render_model/typed_grid_types.rs`, `crates/katana-ui-core/src/molecule/generic_grid/component_span.rs`.
- [x] 5.6 Preserve span geometry, appearance, hit-testing, and typed validation in the render contract. Evidence: `crates/katana-ui-core/src/molecule/generic_grid/geometry.rs`, `crates/katana-ui-core/tests/generic_grid_component_contract.rs`.

## 6. Public boundary and consumer proof

- [x] 6.1 Re-export the generic grid API from the established KUC public module boundaries. Evidence: `crates/katana-ui-core/src/molecule/mod.rs`, `crates/katana-ui-core/src/widget/molecules.rs`.
- [x] 6.2 Add an AST/dependency guard that rejects XLSX, formula, chart, pivot, KDV, and framework-specific dependencies in the grid implementation. Evidence: `scripts/kuc_guardrails.py`, `scripts/test_kuc_guardrails.py`.
- [x] 6.3 Extend the consumer app to build and exercise a 100,000-cell model using only public KUC APIs. Evidence: `examples/kuc-consumer-app/src/fixtures.rs`, `examples/kuc-consumer-app/src/lib.rs`.
- [x] 6.4 Add a cross-platform pure-Rust consumer contract test for bounded materialization, geometry, selection, and hit-testing. Evidence: `examples/kuc-consumer-app/tests/generic_public_contract.rs`.

## 7. Quality and release readiness

- [x] 7.1 Pass focused generic-grid unit and integration tests. Evidence: `crates/katana-ui-core/tests/generic_grid_axis_contract.rs`, `crates/katana-ui-core/tests/generic_grid_component_contract.rs`, `crates/katana-ui-core/tests/generic_grid_render_contract.rs`.
- [x] 7.2 Pass formatting, AST lint, clippy with warnings denied, and the repository `check` gate. Evidence: `Justfile`, `scripts/kuc_guardrails.py`.
- [x] 7.3 Pass strict coverage at 100% with zero uncovered lines without threshold relaxation or exclusions. Evidence: `rtk just coverage` completed with 9,193/9,193 functions, 95,646/95,646 lines, and zero uncovered lines across KUC, Storybook, native Xvfb, and the public consumer app.
- [x] 7.4 Pass strict OpenSpec validation with all implementation evidence linked from this task list. Evidence: `rtk ./scripts/openspec validate v0-2-0-generic-2d-grid --strict`.
- [x] 7.5 Run mandatory self-review and correct every actionable finding. Evidence: `openspec/changes/v0-2-0-generic-2d-grid/self-review.md`.

## 8. KDV handoff

- [x] 8.1 Document the exact public KUC API and version contract consumed by KDV `v0.4.0`. Evidence: `openspec/changes/v0-2-0-generic-2d-grid/kdv-handoff.md`.
- [ ] 8.2 Confirm KDV integration requires no local path dependency and no duplicated private grid implementation.
