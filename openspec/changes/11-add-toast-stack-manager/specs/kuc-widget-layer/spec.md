## MODIFIED Requirements

### Requirement: Molecules compose atoms without stealing state

KUC molecules MUST compose atoms and additional model state without moving child component state into an uncontrolled global store.
Molecule contracts MUST explicitly define options, actions, events, state, presets, automated tests, numeric layout/rendering contracts, preview behavior, settings behavior, and Storybook pages.
Molecules MUST preserve parent and child state identities separately in automated tests and Storybook logs.
The transient notification layer MUST separate:

- `NotificationToast` as the single toast surface molecule
- `ToastStackManager` as the orchestration molecule that handles stacking, queueing, dedup, pause-on-hover, and position

A consumer needing more than one toast at a time MUST use `ToastStackManager` rather than mounting multiple `NotificationToast` instances.

#### Scenario: multiple toasts are requested

- **WHEN** a consumer needs to display multiple toasts over time
- **THEN** the consumer mounts `ToastStackManager`
- **AND** the consumer does not implement queue management or position handling outside of KUC

#### Scenario: single transient confirmation is requested

- **WHEN** a consumer only ever needs one transient toast
- **THEN** the consumer may use `NotificationToast` directly
- **AND** the consumer does not need to import `ToastStackManager`
