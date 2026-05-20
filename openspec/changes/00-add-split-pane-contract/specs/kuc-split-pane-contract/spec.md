## ADDED Requirements

### Requirement: SplitPane exposes a two-pane typed contract

`SplitPane` MUST expose exactly two primary pane slots and typed options for axis, ratio, min ratio, max ratio, reset ratio, handle size, and resize mode.
It MUST NOT expose app shell, sidebar collapse, title bar, status bar, or viewer-editor synchronization options.

#### Scenario: two panes are configured

- **WHEN** a consumer creates a `SplitPane` with first and second pane content
- **THEN** the render model contains two primary pane slots
- **AND** no app shell or sidebar collapse option is required

### Requirement: SplitPane clamps and resets ratio

`SplitPane` MUST clamp ratio changes to `[min_ratio, max_ratio]`.
`ResetRatio` MUST restore the configured reset ratio after clamping.

#### Scenario: ratio is clamped

- **WHEN** `SetRatio(95)` is applied with `max_ratio = 80`
- **THEN** the resulting ratio is `80`
- **AND** `RatioChanged` reports the clamped value

#### Scenario: reset uses configured default

- **WHEN** `ResetRatio` is applied
- **THEN** the ratio becomes the configured reset ratio
- **AND** the result is still within min / max

### Requirement: SplitPane emits deterministic resize events

`SplitPane` MUST emit resize events in a deterministic order for pointer and keyboard resize.
The order MUST be `ResizeStarted`, zero or more `RatioChanged`, then `ResizeEnded`.

#### Scenario: drag resize event order is stable

- **WHEN** the handle is dragged from 50% to 60%
- **THEN** `ResizeStarted` is emitted before `RatioChanged`
- **AND** `ResizeEnded` is emitted after the final ratio

### Requirement: SplitPane supports keyboard resizing

`SplitPane` MUST support keyboard resizing when resize mode includes keyboard.
Arrow keys MUST adjust ratio along the configured axis, and Enter or Space MUST reset ratio.

#### Scenario: keyboard resize follows axis

- **WHEN** a horizontal `SplitPane` handle receives ArrowRight
- **THEN** the ratio increases by the configured step
- **AND** the ratio remains clamped
