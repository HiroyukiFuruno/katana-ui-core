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
Lowering thresholds to bypass failures MUST be treated as a gate failure.

#### Scenario: visual regression runs

- **WHEN** visual regression runs for a component preset
- **THEN** it verifies non-empty rendering, layout bounds, theme application, and expected visual changes
- **AND** placeholder or generic fallback rendering fails the gate

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
