## ADDED Requirements

### Requirement: Widget layer exposes atoms and molecules first

KUC MUST expose the initial widget layer as atoms and molecules.
The MVP MUST NOT require consumers to use organisms, templates, or pages.
Future organisms and templates MUST be addable without breaking atoms and molecules APIs.

#### Scenario: consumer builds UI from atoms and molecules

- **WHEN** a consumer builds a UI from KUC widgets
- **THEN** the consumer can compose atoms and molecules directly
- **AND** the consumer does not need a page or template abstraction

### Requirement: Material UI is the v0.1.0 visual and interaction baseline

KUC MUST use Material UI as the initial baseline for component appearance, hit target behavior, disabled / focused / selected states, and option grouping for Button, TextButton, SvgButton, Switch, Checkbox, Radio, and Tabs.
KUC MUST NOT expose React or Material UI compatible APIs.
The public contract MUST be Rust typed DTOs, presets, partial overrides, complete DTO overrides, internal state, and action-event-state automated tests.

#### Scenario: baseline is applied without compatibility API

- **WHEN** Button, Switch, Checkbox, Radio, or Tabs is proposed as complete
- **THEN** its contract identifies the Material UI behavior or appearance used as the initial baseline
- **AND** the implementation exposes KUC typed DTOs instead of React props or Material UI component names as API compatibility
- **AND** automated tests cover option resolution, state transitions, and action/event output

### Requirement: Common widget props are typed DTOs

KUC MUST model common widget props as typed DTOs.
The common DTO MUST include width, height, disabled, visible, tab-index, z-index, border, and focusable.
Presets MUST generate initial DTO values only.
Consumers MUST be able to use preset + partial override and complete DTO override.

#### Scenario: preset is partially overridden

- **WHEN** a consumer starts from a preset and overrides width, border, or focusable
- **THEN** only the overridden typed fields change
- **AND** unspecified fields keep the preset values

#### Scenario: complete DTO is supplied

- **WHEN** a consumer supplies a complete common props DTO
- **THEN** KUC resolves layout, visibility, focus, and stacking from that DTO
- **AND** preset values are not required to interpret the component

### Requirement: Archived 01-24 requirements are reclassified

KUC MUST reclassify legacy 01-24 UI requirements into current atoms, molecules, or Storybook-internal categories.
Legacy Floem completion checkboxes MUST NOT be used as current KUC completion evidence.

#### Scenario: legacy task is referenced

- **WHEN** an implementation task references an archived 01-24 change
- **THEN** it identifies the current KUC category and required option/action/event/state coverage
- **AND** it does not mark the KUC component complete only because the legacy task was checked

### Requirement: Every component contract includes interaction coverage

Each atom and molecule MUST define options, actions, events, state, presets, preview behavior, settings behavior, automated tests, numeric layout/rendering contracts, and Storybook page requirements.

#### Scenario: component checklist is reviewed

- **WHEN** a component is proposed as complete
- **THEN** its checklist includes option, action, event, state, preset, preview, settings, automated test, numeric layout/rendering contract, and Storybook page entries
- **AND** missing entries block completion

### Requirement: Atoms cover primitive building blocks

KUC atoms MUST cover primitive building blocks such as Text, Icon, Button, Input, Checkbox, Radio, Badge, Divider, Spacer, KeyCap, Spinner, ProgressBar, and ColorSwatch where adopted.
Atom contracts MUST explicitly define options, actions, events, state, presets, tests, numeric layout/rendering contracts, preview behavior, settings behavior, and Storybook pages.
Passive atoms MUST explicitly state that actions or events are `none`; absence of a row MUST block completion.

Button, TextButton, and SvgButton MUST be separate component contracts.
SvgButton MUST render as icon-only and MUST require an accessibility label.
Switch MUST provide a label + switch row component and MUST support whole-row click when enabled.

#### Scenario: atom inventory is checked

- **WHEN** the atom inventory is checked against this change
- **THEN** each adopted atom has a contract, tests, and a Storybook page
- **AND** unimplemented atoms remain unchecked in tasks

#### Scenario: passive atom is reviewed

- **WHEN** Text, Icon, Divider, Spacer, or KeyCap has no direct user action
- **THEN** its contract still records actions and events as `none`
- **AND** rendering, layout, theme, and state coverage remain required

#### Scenario: button variants are reviewed

- **WHEN** Button, TextButton, SvgButton, or IconTextButton is reviewed
- **THEN** each variant has its own option, action, event, state, preset, and typed DTO coverage
- **AND** SvgButton has no visible text label in the rendered button body

#### Scenario: switch row is activated

- **WHEN** a user clicks the label area in a Switch row
- **THEN** the Switch toggles through the same typed action path as clicking the switch body
- **AND** the resulting event and state transition are covered by automated tests

### Requirement: Molecules compose atoms without stealing state

KUC molecules MUST compose atoms and additional model state without moving child component state into an uncontrolled global store.
Molecule contracts MUST explicitly define options, actions, events, state, presets, tests, numeric layout/rendering contracts, preview behavior, settings behavior, and Storybook pages.
Molecules MUST preserve parent and child state identities separately in automated tests and Storybook logs.

#### Scenario: molecule contains interactive atoms

- **WHEN** a Card, SearchBox, SelectBox, ColorPicker, CodeDiff, or TreeView composes child atoms
- **THEN** actions update the intended component state
- **AND** child state remains uniquely addressable

#### Scenario: molecule contract is reviewed

- **WHEN** a molecule is proposed as complete
- **THEN** its contract covers open, close, select, input, drag, dismiss, or mode switch behavior where applicable
- **AND** missing action, event, state, preset, test, numeric layout/rendering contract, or Storybook coverage blocks completion

### Requirement: TreeView exposes configurable tree behavior

TreeView は Storybook navigation に使えるだけでなく、通常の molecule として構成できる option を MUST とする。
TreeView は垂直線の有無、垂直線の種類、太さ、directory icon、file icon、icon 表示可否、font role、theme id、空領域 context menu 表示可否、既定の開閉状態、開閉用 SVG icon、開閉発火領域を typed API として持つ必要がある。

#### Scenario: tree display options are configured

- **WHEN** consumer configures vertical lines, icons, font, theme, and default open state
- **THEN** TreeView keeps those values in typed model accessors
- **AND** Storybook can expose the same values in settings without ad hoc string parsing

#### Scenario: tree directory toggles like an accordion

- **WHEN** a directory node receives a configured toggle action
- **THEN** it expands or collapses according to the configured trigger area
- **AND** Accordion and TreeView can share the same disclosure semantics for icon only, icon + text, whole element, or text only triggers

#### Scenario: tree empty area context menu is controlled

- **WHEN** right click occurs on the empty area
- **THEN** TreeView opens a context menu only when that option is enabled
- **AND** disabling the option prevents hidden state mutation
