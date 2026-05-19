## ADDED Requirements

### Requirement: TitleBar exposes style, position, height, slots, and controls

`TitleBar` molecule MUST expose `style = Native | EmbeddedNative | Custom`, `position = Leading | Trailing | Auto`, `height = Compact | Default | Tall`, `leading_slot`, `center_slot`, `trailing_slot`, and `controls`.
Each option MUST be a typed enum or typed slot; stringly typed values MUST be rejected.

#### Scenario: macOS-style preset uses Leading position

- **WHEN** `position = Leading` and the adapter reports macOS
- **THEN** the rendered window controls (close/minimize/maximize) appear on the leading side
- **AND** the title is centered in the remaining space if `center_slot` is empty

#### Scenario: Windows-style preset uses Trailing position

- **WHEN** `position = Trailing` and the adapter reports Windows
- **THEN** the rendered window controls appear on the trailing side
- **AND** the title is left-aligned next to leading_slot if present

### Requirement: TitleBar auto-carves drag regions around interactive content

`TitleBar` MUST publish `draggable_regions` to the adapter as the bounding rects of the title bar minus interactive elements (controls, buttons inside slots, breadcrumbs).
Interactive child elements MUST automatically subtract from the drag region.

#### Scenario: control button does not extend the drag region

- **WHEN** a window control button is rendered inside the title bar
- **THEN** its rect is excluded from `draggable_regions`
- **AND** the adapter receives the carved-out regions only

#### Scenario: slot button is excluded by static analysis

- **WHEN** a `leading_slot` contains a `Button` atom
- **THEN** the button's rect is excluded from `draggable_regions`
- **AND** non-interactive text in the same slot remains draggable

### Requirement: TitleBar controls dispatch typed WindowCommand events

`TitleBar` controls (`Minimize`, `Maximize`/`Restore`, `Close`, `EnterFullscreen`, `ExitFullscreen`) MUST emit `ControlPressed { which }` and forward the corresponding `WindowCommand` to the runtime.
Custom controls MUST emit `ControlPressed` with their custom id; the consumer is responsible for further dispatch.

#### Scenario: Close button forwards WindowCommand::Close

- **WHEN** the user presses the Close control
- **THEN** `ControlPressed { which: Close }` is emitted
- **AND** the runtime receives `WindowCommand::Close` for the owning window

#### Scenario: custom control routes via consumer

- **WHEN** the user presses a `CustomList(["settings"])` control
- **THEN** `ControlPressed { which: Custom("settings") }` is emitted
- **AND** no implicit window command is dispatched; the consumer handles the action

### Requirement: TitleBar supports auto-hide during fullscreen

When the window is in fullscreen, `TitleBar` MUST be hideable via `WindowCommand::EnterFullscreen`.
Hovering near the top of the screen MUST optionally re-show the title bar based on a configurable policy.

#### Scenario: enter fullscreen hides title bar by default

- **WHEN** `EnterFullscreen` is dispatched
- **THEN** the title bar is hidden from layout (height collapses to zero)
- **AND** the `WindowState.fullscreen = true`

#### Scenario: pointer near top re-shows title bar

- **WHEN** `auto_show_on_hover = true` and the pointer enters the top edge zone during fullscreen
- **THEN** the title bar re-emerges with the configured animation
- **AND** leaving the zone after the close delay re-hides it
