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
The navigation TreeView MUST expose settings for vertical line visibility/style/width, directory and file icons, font role, theme, empty-area context menu, default open state, and directory toggle trigger area.
The navigation TreeView MUST support category open/close state, selected-row rendering, and aligned disclosure icon, file/folder icon, and row text within the same row box.

#### Scenario: component list is displayed

- **WHEN** Storybook opens
- **THEN** the left pane displays atoms and molecules in a nested TreeView
- **AND** selecting a TreeView item changes the preview page
- **AND** clicking a category row opens or closes its child rows

#### Scenario: TreeView row alignment is inspected

- **WHEN** Storybook renders a navigation row
- **THEN** the disclosure icon, directory/file icon, and label text share the same vertical center
- **AND** the click target matches the visible row bounds

#### Scenario: navigation tree options are changed

- **WHEN** a user changes TreeView settings on the Storybook page
- **THEN** the navigation preview reflects line, icon, font, theme, context menu, default open, and trigger-area options
- **AND** action and event history records the changed option

### Requirement: Component pages include preview and settings

Every component page MUST include a preview area and a settings area.
Settings MUST allow option values to be changed on screen and reflected in preview.
Every component page MUST also expose state summary, event history, action history, and requirement status.
Every component page MUST show a readable component contract for option, action, event, state, preset, preview, settings, test, and visual coverage.
Placeholder text-only pages MUST fail the Storybook catalog gate.

#### Scenario: option value is changed

- **WHEN** a user changes an option in settings
- **THEN** the preview updates to reflect the option
- **AND** the state and action logs record the change

### Requirement: Presets use KUC Tabs

Every component page MUST provide multiple presets where meaningful.
Preset switching MUST use KUC Tabs.
Preset switching MUST update preview, settings initial values, state summary, and current preset display in the action and event history.
Preset tabs MUST have measured width, height, gap, and active-state indicator values so visual regressions can detect degraded tab styling.
Preset tabs MUST follow the Katana app tab direction: no visual gaps, equal active/inactive height, and a selected bottom-edge accent.
The active preset tab MUST NOT grow larger than inactive tabs and MUST NOT look like an isolated button in a button row.
Preset state MUST be index-based so every visible tab can become selected.
The selected tab highlight MUST be shown on the bottom edge.
The Storybook viewport MUST support vertical scrolling for content that exceeds the visible window.
The Storybook viewport MUST expose scrollbar visibility as state so visible and hidden scrollbar modes can be checked.

#### Scenario: preset is switched

- **WHEN** a user switches a component preset tab
- **THEN** the preview and settings reflect the selected preset
- **AND** the selected preset is visible in the state summary

#### Scenario: preset tab layout is measured

- **WHEN** Storybook visual regression runs
- **THEN** tab bounds, zero-gap spacing, equal active/inactive height, active bottom indicator pixels, and scroll viewport differences are checked by automated tests
- **AND** an unstyled, overlapping, or button-like tab strip fails the gate

#### Scenario: scrollbar visibility is changed

- **WHEN** the scrollbar visibility option is toggled
- **THEN** the viewport scrollbar appears or disappears from the rendered Storybook panel
- **AND** the state is available to snapshot generation

#### Scenario: later preset tabs are selected

- **WHEN** a user clicks the third or fourth preset tab
- **THEN** that preset becomes the selected tab
- **AND** the preview state changes from the previous preset

### Requirement: Storybook window supports desktop resizing

The Storybook main window MUST be resizable so macOS zoom/maximize can be used for manual inspection.
Modal windows MAY keep fixed sizing when their placement behavior is under test.

#### Scenario: main window is opened

- **WHEN** Storybook opens its main window
- **THEN** the window options allow resize and macOS zoom/maximize operation
- **AND** this behavior is covered by an automated contract test

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
