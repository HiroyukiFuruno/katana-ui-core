## ADDED Requirements

### Requirement: Core foundation is established before component work

KUC MUST establish theme, font, text rendering, input, event routing, state ownership, and layout contracts before marking atoms or molecules as complete.
The foundation MUST be framework-neutral and MUST NOT depend on Floem, egui, or gpui.

#### Scenario: foundation contract is inspected

- **WHEN** implementers start atoms or molecules work
- **THEN** the KUC core contract defines theme, font, text, input, event, state, and layout behavior
- **AND** the contract does not require a framework-native view type

### Requirement: Theme and font are configurable through KUC

KUC MUST provide an external entry point for theme and font configuration.
The default theme MUST use Katana accent colors.
Font configuration MUST use abstract font roles such as proportional and monospace, rather than OS-specific paths in core.
The external entry point MUST be a KUC facade that owns the active `ThemeSnapshot`, style sheet, global UI state, and default font role.
The facade MUST allow replacing theme and style sheet without rebuilding component-local state.
Global UI state MUST be limited to cross-component concerns such as focus target, active overlay, and modal stack; component-local state MUST remain owned by each component instance.

#### Scenario: consumer replaces theme and font

- **WHEN** a consumer supplies a different theme snapshot and font role mapping
- **THEN** atoms, molecules, Storybook panels, and preview surfaces use the supplied values
- **AND** component state identifiers remain unchanged

#### Scenario: facade is configured

- **WHEN** a consumer creates a KUC facade with a theme, style sheet, global state, and default font role
- **THEN** render context, font resolution, and panel theme resolution read those values through the facade
- **AND** duplicate component instances do not share state because of facade-level configuration

### Requirement: Default theme uses Katana accent colors

KUC MUST ship a default dark theme using Katana accent colors.
The default theme MUST include `background`, `surface`, `panel`, `code-background`, `text`, `muted`, `accent`, `border`, and `selection` color tokens.
The default font roles MUST include `body` as proportional and `code` as monospace.
Shortcut or keycap font roles MAY resolve to `code` when a dedicated token is not present.

#### Scenario: default theme is inspected

- **WHEN** the default KUC facade is created
- **THEN** its active theme resolves the Katana accent token
- **AND** `body` resolves to proportional text
- **AND** `code` resolves to monospace text

### Requirement: Text rendering supports mixed language baselines

KUC MUST render English, Japanese, mixed English/Japanese, and emoji text without vertical jitter inside the same line box.
Text layout MUST expose measurable line metrics so automated tests can verify vertical centering.
The regression samples MUST include English-only, Japanese-only, mixed English/Japanese, and emoji-mixed strings.
The regression MUST measure line box, baseline, ascent, descent, and visual center, not only image pixels.

#### Scenario: mixed text samples are measured

- **WHEN** text samples for English, Japanese, mixed English/Japanese, and emoji are rendered in equal-height boxes
- **THEN** their visual centers align within the accepted regression threshold
- **AND** the test records line metrics, not only screenshots

### Requirement: Input supports key, IME, and emoji text

KUC MUST model keyboard input, committed text from Japanese IME, and OS emoji input as core input events.
IME handling MUST distinguish composition/preedit state from committed text where the host exposes it.
Text input components MUST be able to update their internal state from these input events without requiring consumer-owned text state.

#### Scenario: text input receives Japanese and emoji

- **WHEN** a TextInput receives Japanese IME committed text and an OS emoji
- **THEN** KUC emits input events that preserve the committed text
- **AND** the component state updates without requiring consumer-owned text state

### Requirement: Component state is owned per component instance

Each UI component instance MUST own or derive a unique internal state identity.
Duplicate UI instances of the same kind and label MUST NOT share state by accident.
Molecules MUST preserve child component state identity instead of flattening child state into an uncontrolled global store.
Storybook state, event, and action logs MUST include the target state identifier for interactive components.

#### Scenario: duplicate components are rendered

- **WHEN** two Buttons or two TextInputs with the same label are placed in one tree
- **THEN** each component has a unique state identifier
- **AND** actions on one component do not mutate the other component state

### Requirement: Layout behavior is automatically testable

KUC MUST expose layout results for size, spacing, alignment, scroll bounds, and overlay placement so automated tests can verify layout correctness.
Layout regression MUST verify dimensions, padding, gap, border width, vertical and horizontal centering, baseline placement, scroll bounds, overflow, overlay anchor, overlay z-index, and unintended overlap.
Storybook screenshots MUST remain supporting evidence and MUST NOT replace layout regression.

#### Scenario: layout regression is executed

- **WHEN** layout regression tests run for required components
- **THEN** the tests verify dimensions, alignment, overflow, and overlay bounds
- **AND** Storybook screenshots are not the only layout evidence
