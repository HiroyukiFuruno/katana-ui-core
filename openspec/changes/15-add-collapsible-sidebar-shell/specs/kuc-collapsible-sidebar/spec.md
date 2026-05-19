## ADDED Requirements

### Requirement: CollapsibleSidebar exposes four typed modes

`CollapsibleSidebar` MUST expose `mode = Expanded | IconOnly | Collapsed | FloatingOverlay`.
The transition between modes MUST be observable via `ModeChanged` events.

#### Scenario: switching modes emits events

- **WHEN** the sidebar transitions from `Expanded` to `IconOnly`
- **THEN** `ModeChanged { from: Expanded, to: IconOnly }` is emitted
- **AND** state reflects the new mode

#### Scenario: FloatingOverlay does not shrink main

- **WHEN** mode transitions to `FloatingOverlay`
- **THEN** the sibling main content's available width is unchanged
- **AND** the sidebar renders as an overlay on top of the main content with a higher z-index

### Requirement: CollapsibleSidebar supports resizable width with optional persistence

`CollapsibleSidebar` MUST expose `width = ResizableWidth { min, max, default, persist_id }`.
With `resize_handle = true`, dragging the handle MUST clamp the width to `[min, max]` and emit `WidthChanged`.
Double-clicking the handle MUST reset width to `default`.

#### Scenario: drag clamps to min and max

- **WHEN** the user drags the resize handle past `max`
- **THEN** the width is clamped to `max`
- **AND** `WidthChanged { width: max }` is emitted

#### Scenario: double-click resets to default

- **WHEN** the user double-clicks the resize handle
- **THEN** `width` returns to `default`
- **AND** `WidthChanged { width: default }` is emitted

### Requirement: CollapsibleSidebar supports expand-on-hover when unpinned

`CollapsibleSidebar` MUST allow `pinned = false` together with `expand_on_hover = true`.
Hovering the sidebar trigger area MUST temporarily expand to `Expanded` mode.
Leaving the hover area MUST revert to the prior mode.

#### Scenario: hover expands while unpinned

- **WHEN** `pinned = false`, `expand_on_hover = true`, and mode is `IconOnly`
- **THEN** hovering the trigger temporarily renders as `Expanded`
- **AND** `state.hover_open = true` is set while the hover persists

#### Scenario: pinned ignores hover

- **WHEN** `pinned = true`
- **THEN** hover does not change the rendered mode
- **AND** `state.hover_open` remains false

### Requirement: CollapsibleSidebar persistence id is opaque to KUC

`CollapsibleSidebar` MUST accept `persist_id: Option<String>` and report `WidthChanged` for the consumer to store.
KUC MUST NOT touch storage; persistence is the consumer's responsibility.

#### Scenario: consumer subscribes to WidthChanged

- **WHEN** `persist_id = Some("workspace.sidebar.width")` is set
- **THEN** every `WidthChanged` event is emitted with the same `persist_id`
- **AND** KUC does not call any persistence API directly

#### Scenario: no persist_id means session-only

- **WHEN** `persist_id = None`
- **THEN** `WidthChanged` events still fire, but consumers are not expected to persist them
- **AND** the next reload returns to `default`
