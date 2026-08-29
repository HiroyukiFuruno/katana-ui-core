## ADDED Requirements

### Requirement: TextSurface SHALL compose existing KUC text contracts without breaking them

KUC SHALL provide additive `TextSurface` props/state/action/event/frame-record
DTOs that compose `TextArea`, text selection, text span, and platform text-raster
contracts. Existing public TextArea structs/enums SHALL remain source compatible.

#### Scenario: existing TextArea consumer remains source-compatible

- **WHEN** an existing consumer constructs `TextArea` and exhaustively handles `TextAreaEvent`
- **THEN** it compiles without a new required field or enum match arm
- **AND** text-surface behavior is available through new additive DTOs/events

### Requirement: visible text, caret, selection, preedit, and hit-test SHALL share one raster layout

TextSurface SHALL derive visible spans, grapheme bounds, selection rectangles,
caret, IME preedit, and pointer hit-test from one platform text-raster layout.
It SHALL preserve Japanese, variation selectors, combining marks, and ZWJ
graphemes without consumer font measurement or scalar editing.

#### Scenario: star variation selector remains one selectable grapheme

- **WHEN** the focused text surface contains `⭐️` and pointer selection targets that glyph
- **THEN** the frame record reports one full grapheme range for the star and variation selector
- **AND** caret/delete selection does not expose a partial scalar boundary

### Requirement: TextSurface SHALL request clipboard and history work without performing it

TextSurface SHALL emit typed copy, cut, paste, undo, and redo requests. It SHALL
apply generic read-only and selection enablement rules but SHALL NOT access a
clipboard backend or mutate consumer history.

#### Scenario: read-only cut request is suppressed

- **WHEN** a read-only text surface receives a cut interaction
- **THEN** it emits no cut request and leaves text/selection unchanged
- **AND** a copy request remains available for a non-empty selection

### Requirement: annotations SHALL be generic, deterministic, and domain-free

TextSurface annotations SHALL use generic grapheme ranges, visual properties,
semantic tokens, priorities, and labels. KUC SHALL not define Markdown, syntax,
search, or diagnostic enums for them. Overlap precedence SHALL be deterministic.

#### Scenario: selection overlays a lower-priority annotation

- **WHEN** a selection overlaps an explicit lower-priority annotation
- **THEN** the frame record renders the selection above that annotation
- **AND** the annotation range remains available to accessibility/tooltip data

### Requirement: line gutter SHALL be a generic text-position component

TextSurface SHALL support optional generic gutter rows with logical indices,
host-provided display labels, active/hovered states, markers, and typed hit
events. Text line boxes and gutter hit targets SHALL originate from the same
frame record.

#### Scenario: gutter row activation uses shared frame geometry

- **WHEN** a pointer activates a visible gutter row
- **THEN** TextSurface emits its logical row id and marker id if present
- **AND** no consumer-specific coordinate reconstruction is required

### Requirement: automatic gutter state SHALL be resolved and recorded by KUC

For automatic numbered gutters, KUC SHALL derive the active row from the
current TextArea caret and current text layout. A controlled consumer MAY supply
only a deduplicated logical-row hover set. KUC SHALL normalize that set against
the current layout and SHALL NOT accept consumer labels, row bounds, colours or
geometry. `TextSurfaceGutterFrame` SHALL expose the resulting `active` and
`hovered` facts with the existing KUC-issued row id, marker and bounds.

#### Scenario: caret and controlled hover use one gutter frame

- **WHEN** a multiline Japanese surface containing `⭐️` has an automatic gutter,
  a caret on one logical row and a controlled hover request for another row
- **THEN** the frame record, gutter accessibility targets and adapter paint plan
  agree on each row's active/hovered values and KUC-derived bounds
- **AND** an invalid requested logical row produces no synthetic row or geometry

### Requirement: TextSurface SHALL expose a renderer-neutral accessibility tree

TextSurface SHALL expose focus, editable/read-only state, selection, gutter
targets, disabled reasons, and interaction labels in a generic accessibility
tree. The model SHALL not rely on test-only labels or host-specific roles.

#### Scenario: read-only text surface retains accessible selection information

- **WHEN** a read-only surface has keyboard focus and a non-empty selection
- **THEN** its accessibility tree exposes input/text state with selection information
- **AND** cut/paste are absent or disabled while copy remains discoverable
