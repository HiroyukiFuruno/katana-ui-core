# raster-document-typography Specification

## Purpose

Document typography overrides preserve the legacy integer-coordinate contract while allowing hosts to explicitly opt into fractional baseline layout.

## Requirements

### Requirement: Legacy typography preserves integer rendering

A renderer using `UiTreeTextRoleTypography` SHALL retain the integer-coordinate rendering and layout contract.

#### Scenario: Legacy document host

- **WHEN** a host configures only legacy role typography
- **THEN** render and layout use integer origins
- **AND** fractional line-box cursor behavior is not introduced.

### Requirement: Fractional baseline typography is explicit

A renderer SHALL use logical line-box coordinates only when a `UiTreeTextRoleBaselineTypography` override is configured.

#### Scenario: Fractional-baseline document host

- **WHEN** a host configures a `UiTreeTextRoleBaselineTypography` override for a document role
- **THEN** render and layout use logical line-box coordinates for that host
- **AND** integer-coordinate legacy role typography remains available when no baseline override is configured.

### Requirement: Compact paragraph wrapping preserves interactive geometry

The raster host SHALL use the same wrapped lines for paragraph rendering, measured layout, and span link action hit regions.

#### Scenario: Compact paragraph wraps a link onto a following line

- **WHEN** a 14-point paragraph at the document content width wraps a link after preceding text
- **THEN** the link action hit region follows the displayed line and visible link bounds
- **AND** the original unwrapped position does not retain a stale link hit region
- **AND** existing compact body, HTML, export, list, and footnote wrapping contracts remain unchanged.

### Requirement: Explicit candidate selection preserves the selected face

For glyphs supported by the selected face, a raster host using `PlatformTextFaceSelection::FirstCandidate` SHALL preserve the source file and collection index of the first usable proportional or monospace candidate throughout measurement and rasterization.

#### Scenario: Candidate attributes differ from the requested style

- **WHEN** the selected candidate has a different weight, style, or stretch from the text token or span
- **THEN** the host uses the selected face's actual weight, style, and stretch for font matching
- **AND** bold, italic, or another token weight does not silently select a system face
- **AND** another face with the same family name in the collection is not substituted.

#### Scenario: Generic system selection remains available

- **WHEN** the host uses `PlatformTextFaceSelection::System`
- **THEN** the existing token weight, bold, and italic requests remain in effect
- **AND** emoji resolution and missing-candidate generic fallback retain their existing contracts.

#### Scenario: Paragraph fits within its measured draw width

- **WHEN** a paragraph's glyph width fits within the available content width at its configured draw font size
- **THEN** wrapping does not inflate the paragraph font size to compensate for a different selected face.


### Requirement: Text hit bounds contain fractional paint edges

Text node and non-link action hit bounds SHALL contain the integer paint height at the logical text origin without changing the logical layout advance.

#### Scenario: Fractional text origin

- **WHEN** text begins at a fractional logical vertical coordinate
- **THEN** its node and non-link action bounds cover from the floor of the origin through the ceiling of the origin plus its existing integer paint height
- **AND** the following node retains its existing logical origin
- **AND** link action regions retain their per-line clipping contract.

### Requirement: Explicit lower heading typography

A raster host SHALL allow consumers to configure independent legacy or fractional baseline typography for the exact heading-4, heading-5, and heading-6 text roles.

#### Scenario: Consumer preserves each lower heading line box

- **WHEN** a consumer explicitly configures typography for heading-4, heading-5, or heading-6
- **THEN** measurement, rendering, and hit collection use that role's configured font size, line-box height, and baseline
- **AND** configuring one role does not replace another role's typography
- **AND** existing heading-1 through heading-3 and unconfigured or unknown text roles retain their existing behavior.

### Requirement: Explicit centered rows preserve fractional child placement

When document baseline typography enables logical layout, a row with an explicit height and centered vertical alignment SHALL use the available vertical space to center its child line boxes at fractional logical coordinates.

#### Scenario: A body line is centered in a taller row

- **WHEN** a centered row allocates 28 logical pixels to a child with a 21-pixel logical line box
- **THEN** the child begins 3.5 logical pixels below the row origin
- **AND** rendering, node hit regions, and action hit regions use the same child placement
- **AND** default start alignment, rows without an explicit height, and legacy integer layout retain their existing contracts.

### Requirement: Inline monospace face survives rich-line rendering

Inline-code and monospace span styles SHALL select the same configured font family during measurement, wrapping, rendering, and link hit collection.

#### Scenario: Normal and inline-code spans share a document line

- **WHEN** a line contains both ordinary text and an explicitly monospace or inline-code span
- **THEN** rich-line rendering preserves the span's font-family selection
- **AND** displayed glyph positions and the measured span advances identify the same link action regions
- **AND** the ordinary text's font selection, existing public APIs, and integer layout cursor contract remain unchanged.

### Requirement: Fractional button hit bounds contain the painted control

Button, text-button, and icon-text-button interaction bounds SHALL contain the complete painted control when the document layout places it at a fractional logical origin.

#### Scenario: A fixed-height button follows a fractional heading

- **WHEN** a 28-pixel button begins at logical y=31.5
- **THEN** its node and action hit rectangles cover integer y=31 through y=60 exclusively
- **AND** all hover-border pixels remain inside those bounds at both normal and doubled canvas scale
- **AND** the next layout origin still advances by 28 logical pixels.

### Requirement: Inline highlight colors accept explicit theme tokens

The raster host SHALL use the optional `text-highlight-background` theme color for ordinary highlighted spans, preserving the existing color when the token is absent.

#### Scenario: A consumer supplies its document highlight color

- **WHEN** a theme contains `text-highlight-background`
- **THEN** the raster host paints ordinary highlight backgrounds with that color
- **AND** consumers that omit the token retain the existing highlight color in light and dark themes
- **AND** current-highlight priority, inline-code backgrounds, and layout geometry remain unchanged.

### Requirement: Hover surfaces preserve document layout

Wrapping a document node through the public hover helper SHALL preserve its logical advance and the placement of following content.

#### Scenario: A fractional text row is hovered while scrolled

- **WHEN** a document text row with a fractional line box is wrapped in a hover surface
- **THEN** the hovered row retains its ordinary logical height and clipping contract
- **AND** following glyphs, node hits, and action hits remain in the same positions
- **AND** fully and partially visible rows use the same typography as their unhovered counterparts.

#### Scenario: A text row reaches a fractional scroll boundary

- **WHEN** scrolling reaches or passes the logical bottom of an automatic single-text hover surface
- **THEN** both rendering and scroll-clipped action collection omit that row
- **AND** a partially visible fractional row remains eligible before that boundary
- **AND** the following row retains the same placement with or without the hover wrapper.

#### Scenario: A partially visible hover surface contains multiple children

- **WHEN** a hover surface containing multiple children is partially scrolled into view
- **THEN** every visible child is rendered inside its corresponding interaction bounds
- **AND** the single-text optimization does not discard later children
- **AND** explicit-height single-text surfaces retain their existing partial-rendering contract.

### Requirement: Scroll offsets apply once to visible controls

The scroll renderer SHALL pass viewport coordinates to visible child controls after applying the scroll offset once.

#### Scenario: An absolute control remains visible after scrolling

- **WHEN** a ScrollArea scrolls a frame containing an absolute control into view
- **THEN** its hover border is painted inside the visible control hit region
- **AND** equivalent internal and external scroll offsets preserve the same control placement.
