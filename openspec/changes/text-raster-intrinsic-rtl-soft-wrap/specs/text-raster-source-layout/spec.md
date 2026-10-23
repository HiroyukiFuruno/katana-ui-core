## ADDED Requirements

### Requirement: Intrinsic raster preserves local RTL geometry
KUC MUST measure and draw width-unspecified RTL/LTR fragments in local content coordinates without leaking an internal fallback buffer width into glyph, selection, hit or decoration bounds.

#### Scenario: Independent code raster follows a short Arabic fragment
- **WHEN** a System body renderer and independent CandidateChain code renderer draw a short mixed Arabic/code line in a 640px viewport
- **THEN** Arabic glyphs remain visible in the viewport and their fragment bounds agree with selection, link hit and underline geometry

### Requirement: Soft wrapping preserves copied source text
KUC MUST distinguish visual wrapping from source paragraph boundaries and copy only separators present in the source.

#### Scenario: Narrow linked inline code wraps across visual lines
- **WHEN** a linked code span wraps at 24, 48 or 120px and the whole source is selected
- **THEN** copying returns the original string without inserting visual-wrap newlines

#### Scenario: Source paragraph separators remain unchanged
- **WHEN** source contains CR, LF, CRLF, U+2028 or U+2029 across independent raster spans
- **THEN** copying preserves each source separator including CRLF across span boundaries
