## ADDED Requirements

### Requirement: EmptyState exposes heading, optional body, optional actions

`EmptyState` molecule MUST require a non-empty `heading` and accept optional `body`, `icon`, `illustration`, `primary_action`, and `secondary_action`.
`icon` and `illustration` MUST be mutually exclusive; setting both MUST fail the contract test.

#### Scenario: heading is missing

- **WHEN** `heading` is an empty string
- **THEN** the contract test fails
- **AND** the Storybook page rejects the preset as invalid

#### Scenario: both icon and illustration set

- **WHEN** both `icon` and `illustration` are set
- **THEN** the contract test fails
- **AND** the static linter directs the consumer to choose one

### Requirement: EmptyState primary and secondary actions emit typed events

`EmptyState` MUST emit `EmptyStateActioned { id }` for primary and secondary actions.
Both actions MUST be optional; absence MUST simply hide the corresponding button.

#### Scenario: primary action pressed

- **WHEN** primary action is pressed
- **THEN** `EmptyStateActioned { id: Primary }` is emitted
- **AND** focus stays on the action unless the consumer dispatches a follow-up command

#### Scenario: no actions configured

- **WHEN** both actions are absent
- **THEN** the action row is not rendered
- **AND** the layout collapses to zero height in the action area

### Requirement: EmptyState tone × size × alignment matrix is stable

`EmptyState` MUST keep `tone = Neutral | Subtle | Accent | Warning | Danger`, `size = Compact | Default | Large`, and `alignment = Center | Leading` as typed enums.
Layout snapshots MUST remain stable under the same `(tone, size, alignment)` triple.

#### Scenario: tone changes color but not layout

- **WHEN** only `tone` is changed
- **THEN** the layout snapshot (heading rect, body rect, action rect) is identical
- **AND** only color tokens differ in the rendered output

#### Scenario: size changes layout but tone semantics are preserved

- **WHEN** `size` is changed
- **THEN** layout dimensions update accordingly
- **AND** the rendered tone color tokens stay the same for the same `tone` value

### Requirement: EmptyState is embeddable in list molecules

`DiagnosticsList`, `SelectionList`, `TreeView`, `CommandPalette`, and `SearchBox` MUST be able to embed `EmptyState` when their data is empty after applying filters.
Each embedding MUST keep `EmptyState`'s `UiStateId` distinct from the parent list's state.

#### Scenario: SelectionList renders EmptyState when empty

- **WHEN** a `SelectionList` has zero rows after filtering
- **THEN** it renders an `EmptyState` molecule in the body slot
- **AND** the parent and child state ids remain distinct in tests
