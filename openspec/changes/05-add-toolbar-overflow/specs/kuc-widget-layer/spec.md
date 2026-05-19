## MODIFIED Requirements

### Requirement: Molecules compose atoms without stealing state

KUC molecules MUST compose atoms and additional model state without moving child component state into an uncontrolled global store.
Molecule contracts MUST explicitly define options, actions, events, state, presets, tests, visual regression, preview behavior, settings behavior, and Storybook pages.
Molecules MUST preserve parent and child state identities separately in automated tests and Storybook logs.
For `Toolbar` specifically, the molecule MUST own action priority, accelerator, group, split, density, display mode, and overflow strategy options as typed values rather than string/dynamic constructs.

#### Scenario: molecule contains interactive atoms

- **WHEN** a Card, SearchBox, SelectBox, ColorPicker, CodeDiff, TreeView, or Toolbar composes child atoms
- **THEN** actions update the intended component state
- **AND** child state remains uniquely addressable

#### Scenario: toolbar exposes typed actions, not strings

- **WHEN** a consumer constructs a `Toolbar` with split actions, accelerators, priorities, and groups
- **THEN** every option appears as a typed value in the public API
- **AND** the static linter rejects accelerators or priorities encoded as ad-hoc strings
