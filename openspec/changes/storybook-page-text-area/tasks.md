# Tasks — storybook-page-text-area

## 1. Harness 契約

- [x] 1.1 `requirements.rs` と Storybook menu に `text-area` が存在することを guard で検査する。
- [x] 1.2 `visual/dedicated.rs::draw_page` から `text-area` 専用描画へ到達することを guard で検査する。
- [x] 1.3 `catalog/preset_labels.rs` に `text-area` 用の 4 つ以上の preset を定義し、preset ごとの preview / state 差分を自動テストで固定する。
- [x] 1.4 `visual/storybook_ui_option_contract.rs` に `text-area` 用の 4 つ以上の option contract を定義し、Inspector に表示する。
- [x] 1.5 代表 option の操作が state / action / event / preview 差分へ反映されることを `window_interaction` 経由で検査する。受動 UI の場合は受動契約と style 差し替えを自動テストで固定する。
- [x] 1.6 light / dark theme の背景、枠線、文字色、入力面相当が theme token から描かれることを数値化された rendering contract で検査する。

## 2. 品質ゲート / DoD

- [x] 2.1 `rtk just ast-lint` を通す。
- [x] 2.2 `rtk just storybook-check` を通す。
- [x] 2.3 `rtk cargo test -p katana-ui-core-storybook --locked` を通す。
- [x] 2.4 `rtk ./scripts/openspec validate storybook-page-text-area --strict` を通す。

## User Review Phase

- [x] text-area は実画面の入力欄をクリックして focus した後、キーボード入力、Backspace、Enter commit を受け取り、preview / state / action / event に反映する。
- [x] text-area は KUC 契約として、テキスト折り返しを default true かつ option 変更可能、resize を default false かつ option 変更可能、縦横 scroll 有効化を default false かつ option 変更可能、縦横 scrollbar 表示を default false かつ対応軸の scroll 有効時だけ option 設定可能にする。
- [x] ユーザーFB: Storybook 上の text-area resize handle は表示だけでなく drag で幅・高さ state を変え、preview / state / action / event に反映する。
- [x] ユーザーFB: Storybook 上の text-area vertical / horizontal scroll は静的表示ではなく wheel 入力で offset を変え、preview / state / action / event に反映する。
