# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased]

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
