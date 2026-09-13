# consumer-artifact-unicode-evidence Specification

## Purpose

Public consumer artifact execution obtains color-emoji Unicode evidence through KUC-owned platform pin resolution.

## Requirements

### Requirement: Consumer artifact has a KUC-owned Unicode evidence pin

A public consumer artifact issuer SHALL resolve its platform color-emoji pin within KUC before executing a stage.

#### Scenario: Consumer uses the default issuer

- **WHEN** a consumer creates `ConsumerArtifactPlanIssuer::new()` and executes a stage
- **THEN** the consumer does not supply a font path, hash, or fallback renderer
- **AND** KUC either captures evidence with its resolved pin or fails closed with `ConsumerArtifactPlanError::UnicodeEvidence(String)`, which contains the failure description.

#### Scenario: Consumer distinguishes the Unicode evidence failure

- **WHEN** a consumer executes a default-issued stage with `execute_next_with_evidence_error`
- **THEN** a Unicode capture failure retains `KucUnicodeColorGlyphEvidenceError` in `ConsumerArtifactPlanExecutionError::UnicodeEvidence`
- **AND** the consumer can distinguish `ColorEmojiUnavailable` without parsing a message or injecting a font policy
- **AND** other plan failures retain `ConsumerArtifactPlanError` in `ConsumerArtifactPlanExecutionError::Plan`.

#### Scenario: Existing consumer keeps the legacy error contract

- **WHEN** an existing consumer calls `execute_next`
- **THEN** its return type and the variants of `ConsumerArtifactPlanError` remain unchanged
- **AND** Unicode capture failures remain `ConsumerArtifactPlanError::UnicodeEvidence(String)`.
