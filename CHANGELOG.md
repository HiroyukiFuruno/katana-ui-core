# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased]

## [0.3.16] - 2026-09-26

### Changed

- Updated the Storybook development dependency to `katana-markdown-model` 0.2.3 and refreshed compatible lockfile resolutions.

## [0.3.15] - 2026-09-24

- Corrective release; release evidence is recorded in `docs/release/v0.3.15.md`.

### Fixed

- Restored document-coordinate interaction-hit collection before viewport clipping so deeply scrolled accordion actions remain reachable (Issue #52).
- Corrected horizontal viewport-origin handling when `area.x` is nonzero (Issue #52).

## [0.3.13] - 2026-09-23

### Fixed

- Pinned the Linux emoji catalog and runtime font inputs so platform text rendering remains deterministic.

## [0.3.12] - 2026-09-22

### Fixed

- Corrected canvas blit boundaries and release evidence identity checks (Issue #62).
- Added `PlatformTextFaceSelection::CandidateChain` and deterministic font selection for public consumers.
- Restored viewport hit-collection cutoff and highlight alpha-compositing behavior.

## [0.3.11] - 2026-09-13

### Added

- Added an optional `text-highlight-background` theme token for document marks while preserving the existing default color (Issue #52).

- Added independent, opt-in typography overrides for `heading-4`, `heading-5`, and `heading-6`, preserving existing and unconfigured role metrics (Issue #52).
- Added `IssuedConsumerArtifactPlan::execute_next_with_evidence_error` and `ConsumerArtifactPlanExecutionError` so consumers can distinguish Unicode evidence failures by type while existing `execute_next` callers retain their error contract (Issue #57).

### Fixed

- Preserved the selected candidate font face across weight/style/stretch differences instead of silently using a system fallback (Issue #52).
- Kept wrapped link action regions on their displayed lines, including scrolled document and viewport coordinates (Issue #52).
- Included the fractional bottom edge in document text and button node/action hit regions without changing the logical layout cursor (Issue #52).
- Aligned fixed-height document rows and their hit regions using the same fractional vertical center offset (Issue #52).
- Preserved inline-code and monospace font selection from span measurement through rich-line rendering (Issue #52).
- Preserved fractional document row positions when the public hover helper wraps visible or scrolled text (Issue #52).
- Matched fractional scroll visibility between hover rendering and action hits, and retained later children in partially visible hover surfaces (Issue #52).
- Kept absolute media controls visible during partial scrolling and applied ScrollArea offsets once (Issue #52).

## [0.3.10] - 2026-09-13

### Fixed

- Corrected the legacy document-typography coordinate regression.
- Connected KUC's Unicode evidence pin resolver so consumer artifacts execute without host policy injection.

## [0.3.9] - 2026-09-12

- Published the corrective consumer-boundary release; public API and verification details are recorded in `docs/release/v0.3.9.md`.

## [0.3.8] - 2026-09-09

- Published the unified consumer-contract release; public scope and verification details are recorded in `docs/release/v0.3.8.md`.

## [0.3.7] - 2026-09-05

### Added

- Added role-specific document typography with renderer, Storybook-host, surface-host, measurement, and hit-geometry integration.

## [0.3.6] - 2026-09-05

### Added

- Added typed, opt-in canonical proportional and monospace face selection to the public raster host APIs: `UiTreeCanvasRenderer`, `UiTreeStorybookHost`, and `UiTreeSurfaceHost` (Issue #41).

### Fixed

- Preserved the existing `System` generic-family default and emoji contract while allowing registry-only consumers to reproduce their platform candidate policy without path or Git overrides.

### Changed

- Audited direct and transitive dependencies and updated compatible `Cargo.lock` resolutions before the release gate.

## [0.3.5] - 2026-09-04

### Added

- Published the additive, framework-neutral `raster_host` API required by registry consumers, including the `raster-host` feature for KDV document rendering (Issues #35 and #37).
- Published typed per-side custom grid borders and the ScrollArea/Grid atomic-layout contract through the public `katana-ui-core` crate.

### Fixed

- Restored registry-only KDV compatibility without path or Git dependency overrides, while preserving Storybook's legacy root facade compatibility.

## [0.3.4] - 2026-09-02

### Added

- Added an additive opaque writer for variable-viewport full-motion sequences that normalizes KUC-issued frames into one fixed-canvas GIF/MP4 artifact without exposing raster or paint-plan internals (Issue #34).
- Added a dedicated versioned manifest that binds source viewport dimensions and PNG hashes, normalized source/decode hashes, root provenance, and Unicode/IME/hit-test/AccessKit evidence.

### Changed

- Updated compatible transitive dependencies while preserving the complete release quality gates.

### Fixed

- Preserved the existing fixed-dimension `write_opaque` contract while allowing resize stages to use the new variable-viewport path without consumer-side compositing or input rewriting.

## [0.3.2] - 2026-08-30

### Added

- Added a generic typed pointer gesture contract for pan, smooth-scroll, pinch/trackpad zoom, and fullscreen state propagation through KUC-owned hit targets.
- Added the retained full-editor root projection and cross-platform text-raster evidence required by downstream editor hosts.
- Added issue-first dependency-evidence hooks and post-publication branch/worktree cleanup automation.

### Changed

- Consolidated all outstanding KUC release requirements into one patch release and updated compatible direct, transitive, and lockfile dependencies.
- Kept crates.io publication in GitHub Actions, including a tag-bound retry workflow; local registry login is not part of the release flow.

### Fixed

- Added real `egui::RawInput` pointer-resolution regressions and restored strict line/function coverage requirements without exclusions or threshold changes.
- Preserved legacy opaque host-token behavior while applying explicit command-family identities only through the versioned envelope.

## [0.3.1] - 2026-08-28

### Added

- Added framework-neutral platform text and SVG raster runtimes with deterministic layout, color emoji, grapheme hit-testing, and cache contracts.
- Added generic text-surface and command-chrome models plus the optional KUC-owned egui adapter for text, toolbar, floating toolbar, search, context-menu, IME, and accessibility integration.
- Added host-projected opaque command-family identities through a versioned token envelope without adding required fields to existing public presentation struct literals.

### Changed

- Updated compatible direct and transitive dependencies without changing release quality gates.
- Extended strict Linux coverage to all publishable runtime and adapter crates, including deterministic font and motion-artifact prerequisites.

### Fixed

- Preserved legacy host-token decoding and rendering behavior while rejecting duplicate explicitly projected command families and unknown envelope versions fail closed.

## [0.3.0] - 2026-08-02

### Fixed

- Preserved generic grid-line visibility through the public grid model and typed render props, including backward-compatible defaults for existing consumers.

## [0.2.0] - 2026-07-30

### Added

- Added a format-neutral, two-dimensional virtualized grid with fixed and variable tracks, frozen rows and columns, bounded visible-cell materialization, merged-cell spans, and typed cell appearance.
- Added typed pointer hit-testing, keyboard navigation, active-cell state, and range selection through the public KUC API.
- Added a public consumer contract for KDV `v0.4.0` without document-format semantics or framework-specific dependencies.

### Changed

- Updated compatible direct and transitive dependencies before the release gate.

## [0.1.2] - 2026-06-24

### Added

- Added a unified `UiTreeInteractionTarget` contract for host action, hover node, cursor, and node hit resolution.
- Extended the Storybook interaction surface so hosts can consume rendered KUC targets without reconstructing row or control geometry.

## [0.1.1] - 2026-06-24

### Added

- Added typed `UiContextMenuItem` host actions and task-state payloads so hosts can consume context-menu selections without parsing item ids.
- Added Storybook host query support for resolving rendered context-menu item hits to `UiHostActionPlan`.

### Fixed

- Kept the Storybook crate private/internal and restored release publishing to the public `katana-ui-core` crate only.

<!-- next-url -->
