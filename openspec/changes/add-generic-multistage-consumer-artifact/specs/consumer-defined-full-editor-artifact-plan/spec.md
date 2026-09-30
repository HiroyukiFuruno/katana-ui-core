## MODIFIED Requirements

### Requirement: consumer-defined generic artifact plan

The public `egui` API SHALL retain the v1 fixed full-editor plan and SHALL additionally accept a versioned variable-length generic plan. Both plans contain only opaque leaf IDs, KUC-defined generic interaction/effect classes, opaque host-projected tokens, and KUC-issued stage bindings. The API MUST NOT accept or expose KLE/KatanA types, document content, paths, URLs, coordinates, RawInput construction, renderer callbacks, or opaque token payloads.

#### Scenario: v1 compatibility is retained
- **WHEN** a consumer supplies a v1 plan that does not exactly match the fixed full-editor sequence
- **THEN** KUC returns the existing incomplete-sequence typed error
