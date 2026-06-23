## ADDED Requirements

### Requirement: UI components expose typed interaction models

KUC MUST provide UI-specific props, state, and action models for interactive atom and molecule components.
Generic `UiInteractionState` MAY remain as a summary, but MUST NOT be the only source of truth for UI-specific behavior.

#### Scenario: action targets component-owned state

- **WHEN** two components share the same label
- **AND** a `UiAction` targets one component's `UiStateId`
- **THEN** only the matching component handles the action
- **AND** the result includes before / after summary and callback log

#### Scenario: CodeDiff has a typed model

- **WHEN** a `CodeDiff` component is created
- **THEN** it exposes sources, rows, display mode, highlight ranges, and collapsed unchanged blocks as KUC-owned model values
- **AND** it is not represented only by `item_count` or generic `value`

#### Scenario: ColorPicker has a typed model

- **WHEN** a `ColorPicker` component is created
- **THEN** it exposes RGBA value, channel controls, hue control, alpha control, readonly and disabled states
- **AND** operations update component-owned state without requiring an external store

### Requirement: Core facade controls theme font style and global UI state

KUC MUST provide an explicit core facade for theme, font role, style sheet, and global UI state.
The facade MUST be passed as a value and MUST NOT rely on a hidden mutable singleton.

#### Scenario: font roles stay platform neutral

- **WHEN** a component or render node specifies a font
- **THEN** it uses a KUC font role such as `body` or `code`
- **AND** the role resolves to `FontFamily::Proportional` or `FontFamily::Monospace`
- **AND** core does not store OS-specific font paths
- **AND** Storybook verifies Japanese text and emoji through the same text rendering path

#### Scenario: mixed language text stays vertically centered

- **WHEN** English, Japanese, mixed English/Japanese, and emoji text are rendered in equal-height UI rows
- **THEN** their visual centers align to the row center
- **AND** the verification fails when mixed text drifts vertically from English-only text

#### Scenario: global state does not replace component-owned state

- **WHEN** global UI state is configured through the facade
- **THEN** theme, focus target, active overlay, and modal stack can be shared across the tree
- **AND** component-specific values such as input text, checked state, selected index, and color value remain owned by each component instance

### Requirement: Storybook panel supports operation verification

The KUC Storybook MUST render through `katana-ui-core::panel::Panel` and allow story selection, theme switching, operation execution, and callback log inspection in the same visual surface.

#### Scenario: user selects and operates a story

- **WHEN** the Storybook panel selects an interactive story and runs its primary action
- **THEN** the preview reflects the updated component state
- **AND** the callback log records target state id, action name, and before / after summary

#### Scenario: theme is switched

- **WHEN** the Storybook panel switches between light and dark themes
- **THEN** root panel, navigation panel, preview panel, and story roots receive the new theme id
- **AND** screenshot output differs between the two themes

### Requirement: Visual renderer covers every required UI kind

The Storybook visual renderer MUST provide UI-kind-specific rendering for every required story.
Required stories MUST NOT fall back to a generic `node` hint.

#### Scenario: visual coverage is checked

- **WHEN** `just storybook-regression` runs
- **THEN** every required UI kind has a dedicated renderer or dedicated visual rule
- **AND** missing coverage fails the gate

#### Scenario: screenshot is checked

- **WHEN** screenshot output is generated
- **THEN** non-empty pixel coverage, theme difference, and operation-after difference are checked
- **AND** the evidence paths are recorded in docs

### Requirement: KUC guardrails stay repo-local

KUC-specific UI ownership and Storybook rules MUST be implemented inside this repository.
The change MUST NOT add rules to `kal` or rely on `kal` changes for KUC-specific behavior.

#### Scenario: guard location is checked

- **WHEN** KUC state ownership, Storybook panel operation, or visual fallback guards are added
- **THEN** the scripts live under this repository
- **AND** no `kal` repository changes are required
