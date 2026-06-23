## MODIFIED Requirements

### Requirement: Widget layer exposes atoms and molecules first

KUC MUST expose the initial widget layer as atoms and molecules.
The MVP MUST NOT require consumers to use organisms, templates, or pages.
Future organisms and templates MUST be addable without breaking atoms and molecules APIs.
The molecule layer MUST distinguish anchor-only menus (`Menu`, `MenuButton`) from pointer-anchored context menus (`ContextMenu`), keeping their public APIs separate.

#### Scenario: consumer builds UI from atoms and molecules

- **WHEN** a consumer builds a UI from KUC widgets
- **THEN** the consumer can compose atoms and molecules directly
- **AND** the consumer does not need a page or template abstraction

#### Scenario: pointer-anchored menu is requested

- **WHEN** a consumer needs a right-click or pointer-anchored menu
- **THEN** the consumer uses `ContextMenu` molecule
- **AND** the consumer does not extend `Menu` or `MenuButton` to accept pointer anchors
