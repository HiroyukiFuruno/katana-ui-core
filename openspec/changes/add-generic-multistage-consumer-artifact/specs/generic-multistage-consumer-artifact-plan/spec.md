## ADDED Requirements

### Requirement: generic variable-length consumer artifact plan

The public `egui` API SHALL provide a versioned plan that issues an arbitrary non-empty sequence of distinct opaque consumer leaves from one generic KUC-owned full-editor root. The input SHALL accept only KUC-defined generic interaction/effect categories and opaque host-projected bindings.

#### Scenario: consumer issues more than ten stages

- **WHEN** a consumer provides eleven or more unique opaque leaves with supported generic interactions and current root revisions
- **THEN** KUC issues one plan with a distinct KUC stage ID for every binding without exposing host payloads, RawInput, renderer callbacks, or consumer-domain types

#### Scenario: invalid variable binding is rejected

- **WHEN** the plan is empty, duplicates a leaf, supplies an unsupported effect, has a stale binding revision, or binds another root
- **THEN** KUC returns a typed error before a frame or media file is emitted

### Requirement: exclusive per-stage evidence remains intact

KUC SHALL bind each variable-length stage to its own media, AccessKit evidence, Unicode/IME/measurement/hit-test evidence, and one-shot receipt. Missing, duplicate, stale, reused, or overwritten records SHALL fail closed.

#### Scenario: receipt and media cannot be reused

- **WHEN** a consumer consumes a receipt twice, cross-binds it, or writes into an existing stage media target
- **THEN** KUC rejects the operation with a typed error and emits no partial replacement
