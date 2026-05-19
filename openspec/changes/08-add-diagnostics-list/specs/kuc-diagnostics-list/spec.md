## ADDED Requirements

### Requirement: DiagnosticsList groups, sorts, and filters items deterministically

`DiagnosticsList` MUST expose `group_by = Severity | Source | Location | None`, `sort_by = Severity | Location | Source | Order`, and a typed `severity_filter` set.
Given the same items and options, the resulting grouped/sorted/filtered view MUST be deterministic and unit-testable as pure functions.

#### Scenario: group_by Severity orders Error > Warning > Info > Hint

- **WHEN** `group_by = Severity` is set and the items contain mixed severities
- **THEN** groups appear in the order Error, Warning, Info, Hint
- **AND** each group's `count` reflects only items currently passing the filter

#### Scenario: severity_filter removes hidden items

- **WHEN** `severity_filter` excludes `Warning`
- **THEN** items with severity `Warning` are not visible
- **AND** the group counts and the total counter are updated accordingly

### Requirement: DiagnosticsList embeds CodeDiff for fix preview

`DiagnosticsList` items MUST be able to carry a `fix_preview` slot consumed by the existing `CodeDiff` molecule.
Expanding an item MUST reveal its `fix_preview`, applying the fix MUST emit `DiagnosticFixApplied { id }`.

#### Scenario: expand reveals CodeDiff

- **WHEN** an item with a `fix_preview` is expanded
- **THEN** the `CodeDiff` molecule renders the snapshot
- **AND** the child `UiStateId` is distinct from the parent list state

#### Scenario: apply fix removes the diagnostic

- **WHEN** `ApplyFix` is dispatched on an expanded item
- **THEN** `DiagnosticFixApplied { id }` is emitted
- **AND** the consumer is responsible for removing the item from the next state snapshot

### Requirement: DiagnosticsList supports bulk fix with preview

`DiagnosticsList` MUST allow a bulk action with a dry-run preview surfaced via `ModalOverlay`.
The bulk apply MUST emit `BulkFixApplied { applied_ids, skipped_ids }`.

#### Scenario: bulk preview opens modal

- **WHEN** the bulk action is invoked
- **THEN** a `ModalOverlay` opens with the dry-run preview content provided by the consumer callback
- **AND** confirming applies the bulk fix and emits the typed event with applied/skipped ids

#### Scenario: bulk apply respects severity filter

- **WHEN** a severity filter excludes some items at the moment of bulk apply
- **THEN** excluded items appear in `skipped_ids` with reason `FilteredOut`
- **AND** they are not modified

### Requirement: DiagnosticsList surfaces empty and loading states

`DiagnosticsList` MUST render an empty state when no items pass the filter and `state.empty_state = true`.
`DiagnosticsList` MUST render a skeleton state when `state.loading = true`.
The empty and loading states MUST use the KUC `EmptyState` and `Skeleton` molecules respectively (when those changes are implemented).

#### Scenario: filter results in zero items

- **WHEN** items remain but the filter excludes all of them
- **THEN** the molecule renders the `EmptyState` with a "no matches" message
- **AND** the bulk action is disabled

#### Scenario: loading transitions to result

- **WHEN** `state.loading = true` flips to `false` with items present
- **THEN** the skeleton is replaced by the actual list
- **AND** the focused item is restored if it still exists, otherwise focus falls back to the first item

### Requirement: DiagnosticsList keyboard navigation matches problems panel conventions

`DiagnosticsList` MUST handle `ArrowUp/Down` to move selection, `ArrowLeft/Right` to collapse/expand groups, `Enter` to navigate, and `Space` to apply quick fix.
`DiagnosticsList` MUST also support `F8` / `Shift+F8` accelerators to jump to next/previous Error.

#### Scenario: F8 jumps to next Error

- **WHEN** `F8` is pressed
- **THEN** selection moves to the next `Error` item (wrapping at the end if configured)
- **AND** `DiagnosticSelected { id }` is emitted with the destination item

#### Scenario: Space applies quickfix when available

- **WHEN** `Space` is pressed on a selected item with a quickfix action
- **THEN** `ApplyFix` runs and `DiagnosticFixApplied` is emitted
- **AND** if no quickfix is available, the key is ignored (no event)
