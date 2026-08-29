# Fallible Text Command Root Design

## Status

This design is required before KUC can be a release candidate for KLE v0.1.0.
The current adapter has `expect` and `panic!` paths around catalog policy,
theme lookup, text rasterization, and paint-plan production. Those paths are
release blockers: a failed generic KUC frame must never be converted into a
process abort, a blank fallback frame, or KLE-owned recovery.

## Ownership

- KUC owns catalog policy validation, theme tokens, child construction, frame
  composition, rasterization, and typed errors.
- KLE receives only a closed KUC result or error. It must not retry with a
  different catalog, inspect child state, synthesize a paint plan, or render a
  fallback surface.
- KatanA remains responsible for its host effects after a separately approved
  KLE adoption. It is not an error-recovery path for KUC presentation.

## Public Contract

1. Every constructor that combines a `PlatformFontCatalog` and
   `PlatformTextRasterConfig` is fallible. `with_catalog*` returns the local
   adapter error containing `PlatformTextRasterError`; it does not return
   `Self` by calling `expect`.
2. Every standalone constructor that creates a catalog is also fallible. The
   public name may change to `try_new`, but a retained root cannot keep a
   non-fallible `new` that hides an invalid catalog configuration.
3. The retained `EguiTextCommandSurfaceAdapter` constructs all children through
   one fallible root factory. Its root error enum preserves the originating
   child class: text surface, command chrome, context menu, source address,
   status bar, diagnostics, tab strip, or style.
4. `TextCommandSurfaceStyle::from_theme` returns a typed missing/invalid token
   error. `standard` propagates that result. A missing color, font, spacing, or
   invalid spacing value is not replaced with a guessed value.
5. `show` methods that rasterize or compose return `Result`. A missing paint
   plan becomes a specific error and no incomplete output is returned. Raster
   and measurement errors retain their source error via `From`/variant mapping.
6. Default construction, a catalog mismatch, an unavailable glyph face, a
   failed raster, a missing child frame, and a missing union bound each have
   negative tests. Assertions in tests return `Result` instead of `expect` or
   `panic!` so strict Clippy applies without exclusions.

## Migration Order

1. Add typed errors to the style factory and each leaf adapter without fallback
   branches.
2. Convert text surface, command chrome, context menu, source address, status
   bar, diagnostics, and tab-strip constructors to `Result`.
3. Convert child `show` and composition to propagate typed errors. Keep the
   root frame atomic: either all required child plans are present or the frame
   fails.
4. Update the retained root factory and all real consumers to propagate the
   closed error. KLE remains opaque throughout.
5. Add contract tests for every error variant, then run full KUC check on all
   three native CI profiles. No lint suppression, `unwrap_or`, default raster,
   or error-to-empty-output conversion is permitted.

## Acceptance

- The strict KUC Clippy command has zero `expect_used` and `panic` violations.
- A catalog policy mismatch is observable as a typed KUC error at the root.
- A child raster or missing paint plan prevents an opaque root receipt.
- The normal root keeps one catalog and preserves Japanese IME, exact `⭐️`
  (`U+2B50 U+FE0F`), `☆`, ZWJ, AccessKit, and deterministic artifact tests.
- KLE's AST boundary continues to reject local presentation or recovery logic.
