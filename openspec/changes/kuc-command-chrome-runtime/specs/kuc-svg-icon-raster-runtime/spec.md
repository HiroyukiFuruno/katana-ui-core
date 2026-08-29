## ADDED Requirements

### Requirement: KUC SHALL provide a renderer-neutral SVG icon raster runtime

KUC SHALL provide a public runtime crate that accepts `UiIconProps`, physical
pixel dimensions, and a semantic `RgbaColor`, and returns unpremultiplied RGBA
pixels plus deterministic metadata. The runtime SHALL not accept or expose
KatanA, KLE, KDV, Markdown, editor, viewer, or host command types.

#### Scenario: host-supplied SVG is rasterized without host semantics

- **WHEN** a consumer submits a valid `UiIconProps` with a physical size and color
- **THEN** the runtime returns RGBA pixels whose dimensions equal the request
- **AND** the result metadata contains no host command name or host enum value

### Requirement: SVG paint policy SHALL be deterministic and explicit

The runtime SHALL apply `UiSvgPaintPolicy` consistently for `currentColor`,
stroke, fill, and alpha. The color in the request SHALL participate in the
raster result and cache identity.

#### Scenario: same SVG with different colors does not reuse incorrect pixels

- **WHEN** the same icon and size are rasterized with two different RGBA colors
- **THEN** each output uses its requested color policy
- **AND** the cache does not return the first color's pixels for the second request

### Requirement: invalid or unsafe requests SHALL return typed errors

The runtime SHALL reject invalid SVG, zero dimensions, dimensions above its
configured maximum, and allocation overflow with a typed error. It SHALL NOT
replace a failed icon with a Unicode character, text glyph, or OS-dependent
emoji fallback.

#### Scenario: invalid SVG cannot become a font-based substitute

- **WHEN** a request contains invalid SVG source
- **THEN** rasterization returns a typed invalid-SVG error
- **AND** no text or emoji substitute pixels are emitted

### Requirement: SVG cache behavior SHALL be bounded and reproducible

Each rasterizer instance SHALL own a bounded cache. Stable requests SHALL yield
pixel-equal outputs and observable cache reuse; eviction SHALL follow a
deterministic policy defined by the runtime.

#### Scenario: stable request is pixel-equal and reports a cache hit

- **WHEN** the same valid request is rasterized twice by one rasterizer instance
- **THEN** both RGBA buffers are byte-equal
- **AND** the second operation reports reuse according to the runtime cache statistics

### Requirement: private Storybook SVG rasterization SHALL not remain a second runtime

KUC Storybook SHALL consume the public SVG icon raster runtime for generic icon
controls. It SHALL not retain an independent SVG parser, paint-policy resolver,
or icon pixel cache.

#### Scenario: Storybook icon control uses public runtime output

- **WHEN** a Storybook command-icon fixture is rendered
- **THEN** its RGBA layer is produced by the public SVG raster runtime
- **AND** a dependency/AST guard rejects a second private icon raster path
