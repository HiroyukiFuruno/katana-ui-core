## MODIFIED Requirements

### Requirement: Layout molecules keep split and collapse roles separate

KUC MUST keep `SplitPane` and `CollapsiblePanel` as separate public contracts.
`SplitPane` MUST represent symmetric two-pane resizing.
`CollapsiblePanel` MUST represent a single panel that can collapse, float, or resize.
Neither molecule MUST expose an application shell or page template.

#### Scenario: consumer needs editor and preview split

- **WHEN** a consumer needs editor / preview or TOC / viewer panes
- **THEN** it uses `SplitPane` for the symmetric pane boundary
- **AND** it manages the editor / viewer content and synchronization outside KUC

#### Scenario: consumer needs sidebar collapse

- **WHEN** a consumer needs icon-only or hover-expanded sidebar behavior
- **THEN** it uses `CollapsiblePanel`
- **AND** it does not add collapse options to `SplitPane`
