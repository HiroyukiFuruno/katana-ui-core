# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased]

## [0.1.1] - 2026-06-24

### Added

- Added typed `UiContextMenuItem` host actions and task-state payloads so hosts can consume context-menu selections without parsing item ids.
- Added Storybook host query support for resolving rendered context-menu item hits to `UiHostActionPlan`.

### Fixed

- Kept the Storybook crate private/internal and restored release publishing to the public `katana-ui-core` crate only.

<!-- next-url -->
