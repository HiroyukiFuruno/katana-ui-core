## ADDED Requirements

### Requirement: StatusBar exposes mode option for single message and multi segment

`StatusBar` MUST expose `mode = SingleMessage | MultiSegment`.
`SingleMessage` MUST preserve the existing severity + message + actions + dismiss contract.
`MultiSegment` MUST expose a typed `segments: Vec<StatusBarSegment>` and disable the single-message fields.

#### Scenario: SingleMessage stays backward compatible

- **WHEN** `mode = SingleMessage` (default)
- **THEN** the API and behavior match the existing `StatusBar` contract
- **AND** consumers do not need migration

#### Scenario: MultiSegment rejects single-message fields

- **WHEN** `mode = MultiSegment` and a consumer also sets the single-message `message`
- **THEN** the static linter reports a conflict
- **AND** validation fails the contract test

### Requirement: StatusBar segments place by alignment

`StatusBarSegment` MUST expose `alignment = Leading | Center | Trailing`.
Segments MUST be laid out into three columns by alignment, in the order they appear in the `segments` array within each column.

#### Scenario: segments grouped by alignment

- **WHEN** segments include Leading, Center, Trailing entries in mixed order
- **THEN** the rendered layout groups them by alignment in the same column
- **AND** within each column they preserve the original array order

#### Scenario: center overflow uses elision

- **WHEN** center segments do not fit horizontally
- **THEN** center segments are elided rather than scrolled
- **AND** an overflow indicator is rendered if configured

### Requirement: StatusBar segment can be interactive with optional popover

`StatusBarSegment` MUST expose `interactive: bool` and `popover: Option<PopoverSpec>`.
Interactive segments MUST fire `SegmentPressed` on click and on `Enter`/`Space` keyboard activation.
Segments with `popover = Some(...)` MUST open the configured popover using the shared placement engine.

#### Scenario: interactive segment fires SegmentPressed

- **WHEN** an interactive segment is clicked
- **THEN** `SegmentPressed { id }` is emitted
- **AND** focus moves to the segment for keyboard follow-up

#### Scenario: popover segment opens on click

- **WHEN** an interactive segment with a `popover` spec is activated
- **THEN** the popover opens at the segment anchor
- **AND** `SegmentPopoverOpened { id }` is emitted

### Requirement: StatusBar segment supports background progress overlay

`StatusBarSegment` MUST allow `progress: Option<f32>` clamped to `[0.0, 1.0]`.
When set, the segment MUST render a thin progress overlay behind the label and icon.

#### Scenario: progress at 0.6 renders 60% overlay

- **WHEN** `progress = Some(0.6)` is set
- **THEN** the segment renders a 60% width overlay using the configured progress tone
- **AND** the label and icon remain readable on top of the overlay

#### Scenario: progress unset hides overlay

- **WHEN** `progress = None`
- **THEN** no overlay is rendered
- **AND** segment layout matches the non-progress preset

### Requirement: StatusBar accessibility announces segments in reading order

`StatusBar` MUST expose a `aria-live = polite` container.
When any segment's label changes, the live region announce MUST include the segment's accessibility label.
Reading order MUST follow Leading → Center → Trailing within the rendered DOM.

#### Scenario: segment label change announces

- **WHEN** a segment's label changes
- **THEN** the live region announces the new label using `accessibility_label` if set, otherwise `label`
- **AND** the announce is polite (not assertive) to avoid interrupting typing
