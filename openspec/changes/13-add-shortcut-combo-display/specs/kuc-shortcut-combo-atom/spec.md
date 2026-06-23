## ADDED Requirements

### Requirement: ShortcutCombo renders a typed key combination

`ShortcutCombo` atom MUST accept a typed `KeyCombo { modifiers, key }` and MUST NOT accept stringly-typed combinations.
`ShortcutCombo` MUST expose `separator`, `platform_display`, `size`, and `tone` as typed options.

#### Scenario: typed combination renders consistently

- **WHEN** a consumer constructs `ShortcutCombo` with `modifiers = Cmd | Shift` and `key = Char('P')`
- **THEN** the rendered output uses platform-appropriate modifier glyphs or names
- **AND** changing `platform_display` re-resolves the rendering without source changes

#### Scenario: stringly-typed shortcut is rejected

- **WHEN** a consumer attempts to pass `"Cmd+Shift+P"` as a string into `ShortcutCombo`
- **THEN** the static linter reports a contract violation
- **AND** the consumer is directed to construct a typed `KeyCombo`

### Requirement: ShortcutCombo platform display switches modifier rendering

`ShortcutCombo` MUST expose `platform_display = Auto | MacOS | Windows | Linux`.
`MacOS` MUST render modifier glyphs (e.g. `⌘`, `⌥`, `⌃`, `⇧`). `Windows` and `Linux` MUST render the textual names (`Ctrl`, `Win` / `Super`, `Alt`, `Shift`).
`Auto` MUST consult the runtime platform via the adapter callback.

#### Scenario: MacOS display uses glyphs

- **WHEN** `platform_display = MacOS` and modifiers include `Cmd | Shift`
- **THEN** the rendered output starts with `⌘⇧`
- **AND** no `+` separator is rendered when the default separator is `None`

#### Scenario: Auto follows runtime

- **WHEN** `platform_display = Auto` and the runtime adapter reports Windows
- **THEN** the rendered output uses `Ctrl+Shift+P` defaults
- **AND** changing the runtime hint switches the rendering accordingly

### Requirement: ShortcutCombo separator is typed and platform-aware

`ShortcutCombo` MUST expose `separator = Plus | Space | Arrow | None`.
The default separator MUST be `None` for MacOS display and `Plus` for Windows / Linux, unless overridden.

#### Scenario: separator override beats platform default

- **WHEN** `platform_display = MacOS` and `separator = Plus`
- **THEN** the rendered output uses `⌘+⇧+P`
- **AND** the explicit separator overrides the platform default

### Requirement: ShortcutCombo accessibility label is auto-generated

`ShortcutCombo` MUST auto-generate an accessibility label from the `KeyCombo`.
The auto-generated label MUST use platform-neutral textual names ("Command + Shift + P", "Control + Shift + P", etc.).
Consumers MAY override the label.

#### Scenario: auto label reflects modifiers and key

- **WHEN** `modifiers = Cmd | Shift` and `key = Char('P')` on a MacOS display
- **THEN** the auto label is `Command + Shift + P`
- **AND** the visual rendering remains glyph-based per the display setting

#### Scenario: explicit accessibility label overrides auto

- **WHEN** a consumer provides `accessibility_label = "Open Command Palette"`
- **THEN** the override is used instead of the auto-generated string
- **AND** the visual rendering is unaffected
