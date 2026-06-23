## ADDED Requirements

### Requirement: HoverCard supports hover and focus triggers with separate open and close delays

`HoverCard` molecule MUST open on pointer enter or anchor focus, and close on pointer leave / anchor blur after the configured close delay.
Open and close delays MUST be configurable independently and MUST be observable in state for tests.

#### Scenario: hover triggers open after open delay

- **WHEN** the pointer enters the anchor and stays for the configured open delay
- **THEN** `HoverCardOpened` is emitted
- **AND** state reports `open = true` with the elapsed delay

#### Scenario: pointer entering card body pauses close delay

- **WHEN** the close delay has started and the pointer enters the card body
- **THEN** the close delay timer is paused
- **AND** the card remains open while the pointer stays in the card or moves back to the anchor

### Requirement: HoverCard supports typed content slots

`HoverCard` MUST expose `heading`, `body`, `footer`, and `actions` as typed slots.
Slots MUST be optional (`None` is valid) and MUST be rendered in slot order even when some slots are empty.

#### Scenario: only body and actions are provided

- **WHEN** only `body` and `actions` slots are set
- **THEN** the card renders without heading or footer affordances
- **AND** layout collapses the absent slots to zero height with no remaining gap

#### Scenario: actions slot keeps card open while focused

- **WHEN** focus enters an interactive node inside the `actions` slot
- **THEN** the close delay timer is suspended
- **AND** the card closes only when focus leaves both the anchor and the card

### Requirement: HoverCard supports pointer-follow anchoring

`HoverCard` MUST allow `pointer_follow = true` so the anchor tracks the pointer position.
The card MUST close after the configured close delay once the pointer leaves the tracked region.

#### Scenario: pointer follow tracks movement

- **WHEN** `pointer_follow = true` is set and the pointer moves within the anchor area
- **THEN** the placement engine recomputes the card position each frame
- **AND** the card remains open as long as the pointer is inside the tracked region

### Requirement: HoverCard reports open and close reasons

`HoverCard` MUST emit `HoverCardOpened` and `HoverCardClosed` events with typed `reason`.
Close reasons MUST cover `PointerLeave`, `Blur`, `Escape`, `Programmatic`, `OutsideClick`.

#### Scenario: Esc closes the card

- **WHEN** the user presses `Esc` while the card is open
- **THEN** `HoverCardClosed` is emitted with `reason = Escape`
- **AND** focus returns to the anchor

#### Scenario: outside click closes the card

- **WHEN** the user clicks outside the card and the anchor
- **THEN** `HoverCardClosed` is emitted with `reason = OutsideClick`
- **AND** the outside click event reaches its original target unless explicitly swallowed
