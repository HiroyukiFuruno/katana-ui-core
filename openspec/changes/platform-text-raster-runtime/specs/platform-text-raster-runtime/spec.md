## ADDED Requirements

### Requirement: Platform text raster runtime is public and framework-neutral
KUC SHALL provide the public `katana_ui_core::text_raster` module behind the `katana-ui-core` `text-raster` optional feature. The default feature set MUST remain framework-neutral, and the module MUST NOT expose egui, KLE, KDV, KatanA, windowing, clipboard, or file-IO types in its public API.

#### Scenario: Consumer constructs a renderer from generic configuration
- **WHEN** a Rust consumer creates `PlatformTextRasterizer` from `PlatformTextRasterConfig`
- **THEN** it can rasterize generic `UiTextSpan` runs without importing a KatanA-series product crate

### Requirement: Emoji segmentation and platform font selection have one generic source of truth
The runtime MUST derive emoji runs from `UiEmojiTextSegments` / `UiTextSpanStyle::emoji` and MUST resolve emoji font families through generic platform configuration. For a monospace request, it MUST retain monospace selection for ASCII code text and use the platform proportional fallback for non-ASCII text when the monospace face cannot guarantee that script's glyph coverage. KLE and KDV MUST NOT implement an independent emoji segmentation or OS emoji font-family resolver.

#### Scenario: Star variation selector is preserved as one emoji run
- **WHEN** a request contains `⭐️` between non-emoji text
- **THEN** the runtime reports one emoji grapheme run containing both the star and variation selector and selects the configured platform emoji family

#### Scenario: Japanese text remains visible in a monospace editor request
- **WHEN** a request uses a monospace font token and includes Japanese text
- **THEN** the Japanese glyph pixels are produced through the platform proportional fallback without moving font resolution into KLE or KDV

### Requirement: Raster output preserves platform color glyph pixels
The runtime SHALL return RGBA pixels produced by the selected platform font. It MUST preserve glyph callback color for color emoji rather than recoloring it with the foreground text color. Absence of a color-capable platform font MUST be returned as explicit report/error state, not accepted as color emoji success.

#### Scenario: Color emoji raster is verifiable
- **WHEN** a color-capable emoji font is available and a request contains `🔥` or `⭐️`
- **THEN** the returned raster contains chromatic non-background pixels and its report identifies the resolved emoji font family

### Requirement: Layout, grapheme bounds, and hit-test share shaping output
The runtime MUST return grapheme byte ranges and bounds derived from the same shaped layout as its pixels. Hit-test and caret queries MUST resolve through those bounds and MUST preserve UTF-8, ZWJ, and variation-selector grapheme boundaries.

#### Scenario: Hit-test on a variation-selector emoji returns the whole grapheme
- **WHEN** a hit-test coordinate falls inside the rendered `⭐️` glyph
- **THEN** the result identifies the byte range for the full `⭐️` grapheme rather than a partial scalar value

### Requirement: Grapheme edit boundaries are public and shared
The runtime SHALL expose framework-neutral previous and next grapheme byte ranges for a source string. Consumers that provide text editing MUST use these ranges for cursor movement and deletion so a variation selector, combining mark, or ZWJ sequence is never separated by an editor-specific scalar implementation.

#### Scenario: Backspace resolves the full star grapheme
- **WHEN** the byte cursor is immediately after `⭐️`
- **THEN** the previous grapheme range contains both the star and its variation selector

### Requirement: Stable inputs reuse runtime state without layout jitter
The rasterizer MUST reuse its font database and cache for identical configuration/request keys. It MUST expose a deterministic report or statistic sufficient to verify cache reuse without inspecting implementation internals.

#### Scenario: Repeated identical raster request is a cache hit
- **WHEN** the same request is rasterized twice with one rasterizer instance
- **THEN** the second result preserves pixel/layout output and records cache reuse without reloading the font database

### Requirement: Raster allocation is bounded and non-finite inputs are explicit
The runtime SHALL normalize a non-finite wrap width to its documented safe fallback and SHALL return a typed error before allocating pixels for a non-finite scale or an extent beyond the configured raster allocation bound. It MUST NOT panic from `width * height` overflow.

#### Scenario: Infinite wrap width cannot overflow the pixel buffer
- **WHEN** a consumer supplies an infinite wrap width
- **THEN** the rasterizer uses its safe fallback width and returns a finite raster or typed error without panicking
