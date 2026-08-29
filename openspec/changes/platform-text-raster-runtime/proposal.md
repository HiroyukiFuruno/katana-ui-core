## Why

KLE は OS emoji を `egui::TextEdit` の font atlas に任せ、KDV は独自の `cosmic-text` rasterizer を持っている。この分離では `⭐️` の color glyph、variation selector、grapheme の計測と hit-test を共通に保証できず、KLE/KDV が同じ問題を別々に実装する。

## What Changes

- `katana-ui-core-text-raster` という public renderer runtime crate を追加する。`katana-ui-core` の domain contract を入力にして、platform font selection、rich text shaping、color glyph raster、grapheme bounds、hit-test を提供する。
- `UiEmojiTextSegments` と `UiTextSpan` を唯一の emoji/run 分割 source of truth とし、KLE/KDV 固有の emoji segmentation、OS font-family lookup、raster cache を禁止する。
- stable input/configuration に対して font system や raster cache を再初期化しない contract を設け、interactive text input の layout jitter を防ぐ。
- KUC Storybook 内部の text raster 実装を public runtime へ移し、KDV と KLE は thin adapter として同じ runtime を利用する。
- `⭐️`、ZWJ sequence、variation selector、Japanese text、grapheme caret/hit-test、color pixel output を契約テストと数値検証で固定する。

## Capabilities

### New Capabilities

- `platform-text-raster-runtime`: framework-neutral rich text raster、platform font resolution、color emoji pixel output、measurement、grapheme hit-test の public contract。
- `platform-text-raster-adapters`: KUC Storybook、KDV、KLE が同一 runtime を使用し、duplicate renderer を残さない integration contract。

### Modified Capabilities

- なし。

## Impact

- `Cargo.toml` と `crates/katana-ui-core-text-raster/` に renderer runtime と `cosmic-text` 依存を追加する。
- `crates/katana-ui-core-storybook/` の internal text raster を runtime crate へ移行する。
- local KDV の document/export text rendering と local KLE の emoji editor surface/Storybook artifact が同 crate の adapter を使う。
- `katana-ui-core` core crate の public UI DTO と framework-neutral 依存境界は維持する。
