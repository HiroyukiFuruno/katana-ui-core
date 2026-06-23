## ADDED Requirements

### Requirement: Skeleton atom exposes typed shape, size, and animation

`Skeleton` atom MUST expose `shape = Rect | Circle | Line | Text`, `width`, `height`, `radius`, `tone`, `animation`, `accessibility_label`, and `aspect_ratio`.
`Text` shape MUST take `lines: usize` and `last_line_ratio: f32`.
`SkeletonSize` MUST cover `Fixed(f32) | Fill | Auto`.

#### Scenario: Text shape renders multiple lines with a shorter last line

- **WHEN** a `Skeleton` is configured with `shape = Text { lines: 3, last_line_ratio: 0.6 }`
- **THEN** three line-shapes are rendered, with the third at 60% of the line width
- **AND** the snapshot is stable across renders

#### Scenario: SkeletonSize Fill stretches with the parent

- **WHEN** `width = Fill` and the parent width changes
- **THEN** the skeleton renders at the new parent width
- **AND** the height is unaffected unless tied via aspect_ratio

### Requirement: Skeleton animation respects reduced-motion preference

`Skeleton` MUST expose `animation = None | Pulse | Shimmer | Wave`.
When the runtime reports `reduced_motion = true`, the rendered animation MUST be `None` regardless of the configured option.

#### Scenario: reduced-motion downgrades animation

- **WHEN** `animation = Shimmer` is set and `reduced_motion = true` is reported by the runtime
- **THEN** the rendered output uses `None` (static)
- **AND** an internal trace records the downgrade for tests

#### Scenario: reduced-motion off keeps configured animation

- **WHEN** `animation = Pulse` is set and `reduced_motion = false`
- **THEN** the rendered output animates as Pulse
- **AND** the inputs to the animation curve match the documented parameters

### Requirement: Skeleton accessibility announces once per cluster

When many `Skeleton` atoms are rendered together (e.g., via `SkeletonCluster`), the accessibility live region MUST emit at most one announcement for the cluster, not one per atom.
Each `Skeleton` MAY still own its own `accessibility_label` for fine-grained queries.

#### Scenario: cluster announces once

- **WHEN** a `SkeletonCluster` mounts with 5 inner skeletons
- **THEN** the cluster emits one polite announcement (e.g., "Loading list")
- **AND** the inner skeletons do not emit individual announcements

#### Scenario: standalone skeleton can announce individually

- **WHEN** a single `Skeleton` is used outside any cluster and has `accessibility_label = "Loading title"`
- **THEN** one polite announcement is emitted for that atom
- **AND** repeating renders of the same atom do not re-announce
