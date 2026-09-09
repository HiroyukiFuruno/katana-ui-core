## ADDED Requirements

### Requirement: synchronized scenario session preserves search projection
A retained `FullTextCommandSurfaceScenarioSession` SHALL preserve search visibility, query, replace value, options, replace mode, and result position across each synchronized lease. It MUST NOT infer navigation result, replacement result, or document content from an external request.

#### Scenario: close remains closed on the next synchronized frame
- **WHEN** a retained session synchronizes a search close event and emits its next lease
- **THEN** the next root presentation has no search child and the close continuation verifies the closed state

#### Scenario: explicit reopen restores saved search state
- **WHEN** a caller invokes the KUC-owned explicit reopen operation after close and synchronizes the next lease
- **THEN** search becomes visible with the saved query, replace value, options, replace mode, and result position

#### Scenario: external effects are not projected as state
- **WHEN** navigation or replace actions emit one-shot receipts
- **THEN** the receipt is available for forwarding while the session does not invent a new result position, search result, or document value

### Requirement: session synchronization is consumer-independent
The session API SHALL not require a consumer to maintain a search-visible flag, query parser, child setter, or callback to preserve the KUC root projection.

#### Scenario: retain once and synchronize every frame
- **WHEN** a consumer retains the session once and synchronizes query input, option toggle, navigation, close, verification, and explicit reopen on successive frames
- **THEN** KUC produces the corresponding root, locator, AccessKit, frame, and receipt evidence without consumer-owned search state
