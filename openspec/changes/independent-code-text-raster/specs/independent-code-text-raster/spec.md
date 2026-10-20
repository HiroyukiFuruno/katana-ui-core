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
