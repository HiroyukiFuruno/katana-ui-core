## ADDED Requirements

### Requirement: Theme provides motion tokens

KUC theme MUST expose `MotionDurationToken`, `MotionEasingToken`, and `MotionDistanceToken`.
All animation specs MUST resolve their numeric values through these tokens, not as ad-hoc literals.

#### Scenario: molecule resolves duration via token

- **WHEN** a molecule's default open animation uses `MotionDurationToken::Default`
- **THEN** the resolved duration is the token's configured value (e.g., 200ms)
- **AND** changing the token in theme updates all molecules using `Default` simultaneously

#### Scenario: ad-hoc literal is rejected by the linter

- **WHEN** a molecule's `MotionSpec` is constructed with a literal `duration = 350ms`
- **THEN** the static linter reports a contract violation
- **AND** the consumer is directed to use or extend `MotionDurationToken`

### Requirement: MotionPolicy supports Respect, Force, Ignore for reduced motion

`MotionPolicy.reduced_motion` MUST be a typed enum of `Respect | Force | Ignore`.
`Respect` MUST follow the OS / adapter signal. `Force` MUST always treat as reduced. `Ignore` MUST always play full animation regardless of OS signal.

#### Scenario: Respect mirrors OS signal

- **WHEN** `reduced_motion = Respect` and the adapter reports `prefers_reduced_motion = true`
- **THEN** all animations downgrade to Instant
- **AND** Shimmer is disabled

#### Scenario: Ignore plays full animation in test mode

- **WHEN** `reduced_motion = Ignore` and the adapter reports `prefers_reduced_motion = true`
- **THEN** animations still play at their configured spec
- **AND** the override is logged for test diagnostics

### Requirement: Animation primitives are limited to four typed kinds

KUC motion primitives MUST be `Fade`, `Slide`, `Scale`, `Shimmer`.
Each primitive MUST be a typed variant of `MotionPrimitive` and consume tokens for its numeric parameters.

#### Scenario: Slide primitive uses distance token

- **WHEN** a molecule animates with `Slide { distance: MotionDistanceToken::Default, direction: Up }`
- **THEN** the rendered displacement uses the token's pixel value
- **AND** the token can be remapped in theme without molecule source changes

#### Scenario: Shimmer disabled under reduced motion

- **WHEN** reduced motion is active (per policy)
- **THEN** any `Shimmer` primitive renders as no animation
- **AND** the trace records the downgrade

### Requirement: MotionPolicy can disable animation in specific contexts

`MotionPolicy.disable_in: Vec<MotionContext>` MUST allow disabling animation for declared contexts (e.g., `Storybook`, `Test`, `OverlayInsideOverlay`).
Disabled contexts MUST behave identically to reduced-motion downgrade.

#### Scenario: Storybook context downgrades to Instant

- **WHEN** `disable_in` contains `Storybook` and the molecule is rendered inside the Storybook catalog
- **THEN** all animations downgrade to Instant
- **AND** image regression captures static frames only

#### Scenario: nested overlay context can be opted out

- **WHEN** `disable_in` contains `OverlayInsideOverlay` and a Popover opens inside a Modal
- **THEN** the Popover's open/close animation is downgraded to Instant
- **AND** the outer Modal's animation is unaffected

### Requirement: Disclosure and loading molecules consume MotionSpec uniformly

`Popover`, `HoverCard`, `ContextMenu`, `Modal`, `NotificationToast`, `ToastStackManager`, `Banner`, `Accordion`, `DragPreview`, `Skeleton`, `SkeletonCluster` MUST accept a `motion: MotionSpec` option.
Default `MotionSpec` MUST come from documented per-molecule defaults using motion tokens.

#### Scenario: default motion uses tokens

- **WHEN** a molecule is mounted without explicit `motion`
- **THEN** it animates with the documented default for that molecule (e.g., Modal: Fade + Scale(0.96→1.0) with `MotionDurationToken::Default`)
- **AND** changing the token globally updates the molecule's animation

#### Scenario: override per molecule

- **WHEN** a consumer overrides `motion` on a specific molecule instance
- **THEN** that instance animates per the override
- **AND** other instances and other molecules are unaffected
