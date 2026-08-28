## ADDED Requirements

### Requirement: Command search presentation SHALL receive all visible and accessible strings from the host

KUC SHALL provide `SearchControlStrings` for strip label, query and replace
placeholders, option labels/tooltips/accessibility labels, navigation, result
summary, replace-one, replace-all, close, and disabled reasons. New command
chrome rendering SHALL not contain fixed visible English literals.

#### Scenario: localized host strings reach every visible search control

- **WHEN** a consumer supplies non-English `SearchControlStrings`
- **THEN** query, options, navigation, result summary, replace, and close controls use those strings
- **AND** no fixed English literal is needed to label or activate a control

### Requirement: Command search strip SHALL compose existing search state without owning search execution

`CommandChromeSearchStrip` SHALL compose existing `SearchControlStrip` query,
options, navigation, replace state, and events while adding presentation,
capability, and close behavior through new additive DTOs/events. KUC SHALL NOT
execute search, regex, replacement, or editor/viewer mutation.

#### Scenario: replace-all remains a typed consumer request

- **WHEN** a visible and enabled command search strip receives replace-all activation
- **THEN** it emits a typed replace-all request with the current replacement value
- **AND** KUC does not change consumer content or compute search results

### Requirement: Search capabilities SHALL expose unsupported controls explicitly

`SearchControlCapabilities` SHALL model regex, replace, close, and navigation
availability plus an injected disabled reason. Unsupported capability controls
SHALL be disabled and SHALL NOT emit an event that claims the operation ran.

#### Scenario: unavailable regex cannot be activated

- **WHEN** the host marks regex unavailable and supplies a disabled reason
- **THEN** the regex control is disabled with that reason available to accessibility/presentation
- **AND** pointer or keyboard activation emits no regex option event

### Requirement: Close SHALL be additive and restore responsibility to the consumer

Command search close SHALL be represented by new
`CommandChromeSearchAction::RequestClose` and
`CommandChromeSearchEvent::CloseRequested` types rather than a new variant on
the existing `SearchControlStripAction` or `SearchControlStripEvent`. Focus
restoration and search-state clearing remain consumer responsibilities.

#### Scenario: close is emitted without mutating search state

- **WHEN** the close control is activated while close capability is enabled
- **THEN** the command search strip emits `CloseRequested`
- **AND** query, replacement value, and consumer search state are unchanged by KUC

### Requirement: Legacy search-strip rendering SHALL not be used for command chrome

KUC SHALL preserve legacy `SearchControlStrip` construction and rendering for
backward compatibility, but the new command chrome adapter and consumers SHALL
use the injected presentation path. Readonly compatibility accessors, if
needed, SHALL be additive methods only.

#### Scenario: command chrome render is independent of legacy English rendering

- **WHEN** the egui command chrome adapter renders a search strip
- **THEN** it uses `SearchControlStrings` and command-chrome presentation data
- **AND** it does not invoke the legacy fixed-string render path
