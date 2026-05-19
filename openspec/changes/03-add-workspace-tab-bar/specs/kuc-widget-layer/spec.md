## MODIFIED Requirements

### Requirement: Widget layer exposes atoms and molecules first

KUC MUST expose the initial widget layer as atoms and molecules.
The MVP MUST NOT require consumers to use organisms, templates, or pages.
Future organisms and templates MUST be addable without breaking atoms and molecules APIs.
The molecule layer MUST distinguish:

- anchor-only menus (`Menu`, `MenuButton`) vs pointer-anchored menus (`ContextMenu`)
- segmented tab controls (`Tabs`) vs closeable / draggable tab strips (`CloseableTabStrip`)

These molecules MUST keep their public APIs separate so that segmented and closeable tab use cases do not bleed options into each other.
`CloseableTabStrip` MUST NOT expose workspace, document, chat session, or file domain fields.

#### Scenario: consumer builds UI from atoms and molecules

- **WHEN** a consumer builds a UI from KUC widgets
- **THEN** the consumer can compose atoms and molecules directly
- **AND** the consumer does not need a page or template abstraction

#### Scenario: closeable tab strip is requested

- **WHEN** a consumer needs closeable, draggable, groupable tabs for hosting documents or sessions
- **THEN** the consumer uses `CloseableTabStrip` molecule
- **AND** the consumer keeps document / workspace / session semantics in its own state
- **AND** the consumer does not extend `Tabs` (segmented) with closeable-tab-only options
