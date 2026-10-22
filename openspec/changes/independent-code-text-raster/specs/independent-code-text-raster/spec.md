## ADDED Requirements

### Requirement: Independent code raster configuration
KUC SHALL expose a renderer builder accepting an independent code raster configuration and face selection. It SHALL preserve body renderers, document typography, code font token and monospace model semantics. Drawing, measurement, wrapping and hit testing SHALL share the configured code renderer.

#### Scenario: System body with candidate code face
- **WHEN** the code renderer selects proportional font candidates with CandidateChain while body uses System
- **THEN** body pixels and hit metrics remain unchanged
- **AND** monospace code span pixels and hit metrics match the same explicit code configuration

#### Scenario: Reconfigure after rendering
- **WHEN** code raster configuration is changed after rendering scroll content
- **THEN** cached scroll measurement is invalidated and subsequent output matches a freshly configured renderer

#### Scenario: Existing constructors
- **WHEN** consumers use existing constructors without the new builder
- **THEN** existing body and code face selection behavior is preserved

#### Scenario: Code-first mixed line
- **WHEN** independently configured code precedes plain body text in a fractional-baseline line
- **THEN** following body glyphs and decorations retain their body raster Y positions regardless of the leading code face
- **AND** line decorations use the body renderer baseline while individual runs align to the configured line-box baseline

#### Scenario: Bidirectional mixed line
- **WHEN** an RTL body line contains an LTR inline-code span and an independent code raster configuration is active
- **THEN** the plain and code runs follow the rich-line visual order
- **AND** copying the line preserves its logical source order

#### Scenario: Bidirectional fragment boundary
- **WHEN** one plain or code run contains more than one bidi visual range around an independently rendered inline-code span
- **THEN** KUC splits the run only at UTF-8 character boundaries and places every fragment in rich-line visual order
- **AND** copying the fragments preserves their logical source order

#### Scenario: Bidirectional span geometry
- **WHEN** a visually reordered independently rendered line contains a highlighted, decorated, or linked span
- **THEN** its background, decoration, and link hit rectangle use the same visual fragment coordinates as its glyph raster run

#### Scenario: Explicit bidi controls cross a raster fragment boundary
- **WHEN** an embedding, override, or isolate starts before an independently rasterized fragment and closes after it
- **THEN** KUC preserves that active bidi context only in the raster request for the fragment
- **AND** selection, copy, and link payload retain the original source text without synthetic control characters

#### Scenario: Link whitespace at an internal bidi fragment edge
- **WHEN** one linked source span is split into multiple bidi visual fragments
- **THEN** only whitespace at the original source span's leading or trailing edge may be excluded from its decoration and link hit rectangle
- **AND** whitespace at an internal fragment edge remains clickable and decorated
