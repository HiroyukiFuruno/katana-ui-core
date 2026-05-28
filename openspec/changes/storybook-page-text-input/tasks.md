# Tasks — storybook-page-text-input

## 1. Harness 契約

- [x] 1.1 `requirements.rs` と Storybook menu に `text-input` が存在することを guard で検査する。
- [x] 1.2 `visual/dedicated.rs::draw_page` から `text-input` 専用描画へ到達することを guard で検査する。
- [x] 1.3 `catalog/preset_labels.rs` に `text-input` 用の 4 つ以上の preset を定義し、preset ごとの preview / state 差分を自動テストで固定する。
- [x] 1.4 `visual/storybook_ui_option_contract.rs` に `text-input` 用の 4 つ以上の option contract を定義し、Inspector に表示する。
- [x] 1.5 代表 option の操作が state / action / event / preview 差分へ反映されることを `window_interaction` 経由で検査する。受動 UI の場合は受動契約と style 差し替えを自動テストで固定する。
- [x] 1.6 light / dark theme の背景、枠線、文字色、入力面相当が theme token から描かれることを数値化された rendering contract で検査する。

## 2. 品質ゲート / DoD

- [x] 2.1 `rtk just ast-lint` を通す。
- [x] 2.2 `rtk just storybook-check` を通す。
- [x] 2.3 `rtk cargo test -p katana-ui-core-storybook --locked` を通す。
- [x] 2.4 `rtk ./scripts/openspec validate storybook-page-text-input --strict` を通す。

## User Review Phase

- [x] text-input は実画面の入力欄をクリックして focus した後、キーボード入力、Backspace、Enter commit を受け取り、preview / state / action / event に反映する。
- [/] ユーザーFB: `readonly` / `placeholder` は core の `Input` 契約に存在するだけでなく、Storybook preset と Inspector option から確認できるようにする。
- [/] ユーザーFB: text-input の既定入力開始位置は field 左端 + 2px とし、左アイコン用の余白は既定では確保しない。
- [/] ユーザーFB: 左アイコン領域を「アイコンなし・領域だけ確保」できる option / preset として設計し、`UiSlotSpec` では `icon: Option<UiIconProps>` と `reserve_space` を持たせる。
- [/] ユーザーFB: Storybook に検索 SVG アイコンの preset を追加する。ただし text-input 本体は汎用部品として SVG icon prop を受け取るだけにする。
- [/] ユーザーFB: text-input 内右側に VSCode 風の SVG icon button を配置できる契約を追加し、複数 button と callback id を render model / Storybook preset / interaction test で固定する。
- [/] ユーザーFB: `readonly` preset は focus できても keyboard input / Backspace で値を書き換えず、state / action / event で readonly block を示す。
- [/] ユーザーFB: text-input の runtime state は atom instance ごとに内部保持し、preset/tab 切り替えや複数 text-input 配置で値・focus・caret が同期しないことを自動テストで固定する。
- [/] ユーザーFB: Storybook preset tab の label はタブ領域内で計測・縮小・clip し、隣接 tab へ文字がめり込まないことを自動テストで固定する。
- [/] ユーザーFB: leading SVG icon と入力文字は text-input field 内で上下中央を揃え、current caret の左開始位置は枠線内側から約 2px にすることを layout test で固定する。
- [x] ユーザーFB: Storybook の検索 SVG icon は Katana 本体の `assets/icons/katana/ui/search.svg` と同じ SVG source を使う。
- [x] ユーザーFB: text-input の入力値変更はリアルタイムに `text_input_changed` event として Storybook の state / action / event 行で確認できることを自動テストで固定する。
- [x] ユーザーFB: text-input 内右側の SVG icon button は hover 時に visible border を描き、native window では pointing hand cursor を使うことを自動テストで固定する。
- [x] ユーザーFB: text-input 内右側の SVG icon button label は固定座標調整ではなく、CSS の center 相当の text-in-rect layout API で上下左右中央配置する。
