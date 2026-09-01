## Context

`katana-ui-core` は framework-neutral な UI model crate であり、`cosmic-text` のような raster dependency を core に持たない。一方で KUC Storybook には `cosmic-text` の `FontSystem`、`SwashCache`、color glyph callback、rich text cache があり、KDV も同種の font/raster code を持つ。KLE は egui font atlas を使うため、OS color emoji の表示がこの実装群と一致しない。

## Goals / Non-Goals

**Goals:**

- framework-neutral KUC model を入力に、platform font resolution、color glyph raster、grapheme measurement、hit-test を提供する public runtime を作る。
- `UiEmojiTextSegments` / `UiTextSpan` を text run 分割の唯一の source of truth にし、KLE/KDV の duplicate implementation を除く。
- stable request/configuration に対する cache reuse を API contract と test で保証する。
- KUC Storybook、KDV、KLE が同じ runtime を使うことを compile/runtime contract で検証する。

**Non-Goals:**

- `katana-ui-core` core crate に `cosmic-text`、windowing、egui、KatanA/KLE/KDV 固有 enum を入れない。
- Markdown parsing、editor command、clipboard、host file IO を renderer runtime の責務にしない。
- OS に存在しない font asset を bundled font で偽装して platform color glyph と主張しない。

## Decisions

### Public renderer module を optional feature 境界に置く

単一の public `katana-ui-core` crate は `text-raster` optional feature で `katana_ui_core::text_raster` module を公開し、`cosmic-text` をこの feature に閉じる。公開 API は `PlatformTextRasterizer`、`PlatformTextRasterConfig`、`PlatformTextRasterRequest`、`PlatformTextRaster`、`PlatformTextLayout`、`PlatformTextGraphemeBounds`、`PlatformTextRasterError` に限定する。KLE/KDV 固有型は受け取らず、run は `UiTextSpan` と generic font/style/configuration から構成する。

代替として raster dependency を default feature に直接追加する案は、default core の framework-neutral dependency boundary を壊すため採用しない。Storybook crate に留める案は downstream consumer が再利用できず duplicate code を残すため採用しない。

### Raster と layout を同じ shaping 結果から返す

request は source text、`UiTextSpan`、font role/family/size/line height、foreground、wrap width、scale factor を持つ。rasterizer は `cosmic-text` の advanced shaping を一度だけ行い、同一 layout から RGBA pixels、size、grapheme byte range/bounds、hit-test map を返す。emoji span は `UiEmojiTextSegments` の結果と `UiTextSpanStyle::emoji` から platform emoji family を選ぶ。color glyph callback が返す glyph color は text foreground で上書きしない。

別の measurement implementation を作る案は、KLE caret と visible glyph がずれるため採用しない。

### Platform font selection は explicit configuration と generic defaults を併用する

`PlatformTextRasterConfig` は proportional/monospace/emoji candidate paths と optional family overrides を持つ。caller が指定した candidates を優先し、指定がない場合は OS generic default resolver を使う。macOS の emoji run は `Apple Color Emoji` family、その他 OS は system delegate を選ぶ。font discovery failure は empty output や silent fallback にせず typed error/report で返す。

### Cache は rasterizer instance に閉じる

font system、swash cache、raster/layout cache は `PlatformTextRasterizer` instance に所有させ、request/style/font/scale/wrap/config signature を key にする。stable key では font database を再ロードせず、cache hit/miss statistics を test-only or diagnostic report で観測可能にする。KLE interactive input は frame ごとの font/cache initialization を禁止する。

### Migration は source ownership を移す

まず KUC Storybook の internal renderer を `katana_ui_core::text_raster` module の adapter へ置換する。次に KDV の emoji/font/raster path を同 module に置換し、最後に KUC-owned shared egui adapter の TextSurface を runtime output へ接続する。KLE editor surface と Storybook artifact はその adapter を consumer として利用する。KLE の neutral crate には runtime 型を漏らさず、KLE egui binding と Storybook host だけが `text-raster` feature を有効化する。

## Risks / Trade-offs

- [OS font availability] → candidate/family report と platform-conditional contract tests を用意し、color-capable font が無い状態を color emoji success と誤認しない。
- [KLE text edit metrics mismatch] → renderer layout の grapheme bounds/hit-test を editor adapter の caret/selection source にし、egui font metrics を emoji caret 根拠にしない。
- [Large documents/cache growth] → cache key を bounded/LRU policy にし、cache stats と eviction test を持つ。
- [Cross-repo migration overlap] → KUC runtime API と unit tests を先に確定し、KDV/KLE adapters は別々の write scope で移行する。既存 user changes は戻さない。

## Migration Plan

1. public `katana_ui_core::text_raster` module と contract tests を `katana-ui-core` crate に追加する。
2. KUC Storybook の text renderer を public runtime adapter に置換して既存 visual/color tests を移す。
3. KDV を runtime adapter へ移行し、direct `cosmic-text` font/raster code を削除する前に output/metric tests を通す。
4. KUC-owned shared egui adapter が runtime raster/layout を使う TextSurface を提供し、KLE は thin binding と same-surface Storybook artifact の consumer になる。
5. KLE/KUC/KDV の AST/dependency guard に duplicate renderer 禁止と runtime ownership を追加する。

## Open Questions

- custom KUC-backed TextSurface と shared egui adapter に完全移行する。KLE local transparent overlay は generic renderer duplication になるため採用しない。visible glyph/caret/hit-test は同一 layout/frame record を使う。
