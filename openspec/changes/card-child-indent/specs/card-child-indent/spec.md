## ADDED Requirements

### Requirement: Configurable Card child origin

KUC SHALL expose an optional nonnegative pixel child_indent on UiCommonProps and apply it only to Card, consistently across raster drawing, measurement, scrolling and hit testing. Padding SHALL remain additive.

#### Scenario: Explicit zero and custom indent
- **WHEN** a Card sets child_indent to 0 or 24
- **THEN** its child origin SHALL be the Card origin plus 0 or 24 pixels, before padding, in both core and Storybook helpers

#### Scenario: Body width contract
- **WHEN** an unpadded Card in a 560px area sets child_indent to 0
- **THEN** its child hit rectangle SHALL start at the Card origin and retain width 560px

### Requirement: Legacy Card compatibility

KUC SHALL preserve the existing 16px Card child indent when the property is absent or None, and SHALL deserialize legacy common properties without child_indent.

#### Scenario: Legacy serialized props
- **WHEN** common props JSON omits child_indent
- **THEN** deserialization SHALL succeed with None

#### Scenario: Default and other node kinds
- **WHEN** no override is specified or another kind sets an override
- **THEN** existing child origin behavior SHALL remain unchanged
