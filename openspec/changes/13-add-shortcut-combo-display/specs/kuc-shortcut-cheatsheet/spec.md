## ADDED Requirements

### Requirement: ShortcutCheatsheet displays grouped shortcuts with search

`ShortcutCheatsheet` molecule MUST expose `groups`, `query`, and `group_layout = TwoColumn | OneColumn`.
Each item MUST embed a `ShortcutCombo` and a label.
The query filter MUST match against labels and group titles using case-insensitive substring.

#### Scenario: query filters items in place

- **WHEN** a consumer sets `query = "open"`
- **THEN** only items whose label contains "open" (case-insensitive) remain visible
- **AND** group titles that match also keep their items even when item labels do not match

#### Scenario: empty query shows full sheet

- **WHEN** `query = ""`
- **THEN** all groups and items are visible
- **AND** the filter clears any previous selection state

### Requirement: ShortcutCheatsheet emits typed selection event

`ShortcutCheatsheet` MUST emit `ShortcutSelected { id, combo }` when a user activates an item by click or keyboard.
The molecule MUST NOT execute the shortcut itself; execution remains the consumer's responsibility.

#### Scenario: click on an item emits event

- **WHEN** the user clicks an item in the cheatsheet
- **THEN** `ShortcutSelected { id, combo }` is emitted
- **AND** no internal navigation occurs unless the consumer dispatches one

#### Scenario: keyboard Enter on focused item emits event

- **WHEN** an item is focused via keyboard navigation and Enter is pressed
- **THEN** the same `ShortcutSelected` event is emitted
- **AND** focus stays on the item for follow-up navigation
