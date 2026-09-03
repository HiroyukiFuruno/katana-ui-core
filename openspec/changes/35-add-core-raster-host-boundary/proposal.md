## Why

KDV v0.5.6 の公開 runtime と KatanA は crates.io の `katana-ui-core` 0.3.3 を解決する一方、KDV の visual gate は Git tag の private `katana-ui-core-storybook` 0.3.0 を解決している。
この二重解決により KatanA canonical `sample_diagrams.md` crop と KDV Storybook の同一内容領域比較は 88/95 となり、95 点 gate を満たさない。

private Storybook crate をそのまま採用すると `eframe` / `egui` が KDV の document surface boundary に混入するため、KDV はその回避実装や自己比較をしてはならない。

Issue: <https://github.com/HiroyukiFuruno/katana-ui-core/issues/35>

## What Changes

- 公開 `katana-ui-core` crate に `raster-host` feature を追加する。
- canvas、UI tree rasterization、presentation、hit-test の framework-neutral な API を `katana_ui_core::raster_host` として公開する。
- private `katana-ui-core-storybook` は interactive `eframe` / `egui` wrapper のまま残し、公開 host API を再利用する。
- KDV は公開済み同一 KUC release line の `raster-host` だけを利用できるようにする。

## Constraints

- `raster-host` の依存 graph に `egui`、`eframe`、`winit` を入れない。
- public crate を分割・追加しない。
- KDV は KUC 所有の raster / hit-test を再実装しない。
- 既存 private Storybook の interactive、pixel、hit-test 契約を壊さない。
- KDV の 95 点 gate と document-surface boundary gate を緩めない。

## Impact

- `katana-ui-core` の feature と公開 API が増える。
- private Storybook の中立部分が公開 core へ移動し、wrapper は re-export / delegation する。
- KDV は KUC v0.3.4 の公開後に Git-only Storybook dependency を除去できる。
