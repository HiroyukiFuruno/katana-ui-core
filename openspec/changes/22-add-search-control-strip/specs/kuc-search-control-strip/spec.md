## ADDED Requirements

### Requirement: SearchControlStrip exposes typed search options

`SearchControlStrip` MUST expose query, match case, whole word, regex, result count, active index, replace mode, and replace value as typed state.
It MUST NOT execute search or replace operations itself.

#### Scenario: query changes

- **WHEN** `SetSearchQuery("heading")` is applied
- **THEN** the query state becomes `"heading"`
- **AND** `SearchQueryChanged` is emitted for the consumer

#### Scenario: option toggles

- **WHEN** `ToggleSearchOption(UseRegex)` is applied
- **THEN** the regex option changes
- **AND** `SearchOptionChanged` is emitted

### Requirement: SearchControlStrip emits navigation requests

`SearchControlStrip` MUST emit typed navigation events for previous and next.
It MUST keep navigation separate from search result generation.

#### Scenario: next result requested

- **WHEN** the user activates next
- **THEN** `SearchNavigationRequested { direction: Next }` is emitted
- **AND** KUC does not compute the next match

### Requirement: SearchControlStrip supports optional replace controls

`SearchControlStrip` MUST support `ReplaceMode = Hidden | Visible | Disabled`.
Replace actions MUST emit `ReplaceRequested` only when replace mode is visible and enabled.

#### Scenario: replace is hidden

- **WHEN** replace mode is `Hidden`
- **THEN** replace input and replace actions are not rendered
- **AND** replace actions are ignored

#### Scenario: replace all requested

- **WHEN** replace mode is `Visible` and replace all is activated
- **THEN** `ReplaceRequested { scope: All }` is emitted
- **AND** KUC does not mutate editor or viewer content

### Requirement: SearchControlStrip presents result count without domain logic

`SearchControlStrip` MUST present result count and active index using consumer-provided values.
It MUST handle zero, one, and many results without changing query or option state.

#### Scenario: zero results

- **WHEN** result count is `0`
- **THEN** the strip presents a zero-results state
- **AND** previous / next controls are disabled unless the consumer explicitly enables them
