## ADDED Requirements

### Requirement: Automated tests are the primary quality gate

KUC MUST treat automated tests and guards as the primary proof of component correctness.
Storybook screenshots and manual operation are supporting evidence only.
The quality gate MUST include core contract tests, atom contract tests, molecule contract tests, visual regression, input regression, and static guards.
KUC-specific guards MUST live in this repository and MUST NOT require adding KUC-only rules to `kal`.

#### Scenario: component completion is evaluated

- **WHEN** a component is marked complete
- **THEN** automated contract tests, layout tests, input tests, visual regression, and guards have passed
- **AND** Storybook-only evidence is insufficient

### Requirement: Visual regression covers component presets

KUC MUST provide visual regression coverage for required component presets.
The regression MUST check meaningful layout and pixel output instead of only checking that a window opened.
The regression MUST verify non-empty rendering, dedicated renderer coverage, layout bounds, theme application, operation differences, and readable Japanese/mixed/emoji text samples.
The regression MUST include numeric layout contracts for Storybook regions, Katana-style connected preset tab dimensions, zero-gap tab spacing, equal active/inactive tab height, active tab bottom accent, TreeView row alignment, scrollbar visibility, scroll viewport differences, component contract content, and preview card bounds.
Fixed-path screenshot generation MUST remove stale output before writing and MUST print generated file evidence such as byte size and modified timestamp.
Lowering thresholds to bypass failures MUST be treated as a gate failure.

#### Scenario: visual regression runs

- **WHEN** visual regression runs for a component preset
- **THEN** it verifies non-empty rendering, layout bounds, theme application, and expected visual changes
- **AND** placeholder or generic fallback rendering fails the gate

#### Scenario: Storybook layout contract runs

- **WHEN** the Storybook visual tests run
- **THEN** navigation, preview, inspector, preset tabs, TreeView rows, scrollbar, component contract, and component cards are checked against numeric bounds
- **AND** overlaps, hidden cards, degraded tab active-state pixels, missing TreeView open/close behavior, missing scrollbar control, missing scroll behavior, and button-like disconnected tabs fail the gate

#### Scenario: fixed screenshot output is refreshed

- **WHEN** Storybook writes a screenshot to a fixed output path
- **THEN** any stale file at that path is removed before rendering
- **AND** the command output includes file evidence so cached or old screenshots can be detected

### Requirement: Interaction regression covers Storybook operations

KUC MUST verify Storybook operation paths with automated interaction tests.
The tests MUST cover theme switching, preset switching, and visible navigation selection by asserting both state changes and rendered pixel differences.
Manual screenshots MUST NOT be accepted as the only proof that these operations work.

#### Scenario: Storybook operation regression runs

- **WHEN** a Storybook control is clicked in an automated test
- **THEN** the selected theme, preset, or page state changes
- **AND** the rendered canvas changes by an expected pixel threshold

### Requirement: Input regression covers Japanese IME and emoji

KUC MUST include regression coverage for keyboard input, Japanese IME committed text, and OS emoji input.
The regression MUST verify composition/preedit and committed text separately where the host exposes those states.
Fixed waits and manual-only confirmation MUST NOT be accepted as input regression evidence.

#### Scenario: input regression runs

- **WHEN** input regression runs for TextInput and related molecules
- **THEN** it verifies key input, Japanese committed text, and emoji input event conversion
- **AND** it verifies component state after input

### Requirement: Static guards enforce architectural constraints

KUC MUST include static guards for framework dependency leaks, state ownership violations, placeholder Storybook pages, uncovered options, uncovered events, uncovered actions, and missing Japanese/emoji validation.
The guards MUST also reject Storybook-only completion evidence and missing preset/test coverage.

#### Scenario: guard detects incomplete coverage

- **WHEN** a component lacks option, action, event, or state coverage
- **THEN** the guard fails
- **AND** the component cannot be marked complete

### Requirement: Legacy completion evidence is blocked

KUC MUST reject legacy framework-specific completion evidence as current KUC completion evidence unless the requirement is revalidated under this change.

#### Scenario: legacy evidence is used

- **WHEN** a task cites old Storybook smoke, old Floem screenshots, or old checkbox completion as final proof
- **THEN** the quality gate rejects that proof
- **AND** the task must point to current KUC tests and catalog evidence
