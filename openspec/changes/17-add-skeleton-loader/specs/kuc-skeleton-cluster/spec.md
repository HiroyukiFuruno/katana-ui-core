## ADDED Requirements

### Requirement: SkeletonCluster ships canonical loading presets

`SkeletonCluster` MUST ship presets `Card`, `ListRow`, `Message`, `Paragraph`, `ImageCard`, `CodeBlock`.
Each preset MUST be a deterministic layout of `Skeleton` atoms with stable proportions.

#### Scenario: ListRow preset layout is stable

- **WHEN** a `SkeletonCluster::list_row()` is rendered at the same size twice
- **THEN** the layout snapshots are identical
- **AND** the children include a leading circle and two text lines per row

#### Scenario: CodeBlock preset varies line widths

- **WHEN** a `SkeletonCluster::code_block()` is rendered
- **THEN** the children consist of multiple rect lines with varying widths to evoke a code block
- **AND** the count and width ratios match the preset specification

### Requirement: SkeletonCluster emits a single accessibility announcement

`SkeletonCluster` MUST emit a polite live-region announcement on mount (e.g., "Loading <label>") and MUST NOT re-announce on identical re-renders.

#### Scenario: mount triggers announcement

- **WHEN** a `SkeletonCluster` mounts with `label = "messages"`
- **THEN** one polite announcement "Loading messages" is emitted
- **AND** the inner skeleton atoms do not emit individual announcements

#### Scenario: identical re-render skips announcement

- **WHEN** the same `SkeletonCluster` re-renders without prop changes
- **THEN** no additional announcement is emitted
- **AND** if `label` changes, a new announcement is emitted reflecting the new label
