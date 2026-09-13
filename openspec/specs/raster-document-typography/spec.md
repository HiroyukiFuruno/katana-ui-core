# raster-document-typography Specification

## Purpose

Document typography overrides preserve the legacy integer-coordinate contract while allowing hosts to explicitly opt into fractional baseline layout.

## Requirements

### Requirement: Legacy typography preserves integer rendering

A renderer using `UiTreeTextRoleTypography` SHALL retain the integer-coordinate rendering and layout contract.

#### Scenario: Legacy document host

- **WHEN** a host configures only legacy role typography
- **THEN** render and layout use integer origins
- **AND** fractional line-box cursor behavior is not introduced.

### Requirement: Fractional baseline typography is explicit

A renderer SHALL use logical line-box coordinates only when a `UiTreeTextRoleBaselineTypography` override is configured.

#### Scenario: Fractional-baseline document host

- **WHEN** a host configures a `UiTreeTextRoleBaselineTypography` override for a document role
- **THEN** render and layout use logical line-box coordinates for that host
- **AND** integer-coordinate legacy role typography remains available when no baseline override is configured.
