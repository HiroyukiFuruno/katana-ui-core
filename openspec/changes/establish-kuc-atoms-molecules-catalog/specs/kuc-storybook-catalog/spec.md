## ADDED Requirements

### Requirement: Storybook is an interactive component catalog

KUC Storybook MUST be a component catalog for visual and manual operation checks.
It MUST NOT be the sole proof that components are correct.

#### Scenario: Storybook role is inspected

- **WHEN** developers read the Storybook documentation
- **THEN** Storybook is described as a catalog and manual confirmation surface
- **AND** automated tests and guards are described as the quality gate

### Requirement: Catalog navigation uses KUC TreeView

The Storybook left pane MUST use KUC TreeView to show provided components.
The tree MUST group entries by atomic design category and nested component category.
The tree MUST include atoms, molecules, and Storybook-internal sections.
Selecting a TreeView item MUST update preview, settings, state summary, event history, and action history context.

#### Scenario: component list is displayed

- **WHEN** Storybook opens
- **THEN** the left pane displays atoms and molecules in a nested TreeView
- **AND** selecting a TreeView item changes the preview page

### Requirement: Component pages include preview and settings

Every component page MUST include a preview area and a settings area.
Settings MUST allow option values to be changed on screen and reflected in preview.
Every component page MUST also expose state summary, event history, action history, and requirement status.
Placeholder text-only pages MUST fail the Storybook catalog gate.

#### Scenario: option value is changed

- **WHEN** a user changes an option in settings
- **THEN** the preview updates to reflect the option
- **AND** the state and action logs record the change

### Requirement: Presets use KUC Tabs

Every component page MUST provide multiple presets where meaningful.
Preset switching MUST use KUC Tabs.
Preset switching MUST update preview, settings initial values, state summary, and current preset display in the action and event history.

#### Scenario: preset is switched

- **WHEN** a user switches a component preset tab
- **THEN** the preview and settings reflect the selected preset
- **AND** the selected preset is visible in the state summary

### Requirement: Catalog exposes state, event, and action history

Storybook MUST show component state, event history, and action history for interactive components.
Settings changes MUST be recorded as actions; hidden state mutation without an action record MUST NOT be used for catalog interaction.

#### Scenario: component is operated

- **WHEN** a user clicks, types, opens, closes, selects, drags, or changes a component
- **THEN** Storybook records the action, emitted event, target state id, and before/after summary
- **AND** the record is visible on the component page

### Requirement: Storybook may use internal organisms

Storybook MUST keep any larger internal structures, such as catalog shell, navigation tree, preview workspace, and settings inspector, inside the Storybook implementation boundary.
These internal organisms MUST NOT become public widget API in this change.

#### Scenario: internal shell is implemented

- **WHEN** Storybook needs a larger structure for navigation or inspection
- **THEN** it can implement an internal organism
- **AND** public KUC widget scope remains atoms and molecules
