## ADDED Requirements

### Requirement: v0.1.0 DoD は利用側 app UI 構築可能性で判定する

KUC v0.1.0 MUST use the ability for `katana` and `katana-chat-ui` to build app UI only with `katana-ui-core` as the DoD.
This judgment MUST be verified by public API contracts、automated tests、numeric layout/rendering contracts、interaction tests、input regression、state/event tests、static guards.
Image-based evidence MUST NOT be used for release readiness.

#### Scenario: v0.1.0 DoD を検証する

- **WHEN** v0.1.0 の品質ゲートを実行する
- **THEN** `katana` と `katana-chat-ui` 向けに必要な atoms、molecules、panel、event、state、layout 契約が検証される
- **AND** Storybook だけを根拠に完了扱いしない

### Requirement: scrollbar と drag は品質ゲート対象である

KUC MUST verify each panel's scrollbar visibility、track bounds、thumb bounds、offset、drag state in the quality gate.
Implementations that mix parent panel and child panel scroll state MUST fail the gate.

#### Scenario: scrollbar regression を検証する

- **WHEN** scrollbar regression が実行される
- **THEN** panel ごとの offset、thumb bounds、drag 後差分が確認される
- **AND** 別 panel の scroll state が変わった場合は失敗する

### Requirement: Automated tests are the release quality gate

KUC MUST treat automated tests and guards as the primary proof of component correctness.
Storybook is an interactive feedback surface and MUST NOT make users responsible for verification.
The quality gate MUST include core contract tests, atom contract tests, molecule contract tests, numeric layout/rendering contracts, interaction tests, input regression, state/event tests, and static guards.
KUC-specific guards MUST live in this repository and MUST NOT require adding KUC-only rules to `kal`.

#### Scenario: component completion is evaluated

- **WHEN** a component is marked complete
- **THEN** automated contract tests, layout tests, rendering contract tests, interaction/input/state/event tests, and guards have passed
- **AND** Storybook-only or image-based evidence is rejected

### Requirement: Numeric rendering contracts cover component presets

KUC MUST provide numeric layout/rendering contract coverage for required component presets.
The regression MUST check meaningful layout and render-model output instead of only checking that a window opened.
The regression MUST verify non-empty rendering commands, dedicated renderer coverage, layout bounds, theme application, operation differences, and readable Japanese/mixed/emoji text metrics.
The regression MUST include numeric layout contracts for Storybook regions, Katana-style connected preset tab dimensions, zero-gap tab spacing, active tab accent markers, TreeView row alignment, scrollbar visibility, panel-local scroll viewport differences, selected component contract content, selected component detail bounds, and selected preview rendering changes after settings or action operations.
Lowering thresholds to bypass failures MUST be treated as a gate failure.

#### Scenario: numeric rendering contract runs

- **WHEN** numeric rendering contract tests run for a component preset
- **THEN** they verify non-empty rendering commands, layout bounds, theme application, and expected render-model changes
- **AND** placeholder or generic fallback rendering fails the gate

#### Scenario: Storybook layout contract runs

- **WHEN** the Storybook layout/rendering contract tests run
- **THEN** navigation, selected preview, inspector, selected component contract, preset tabs, TreeView rows, and scrollbar are checked against numeric bounds
- **AND** overlaps, degraded tab active-state pixels, missing selected component detail, missing TreeView open/close behavior, missing scrollbar control, missing panel-local scroll behavior, unchanged preview rendering after operation, and button-like disconnected tabs fail the gate

### Requirement: Interaction regression covers Storybook operations

KUC MUST verify Storybook operation paths with automated interaction tests.
The tests MUST cover theme switching, preset switching, visible navigation selection, settings changes, rendered hit-target operations, and panel-local scrolling by asserting state, event, action, input, and render-model changes.
Image-based confirmation MUST NOT be accepted as proof that these operations work.

#### Scenario: Storybook operation regression runs

- **WHEN** a Storybook control is clicked in an automated test
- **THEN** the selected theme, preset, or page state changes
- **AND** the selected preview render model changes by an expected numeric threshold

#### Scenario: Storybook component interaction regression runs

- **WHEN** a component preview hit target or settings control is operated
- **THEN** option, action, event, and state evidence changes
- **AND** the selected preview body renders a meaningful difference

### Requirement: Input regression covers Japanese IME and emoji

KUC MUST include regression coverage for keyboard input, Japanese IME committed text, and OS emoji input.
The regression MUST verify composition/preedit and committed text separately where the host exposes those states.
Fixed waits and manual-only confirmation MUST NOT be accepted as input regression evidence.

#### Scenario: input regression runs

- **WHEN** input regression runs for TextInput and related molecules
- **THEN** it verifies key input, Japanese committed text, and emoji input event conversion
- **AND** it verifies component state after input

### Requirement: Static guards enforce architectural constraints

KUC MUST include static guards for framework dependency leaks, state ownership violations, placeholder Storybook pages, static all-components catalog pages, shared panel scroll state, uncovered options, uncovered events, uncovered actions, and missing Japanese/emoji validation.
The guards MUST also reject Storybook-only completion evidence and missing preset/test coverage.

#### Scenario: guard detects incomplete coverage

- **WHEN** a component lacks option, action, event, or state coverage
- **THEN** the guard fails
- **AND** the component cannot be marked complete

### Requirement: Legacy completion evidence is blocked

KUC MUST reject legacy framework-specific completion evidence as current KUC completion evidence unless the requirement is revalidated under this change.

#### Scenario: legacy evidence is used

- **WHEN** a task cites old Storybook smoke, old Floem image evidence, or old checkbox completion as final proof
- **THEN** the quality gate rejects that proof
- **AND** the task must point to current KUC tests and automated coverage evidence
