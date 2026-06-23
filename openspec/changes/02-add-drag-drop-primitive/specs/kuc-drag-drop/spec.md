## ADDED Requirements

### Requirement: DragData is typed with tag and payload

`DragData` MUST carry a string `tag`, a serializable `payload`, and metadata for display.
Core MUST NOT interpret the `payload` content; only the `tag` is used for compatibility decisions.
The tag namespace MUST be reserved: `os/*` for native OS payloads, `katana-ui-core/*` for KUC-internal payloads, and `consumer/*` for downstream consumers.

#### Scenario: drop target rejects mismatched tag

- **WHEN** a drop target's `accept` callback receives a `DragData` whose tag is outside the target's accepted tag set
- **THEN** the callback returns `DropAcceptance::Reject`
- **AND** the drop indicator is not displayed for that target

#### Scenario: OS file drop is translated by adapter

- **WHEN** a native OS file drop reaches a KUC adapter
- **THEN** the adapter converts the OS payload into `DragData { tag: "os/file-list", payload, metadata }`
- **AND** the core consumer interprets `payload` according to its own schema without exposing OS types

### Requirement: DropTarget distinguishes insert and contain by position

`DropTarget` MUST emit a `DropIndicator` whose kind depends on pointer position within the target rect.
Valid indicator kinds MUST cover `Before`, `After`, `Inside`, and `None`.
Threshold ratios for switching between indicator kinds MUST be deterministic and unit-testable.

#### Scenario: pointer near top edge shows Before indicator

- **WHEN** the pointer is within the top threshold ratio of a list item
- **THEN** the indicator is `Before`
- **AND** the drop will reorder above the item if accepted

#### Scenario: pointer in middle shows Inside indicator

- **WHEN** the pointer is within the middle threshold ratio
- **THEN** the indicator is `Inside`
- **AND** the drop will nest the payload inside the item if accepted

### Requirement: Drag events emit on a deterministic sequence

`DragEvent` MUST emit in the order: `DragStart` → (`DragEnter` → `DragOver`* → (`DragLeave` | `Drop`))* → `DragEnd`.
A `DragCancel` MUST always be followed by `DragEnd { committed: false }`.
Disabled nodes MUST be skipped in event bubbling for drag events.

#### Scenario: cancel during drag emits End with committed=false

- **WHEN** Esc is pressed during a drag
- **THEN** `DragCancel` is emitted
- **AND** the next event is `DragEnd` with `committed = false`
- **AND** the drop indicator is cleared and focus returns to the drag source

#### Scenario: successful drop emits End with committed=true

- **WHEN** the pointer is released over an accepting target
- **THEN** `Drop` is emitted with the chosen `DropEffect`
- **AND** the following `DragEnd` has `committed = true`

### Requirement: Keyboard drag is supported when source opts in

`DragSource` MUST allow `keyboard_draggable = true` to enable Space/Enter pick up, arrow-key focus traversal, Space/Enter drop, and Esc cancel.
Accessibility announcements MUST be produced for pick up, move-over-target, accept-or-reject, and drop / cancel transitions.

#### Scenario: Space picks up and arrow moves focus

- **WHEN** focus is on a `keyboard_draggable` source and Space is pressed
- **THEN** `DragStart` is emitted
- **AND** subsequent arrow keys move focus across `DropTarget` nodes, firing `DragEnter` / `DragLeave` accordingly

#### Scenario: Space on accepting target drops

- **WHEN** focus is on an accepting `DropTarget` during a keyboard drag and Space is pressed
- **THEN** `Drop` is emitted with the accepted effect
- **AND** `DragEnd { committed: true }` follows

### Requirement: Autoscroll engages near scrollable edges

`DropTarget` MUST emit autoscroll requests when pointer or keyboard-drag focus enters the configured edge zone of a scrollable ancestor.
The autoscroll speed MUST follow the configured acceleration curve and MUST stop when focus exits the edge zone or the drag ends.

#### Scenario: pointer rests in edge zone

- **WHEN** the pointer stays inside the top edge zone of a scrollable list during a drag
- **THEN** the engine emits scroll requests at the configured cadence
- **AND** the scroll speed increases per the configured curve until the maximum is reached

#### Scenario: edge zone is disabled

- **WHEN** `AutoScrollPolicy::disabled()` is configured
- **THEN** no scroll requests are emitted regardless of pointer position
