## ADDED Requirements

### Requirement: Startup state is typed and transitions are observable

`StartupStatePanel` or equivalent composition contract MUST expose `state = Idle | Loading { progress, label } | Error { message, retry }`.
Transitions between state values MUST emit `StartupStateChanged { from, to }`.
KUC MUST NOT expose a full-screen splash template.

#### Scenario: Idle to Loading emits transition

- **WHEN** state moves from `Idle` to `Loading`
- **THEN** `StartupStateChanged { from: Idle, to: Loading }` is emitted
- **AND** the rendered output shows the loading affordance with `ProgressBar`, `Spinner`, or `LoadingDots`

#### Scenario: Loading to Error emits transition

- **WHEN** state moves from `Loading` to `Error`
- **THEN** the rendered output shows the error message and retry button if `retry = true`
- **AND** keyboard focus moves to the retry button

### Requirement: Startup progress is indeterminate or determinate

When state is `Loading { progress: Some(u8), .. }`, the molecule MUST render a determinate progress bar clamped to `[0, 100]`.
When `progress = None`, the molecule MUST render an indeterminate spinner or progress indicator.

#### Scenario: progress=42 shows 42% determinate

- **WHEN** progress is `Some(42)`
- **THEN** the rendered determinate bar shows 42% fill
- **AND** the optional label is rendered near the bar

#### Scenario: progress=None shows indeterminate spinner

- **WHEN** progress is `None`
- **THEN** the rendered output uses an indeterminate spinner
- **AND** the spinner respects reduced-motion downgrade

### Requirement: Startup error supports retry and cancel

When state is `Error { retry: true, .. }`, the composition MUST render a retry action that emits `StartupRetried`.
The composition MAY support a cancel affordance that emits `StartupCancelled`.

#### Scenario: retry reports intent

- **WHEN** the user presses the retry button
- **THEN** `StartupRetried` is emitted
- **AND** the consumer is expected to set state to `Idle` or `Loading` in the next render

#### Scenario: cancel reports intent

- **WHEN** the user presses cancel or Esc when configured
- **THEN** `StartupCancelled` is emitted
- **AND** the consumer decides whether to dismount the startup view or proceed in degraded mode

### Requirement: Full-screen template concerns stay out of KUC

KUC MUST NOT expose background image, full-screen size, centered window layout, brand logo placement, or app boot lifecycle as part of this change.

#### Scenario: consumer needs a branded splash screen

- **WHEN** consumer needs logo, background image, and full-screen placement
- **THEN** consumer composes those layout concerns outside KUC
- **AND** KUC only supplies atoms / molecules for state, progress, error, and actions
