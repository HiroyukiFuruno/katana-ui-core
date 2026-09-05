## Why

KDV v0.5.6 の KatanA canonical crop gate は 95/95 を維持する必要がある。しかし crates.io の `katana-ui-core` 0.3.6 は document role の raster font size、line height、vertical baseline を theme 由来の一体値として導出するため、横方向のtext advanceを調整すると縦方向のlayoutまで変化する。

KDV側でKatanA固有の座標補正やreference置換を行うと、公開coreのownershipと95点gateの意味を損なう。framework-neutralなKUC hostが、roleごとの最終raster metricsを描画・layout・hit-testに一貫して渡す必要がある。

Issue: <https://github.com/HiroyukiFuruno/katana-ui-core/issues/35>

## What Changes

- 公開 `katana-ui-core` crate に `raster-host` feature を追加する。
- canvas、UI tree rasterization、presentation、hit-test の framework-neutral な API を `katana_ui_core::raster_host` として公開する。
- private `katana-ui-core-storybook` は interactive `eframe` / `egui` wrapper のまま残し、公開 host API を再利用する。
- KDV は公開済み同一 KUC release line の `raster-host` だけを利用できるようにする。
- KUC v0.3.7 で本文・H1〜H3の `font_size`、`line_height`、baseline位置を独立指定できるtypedなdocument typography APIを追加する。
- public host、canvas renderer、layout測定、node/action hit-testが同じ最終metricsを参照する。

## Constraints

- `raster-host` の依存 graph に `egui`、`eframe`、`winit` を入れない。
- public crate を分割・追加しない。
- KDV は KUC 所有の raster / hit-test を再実装しない。
- 既存 private Storybook の interactive、pixel、hit-test 契約を壊さない。
- KDV の 95 点 gate と document-surface boundary gate を緩めない。
- KatanA固有の座標・style補正、reference画像の置換、score閾値の変更を行わない。

## Impact

- `katana-ui-core` v0.3.7 の公開APIが増える。
- private Storybook の中立部分が公開 core へ移動し、wrapper は re-export / delegation する。
- KDV はKUC v0.3.7公開後、registry-only dependencyとしてこのtyped APIを採用する。
