## ADDED Requirements

### Requirement: Chip atom is interactive and dismissible

`Chip` atom MUST expose typed options: `label`, `leading_icon`, `trailing_icon`, `tone`, `variant`, `size`, `interactive`, `selected`, `disabled`, `dismissible`, `accessibility_label`.
`Chip` MUST emit `ChipPressed` when interactive and pressed, and `ChipDismissed` when dismissible and dismissed.

#### Scenario: dismissible Chip removes via keyboard

- **WHEN** a `Chip` has `dismissible = true` and focus, and the user presses `Backspace` or `Delete`
- **THEN** `ChipDismissed` is emitted with the chip id
- **AND** focus moves to the previous chip in a `ChipGroup`, or back to the prior focus holder if standalone

#### Scenario: disabled Chip suppresses both press and dismiss

- **WHEN** a `Chip` has `disabled = true`
- **THEN** neither `ChipPressed` nor `ChipDismissed` fires from any action
- **AND** focus ring is not rendered (or is rendered as muted) per theme contract

### Requirement: Chip variant and tone resolve through theme tokens

`Chip` MUST resolve its colors from theme tokens based on `variant = Solid | Soft | Outline | Ghost` and `tone = Neutral | Accent | Success | Warning | Danger | Muted`.
The same `(variant, tone)` pair MUST produce stable rendering across light and dark themes.

#### Scenario: variant=Solid tone=Danger resolves danger token in both themes

- **WHEN** a `Chip` with `variant = Solid` and `tone = Danger` is rendered under both light and dark themes
- **THEN** the resolved background uses the danger token from the active theme
- **AND** the contrast against the text token meets the configured minimum contrast threshold

### Requirement: Chip differs from Badge in interactivity and dismissibility

`Badge` MUST remain passive (no interactive press, no dismiss).
`Chip` MUST be the canonical interactive / dismissible alternative.
The static linter MUST flag any consumer attempting to attach `Dismiss` or `Press` semantics directly to `Badge`.

#### Scenario: linter flags badge with dismiss

- **WHEN** a consumer attaches a `Dismiss` callback to `Badge`
- **THEN** the linter reports a contract violation
- **AND** the consumer is directed to `Chip`
