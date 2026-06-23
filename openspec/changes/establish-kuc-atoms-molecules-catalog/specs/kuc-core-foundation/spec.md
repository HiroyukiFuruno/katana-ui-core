## ADDED Requirements

### Requirement: Core foundation is established before component work

KUC MUST establish theme, font, text rendering, input, event routing, state ownership, and layout contracts before marking atoms or molecules as complete.
The foundation MUST be framework-neutral and MUST NOT depend on Adapter, adapter, or adapter.

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
When a consumer needs app-level control such as loading, saving, or remote data completion, KUC MUST expose a short-lived state handle API that reads component state by snapshot and writes it back through `set` or `update`.
The handle MUST NOT require moving component-local state into the facade global state.

#### Scenario: consumer replaces theme and font

- **WHEN** a consumer supplies a different theme snapshot and font role mapping
- **THEN** atoms, molecules, Storybook panels, and preview surfaces use the supplied values
- **AND** component state identifiers remain unchanged

#### Scenario: facade is configured

- **WHEN** a consumer creates a KUC facade with a theme, style sheet, global state, and default font role
- **THEN** render context, font resolution, and panel theme resolution read those values through the facade
- **AND** duplicate component instances do not share state because of facade-level configuration

#### Scenario: app global state updates a component-owned loading state

- **WHEN** a consumer keeps an app-level loading flag for a Button
- **THEN** the consumer updates the component-owned state through a `UiStateHandle`
- **AND** the Button accepts press actions again after the handle writes `loading=false`
- **AND** the facade global state does not become the storage location for that Button state

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
- **AND** the test records line metrics directly

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
Storybook output MUST NOT replace layout regression.

#### Scenario: layout regression is executed

- **WHEN** layout regression tests run for required components
- **THEN** the tests verify dimensions, alignment, overflow, and overlay bounds
- **AND** Storybook output is not accepted as layout evidence

### Requirement: Core action model supports generic click events

KUC は Button 専用の press だけでなく、任意の component が利用できる汎用 click action を MUST とする。
汎用 click action は target state id、action source、callback log の action name を保持し、TreeView row、Accordion header、Icon、Text などの開閉・選択操作に使える必要がある。

#### Scenario: non-button component handles click

- **WHEN** TreeView row や Accordion header が汎用 click action を受け取る
- **THEN** callback log は `click` と target state id を記録する
- **AND** Button 専用の `button_press` と混同しない

### Requirement: Core carries SVG icon primitives as typed props

KUC は SVG アイコンを単なる文字や style class ではなく、core が扱える typed props として MUST で持つ。
SVG アイコン atom は source、role、size、color token、accessibility label を持ち、TreeView directory / file icon、Accordion indicator、toolbar icon に再利用できる必要がある。

#### Scenario: icon consumer inspects SVG props

- **WHEN** Icon または SvgButton が SVG source を指定される
- **THEN** render model は SVG source と accessibility label を typed props として保持する
- **AND** Storybook の代替文字だけでは完了扱いにしない
