# rgba-color-picker Specification Delta

## ADDED Requirements

### Requirement: RGB / RGBA editable color picker

`ColorPickerRgba` MUST provide an interactive GUI that edits red, green, blue, and, when enabled, alpha values.

#### Scenario: RGBA mode exposes alpha

- **WHEN** the picker is configured with alpha support
- **THEN** the panel exposes red, green, blue, and alpha controls
- **AND** changes are reflected in the selected color callback

#### Scenario: RGB-only mode hides alpha

- **WHEN** the picker is configured as opaque color
- **THEN** the panel exposes red, green, and blue controls
- **AND** alpha is fixed to fully opaque

### Requirement: Floating panel behavior

The picker panel MUST open above the page content as a floating panel, not as inline content that changes page layout.

#### Scenario: Open panel is in front of storybook content

- **WHEN** the trigger is clicked
- **THEN** the panel appears above surrounding storybook content
- **AND** it is not hidden behind sibling panels or scroll containers

#### Scenario: Panel can be closed

- **WHEN** the user clicks outside the panel
- **OR** presses Escape
- **OR** clicks the close button
- **THEN** the panel closes

### Requirement: Color-only trigger

The default trigger MUST be a compact color-only button.
It MUST NOT show numeric RGB / RGBA values inside the trigger.

#### Scenario: RGBA trigger preview

- **WHEN** the selected color has alpha
- **THEN** the trigger shows a left preview with transparency applied on a checker background
- **AND** the trigger shows a right preview with the opaque RGB color

#### Scenario: Value proof is separate from trigger

- **WHEN** Storybook needs to prove the selected value
- **THEN** the value is displayed outside the trigger as separate text
- **AND** the trigger remains color-only

#### Scenario: Trigger size presets

- **WHEN** the caller selects xs, sm, mid, large, or xlarge
- **THEN** the trigger color button is rendered at the selected preset size
- **AND** mid is the default size

#### Scenario: Trigger border option

- **WHEN** the caller disables the trigger border
- **THEN** the outer button frame is not rendered
- **AND** the default trigger still renders with a border

#### Scenario: Trigger border is not doubled

- **WHEN** the trigger border is enabled
- **THEN** only the parent trigger node renders a border
- **AND** the inner color preview does not render its own outer border

#### Scenario: Trigger rendering does not crash Floem

- **WHEN** the Storybook page renders ColorPicker trigger presets
- **THEN** the trigger is built from normal Floem nodes
- **AND** it does not panic in Floem view state updates

### Requirement: No fake picker controls

The picker MUST NOT show controls that do not have a complete widget-level implementation.

#### Scenario: Eyedropper is not implemented

- **WHEN** OS-level color picking is not implemented
- **THEN** the panel does not show an eyedropper control

### Requirement: Configurable panel size

The picker MUST provide a configurable panel scale.
The default scale MUST be 75% of the large reference panel.

#### Scenario: Default panel scale

- **WHEN** no scale is specified
- **THEN** the panel uses the compact default scale

#### Scenario: Custom panel scale

- **WHEN** the caller specifies a panel scale
- **THEN** the panel, color plane, preview, and sliders use that scale

### Requirement: Usable color surface

The color plane, hue control, and alpha control MUST provide a clear visual indication of the selected color without obvious decorative grid artifacts.

#### Scenario: Color plane updates color

- **WHEN** the user drags or clicks on the color plane
- **THEN** saturation and brightness change
- **AND** the selected RGB / RGBA value updates

#### Scenario: Alpha control updates transparency

- **WHEN** alpha is enabled and the user changes the alpha control
- **THEN** the selected alpha value updates
- **AND** the preview uses a checker background only as transparency indication

#### Scenario: Alpha drag updates continuously

- **WHEN** the user drags the alpha control
- **THEN** the alpha value follows the pointer continuously

#### Scenario: Hue bar is seamless

- **WHEN** the hue control is rendered
- **THEN** the color gradient is continuous
- **AND** vertical segment boundaries are not visible
