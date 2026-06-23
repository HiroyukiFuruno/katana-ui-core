## MODIFIED Requirements

### Requirement: Molecules compose atoms without stealing state

KUC molecules MUST compose atoms and additional model state without moving child component state into an uncontrolled global store.
Molecule contracts MUST explicitly define options, actions, events, state, presets, tests, visual regression, preview behavior, settings behavior, and Storybook pages.
Molecules MUST preserve parent and child state identities separately in automated tests and Storybook logs.
Disclosure / overlay / loading molecules (`Popover`, `HoverCard`, `ContextMenu`, `Modal`, `NotificationToast`, `ToastStackManager`, `Banner`, `Accordion`, `DragPreview`, `Skeleton`, `SkeletonCluster`) MUST accept a typed `motion: MotionSpec` option and MUST respect the global `MotionPolicy` (including reduced-motion downgrade).

#### Scenario: reduced motion downgrades all disclosure animations

- **WHEN** `MotionPolicy.reduced_motion = Respect` and the OS reports reduced motion
- **THEN** all configured molecule animations downgrade to Instant
- **AND** the Storybook regression captures static open/close frames

#### Scenario: per-molecule motion override is honored

- **WHEN** a consumer overrides `motion` for a single Popover instance
- **THEN** that instance animates per the override
- **AND** other Popover instances continue to use the molecule's default
