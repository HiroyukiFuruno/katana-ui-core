## ADDED Requirements

### Requirement: logical and physical text raster dimensions agree
KUC SHALL allocate text textures in physical pixels and SHALL derive every layout reservation, paint rectangle, clip rectangle, and hit bound from the same logical conversion `ceil(physical / scale)`. `max_width_px` supplied by an egui caller SHALL be a logical width and text raster layout SHALL perform the only logical-to-physical conversion.

#### Scenario: TextSurface gutter remains inside its reservation
- **WHEN** automatic gutter labels are rasterized at scale 1, 1.5, or 2
- **THEN** each label paint rectangle has the converted logical dimensions, fits the reserved gutter and row height, and does not overlap the text viewport

#### Scenario: Diagnostics reservation matches painted text
- **WHEN** Diagnostics labels and filters containing Japanese text and `⭐️` are rasterized at scale 1, 1.5, or 2
- **THEN** their reserved widths contain the converted texture bounds, adjacent controls do not intersect, and wrapping/clipping stays within the requested logical bounds

### Requirement: shared text-raster callers are audited
KUC SHALL apply the same conversion contract to StatusBar and CommandChrome callers that consume physical raster dimensions, or SHALL add a numerical regression proving that their existing conversion is equivalent.

#### Scenario: audited caller paints and hits the same logical bounds
- **WHEN** an audited caller renders a non-unit-scale text raster
- **THEN** its paint and hit bounds use the same logical extent as its reservation
