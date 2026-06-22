## Why

エディタ + プレビュー / サイドバー + メイン / 上下分割など、可変サイズの 2 ペイン分割は汎用 UI で頻出。`../katana/crates/katana-ui/src/views/layout/` (split.rs / split_horizontal.rs / split_vertical.rs / split_handle.rs) の役割を Adapter に移植する。各ペインの中身は consumer の責務とし、`SplitPane` はサイズ管理のみを担う。

## What Changes

- `layout/split/` に `SplitPane` widget を提供。
- props: `direction`（`Horizontal` / `Vertical`）、`ratio: f32`（0.0〜1.0）、`on_ratio_change: Fn(f32)`、`min_first` / `min_second`（pt 指定）、`first: View`、`second: View`。
- ハンドル部はドラッグで ratio を更新。double-click で 50/50 に戻す（v0.1 では実装、不要なら off にできる prop は将来）。
- ハンドル幅と hover 色は theme トークン。

## Capabilities

### New Capabilities

- `widget-split-pane`: 2 ペインの可変サイズ分割。direction × ratio × min 制約を統一。

## Impact

- ダッシュボード / エディタ / プレビューなどの汎用シェル骨格の基盤。
- 3 ペイン以上が必要なケースは `SplitPane` を入れ子にする運用とし、専用 widget は当面提供しない。
