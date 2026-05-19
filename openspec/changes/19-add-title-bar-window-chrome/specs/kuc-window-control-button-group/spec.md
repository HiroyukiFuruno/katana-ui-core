## ADDED Requirements

### Requirement: WindowControlButtonGroup exposes typed controls

`WindowControlButtonGroup` molecule MUST expose `controls`, `position`, `size`, and `visibility` as typed options.
It MUST NOT expose title text, breadcrumb slot, draggable region, native decoration, or full title bar layout.

#### Scenario: macOS-style preset uses Leading position

- **WHEN** `position = Leading`
- **THEN** the rendered window controls appear on the leading side
- **AND** no title, breadcrumb, or drag region props are required

#### Scenario: Windows-style preset uses Trailing position

- **WHEN** `position = Trailing`
- **THEN** the rendered window controls appear on the trailing side
- **AND** the consumer decides where the group is mounted in its header

### Requirement: WindowControlButtonGroup does not own draggable regions

KUC MUST NOT compute or publish draggable title bar regions in this molecule.
Draggable regions are adapter / consumer responsibility.

#### Scenario: public API is inspected

- **WHEN** the public API for `WindowControlButtonGroup` is checked
- **THEN** no `draggable_regions` field is exposed
- **AND** no title bar slot is exposed

### Requirement: Window controls dispatch typed intent events

`WindowControlButtonGroup` controls (`Minimize`, `Maximize`, `Restore`, `Close`) MUST emit `ControlPressed { which }`.
KUC MUST NOT directly call OS window APIs.

#### Scenario: Close button emits close intent

- **WHEN** the user presses the Close control
- **THEN** `ControlPressed { which: Close }` is emitted
- **AND** the consumer or adapter decides how to close the window

#### Scenario: hover visibility changes

- **WHEN** `visibility = FullscreenHover` and hover becomes true
- **THEN** `VisibilityChanged` is emitted
- **AND** the rendered state becomes visible without changing consumer layout ownership
