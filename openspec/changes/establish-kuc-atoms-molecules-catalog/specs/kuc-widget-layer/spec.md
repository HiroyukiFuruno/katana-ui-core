## ADDED Requirements

### Requirement: Widget layer exposes atoms and molecules first

KUC MUST expose the initial widget layer as atoms and molecules.
The MVP MUST NOT require consumers to use organisms, templates, or pages.
Future organisms and templates MUST be addable without breaking atoms and molecules APIs.

#### Scenario: consumer builds UI from atoms and molecules

- **WHEN** a consumer builds a UI from KUC widgets
- **THEN** the consumer can compose atoms and molecules directly
- **AND** the consumer does not need a page or template abstraction

### Requirement: Archived 01-24 requirements are reclassified

KUC MUST reclassify legacy 01-24 UI requirements into current atoms, molecules, or Storybook-internal categories.
Legacy Floem completion checkboxes MUST NOT be used as current KUC completion evidence.

#### Scenario: legacy task is referenced

- **WHEN** an implementation task references an archived 01-24 change
- **THEN** it identifies the current KUC category and required option/action/event/state coverage
- **AND** it does not mark the KUC component complete only because the legacy task was checked

### Requirement: Every component contract includes interaction coverage

Each atom and molecule MUST define options, actions, events, state, presets, preview behavior, settings behavior, automated tests, visual regression, and Storybook page requirements.

#### Scenario: component checklist is reviewed

- **WHEN** a component is proposed as complete
- **THEN** its checklist includes option, action, event, state, preset, preview, settings, automated test, visual regression, and Storybook page entries
- **AND** missing entries block completion

### Requirement: Atoms cover primitive building blocks

KUC atoms MUST cover primitive building blocks such as Text, Icon, Button, Input, Checkbox, Radio, Badge, Divider, Spacer, KeyCap, Spinner, ProgressBar, and ColorSwatch where adopted.
Atom contracts MUST explicitly define options, actions, events, state, presets, tests, visual regression, preview behavior, settings behavior, and Storybook pages.
Passive atoms MUST explicitly state that actions or events are `none`; absence of a row MUST block completion.

#### Scenario: atom inventory is checked

- **WHEN** the atom inventory is checked against this change
- **THEN** each adopted atom has a contract, tests, and a Storybook catalog page
- **AND** unimplemented atoms remain unchecked in tasks

#### Scenario: passive atom is reviewed

- **WHEN** Text, Icon, Divider, Spacer, or KeyCap has no direct user action
- **THEN** its contract still records actions and events as `none`
- **AND** visual, layout, theme, and state coverage remain required

### Requirement: Molecules compose atoms without stealing state

KUC molecules MUST compose atoms and additional model state without moving child component state into an uncontrolled global store.
Molecule contracts MUST explicitly define options, actions, events, state, presets, tests, visual regression, preview behavior, settings behavior, and Storybook pages.
Molecules MUST preserve parent and child state identities separately in automated tests and Storybook logs.

#### Scenario: molecule contains interactive atoms

- **WHEN** a Card, SearchBox, SelectBox, ColorPicker, CodeDiff, or TreeView composes child atoms
- **THEN** actions update the intended component state
- **AND** child state remains uniquely addressable

#### Scenario: molecule contract is reviewed

- **WHEN** a molecule is proposed as complete
- **THEN** its contract covers open, close, select, input, drag, dismiss, or mode switch behavior where applicable
- **AND** missing action, event, state, preset, test, visual regression, or Storybook coverage blocks completion
