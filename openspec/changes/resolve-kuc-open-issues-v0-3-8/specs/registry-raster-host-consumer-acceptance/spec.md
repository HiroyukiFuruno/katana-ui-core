## ADDED Requirements

### Requirement: framework-neutral document raster fidelity
The public `raster-host` SHALL apply the same final role typography and logical/physical metrics to document rasterization, wrapping, layout measurement, and node/action hit testing. It MUST NOT require KDV/KatanA-specific coordinate or style correction.

#### Scenario: KDV registry consumer meets canonical crop gate
- **WHEN** KDV resolves the published exact KUC version using only `raster-host` and renders the canonical KatanA document crop
- **THEN** the 95/95 score gate passes without changing its reference, crop convention, or threshold

### Requirement: registry-only per-side grid border consumption
The published `raster-host` SHALL preserve the backward-compatible per-side grid border API and render each side's visibility, style, color, clipping, and merged-anchor behavior. A registry consumer MUST be able to use it without a path or git override and without GUI runtime dependencies.

#### Scenario: consumer projects a grid frame through the public API
- **WHEN** KDV resolves the published exact KUC version and projects a grid cell with distinct four-side borders
- **THEN** the boundary check passes and the rendered output matches the public border contract

### Requirement: release evidence retains existing quality gates
KUC SHALL run its strict quality and release gates on the integrated release HEAD, and SHALL treat KDV/KLE registry consumer evidence as a separate required public-artifact gate.

#### Scenario: public release is verified before Issues close
- **WHEN** v0.3.8 is released
- **THEN** the tag, GitHub Release, crates.io artifact, exact registry consumer resolution, KDV score/border evidence, and KLE artifact evidence are recorded before #35, #37, and #40 are closed
