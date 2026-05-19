## ADDED Requirements

### Requirement: ContextMenu supports pointer, virtual rect, and node anchors

`ContextMenu` molecule MUST accept three anchor kinds: pointer coordinates, virtual rect, and existing `UiNodeId`.
The anchor kind MUST be typed (not stringly), and the molecule MUST keep the anchor in its state for callback reporting.

#### Scenario: pointer-anchored menu opens at the click location

- **WHEN** the consumer opens `ContextMenu` with `ContextMenuAnchor::Pointer { x, y }`
- **THEN** the menu's `placement_used` origin is the pointer coordinate clamped into the viewport
- **AND** the `ContextMenuOpened` event reports the same anchor

#### Scenario: node-anchored menu reuses Menu placement contract

- **WHEN** the consumer opens `ContextMenu` with `ContextMenuAnchor::NodeId(id)`
- **THEN** placement priority falls back to the same list used by `Menu` molecule
- **AND** the focus return target defaults to the anchor node

### Requirement: ContextMenu items cover sections, dividers, submenus, toggles, radios

`ContextMenu` items MUST be a typed enum covering `Action`, `Toggle`, `Radio`, `Submenu`, `Section`, `Divider`.
`Action` items MUST support leading icon, trailing shortcut key cap, disabled flag, destructive flag, and accessibility label.
`Section` items MUST own their inner items; `Divider` MUST have a tone option.

#### Scenario: section header groups subsequent items

- **WHEN** a section is rendered with inner items
- **THEN** the section header is non-interactive
- **AND** keyboard navigation skips the section header and lands on the next enabled item

#### Scenario: destructive action is visually distinct

- **WHEN** an `Action` item has `destructive = true`
- **THEN** its rendered tone uses the destructive color token from the active theme
- **AND** the contract test verifies the token resolution in both light and dark themes

### Requirement: ContextMenu edge-flips inside the viewport

`ContextMenu` MUST flip placement when the chosen placement would render outside the viewport.
The flip MUST follow a deterministic priority list and MUST clamp the menu height with internal scroll when it still overflows.

#### Scenario: pointer anchored menu near bottom of viewport flips upward

- **WHEN** opening near the bottom edge with insufficient downward space
- **THEN** placement falls back to `AboveStart` (or further in the list) so the menu fits
- **AND** the `placement_used` reported on `ContextMenuOpened` reflects the flipped placement

#### Scenario: very tall menu overflows the remaining viewport

- **WHEN** items do not fit even after edge flip
- **THEN** the menu height is clamped to remaining viewport minus a configured margin
- **AND** the menu enables vertical scroll inside the panel without losing keyboard navigation

### Requirement: ContextMenu submenus open on hover and arrow key

`ContextMenu` submenu MUST open on hover after a configured delay, and immediately on `ArrowRight` from the highlighted parent.
The submenu MUST close on `ArrowLeft`, pointer leave (after a closing delay), or selection completion.
The submenu's child state MUST have a distinct `UiStateId` from the parent.

#### Scenario: hover delay prevents accidental submenu toggling

- **WHEN** the pointer briefly passes over a submenu item
- **THEN** the submenu does not open until the configured open delay elapses
- **AND** the submenu close delay prevents flicker when the pointer crosses a sibling

#### Scenario: keyboard navigation opens and closes submenus

- **WHEN** the user presses `ArrowRight` on a highlighted submenu item
- **THEN** the submenu opens and the first enabled child item is highlighted
- **AND** pressing `ArrowLeft` closes the submenu and restores parent highlight

### Requirement: ContextMenu reports close reason and returns focus

`ContextMenu` MUST emit `ContextMenuClosed` with a typed `reason` (Escape, OutsideClick, Selected, FocusReturn).
On close, focus MUST return to either the opener anchor (for `NodeId` anchors) or the caller-specified return target.

#### Scenario: Esc closes menu and returns focus

- **WHEN** the user presses `Esc` while the menu is open
- **THEN** the menu closes with `reason = Escape`
- **AND** focus is restored to the opener anchor node when the anchor was `NodeId`

#### Scenario: outside click closes menu with explicit reason

- **WHEN** a pointer down event occurs outside the menu
- **THEN** the menu closes with `reason = OutsideClick`
- **AND** the outside click event continues to its original target (no swallow), unless the caller opted into swallowing
