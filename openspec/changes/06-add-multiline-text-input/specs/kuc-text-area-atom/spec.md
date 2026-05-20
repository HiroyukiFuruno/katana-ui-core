## ADDED Requirements

### Requirement: TextArea exposes typed multiline input options

`TextArea` atom MUST expose typed options covering `value`, `placeholder`, `font_role`, `disabled`, `readonly`, `invalid`, `min_rows`, `max_rows`, `auto_grow`, `wrap_policy`, `submit_key`, `newline_key`, `tab_behavior`, `ime_enabled`, `leading_slot`, and `trailing_slot`.
`TextArea` MUST NOT be confused with the document editor; it is a form control for multi-line input.

#### Scenario: consumer constructs TextArea with chat-composer presets

- **WHEN** a consumer creates `TextArea` with `submit_key = Enter` and `newline_key = ShiftEnter`
- **THEN** pressing Enter fires `Submit`
- **AND** pressing Shift+Enter fires `InsertNewline`

#### Scenario: consumer cannot reuse Input atom for multi-line input

- **WHEN** a consumer attempts to set multi-line content on the `Input` atom
- **THEN** the contract test rejects multi-line strings in `Input`
- **AND** the consumer is directed to `TextArea`

### Requirement: TextArea forbids conflicting submit and newline keys

`TextArea` MUST fail the static contract check when `submit_key` and `newline_key` resolve to the same key combination.
Both fields MUST also accept `Disabled` to opt out of either behavior.

#### Scenario: both keys configured to Enter

- **WHEN** `submit_key = Enter` and `newline_key = Enter` are passed
- **THEN** the static linter reports a conflict
- **AND** the contract test fails

#### Scenario: both keys disabled

- **WHEN** `submit_key = Disabled` and `newline_key = Disabled` are passed
- **THEN** Enter inserts a literal newline only if the adapter falls back
- **AND** Submit and InsertNewline events are not emitted from this molecule

### Requirement: TextArea auto-grows between min_rows and max_rows

`TextArea` MUST grow its rendered row count from `min_rows` up to `max_rows` based on content height when `auto_grow = true`.
Content beyond `max_rows` MUST scroll internally without losing characters.

#### Scenario: auto_grow expands as content grows

- **WHEN** content height increases inside `[min_rows, max_rows]`
- **THEN** `Resize` is emitted with the new row count
- **AND** the rendered height changes accordingly

#### Scenario: content exceeds max_rows

- **WHEN** content height exceeds `max_rows`
- **THEN** the rendered height stays at `max_rows`
- **AND** internal vertical scroll is enabled to view the rest

### Requirement: TextArea handles IME composition across lines and emoji graphemes

`TextArea` MUST keep the preedit string visible during IME composition, update caret position as the composition evolves, and emit `IMECommit` only when composition completes.
Caret movement and deletion MUST treat grapheme clusters (surrogate pairs, ZWJ-joined emoji) as one unit.
Adapters MUST expose a neutral IME request carrying `input_kind = Multiline`, `phase`, `preedit`, `commit_text`, and `caret`.

#### Scenario: IME composition continues across a newline

- **WHEN** a composition crosses a line break inside the visible preedit string
- **THEN** the composition string and caret position remain consistent
- **AND** `IMEComposition { phase, string }` is emitted per phase, followed by a single `IMECommit`

#### Scenario: adapter reports multiline preedit and caret

- **WHEN** a Floem, egui, or GPUI adapter receives multiline IME preedit
- **THEN** it maps the update to the neutral IME request DTO
- **AND** the request carries `input_kind = Multiline`, the preedit string, and the caret byte position

#### Scenario: emoji deletion removes the full grapheme

- **WHEN** the caret is positioned after a ZWJ-joined emoji and the user presses Backspace
- **THEN** the whole emoji grapheme is removed in one operation
- **AND** the resulting `value` is shorter by exactly the grapheme byte length

### Requirement: TextArea tab behavior is explicit

`TextArea` MUST expose `tab_behavior = InsertTab | MoveFocus`.
The molecule MUST NOT silently fall back to one or the other when not configured; `tab_behavior` MUST always be set in the public API.

#### Scenario: MoveFocus moves focus

- **WHEN** `tab_behavior = MoveFocus` and Tab is pressed
- **THEN** focus moves to the next focusable element
- **AND** no tab character is inserted in `value`

#### Scenario: InsertTab inserts tab character

- **WHEN** `tab_behavior = InsertTab` and Tab is pressed
- **THEN** a `\t` character is inserted at the caret
- **AND** focus does not change

### Requirement: TextArea Storybook exposes presets and settings

Storybook MUST place `text-area` under `Atoms` and expose component-specific presets for chat composer, search multiline, long text, auto grow, max rows overflow, IME input, and emoji input.
Storybook MUST show settings mutations for `submit_key`, `newline_key`, `tab_behavior`, `auto_grow`, and `wrap_policy`.

#### Scenario: TextArea page exposes interactive settings evidence

- **WHEN** the Storybook interaction report is built
- **THEN** it contains settings mutation reports for all TextArea configurable options
- **AND** state, event, action, preset, and preview rows are present for the TextArea page
