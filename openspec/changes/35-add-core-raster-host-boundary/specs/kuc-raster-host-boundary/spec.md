## ADDED Requirements

### Requirement: Raster host is a framework-neutral public feature

`katana-ui-core` MUST expose its canvas, UI-tree rasterization, presentation, and hit-test APIs through the optional `raster-host` feature.
The feature dependency graph MUST NOT include `egui`, `eframe`, or `winit`.

#### Scenario: Core-only consumer enables the feature

- **WHEN** a consumer enables `katana-ui-core` with `raster-host`
- **THEN** it can construct and render a UI tree through the public raster host API
- **AND** the resolved dependency graph contains no GUI runtime dependency

### Requirement: Document typography has one final metric contract

`UiTreeDocumentTypography` MUST let consumers independently override the font size, line height, and baseline offset for body, H1, H2, and H3 roles.
For every valid override, canvas rasterization, layout measurement, document node hits, and action hits MUST use the same final metrics.

#### Scenario: Valid body override is reflected by draw and hits

- **WHEN** a consumer supplies a valid body override with a line height below the legacy UI minimum
- **THEN** the rendered line, node hit, and action hit use that override's line height
- **AND** following document and accordion child hits begin at the matching measured offset

#### Scenario: Invalid role override falls back safely

- **WHEN** a consumer supplies a non-finite or non-positive font size, a zero line height, or a baseline offset outside the line box
- **THEN** KUC ignores that role override
- **AND** the role uses its theme-derived metrics without panicking
