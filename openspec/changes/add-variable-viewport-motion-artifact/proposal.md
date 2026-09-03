## Why

KUC が発行する full-motion plan には resize により viewport 寸法が変わる frame が含まれる一方、既存の `MotionArtifactWriter::write_opaque` は固定寸法 sequence だけを受理する。consumer に raster/compositor 実装や input 改変を持たせず、KUC-owned provenance を保った一つのレビュー可能な artifact を生成できる汎用 opaque API が必要である。

## What Changes

- 既存の固定寸法 `write_opaque` 契約を維持したまま、可変 viewport の opaque receipt sequence を一つの固定 export canvas へ正規化して GIF/MP4 を生成する公開 API を追加する。
- export canvas は source viewport の最大幅・最大高から決定し、各 frame は拡大縮小や crop を行わず決定論的に配置する。
- artifact manifest に source viewport 寸法列、固定 export 寸法、source/decode frame hash、root record hash を記録する。
- 異寸法 sequence、固定寸法 sequence、provenance 不整合、platform profile の回帰を自動テストと release gate で検証する。
- 破壊的変更は行わない。

## Capabilities

### New Capabilities

- `variable-viewport-motion-artifact`: 可変 viewport を含む opaque motion receipt sequence を、KUC-owned provenance を保持した単一の検証可能な GIF/MP4 artifact へ正規化する契約。

### Modified Capabilities

なし。

## Impact

- `katana_ui_core::egui::MotionArtifactWriter` に additive な公開 API を追加する。
- motion artifact の正規化処理、manifest 型、失敗注入用の unit test と実FFmpegによる全motion planの integration test が対象となる。
- 既存の `write` / `write_opaque` と固定寸法 manifest schema の意味は変更しない。
- 出力先の差し替えによる入力破壊と返却pathの不整合を防ぐため、`storybook-artifacts` feature 限定で `cap-std`、`cap-fs-ext`、`same-file`、`tempfile` を追加する。default feature と公開crateの分割方針は変更しない。
