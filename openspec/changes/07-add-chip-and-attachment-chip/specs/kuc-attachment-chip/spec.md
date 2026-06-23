## ADDED Requirements

### Requirement: AttachmentChip carries kind, status, and progress

`AttachmentChip` molecule MUST expose `kind = File | Image | URL | Paste | Resource`, `name`, `meta`, `icon` or `thumbnail`, `progress`, `status = Pending | Uploading | Ready | Error`, and `actions`.
`AttachmentChip` MUST emit status transitions via `AttachmentChipStatusChanged`.

#### Scenario: status transitions in order

- **WHEN** an `AttachmentChip` proceeds from `Pending` to `Uploading` to `Ready`
- **THEN** `AttachmentChipStatusChanged` is emitted for each transition
- **AND** the event payload includes the previous and new status

#### Scenario: Error status surfaces retry action

- **WHEN** status becomes `Error`
- **THEN** a retry action is rendered as a child `Button`
- **AND** activating retry transitions status back to `Pending` and emits `AttachmentChipRetry`

### Requirement: AttachmentChip thumbnail and progress overlays render predictably

`AttachmentChip` MUST render the thumbnail (for Image kind) at a deterministic aspect ratio.
When `progress` is set, a progress overlay MUST be rendered over the leading area (or thumbnail).

#### Scenario: image thumbnail uses centered crop

- **WHEN** an `AttachmentChip` has `kind = Image` and a thumbnail bitmap
- **THEN** the thumbnail is rendered with a centered crop at the configured aspect
- **AND** the thumbnail does not stretch beyond the configured chip size

#### Scenario: progress at 0..1 is reflected in overlay

- **WHEN** `progress = 0.42` is set
- **THEN** the overlay shows 42% fill
- **AND** the chip's status remains `Uploading` until progress reaches 1.0 or moves to `Ready`/`Error`

### Requirement: ChipGroup supports wrap, horizontal scroll, and overflow menu

`ChipGroup` molecule MUST expose `overflow = None | Menu | ScrollHorizontal` and `wrap = true | false`.
With `Menu`, hidden chips MUST appear in the `Menu` molecule attached to the overflow trigger.
With `ScrollHorizontal`, the group MUST emit scroll state for use in tests.

#### Scenario: overflow=Menu hides extras

- **WHEN** chips do not fit in the available width and `overflow = Menu`
- **THEN** the overflow trigger renders the count of hidden chips
- **AND** opening the overflow lists the hidden chips in `Menu`

#### Scenario: reorder opt-in emits ChipReordered

- **WHEN** `reorder = true` is set and a chip is dragged to a new index
- **THEN** `ChipReordered { from, to }` is emitted with the resulting indices
- **AND** without `reorder = true`, drag interactions are ignored
