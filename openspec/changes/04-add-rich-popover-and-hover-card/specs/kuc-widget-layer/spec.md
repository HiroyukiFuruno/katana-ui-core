## MODIFIED Requirements

### Requirement: Widget layer exposes atoms and molecules first

KUC MUST expose the initial widget layer as atoms and molecules.
The MVP MUST NOT require consumers to use organisms, templates, or pages.
Future organisms and templates MUST be addable without breaking atoms and molecules APIs.
The disclosure family MUST separate concerns:

- `Tooltip` for short text-only hints
- `HoverCard` for hover/focus rich content with delay and pointer-follow
- `Popover` for click/programmatic rich content with arrow, slots, and focus management
- `ContextMenu` for pointer/right-click anchored menus

These molecules MUST NOT cross-import each other's option enums.

#### Scenario: consumer builds UI from atoms and molecules

- **WHEN** a consumer builds a UI from KUC widgets
- **THEN** the consumer can compose atoms and molecules directly
- **AND** the consumer does not need a page or template abstraction

#### Scenario: hover rich content is requested

- **WHEN** a consumer needs hover- or focus-triggered rich content with delay
- **THEN** the consumer uses `HoverCard`
- **AND** the consumer does not extend `Tooltip` to accept rich slots or `Popover` to accept hover triggers
