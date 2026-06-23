## ADDED Requirements

### Requirement: Toolbar overflow strategy is typed and deterministic

`Toolbar` MUST expose `overflow_strategy = Hide | Menu | Custom`.
Given measured widths and action priorities, the visible-vs-hidden partitioning MUST be deterministic and unit-testable.

#### Scenario: actions exceed available width with Menu strategy

- **WHEN** the total measured width of actions exceeds the available toolbar width and `overflow_strategy = Menu`
- **THEN** lowest-priority actions are moved into the overflow set until the visible set fits
- **AND** an overflow trigger is rendered to open the overflow menu

#### Scenario: ties in priority are broken by trailing index

- **WHEN** multiple actions share the same `priority` and one must be hidden
- **THEN** the action with the trailing index is hidden first
- **AND** the partitioning is stable across re-renders for the same input

### Requirement: Toolbar supports IconOnly, IconLeading, IconTrailing, and LabelOnly display modes

`Toolbar` MUST expose `display_mode` covering icon-only, icon-leading, icon-trailing, and label-only renders.
In `IconOnly`, each action MUST carry an `accessibility_label` or a non-empty `tooltip`; missing both MUST fail the static check.

#### Scenario: IconOnly action without accessibility label fails the static check

- **WHEN** a toolbar action has neither `accessibility_label` nor `tooltip` and `display_mode = IconOnly`
- **THEN** the static linter reports a contract violation
- **AND** the action is treated as invalid by the contract test

#### Scenario: switching display mode triggers width recomputation

- **WHEN** `display_mode` is changed
- **THEN** measured widths are invalidated
- **AND** the overflow partition is recomputed before the next render

### Requirement: Toolbar supports split actions with primary and secondary

`Toolbar` MUST support `SplitAction { primary, secondary }` where `secondary` opens a dropdown menu.
The two halves MUST be independently disabled, and the accelerator MUST be associated with `primary` (with the menu items inside `secondary` carrying their own accelerators if any).

#### Scenario: primary disabled, secondary enabled

- **WHEN** primary is `disabled = true` and secondary is enabled
- **THEN** the primary half is non-interactive
- **AND** the dropdown half remains pressable

#### Scenario: split dropdown opens via secondary trigger

- **WHEN** the secondary half is pressed
- **THEN** a `Menu` panel opens using the shared placement engine
- **AND** `SplitDropdownOpened` is emitted with the action id

### Requirement: Toolbar supports accelerators that activate without focus

`Toolbar` MUST allow each action to declare a `KeyCombo` accelerator.
Pressing the accelerator MUST fire `AcceleratorTriggered` and the action's `Command` event, regardless of which node currently holds focus, as long as the toolbar is mounted and active.

#### Scenario: matching accelerator fires Command without moving focus

- **WHEN** the accelerator key combination is pressed while another node is focused
- **THEN** the toolbar action fires `Command` for that action id
- **AND** focus does not change to the toolbar action

#### Scenario: disabled accelerator does not fire

- **WHEN** the action is `disabled = true` and its accelerator is pressed
- **THEN** no `Command` event is emitted
- **AND** the keyboard event continues to its original target

### Requirement: Toolbar groups insert dividers only across group boundaries

`Toolbar` MUST treat consecutive actions sharing the same `group_id` as one logical group.
Dividers MUST be inserted only between groups, never inside the same group.
Group labels, when configured, MUST appear as section headers in the overflow menu.

#### Scenario: same-group actions render without internal divider

- **WHEN** three actions share `group_id = "edit"`
- **THEN** no dividers appear between them in the visible toolbar
- **AND** any dividers come from boundaries with other groups

#### Scenario: overflow menu groups labeled sections

- **WHEN** hidden actions span multiple groups with labels
- **THEN** the overflow menu shows a section header per group
- **AND** an unlabeled group renders without a section header
