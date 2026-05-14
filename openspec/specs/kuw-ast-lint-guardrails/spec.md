# kuw-ast-lint-guardrails Specification

## Purpose
katana-ui-widget 固有の Storybook、OpenSpec、widget 責務境界の検査は、汎用 `kal check` ではなく、この repository の local guardrail で担保する。

## Requirements

### Requirement: Runtime APIs MUST not be test-only

`ops.rs` and `view.rs` runtime helper APIs used to satisfy OpenSpec tasks MUST be available outside `#[cfg(test)]` when the task describes runtime behavior.

#### Scenario: split pane drag API is test-only

- **WHEN** `layout/split/ops.rs` defines drag or reset behavior only under `#[cfg(test)]`
- **THEN** `just kuw-guardrails` MUST fail

### Requirement: Interactive components MUST expose operation callbacks

Components that represent user interaction MUST expose callback contracts for their primary action.

#### Scenario: toggle has no callback

- **WHEN** `Toggle` can change value but has no `on_change` or equivalent callback
- **THEN** `just kuw-guardrails` MUST fail

#### Scenario: accordion has no toggle callback

- **WHEN** `Accordion` can expand or collapse but has no `on_toggle` or equivalent callback
- **THEN** `just kuw-guardrails` MUST fail

### Requirement: Storybook MUST not leak strings for lifetime workarounds

Storybook pages MUST not use `Box::leak` to satisfy `'static` lifetimes for display labels.

#### Scenario: Storybook uses Box::leak

- **WHEN** `storybook/src/**/*.rs` contains `Box::leak`
- **THEN** `just kuw-guardrails` MUST fail

### Requirement: view.rs MUST carry view-ready responsibility

`view.rs` files MUST provide view-ready resolved structures or rendering models, not only unrelated constants and trivial getters.

#### Scenario: view file is helper-only

- **WHEN** `view.rs` contains only constants and scalar helper functions
- **THEN** `just kuw-guardrails` MUST report the file as helper-only

### Requirement: Done tasks MUST have evidence

OpenSpec tasks marked `[x]` MUST have minimal static evidence.

#### Scenario: task is checked without implementation file

- **WHEN** a task references an implementation path that does not exist
- **THEN** `just kuw-guardrails` MUST fail

#### Scenario: Storybook task is checked without page registration

- **WHEN** a Storybook task is marked `[x]` but the page is not registered
- **THEN** `just kuw-guardrails` MUST fail

### Requirement: File length findings MUST trigger responsibility review

`file-length` and `type-separation` findings MUST be treated as design boundary signals, not as line-count-only formatting problems.

#### Scenario: view method is moved without responsibility review

- **WHEN** a file-length finding is addressed only by moving `view()` to another file
- **THEN** the implementation task MUST remain incomplete until `types.rs`, `ops.rs`, `mod.rs`, `view.rs`, tests, and Storybook live cells are reviewed as separate responsibilities

#### Scenario: Storybook bypasses the widget runtime API

- **WHEN** Storybook reimplements interaction state instead of using the widget runtime API after a file-length finding
- **THEN** `just kuw-guardrails` MUST report the task as lacking evidence
