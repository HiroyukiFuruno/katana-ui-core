## MODIFIED Requirements

### Requirement: Atoms cover primitive building blocks

KUC atoms MUST cover primitive building blocks such as Text, Icon, Button, Input, TextArea, Checkbox, Radio, Badge, Chip, Divider, Spacer, KeyCap, Spinner, ProgressBar, and ColorSwatch where adopted.
Atom contracts MUST explicitly define options, actions, events, state, presets, tests, visual regression, preview behavior, settings behavior, and Storybook pages.
Passive atoms MUST explicitly state that actions or events are `none`; absence of a row MUST block completion.

#### Scenario: passive vs interactive atom is enforced

- **WHEN** a consumer needs an interactive or dismissible token-like element
- **THEN** the consumer uses `Chip` atom
- **AND** the consumer does not attach interactive callbacks to `Badge`

#### Scenario: atom inventory is checked

- **WHEN** the atom inventory is checked against this change
- **THEN** each adopted atom has a contract, tests, and a Storybook catalog page
- **AND** unimplemented atoms remain unchecked in tasks
