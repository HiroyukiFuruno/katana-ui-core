## ADDED Requirements

### Requirement: Egui command chrome adapter SHALL remain a KUC-owned optional boundary

KUC SHALL provide `katana_ui_core::egui` behind the `katana-ui-core` `egui`
optional feature. The module consumes only KUC command chrome DTOs, SVG/text
raster outputs, theme values, and egui input/context. It
SHALL return only KUC typed actions/events and SHALL not contain KatanA, KLE,
KDV, Markdown, editor, viewer, or host callback types.

#### Scenario: adapter compiles without host dependencies

- **WHEN** `katana-ui-core` is built with the `egui` feature
- **THEN** its feature dependency graph contains KUC raster modules and egui only at the framework boundary
- **AND** no host application crate or host command enum is required

### Requirement: Adapter SHALL use KUC raster output for icons and command-chrome text

The adapter SHALL upload `UiSvgRaster` RGBA pixels for icons and use the KUC
platform text-raster layout for visible command-chrome text, editable
find/replace text, caret, selection, and hit targets. It SHALL NOT use
`egui::TextEdit`, egui font registration, OS font lookup, Unicode icon fallback,
or emoji glyph fallback on the command-chrome path.

#### Scenario: Japanese and variation-selector input uses KUC text layout

- **WHEN** a find or replace value contains Japanese text and `⭐️`
- **THEN** the adapter obtains visible text, caret, and hit-test bounds from KUC text-raster output
- **AND** no egui font atlas measurement is used as the command-chrome text source

### Requirement: Actual egui interactions SHALL map to typed KUC events

The adapter SHALL map actual egui pointer, keyboard, focus, and IME interactions
to command chrome actions/events. Tests SHALL drive the actual egui adapter and
assert typed events without fixed waits, label parsing, or host callback mocks.

#### Scenario: real click activates generic toolbar action once

- **WHEN** an actual egui pointer click targets an enabled command toolbar icon
- **THEN** the adapter emits one generic action event with that action id
- **AND** it does not invoke a host command or infer a command from visible text

### Requirement: Search inputs SHALL use shared TextSurface state and focused key routing

The adapter SHALL retain query and replace `TextSurface` state by the generic
search-strip state id and SHALL synchronize controlled values without emitting a
user event. It SHALL use shared TextSurface raster, IME, selection, and
AccessKit behavior. While query owns focus, Enter and ArrowDown SHALL emit next
navigation; Shift+Enter and ArrowUp SHALL emit previous navigation; Escape
SHALL emit close. These keys SHALL not move the query selection or activate a
toolbar action. The adapter SHALL not execute search or replacement.

#### Scenario: actual IME query input and key routing stay typed

- **WHEN** actual egui input commits Japanese text and `⭐️` to the focused query
- **THEN** the adapter emits exactly one `SearchQueryChanged` event with the unmodified string
- **AND WHEN** it receives Shift+Enter, ArrowDown, or Escape
- **THEN** it emits only the corresponding previous-navigation, next-navigation, or close event
- **AND** the query TextSurface selection remains unchanged by those command keys

### Requirement: Adapter frame record SHALL be the sole artifact source

The adapter SHALL emit a renderer-neutral frame record containing raster layers,
rectangles, interaction targets, focus state, and typed component state. egui
draw and KUC Storybook deterministic artifacts SHALL consume the same record.
No fallback glyph renderer or separate text measurement path SHALL produce
acceptance, motion, or screenshot evidence.

#### Scenario: Storybook artifact equals adapter frame record

- **WHEN** a scripted command-chrome interaction sequence is recorded
- **THEN** the Storybook manifest references the adapter's frame-record layer and bounds
- **AND** a contract test fails if a fallback renderer produces the artifact

### Requirement: KUC guardrails SHALL enforce command-chrome ownership

KUC `kuc-guardrails` and AST lint SHALL reject framework dependencies in core,
private Storybook SVG rasterization, fixed command-search visible literals in
new command chrome rendering, host-specific imports, and glyph/emoji icon
fallback. KLE/KDV duplicate-renderer checks SHALL be added in their own
repositories and invoked by their release gates.

#### Scenario: private SVG raster path is introduced after migration

- **WHEN** a new KUC Storybook or core source file directly parses/rasterizes SVG outside the public runtime
- **THEN** KUC guardrails fail with the violating path
- **AND** the command cannot be accepted as release evidence
