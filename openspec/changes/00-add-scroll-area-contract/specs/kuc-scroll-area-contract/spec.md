## ADDED Requirements

### Requirement: ScrollArea exposes typed scroll state and options

`ScrollArea` MUST expose typed options for axis, current offset, viewport extent, content extent, scrollbar visibility, scrollbar placement, and edge threshold.
The options MUST be serializable through the KUC render model.

#### Scenario: vertical scroll area keeps extents

- **WHEN** a consumer creates a vertical `ScrollArea` with viewport height and content height
- **THEN** the render model preserves both extents
- **AND** the current offset is clamped to the valid vertical range

### Requirement: ScrollArea accepts programmatic scroll actions

`ScrollArea` MUST accept typed `ScrollTo`, `ScrollBy`, `ScrollIntoView`, and `SetScrollbarVisibility` actions.
Actions that cannot be applied because of axis mismatch or invalid extent MUST emit a rejected result instead of silently mutating state.

#### Scenario: scroll into view clamps target

- **WHEN** a target rect is below the current viewport
- **THEN** `ScrollIntoView` updates the offset so the target becomes visible
- **AND** the offset does not exceed the content extent

#### Scenario: horizontal command is rejected on vertical area

- **WHEN** `ScrollBy { dx }` is sent to a vertical-only `ScrollArea`
- **THEN** the horizontal offset remains unchanged
- **AND** `ScrollCommandRejected` records the axis mismatch

### Requirement: ScrollArea emits deterministic scroll events

`ScrollArea` MUST emit `Scrolled`, `ScrollEdgeReached`, and `ScrollCommandRejected` events with the target state id.
Nested scroll areas MUST preserve separate state ids and MUST NOT mutate each other.

#### Scenario: child scroll does not alter parent

- **WHEN** a nested child `ScrollArea` receives `ScrollBy`
- **THEN** only the child offset changes
- **AND** the emitted event target is the child state id

### Requirement: ScrollArea supports keyboard scroll mapping

`ScrollArea` MUST provide keyboard mapping for PageUp, PageDown, Home, and End when keyboard scrolling is enabled.
The mapping MUST use viewport extent and axis settings, not hard-coded pixel constants.

#### Scenario: page down uses viewport height

- **WHEN** PageDown is applied to a vertical `ScrollArea`
- **THEN** the vertical offset increases by the configured viewport height
- **AND** the value is clamped to the content extent
