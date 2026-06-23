## ADDED Requirements

### Requirement: Placement engine resolves anchor, priority, and viewport clamp deterministically

The KUC placement engine MUST expose a pure function `resolve_placement(request) -> result` that consumes anchor kind, preferred placement, priority list, panel size, viewport, and clamp margin.
The function MUST be deterministic for identical inputs and MUST never panic.

#### Scenario: preferred placement fits and is chosen

- **WHEN** the preferred placement fits the panel inside the viewport with clamp margin
- **THEN** `placement_used = preferred`
- **AND** `position` matches the preferred placement geometry

#### Scenario: preferred does not fit, priority list flips

- **WHEN** the preferred placement overflows the viewport in one axis
- **THEN** the engine walks the priority list and picks the first placement that fits
- **AND** `placement_used` reflects the chosen fallback

### Requirement: Placement engine emits arrow offset when arrow is requested

The placement engine MUST compute an `arrow_offset` when the caller passes an `arrow` spec.
The arrow MUST align with the anchor center, clamped to stay inside the panel by the configured margin.

#### Scenario: anchor center inside panel

- **WHEN** the anchor center projects inside the panel's edge
- **THEN** `arrow_offset = Some(distance_from_panel_edge)`
- **AND** the arrow visually points at the anchor center

#### Scenario: anchor pushed beyond panel edge

- **WHEN** the anchor center projects past the panel edge minus margin
- **THEN** `arrow_offset` is clamped to `panel_size - margin`
- **AND** the arrow stays inside the panel rather than detaching

### Requirement: Disclosure molecules share the placement engine

`Tooltip`, `Popover`, `HoverCard`, `ContextMenu`, `Menu`, `MenuButton`, `SelectBox`, and `ComboBox` MUST consume the shared placement engine.
Direct duplication of edge-flip or arrow-offset logic inside these molecules MUST be removed.

#### Scenario: shared engine governs SelectBox panel placement

- **WHEN** `SelectBox` opens its panel near the bottom of the viewport
- **THEN** the panel flips above the trigger using the same priority list as `Popover` would
- **AND** the placement contract test is parameterized over all molecules sharing the engine

#### Scenario: changing the engine updates all consumers

- **WHEN** the engine's priority list defaults are updated
- **THEN** all sharing molecules adopt the new behavior without per-molecule code changes
- **AND** their regression tests detect the behavior change centrally
