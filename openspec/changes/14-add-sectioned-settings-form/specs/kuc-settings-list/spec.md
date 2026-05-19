## ADDED Requirements

### Requirement: SettingsList groups fields under sections with typed controls

`SettingsList` MUST expose `sections: Vec<SettingsSection>` where each section contains an ordered `fields: Vec<SettingsField>`.
Each field MUST carry a `control: SettingsControl` that is a typed enum mapping to an existing KUC atom or molecule (Toggle, Select, Combo, Input, TextArea, Number, Chips, Radio, ColorPicker, Custom).

#### Scenario: section contains heterogeneous controls

- **WHEN** a section contains `Toggle`, `Select`, and `Input` controls
- **THEN** each field renders the typed atom or molecule for its control kind
- **AND** parent and child state ids remain distinct for each field's control

#### Scenario: custom control supplies its own subtree

- **WHEN** a field declares `control = Custom(UiTree)`
- **THEN** the molecule renders the provided subtree under the field label and description
- **AND** the embedded subtree's events still bubble through the molecule's routing

### Requirement: SettingsList supports collapsible sections

`SettingsList` MUST allow `collapsible: bool` and `default_collapsed: bool` per section.
Section headers MUST be activatable via keyboard (Enter / Space) when collapsible.

#### Scenario: collapsible section toggles via keyboard

- **WHEN** a section header is focused and the user presses Enter
- **THEN** the section's collapsed state toggles
- **AND** `SectionCollapsed { section_id, collapsed }` is emitted

#### Scenario: default_collapsed seeds initial state

- **WHEN** `default_collapsed = true` is set on a section
- **THEN** the section starts collapsed on first render
- **AND** subsequent user toggles override the default

### Requirement: SettingsList supports dirty visualization and reset

`SettingsList` MUST expose `dirty_visualization = None | Marker | Highlight`.
Each field MUST be able to declare a `reset_to_default` value; when current value differs from default, a reset affordance MUST be rendered.

#### Scenario: dirty marker appears next to label

- **WHEN** `dirty_visualization = Marker` and a field's current value differs from default
- **THEN** a marker dot is rendered next to the field label
- **AND** the consumer-supplied dirty status reflects in `dirty_field_ids`

#### Scenario: reset restores default and emits event

- **WHEN** the user activates the reset affordance on a dirty field
- **THEN** the field value is restored to `reset_to_default`
- **AND** `FieldReset { field_id }` is emitted

### Requirement: SettingsList query filter shows EmptyState on zero match

`SettingsList` MUST expose a `query: Option<String>` that filters sections and fields by case-insensitive substring match against labels, descriptions, and section titles.
When all sections and fields are filtered out, the molecule MUST render an `EmptyState`.

#### Scenario: query matches by description

- **WHEN** the query matches a field description but not its label
- **THEN** the field remains visible
- **AND** the section that contains it is also visible

#### Scenario: query results in no matches

- **WHEN** no sections or fields match the query
- **THEN** the molecule renders an `EmptyState` with a "no matches" message
- **AND** the EmptyState id is distinct from any field id
