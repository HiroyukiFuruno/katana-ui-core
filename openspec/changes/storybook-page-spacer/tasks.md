# Tasks — storybook-page-spacer

## 1. Harness 契約

- [x] 1.1 `requirements.rs` と Storybook menu に `spacer` が存在することを guard で検査する。
- [x] 1.2 `visual/dedicated.rs::draw_page` から `spacer` 専用描画へ到達することを guard で検査する。
- [x] 1.3 `catalog/preset_labels.rs` に `spacer` 用の 4 つ以上の preset を定義し、preset ごとの preview / state 差分を自動テストで固定する。
- [x] 1.4 `visual/storybook_ui_option_contract.rs` に `spacer` 用の 4 つ以上の option contract を定義し、Inspector に表示する。
- [x] 1.5 代表 option の操作が state / action / event / preview 差分へ反映されることを `window_interaction` 経由で検査する。受動 UI の場合は受動契約と style 差し替えを自動テストで固定する。
- [x] 1.6 light / dark theme の背景、枠線、文字色、入力面相当が theme token から描かれることを数値化された rendering contract で検査する。

## 2. 品質ゲート / DoD

- [x] 2.1 `rtk just ast-lint` を通す。
- [x] 2.2 `rtk just storybook-check` を通す。
- [x] 2.3 `rtk cargo test -p katana-ui-core-storybook --locked` を通す。
- [x] 2.4 `rtk ./scripts/openspec validate storybook-page-spacer --strict` を通す。
