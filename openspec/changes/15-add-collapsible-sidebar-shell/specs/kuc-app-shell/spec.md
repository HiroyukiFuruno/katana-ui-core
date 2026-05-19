## ADDED Requirements

### Requirement: AppShell composes top/bottom bars and leading/trailing sidebars

`AppShell` MUST expose `top_bar`, `bottom_bar`, `leading_sidebar`, `trailing_sidebar`, and required `main`.
The shell's layout MUST be built on the KUC `Grid` layout model.
Each slot MUST be optional except `main`.

#### Scenario: shell with only main renders main full-bleed

- **WHEN** only `main` is provided
- **THEN** the shell renders main filling the available area
- **AND** no sidebar or bar slots reserve space

#### Scenario: leading sidebar collapses without affecting top bar

- **WHEN** `leading_sidebar` transitions to `Collapsed`
- **THEN** the main content takes the freed width
- **AND** the top/bottom bars keep their height unchanged

### Requirement: AppShell sidebars share state with the shell

`AppShell` MUST forward `CollapsibleSidebar` events from its slot sidebars to the consumer.
Sidebar state MUST remain owned by the embedded `CollapsibleSidebar` molecule; the shell MUST NOT duplicate that state.

#### Scenario: sidebar event propagates through shell

- **WHEN** a leading sidebar emits `ModeChanged`
- **THEN** the shell re-emits the same event with a slot tag (`leading`)
- **AND** consumers can subscribe at the shell boundary

#### Scenario: state ids stay distinct

- **WHEN** an `AppShell` mounts both leading and trailing sidebars
- **THEN** their `UiStateId`s are distinct
- **AND** automated tests can address each sidebar independently
