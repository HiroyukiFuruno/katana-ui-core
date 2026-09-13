## ADDED Requirements

### Requirement: Consumer artifact has a KUC-owned Unicode evidence pin

A public consumer artifact issuer SHALL resolve its platform color-emoji pin within KUC before executing a stage.

#### Scenario: Consumer uses the default issuer

- **WHEN** a consumer creates `ConsumerArtifactPlanIssuer::new()` and executes a stage with `execute_next_with_evidence_error`
- **THEN** the consumer does not supply a font path, hash, or fallback renderer
- **AND** KUC either captures evidence with its resolved pin or fails closed with `ConsumerArtifactPlanExecutionError::UnicodeEvidence`, retaining the typed `KucUnicodeColorGlyphEvidenceError` cause.

#### Scenario: Existing consumer keeps the legacy error contract

- **WHEN** an existing consumer calls `execute_next`
- **THEN** its return type and the variants of `ConsumerArtifactPlanError` remain unchanged
- **AND** Unicode capture failures remain `ConsumerArtifactPlanError::UnicodeEvidence(String)`.
