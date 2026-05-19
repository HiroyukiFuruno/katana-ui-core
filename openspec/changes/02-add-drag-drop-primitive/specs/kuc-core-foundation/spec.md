## MODIFIED Requirements

### Requirement: Event model exposes pointer, keyboard, focus, command, and drag events

The KUC event model MUST expose `PointerEvent`, `KeyboardEvent`, `FocusEvent`, `CommandEvent`, and `DragEvent` via the `UiEvent` enum.
The event bubbling and capture policies MUST cover all of the above kinds with the same target traversal rules; disabled nodes MUST be skipped uniformly.

#### Scenario: drag event bubbles through enabled ancestors

- **WHEN** a `Drop` event reaches a deeply nested target
- **THEN** bubbling visits ancestors in the same order as pointer events
- **AND** disabled ancestors are skipped from both bubbling and capture phases

#### Scenario: capture phase handler cancels drag

- **WHEN** a capture-phase handler calls `stop_propagation` on a `DragOver` event
- **THEN** the descendant `DropTarget` does not receive `DragOver`
- **AND** no `DropIndicator` is shown for that target
