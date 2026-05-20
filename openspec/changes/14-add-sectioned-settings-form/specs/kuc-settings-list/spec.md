## ADDED Requirements

### Requirement: SettingsList groups fields under sections with typed controls

`SettingsList` MUST expose `sections: Vec<SettingsSection>` where each section contains an ordered `fields: Vec<SettingsField>`.
Each field MUST carry a `control: SettingsControl` that is a typed enum mapping to an existing KUC atom or molecule (Toggle, Select, Combo, Input, TextArea, Number, Chips, Radio, ColorPicker, Custom).
`SettingsList` MUST expose `density = Compact | Default | Spacious` and render it as stable `UiSize` and style class values.

#### Scenario: section contains heterogeneous controls

- **WHEN** a section contains `Toggle`, `Select`, and `Input` controls
- **THEN** each field renders the typed atom or molecule for its control kind
- **AND** parent and child state ids remain distinct for each field's control

#### Scenario: density maps to numeric rendering props

- **WHEN** density is Compact, Default, or Spacious
- **THEN** the root node renders a stable size token for that density
- **AND** the root node carries a stable density style class for automated rendering contracts

#### Scenario: custom control supplies its own subtree

- **WHEN** a field declares `control = Custom(UiTree)`
- **THEN** the molecule renders the provided subtree under the field label and description
- **AND** the embedded subtree's events still bubble through the molecule's routing

### Requirement: SettingsList supports collapsible sections

`SettingsList` MUST allow `collapsible: bool` and `default_collapsed: bool` per section.
Section headers MUST be activatable via keyboard (Enter / Space) when collapsible.
Section headers MAY expose an icon and section footer; those values MUST be represented in the render tree.

#### Scenario: collapsible section toggles via keyboard

- **WHEN** a section header is focused and the user presses Enter
- **THEN** the section's collapsed state toggles
- **AND** `SectionCollapsed { section_id, collapsed }` is emitted

#### Scenario: default_collapsed seeds initial state

- **WHEN** `default_collapsed = true` is set on a section
- **THEN** the section starts collapsed on first render
- **AND** subsequent user toggles override the default

#### Scenario: section header exposes icon and footer

- **WHEN** a section declares an icon and footer
- **THEN** the header renders the icon next to the label
- **AND** the footer renders after the visible fields

### Requirement: SettingsList keeps focus and callback state internally

`SettingsList` MUST keep focused field state and callback log state inside the molecule.
Keyboard Tab from a field MUST move focus to the next visible field without requiring consumer-side state.

#### Scenario: focus field action updates internal state

- **WHEN** `FocusField { field_id }` is applied
- **THEN** `focused_field_id` changes inside the SettingsList state
- **AND** a `FieldFocused` event is recorded in the callback log

#### Scenario: Tab moves focus to next visible field

- **WHEN** a field receives keyboard Tab
- **THEN** the next visible field becomes focused
- **AND** the focus transition is emitted as a typed event

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
