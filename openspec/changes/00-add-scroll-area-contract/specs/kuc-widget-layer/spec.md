## MODIFIED Requirements

### Requirement: Layout foundation includes typed scroll containers

KUC MUST expose `ScrollArea` as a layout foundation component with typed state, actions, events, and render props.
List, tree, diagnostics, command result, settings, and Storybook panel surfaces MUST be able to compose with the same scroll contract.

#### Scenario: list-like molecule uses shared scroll contract

- **WHEN** a list-like molecule needs scroll state
- **THEN** it can depend on `ScrollArea` offset / viewport / content props
- **AND** it does not invent a separate stringly scroll state

#### Scenario: viewer body remains outside KUC

- **WHEN** KDV builds a Markdown viewer surface
- **THEN** KDV may use KUC scroll primitives around controls or panels
- **AND** the document rendering and scroll synchronization policy remain outside KUC
