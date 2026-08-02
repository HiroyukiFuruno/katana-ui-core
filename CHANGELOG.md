# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased]

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
