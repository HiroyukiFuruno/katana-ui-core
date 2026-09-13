# consumer-artifact-unicode-evidence Specification

## Purpose

Public consumer artifact execution obtains color-emoji Unicode evidence through KUC-owned platform pin resolution.

## Requirements

### Requirement: Consumer artifact has a KUC-owned Unicode evidence pin

A public consumer artifact issuer SHALL resolve its platform color-emoji pin within KUC before executing a stage.

#### Scenario: Consumer uses the default issuer

- **WHEN** a consumer creates `ConsumerArtifactPlanIssuer::new()` and executes a stage
- **THEN** the consumer does not supply a font path, hash, or fallback renderer
- **AND** KUC either captures evidence with its resolved pin or fails closed with a typed platform-unavailable error.
