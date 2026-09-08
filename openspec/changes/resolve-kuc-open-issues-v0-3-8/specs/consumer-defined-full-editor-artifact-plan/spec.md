## ADDED Requirements

### Requirement: consumer-defined generic artifact plan
The public `egui` API SHALL accept a versioned consumer artifact plan that contains only opaque leaf IDs, KUC-defined generic interaction/effect classes, opaque host-projected tokens, and KUC-issued stage bindings. The API MUST NOT accept or expose KLE/KatanA types, document content, paths, URLs, coordinates, RawInput construction, renderer callbacks, or opaque token payloads.

#### Scenario: foreign consumer creates a valid plan
- **WHEN** a registry consumer provides unique leaf IDs, supported generic classes, and tokens bound to the current KUC root revision
- **THEN** KUC issues a plan that the consumer can execute without constructing RawInput or accessing a renderer

#### Scenario: plan rejects invalid bindings
- **WHEN** a plan is empty, has an unsupported schema/class, duplicate leaf/binding/stage, an incomplete stage sequence, or a stale token/root revision
- **THEN** KUC returns a typed error before a frame or media file is emitted

### Requirement: KUC-owned stage evidence and forwarding receipt
KUC SHALL retain one generic full-editor root across every issued stage and SHALL emit numbered PNG, decode/pixel/root-record hashes, current-frame AccessKit hash, Unicode glyph/measurement/hit-test evidence, and a one-shot opaque forwarding receipt for each stage. Opaque host targets and token payloads MUST NOT be serialized, formatted, or emitted in the manifest.

#### Scenario: stage evidence is complete and bound
- **WHEN** a consumer executes an issued stage containing IME input, Japanese text, `⭐️` (`U+2B50 U+FE0F`), selection, hit-test, or accessibility activation
- **THEN** the KUC artifact evidence is bound to the same stage, root revision, and receipt

#### Scenario: receipt or media reuse is rejected
- **WHEN** a receipt is consumed twice or for another leaf/stage/root, a stage is skipped/reordered, or a target media file already exists
- **THEN** KUC returns a typed error and does not publish a partial artifact

### Requirement: fixed scenario compatibility
The existing fixed `FullTextCommandSurfaceScenarioId` catalogue and fixed-dimension `write_opaque` behavior SHALL remain available and SHALL retain their existing validation semantics.

#### Scenario: existing fixed scenario still runs
- **WHEN** a consumer issues an existing fixed scenario or passes variable dimensions to `write_opaque`
- **THEN** the fixed scenario succeeds under its existing contract and `write_opaque` continues to reject mismatched dimensions
