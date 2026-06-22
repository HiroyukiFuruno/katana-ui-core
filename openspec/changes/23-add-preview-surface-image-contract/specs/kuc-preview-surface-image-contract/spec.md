## ADDED Requirements

### Requirement: ImageSurface carries an opaque RGBA preview surface

KUC MUST expose an `ImageSurface` node kind that carries consumer-rendered RGBA surface data without interpreting Markdown, KMM node ids, or document structure.
The surface props MUST include fingerprint, width, height, RGBA payload, content scale, fit, and accessibility label.

#### Scenario: RGBA surface node is built

- **WHEN** a consumer builds an `ImageSurface` from fingerprint, width, height, and RGBA bytes
- **THEN** the resulting `UiNode` kind is `ImageSurface`
- **AND** the surface props preserve the fingerprint, extent, RGBA bytes, content scale, fit, and accessibility label

#### Scenario: RGBA length is invalid

- **WHEN** the RGBA payload length does not equal `width * height * 4`
- **THEN** KUC rejects the props with a validation error
- **AND** KUC does not silently fallback to text labels

### Requirement: ImageSurface exposes highlight rect overlays

KUC MUST allow consumer-provided highlight rects to be attached to the image surface.
KUC MUST treat those rects as overlay descriptors and MUST NOT calculate viewer search hits itself.

#### Scenario: current search hit is attached

- **WHEN** a consumer attaches a current search hit rect
- **THEN** the rect, current marker, and label are preserved in `UiImageSurfaceProps`

### Requirement: adapters receive the same image surface descriptor

adapter, adapter, and integration plans MUST receive the same image surface descriptor from `PaintRequest` / `UiTree`.
Adapter plans MUST preserve fingerprint, extent, RGBA byte length, fit, accessibility label, and highlight rects.

#### Scenario: PaintRequest contains an ImageSurface

- **WHEN** a `PaintRequest` contains an `ImageSurface` root
- **THEN** each integration plan exposes one image surface descriptor
- **AND** the descriptor includes the surface fingerprint and highlight rects

### Requirement: KDV viewer body remains consumer-owned

KUC MUST NOT own Markdown display-list, KMM node mapping, PDF page model, export pipeline, TOC, viewer search engine, or scroll synchronization.

#### Scenario: KDV supplies a rendered surface

- **WHEN** KDV has already rendered HTML / PDF / PNG / JPG equivalent preview content into an RGBA surface
- **THEN** KUC carries that surface as an opaque `ImageSurface`
- **AND** KDV remains responsible for the document semantics and viewer runtime
