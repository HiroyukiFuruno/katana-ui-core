## MODIFIED Requirements

### Requirement: Atoms cover primitive building blocks

KUC atoms MUST cover primitive building blocks such as Text, Icon, Button, Input, TextArea, Checkbox, Radio, Badge, Divider, Spacer, KeyCap, Spinner, ProgressBar, and ColorSwatch where adopted.
Atom contracts MUST explicitly define options, actions, events, state, presets, tests, visual regression, preview behavior, settings behavior, and Storybook pages.
Passive atoms MUST explicitly state that actions or events are `none`; absence of a row MUST block completion.
`Input` and `TextArea` MUST remain distinct atoms: `Input` for single-line entry, `TextArea` for multi-line entry with auto-grow and IME composition.

#### Scenario: atom inventory is checked

- **WHEN** the atom inventory is checked against this change
- **THEN** each adopted atom has a contract, tests, and a Storybook catalog page
- **AND** unimplemented atoms remain unchecked in tasks

#### Scenario: single-line vs multi-line is enforced

- **WHEN** a consumer needs multi-line input
- **THEN** the consumer uses `TextArea` atom
- **AND** the consumer does not configure `Input` to accept newlines or grow vertically
