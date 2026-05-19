## MODIFIED Requirements

### Requirement: Molecules compose atoms without stealing state

KUC molecules MUST compose atoms and additional model state without moving child component state into an uncontrolled global store.
Molecule contracts MUST explicitly define options, actions, events, state, presets, tests, visual regression, preview behavior, settings behavior, and Storybook pages.
Molecules MUST preserve parent and child state identities separately in automated tests and Storybook logs.
The notification family MUST distinguish:

- `NotificationToast` for transient overlay notifications
- `Banner` for persistent inline alerts within a view
- `StatusBar` for severity messages anchored to a status bar footer

These molecules MUST keep their public APIs separate so transient, inline-persistent, and footer-status uses cannot accidentally swap behaviors.

#### Scenario: persistent inline notice is requested

- **WHEN** a consumer needs a persistent alert inside a view
- **THEN** the consumer uses `Banner` molecule
- **AND** the consumer does not configure `NotificationToast` to be non-dismissing or `StatusBar` to host inline content

#### Scenario: contract review of notification widgets

- **WHEN** a notification or alert widget is reviewed
- **THEN** its placement role (transient overlay / persistent inline / footer status) is recorded
- **AND** any mismatch with its molecule choice blocks completion
