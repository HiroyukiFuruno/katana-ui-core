## ADDED Requirements

### Requirement: Banner is a persistent inline alert with severity, actions, dismiss, details

`Banner` molecule MUST expose `severity = Info | Success | Warning | Danger | Neutral`, optional `title`, required `message`, optional `leading_icon`, `actions`, `dismissible`, `expanded_details`, `density`, `placement_hint`.
`Banner` MUST NOT auto-dismiss; it stays visible until the consumer hides it or the user dismisses it.

#### Scenario: severity drives default icon and tone

- **WHEN** `severity = Warning` is set without explicit `leading_icon`
- **THEN** the banner renders the warning icon from the theme icon registry
- **AND** the banner tone follows the warning theme tokens

#### Scenario: severity overridden by explicit leading_icon

- **WHEN** the consumer sets a custom `leading_icon`
- **THEN** the custom icon is used regardless of severity
- **AND** the tone still follows the severity tokens unless tone is explicitly overridden via theme override

### Requirement: Banner accessibility role follows severity semantics

`Banner` MUST set the accessibility role to `status` for `Info | Success | Neutral` and to `alert` for `Warning | Danger`.
Live region announcements MUST be enabled accordingly.

#### Scenario: warning announces immediately

- **WHEN** a `Banner` with `severity = Warning` becomes visible
- **THEN** the role is `alert`
- **AND** the live region announce contains the title (if any) and the message

#### Scenario: success politely announces

- **WHEN** a `Banner` with `severity = Success` becomes visible
- **THEN** the role is `status`
- **AND** the announce uses a polite live region

### Requirement: Banner supports expandable details

`Banner` MUST allow `expanded_details: Option<String>`; setting `Some` MUST render a toggle that opens / closes the detail area.
When details are open, the detail height MUST clamp to a configurable max with internal scroll.

#### Scenario: details toggle opens and closes

- **WHEN** the user clicks the details toggle
- **THEN** `BannerDetailsToggled { open: true }` is emitted
- **AND** the detail area renders with internal scroll if its content exceeds the configured max height

#### Scenario: details unset hides toggle

- **WHEN** `expanded_details = None`
- **THEN** no toggle is rendered
- **AND** the state.details_open value is always false

### Requirement: Banner dismiss emits typed event without persistence

`Banner` MUST emit `BannerDismissed` when dismissed.
The molecule MUST NOT persist dismissed state across consumer-driven renders; consumers are responsible for any persistence policy.

#### Scenario: dismiss hides banner and emits event

- **WHEN** the user dismisses a `dismissible = true` banner
- **THEN** `state.visible` becomes false
- **AND** `BannerDismissed` is emitted with the banner id

#### Scenario: consumer reapplies persistence

- **WHEN** the consumer re-renders the same banner id on next state
- **THEN** it appears again unless the consumer filters out dismissed ids in their state
- **AND** KUC does not silently remember the dismissed id
