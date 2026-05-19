## MODIFIED Requirements

### Requirement: Atoms cover primitive building blocks

KUC atoms MUST cover primitive building blocks such as Text, Icon, Button, Input, TextArea, Checkbox, Radio, Badge, Chip, Divider, Spacer, KeyCap, ShortcutCombo, Spinner, ProgressBar, and ColorSwatch where adopted.
Atom contracts MUST explicitly define options, actions, events, state, presets, tests, visual regression, preview behavior, settings behavior, and Storybook pages.
Passive atoms MUST explicitly state that actions or events are `none`; absence of a row MUST block completion.
`KeyCap` MUST remain a single-key atom; multi-key combinations MUST use `ShortcutCombo`.

#### Scenario: multi-key combination is requested

- **WHEN** a consumer needs to render a key combination such as `Cmd+Shift+P`
- **THEN** the consumer uses `ShortcutCombo`
- **AND** the consumer does not extend `KeyCap` with modifiers

#### Scenario: atom inventory is checked

- **WHEN** the atom inventory is checked against this change
- **THEN** each adopted atom has a contract, tests, and a Storybook catalog page
- **AND** unimplemented atoms remain unchecked in tasks
