## ADDED Requirements

### Requirement: WorkspaceTabBar exposes pinned, closeable, dirty, groupable tabs

`WorkspaceTabBar` molecule MUST expose tab options including `id`, `title`, `icon`, `dirty`, `pinned`, `closeable`, `tone`, `tooltip`, `group_id`, and `accessibility_label`.
Pinned tabs MUST be anchored to the leading edge regardless of insertion order; close buttons on pinned tabs MUST be hidden.

#### Scenario: pinned tab is anchored to the leading edge

- **WHEN** a tab is pinned and the strip is rendered
- **THEN** the pinned tab appears before any unpinned tab in visual order
- **AND** dragging an unpinned tab into the pinned area is rejected by the drop accept callback

#### Scenario: dirty tab requests confirm before close

- **WHEN** a dirty tab is asked to close via `CloseTab` action
- **THEN** the molecule emits `TabCloseRequested` (no immediate removal)
- **AND** the consumer must dispatch `ConfirmClose` for `TabClosed` to fire and the tab to disappear

### Requirement: WorkspaceTabBar overflows excess tabs into a menu

`WorkspaceTabBar` MUST compute hidden tabs deterministically from the measured strip width.
Hidden tabs MUST be exposed via an overflow trigger that opens a menu listing hidden tabs with their icon, title, dirty indicator, and close action.

#### Scenario: strip width shrinks below total tab width

- **WHEN** the available strip width is less than the total measured tab width
- **THEN** the overflow trigger becomes visible
- **AND** the overflow menu lists exactly the hidden tabs in the same order

#### Scenario: selecting a hidden tab from overflow promotes it

- **WHEN** the user selects a hidden tab from the overflow menu
- **THEN** the tab becomes active
- **AND** the strip rerenders so the now-active tab is visible (other tabs may shift into the overflow)

### Requirement: WorkspaceTabBar supports draggable reorder and grouping

`WorkspaceTabBar` MUST integrate with KUC drag-and-drop primitives to support reorder, group insertion, group reordering, and new-group creation by drop.
Drop indicators MUST distinguish `Before`, `After`, `InsideGroup`, and `NewGroup` positions.

#### Scenario: tab dropped between two tabs inserts at that position

- **WHEN** a tab is dragged and dropped between two existing tabs with the `Before` indicator
- **THEN** `TabReordered { from, to }` is emitted with the resulting index
- **AND** focus and active state are preserved on the dragged tab

#### Scenario: tab dropped onto a collapsed group auto-expands it

- **WHEN** a tab is hovered over a collapsed group for the configured delay
- **THEN** the group auto-expands
- **AND** the drop indicator switches to `InsideGroup` once expansion completes

### Requirement: WorkspaceTabBar supports keyboard navigation and shortcuts

`WorkspaceTabBar` MUST handle `Cmd/Ctrl+Tab`, `Cmd/Ctrl+Shift+Tab`, `Cmd/Ctrl+W`, `Cmd/Ctrl+1..9`, and `Cmd/Ctrl+0` as standard shortcuts.
Shortcut handling MUST respect platform display labels (e.g., `⌘` on macOS, `Ctrl` elsewhere) when exposed to KeyCap rendering.

#### Scenario: Ctrl+W on the active dirty tab requests close

- **WHEN** the user presses `Cmd/Ctrl+W` while the active tab is dirty
- **THEN** `TabCloseRequested` is emitted (no immediate close)
- **AND** the focus remains on the still-active tab

#### Scenario: Ctrl+1 selects the first visible tab

- **WHEN** the user presses `Cmd/Ctrl+1`
- **THEN** the first visible tab (pinned first, then unpinned) becomes active
- **AND** `TabSelected` is emitted with that tab id

### Requirement: WorkspaceTabBar opens a tab context menu via ContextMenu molecule

`WorkspaceTabBar` MUST open a `ContextMenu` for tab right-click and group-header right-click using the existing KUC `ContextMenu` molecule.
The molecule MUST NOT embed its own custom menu rendering.

#### Scenario: right-click on tab opens ContextMenu with standard items

- **WHEN** the user right-clicks a tab
- **THEN** a `ContextMenu` opens at the pointer with items: Close, Close Others, Close to the Right, Close All, Pin/Unpin, Move to New Group, Move to Group (submenu)
- **AND** the menu reports selected command via `ContextMenuItemSelected`

#### Scenario: right-click on group header offers group operations

- **WHEN** the user right-clicks a group header
- **THEN** the `ContextMenu` exposes Rename Group, Collapse/Expand, and Move actions
- **AND** group-level state changes emit `GroupCollapseChanged` or `TabReordered` as appropriate
