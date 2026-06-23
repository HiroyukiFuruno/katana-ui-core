## Why

選択肢が多い (>5) 場合は SegmentedToggle ではなくドロップダウン (combo box) が必要。`../katana/crates/katana-ui/src/widgets/combo_box/` の役割を Adapter に移植し、選択値表示 + 開閉動作 + 一覧表示の最小実装にする。検索フィルタは含めない（必要になったら別 change）。

## What Changes

- `composite/selector/select/` に `SelectBox<K>` widget を提供。
- props: `value: K`、`options: Vec<(K, String)>`（K は `Eq + Clone`）、`on_change: Fn(K)`、`placeholder`、`size`、`disabled`、`a11y_label`。
- トリガはラベル + 下向き chevron icon。クリックで一覧パネルを下方向に展開（画面端で上展開に切替）。
- 一覧は内部 popover で表示（`layout/popover` の依存待ち。本 change では暫定的に最小実装でよく、21 完了時に `Popover` へリファクタ）。

## Capabilities

### New Capabilities

- `widget-select-box`: 単一選択ドロップダウン。検索なし最小実装。

## Impact

- 設定画面・フィルタ UI で使う。
- 21 (popover) 完了後に内部実装を Popover ベースに置き換える追従 task を 21 側で持たせる。
