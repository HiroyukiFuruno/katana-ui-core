## MODIFIED Requirements

### Requirement: Theme exposes color, typography, spacing, radius, shadow, border, z-index, and motion tokens

KUC theme MUST expose tokens for color, typography, spacing, radius, shadow, border, z-index, and motion.
Motion tokens MUST include duration (Instant / Fast / Default / Slow), easing (Linear / Standard / Emphasized / Decelerate / Accelerate), and distance (Compact / Default / Spacious).

#### Scenario: theme switch updates motion in addition to color

- **WHEN** a theme override changes motion tokens
- **THEN** molecule animations resolve to the new values
- **AND** color/typography tokens continue to update as before

#### Scenario: motion tokens are required for animation specs

- **WHEN** a molecule constructs a `MotionSpec`
- **THEN** the spec consumes typed tokens for duration, easing, distance
- **AND** literal numeric values are rejected by the static linter
