## Why

`katana` / `katana-chat-ui` の各画面（編集器、エクスプローラ、tab bar、message list、attachment、output handoff）はすべて右クリックでセクション付きメニューを開くことを前提にしている。KUC は `Menu` / `MenuButton` molecule を持つが、いずれも anchor を持つトリガー要素から開く前提であり、画面任意座標（pointer 座標または focused element）に開く文脈メニュー（context menu）に必要な API（pointer 起点 anchor、headers / dividers / submenu、エッジフリップ、キーボード ナビゲーション、外側クリック / Esc クローズ）を備えていない。

これにより consumer は KUC の外で独自の右クリック menu を再実装する必要があり、tab bar の `tab_context_menu`、editor の `context_menu`、explorer の `header_menu` などはすべて KUC 外実装になっている。core が canonical な ContextMenu を提供しないと、入力 / イベント / 状態が ad hoc に重複し、自動テスト / 画像回帰 / 入力回帰の対象から外れる。

## What Changes

- `widget::molecules` に `ContextMenu` molecule を追加し、pointer 座標 / 仮想 anchor / 既存ノード anchor の3種類で開けるようにする。
- セクション分割（section header）、区切り（divider）、サブメニュー、disabled / destructive variant、leading icon、trailing key cap（ショートカット表示）、checked / radio state を typed option として持つ。
- 開閉 / 選択 / Esc / 外側クリック / フォーカスリターン / submenu ホバー遅延を `UiAction` / `UiEvent` として標準化する。
- キーボードナビゲーション（↑↓ で項目移動、→ で submenu 開、← で submenu 閉、Enter / Space で確定、Home / End、Type-ahead）を契約に含める。
- pointer 座標から開いた場合のエッジフリップ（画面外回避）、submenu の自動配置、最小幅、最大高 + 内部スクロールを契約に含める。
- Storybook に編集器 / explorer / tab bar / message 用の preset を追加し、operation log と state 切替えを検査できるようにする。
- 既存 `Menu` molecule を anchor 起点メニューとして残し、両者の責務境界を `widget::molecules` 公開境界の改訂で明確化する。

## Capabilities

### New Capabilities

- `kuc-context-menu`: ContextMenu molecule の option / action / event / state / preset / preview / settings / 自動テスト / 画像回帰 / Storybook ページの完了条件を定義する。

### Modified Capabilities

- `kuc-widget-layer`: 既存 `Menu` / `MenuButton` と新規 `ContextMenu` の責務境界、pointer-anchored 起動の許可範囲を明記する。

## Impact

- `crates/katana-ui-core/src/molecule/selection`（Menu / MenuButton 隣接）に新規モジュールを追加する。
- `crates/katana-ui-core/src/widget/molecules.rs` の re-export を更新する。
- Storybook catalog に `ContextMenu` preset を追加する（[crates/katana-ui-core-storybook/src/catalog/](../../../crates/katana-ui-core-storybook/src/catalog/)）。
- consumer (`katana`, `katana-chat-ui`) は KUC ContextMenu に置き換える前提で migration ガイドが必要になる。
- 既存 `Menu` molecule の Storybook ページは責務縮小に伴い再構成する。
