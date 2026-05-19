## MODIFIED Requirements

### Requirement: Molecules compose atoms without stealing state

KUC molecules MUST compose atoms and additional model state without moving child component state into an uncontrolled global store.
Molecule contracts MUST explicitly define options, actions, events, state, presets, tests, visual regression, preview behavior, settings behavior, and Storybook pages.
Molecules MUST preserve parent and child state identities separately in automated tests and Storybook logs.
The panel layout family MUST distinguish:

- `SplitPane` for generic equal-weight pane splits
- `SideMenu` for menu item lists (vertical or icon-only)
- `CollapsiblePanel` for a single collapsible / icon-only / floating / resizable panel

These molecules MUST keep their option enums and state distinct so that a consumer choosing one does not have to extend another to fill the role.
KUC MUST NOT expose an `AppShell` organism or page template in this change.

#### Scenario: collapsible panel is requested

- **WHEN** a consumer needs an application sidebar with collapse and resize
- **THEN** the consumer uses `CollapsiblePanel`
- **AND** the consumer composes its own shell from layout primitives and molecules
- **AND** the consumer does not extend `SplitPane` with panel-specific options

#### Scenario: split pane is requested

- **WHEN** a consumer needs to split two equal-weight panes
- **THEN** the consumer uses `SplitPane`
- **AND** the consumer does not use `CollapsiblePanel` to render symmetric panes
