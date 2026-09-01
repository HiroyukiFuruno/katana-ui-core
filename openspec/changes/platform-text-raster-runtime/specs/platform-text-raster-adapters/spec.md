## ADDED Requirements

### Requirement: KUC Storybook, KDV, and KLE use the shared platform text runtime
KUC Storybook, KDV, and KLE SHALL use `katana_ui_core::text_raster` with the `katana-ui-core` `text-raster` feature for platform emoji/font/raster/layout behavior. They MUST be thin adapters around the runtime and MUST NOT retain separate emoji segmentation, font-family lookup, or rich text raster caches.

#### Scenario: Adapter dependency and source checks reject duplicate renderers
- **WHEN** KUC/KDV/KLE adapter checks run
- **THEN** they confirm the shared runtime dependency and reject direct duplicate platform text renderer implementations

### Requirement: KLE consumes a KUC-owned surface adapter with one KUC layout
KLE's concrete editor binding MUST consume the KUC-owned shared text-surface adapter, which uses the runtime layout for visible platform emoji glyphs, caret placement, selection, and hit-test. A KLE-local surface renderer, egui font atlas result, or fallback-only artifact MUST NOT be used as acceptance evidence for those properties.

#### Scenario: Editor artifact comes from the live surface render path
- **WHEN** KLE Storybook generates a motion artifact with Japanese text and `⭐️`
- **THEN** its pixels, caret bounds, and hit-test metadata originate from the same KUC-owned TextSurface adapter path used by the interactive window

### Requirement: KDV migration preserves generic renderer behavior
KDV SHALL replace its direct platform emoji/font/raster implementation with the shared runtime while retaining document-specific Markdown/export composition outside the runtime.

#### Scenario: KDV uses shared emoji raster without KatanA-specific runtime API
- **WHEN** KDV renders a document containing `⭐️` and a ZWJ emoji sequence
- **THEN** it uses the KUC runtime API and preserves color pixel and grapheme behavior without adding KatanA identifiers to KUC
