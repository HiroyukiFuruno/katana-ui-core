## ADDED Requirements

### Requirement: shared KUC egui adapter SHALL own actual text-surface rendering

`katana-ui-core-egui-adapter` SHALL be the only KUC-owned actual egui renderer
for TextSurface and command chrome. It SHALL depend on KUC runtime/model crates
and egui at the framework boundary, and SHALL not import host application types.

#### Scenario: adapter build has no consumer application dependency

- **WHEN** the adapter crate is built in the KUC workspace
- **THEN** it compiles without KatanA, KLE, KDV, Markdown, viewer, or host callback crates
- **AND** its public API accepts and returns only KUC types plus egui boundary values

### Requirement: adapter SHALL avoid egui font atlas and TextEdit for text surface

The adapter SHALL use KUC platform text-raster RGBA/layout output for visible
text, selection, caret, preedit, gutter, and annotations. It SHALL NOT use
`egui::TextEdit`, egui font registration/measurement, OS font lookup, local
glyph rasterization, or Unicode/emoji substitutes on the text-surface path.

#### Scenario: Japanese input has no egui atlas measurement path

- **WHEN** actual egui input enters Japanese text and `⭐️` into a TextSurface
- **THEN** visible pixels and grapheme/caret/hit-test bounds come from KUC raster layout
- **AND** an AST test rejects a TextEdit or font-atlas call in the adapter text-surface module

### Requirement: actual egui input SHALL map to TextSurface typed events

The adapter SHALL transform actual pointer, drag, keyboard, focus, IME, context
menu, clipboard/history shortcut, scroll, and accessibility interactions into
TextSurface typed actions/events without invoking consumer callbacks directly.

#### Scenario: actual IME commit produces one KUC event

- **WHEN** egui delivers an IME composition update followed by a `⭐️` commit
- **THEN** the adapter emits the corresponding TextSurface composition and commit events once
- **AND** the same frame record provides the committed glyph and caret bounds

### Requirement: all visual outputs SHALL consume one TextSurfaceFrameRecord

The adapter SHALL generate one frame record before drawing. RGBA texture upload,
egui paint, hit targets, IME output, AccessKit output, Storybook artifact, and
numeric jitter checks SHALL consume that record. Separate fallback canvases,
manual text measurement, or shape counts SHALL not be evidence paths.

#### Scenario: deterministic artifact matches actual adapter record

- **WHEN** a scripted input sequence produces a Storybook motion artifact
- **THEN** each frame manifest references the adapter frame-record hashes and bounds
- **AND** a test fails if an artifact is produced from a fallback renderer

### Requirement: TextSurface Storybook SHALL use the actual shared adapter runtime

The KUC Storybook `text-area` page SHALL dispatch to a Storybook-only `eframe`
runtime which invokes `EguiTextSurfaceAdapter::show` for every live frame. The
existing `minifb`/Canvas catalog MAY remain for other component pages but SHALL
NOT render or provide acceptance evidence for the TextSurface page.

#### Scenario: opening the text-area Storybook page uses the adapter path

- **WHEN** the Storybook opens the `text-area` page
- **THEN** its live runtime calls the shared adapter `show` path with the KUC
  TextSurface fixture
- **AND** it does not use a Storybook-owned glyph, gutter, scroll, IME, or
  selection renderer

### Requirement: Storybook artifacts SHALL consume an adapter-owned paint plan

The shared adapter SHALL derive one immutable paint plan from the actual
`EguiTextSurfaceFrameRecord` and platform raster. Its egui painter and artifact
encoder SHALL consume that plan. Storybook SHALL only persist the resulting
artifact frame and SHALL NOT reconstruct text layout or geometry.

#### Scenario: deterministic scripted artifact records the actual adapter frame

- **WHEN** a scripted TextSurface event sequence emits a motion artifact
- **THEN** every manifest step contains the adapter frame-record hash,
  paint-plan hash, RGBA pixel hash, bounds, scroll offset, and typed events
- **AND** the artifact gate rejects Canvas fallback, manual geometry, direct
  core-action scripting, or shape-count-only evidence

### Requirement: adapter SHALL publish actual accessibility output

The adapter SHALL map the TextSurface accessibility tree to actual egui
AccessKit nodes. Automated tests SHALL query roles/labels/state after focus,
selection, read-only, gutter, and context-menu interactions.

#### Scenario: selection is observable through actual accessibility bridge

- **WHEN** a focused surface has a selected Japanese grapheme range
- **THEN** the actual egui accessibility tree exposes configured text-surface state
- **AND** a consumer-state assertion alone cannot satisfy the test

### Requirement: automatic gutter paint SHALL consume KUC-resolved row state

The adapter SHALL paint and expose automatic gutter active/hover state only
from `TextSurfaceGutterFrame`. It SHALL not read consumer text, derive a line
number, reconstruct a row rectangle, or use a host-specific diagnostic/search
meaning to decide the state.

#### Scenario: actual hover and caret state survive scrolling

- **WHEN** actual RawInput updates a controlled automatic-gutter hover set and
  moves the focused caret before and after scrolling a Japanese/`⭐️` surface
- **THEN** the frame record, AccessKit nodes, paint-plan bounds and artifact
  hashes describe the same active/hovered rows
- **AND** no egui native text or consumer-local gutter renderer is used

### Requirement: KUC and consumer guardrails SHALL prevent duplicate text-surface renderers

KUC guardrails SHALL reject direct text-surface renderer code outside the shared
adapter. KLE/KDV repository guardrails SHALL reject local texture cache, manual
hit-test, local gutter renderer, egui TextEdit/font atlas path, and fallback
artifact use once migration is complete.

#### Scenario: KLE local surface remains after migration

- **WHEN** a KLE release gate finds a local platform text surface or local line-gutter renderer in use
- **THEN** the gate fails with the path and KUC adapter migration requirement
- **AND** no screenshot or motion file can override the failure
