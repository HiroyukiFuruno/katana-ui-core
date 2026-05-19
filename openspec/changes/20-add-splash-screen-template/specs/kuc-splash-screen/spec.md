## ADDED Requirements

### Requirement: SplashScreen status is typed and transitions are observable

`SplashScreen` MUST expose `status = Idle | Loading { progress, label } | Error { message, retry }`.
Transitions between status values MUST emit `SplashStatusChanged { from, to }`.

#### Scenario: Idle to Loading emits transition

- **WHEN** status moves from `Idle` to `Loading`
- **THEN** `SplashStatusChanged { from: Idle, to: Loading }` is emitted
- **AND** the rendered output shows the loading affordance (spinner or progress bar)

#### Scenario: Loading to Error emits transition

- **WHEN** status moves from `Loading` to `Error`
- **THEN** the rendered output shows the error message and retry button if `retry = true`
- **AND** keyboard focus moves to the retry button

### Requirement: SplashScreen progress is indeterminate or determinate

When status is `Loading { progress: Some(f32), .. }`, the molecule MUST render a determinate progress bar clamped to `[0.0, 1.0]`.
When `progress = None`, the molecule MUST render an indeterminate spinner or progress indicator.

#### Scenario: progress=Some(0.42) shows 42% determinate

- **WHEN** progress is `Some(0.42)`
- **THEN** the rendered determinate bar shows 42% fill
- **AND** the optional label is rendered below the bar

#### Scenario: progress=None shows indeterminate spinner

- **WHEN** progress is `None`
- **THEN** the rendered output uses an indeterminate spinner
- **AND** the spinner respects reduced-motion downgrade

### Requirement: SplashScreen Error supports retry and cancel

When status is `Error { retry: true, .. }`, the molecule MUST render a retry button that emits `SplashRetried`.
The molecule MUST also support a cancel affordance that emits `SplashCancelled`.

#### Scenario: retry returns status to Idle

- **WHEN** the user presses the retry button
- **THEN** `SplashRetried` is emitted
- **AND** the consumer is expected to set status to `Idle` or `Loading` in the next render

#### Scenario: cancel exits splash

- **WHEN** the user presses cancel (or Esc when configured)
- **THEN** `SplashCancelled` is emitted
- **AND** the consumer decides whether to dismount the splash or proceed in degraded mode

### Requirement: SplashScreen background is typed and theme-aware

`SplashScreen` MUST expose `background = Solid(ColorToken) | Gradient { from, to, direction } | Image { source, opacity }`.
Solid and Gradient MUST use theme color tokens; Image MUST honor an opacity in `[0.0, 1.0]`.

#### Scenario: Gradient resolves through theme tokens

- **WHEN** `background = Gradient { from: ColorToken::AccentLow, to: ColorToken::AccentHigh, direction: Vertical }`
- **THEN** the rendered output uses the token values from the active theme
- **AND** light/dark theme switching updates the gradient automatically

#### Scenario: Image keeps text legible

- **WHEN** `background = Image { source, opacity: 0.4 }`
- **THEN** the image is rendered at 40% opacity behind the title and subtitle
- **AND** the foreground text tokens preserve contrast per the configured contrast threshold
