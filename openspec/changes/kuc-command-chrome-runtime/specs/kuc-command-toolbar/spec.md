## ADDED Requirements

### Requirement: Command toolbar SHALL compose existing toolbar contracts without breaking them

KUC SHALL provide new `CommandChromeAction`, `CommandChromeToolbar`, and
`FloatingCommandToolbar` DTOs that compose the existing Toolbar action, group,
overflow, and placement behavior. KUC SHALL NOT add required fields to existing
public struct literals or variants to existing public action/event enums.

#### Scenario: existing toolbar consumer remains source-compatible

- **WHEN** an existing consumer constructs and serializes `ToolbarAction` and handles `ToolbarEvent`
- **THEN** it compiles without a required new field or enum match arm
- **AND** command chrome behavior is available through the new additive DTOs

### Requirement: Command chrome actions SHALL support host-provided SVG icons

`CommandChromeAction` SHALL accept an optional `UiIconProps`, visible label,
accessible name, tooltip, enabled state, group, and optional dropdown without
interpreting the action identity as a host command.

#### Scenario: opaque host icon is preserved through toolbar presentation

- **WHEN** a consumer supplies an action id and `UiIconProps`
- **THEN** the command toolbar retains that icon prop through its presentation and event path
- **AND** KUC emits only the supplied generic action id on activation

### Requirement: Icon-only command actions SHALL be accessible and never use glyph fallback

`CommandChromeDisplayMode::IconOnly` SHALL require a non-empty icon and a
non-empty accessible name supplied as tooltip or accessibility label. A missing
icon or name SHALL be a contract violation; labels, Unicode characters, and
emoji SHALL NOT silently replace the icon.

#### Scenario: icon-only action missing an icon is rejected

- **WHEN** a command chrome action is configured as icon-only without `UiIconProps`
- **THEN** contract validation reports the action id and missing icon
- **AND** the adapter does not emit an activation target for that invalid action

### Requirement: Floating toolbar placement and dismissal SHALL be deterministic

`FloatingCommandToolbar` SHALL receive anchor and viewport rectangles, use the
shared placement engine, clamp to the viewport, and emit typed events for
activation, dropdown lifecycle, focus retention, outside/editor click close,
escape close, and focus return. It SHALL not inspect editor coordinates or
Markdown selection semantics.

#### Scenario: toolbar closes on editor click and restores requested focus

- **WHEN** an open floating toolbar receives an editor-click interaction outside its bounds
- **THEN** it emits a typed close event with editor-click reason
- **AND** it emits the configured generic focus-return request exactly once

### Requirement: Disabled command actions SHALL not emit host work

The command toolbar SHALL keep disabled primary and secondary dropdown halves
non-interactive. It SHALL not emit command events for disabled actions or
modify host state.

#### Scenario: disabled split primary remains inactive

- **WHEN** a split action has a disabled primary half and an enabled dropdown half
- **THEN** activating the primary half produces no command event
- **AND** activating the dropdown half emits only the typed dropdown event
