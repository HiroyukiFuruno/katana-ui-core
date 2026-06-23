## ADDED Requirements

### Requirement: ToastStackManager positions and stacks toasts deterministically

`ToastStackManager` molecule MUST expose `position = TopStart | TopCenter | TopEnd | BottomStart | BottomCenter | BottomEnd` and `max_visible`.
Toasts MUST stack in a position-derived direction (top positions stack downward, bottom positions stack upward).
The visible set MUST never exceed `max_visible`; excess toasts MUST be queued.

#### Scenario: enqueue beyond max_visible queues

- **WHEN** the visible toast count equals `max_visible` and another toast is enqueued
- **THEN** the new toast enters the `queued` queue
- **AND** `ToastQueued` is emitted with the queued id

#### Scenario: visible toast dismissed promotes from queue

- **WHEN** a visible toast is dismissed or times out
- **THEN** the next queued toast is promoted to visible
- **AND** `ToastShown` is emitted with the promoted toast id

### Requirement: ToastStackManager deduplicates by configurable strategy

`ToastStackManager` MUST expose `dedup_strategy = None | ById | ByIdAndSeverity` and `replace_resets_duration`.
Under `ById` or `ByIdAndSeverity`, an enqueue that matches an existing visible or queued toast MUST replace it and emit `ToastReplaced`.

#### Scenario: ById replaces in visible

- **WHEN** `dedup_strategy = ById` and an enqueue matches a visible toast's id
- **THEN** the existing toast is replaced with the new payload
- **AND** `ToastReplaced` is emitted with `{ id, kind: Visible }`

#### Scenario: replace_resets_duration toggles timer reset

- **WHEN** `replace_resets_duration = false` and a `ById` replace occurs
- **THEN** the remaining duration is preserved from the previous toast
- **AND** the new payload is shown for the remaining time only

### Requirement: ToastStackManager pauses timers on hover or focus

`ToastStackManager` MUST suspend all duration timers while any visible toast is hovered or contains focus, when `pause_on_hover = true`.
Timers MUST resume when both hover and focus leave.

#### Scenario: hover suspends timers

- **WHEN** the pointer hovers any visible toast
- **THEN** every visible toast's timer pauses
- **AND** resuming occurs only after the pointer leaves all visible toasts

#### Scenario: focus inside action keeps paused

- **WHEN** focus enters an action button inside a toast
- **THEN** timers stay paused regardless of pointer position
- **AND** focus leaving resumes timers if pointer is also outside

### Requirement: ToastStackManager queues have a configured cap

`ToastStackManager` MUST cap the `queued` queue at a configurable size (default 100).
When the cap is exceeded, the oldest queued toast MUST be dropped and a warning event MUST be emitted.

#### Scenario: cap exceeded drops oldest

- **WHEN** the queue reaches the cap and a new enqueue happens
- **THEN** the oldest queued toast is removed
- **AND** a `ToastQueueOverflow { dropped_id }` event is emitted

### Requirement: ToastStackManager action buttons emit typed dismiss reason

`ToastStackManager` MUST render each toast's primary/secondary actions as `Button` atoms.
Action activation MUST emit `ToastDismissed { id, reason: Action(action_id) }` after action handling.

#### Scenario: action triggers dismiss with reason

- **WHEN** a user activates an action inside a visible toast
- **THEN** the toast is dismissed with `reason = Action(action_id)`
- **AND** the next queued toast is promoted if available
