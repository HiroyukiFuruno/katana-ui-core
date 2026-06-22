# Tasks — 19-add-title-bar-window-chrome

## 1. 設計確定

- [x] 1.1 `WindowControlKind`、`WindowControlsPosition`、`WindowControlVisibility` を確定する。
- [x] 1.2 draggable region と title/header layout を KUC 対象外として確定する。
- [x] 1.3 size token と OS デフォルト表示の違いを確定する。
- [x] 1.4 fullscreen 時の controls visibility 仕様を確定する。

## 2. 中核実装

- [x] 2.1 `molecule/selection/window_control_button_group.rs` を新設する。
- [x] 2.2 option / action / event / state を実装する。
- [x] 2.3 controls の layout と hover visibility を実装する。
- [x] 2.4 `widget::molecules` の re-export に `WindowControlButtonGroup` を追加する。

## 3. window 連携

- [x] 3.1 `runtime / window / surface` に window control intent を通す境界を確認する。
- [x] 3.2 adapter contract に window controls dispatch は adapter、draggable region は consumer / adapter 責務と明記する。
- [x] 3.3 external runtime boundary に window command neutral contract を追加する。

## 4. 自動テスト

- [x] 4.1 controls kind と visibility の表示 props を検証する。
- [x] 4.2 position 3 種類で controls 配置が切替わることを検証する。
- [x] 4.3 draggable_regions が public API に存在しないことを guard で検証する。
- [x] 4.4 controls press で `ControlPressed { which }` 発火、`WindowCommand` 発火が一貫していることを検証する。
- [x] 4.5 fullscreen 中の hover visibility trigger を検証する。
- [x] 4.6 size token の layout snapshot を検証する。

## 5. 自動回帰

- [x] 5.1 controls × position × size の主要 subset を数値化された render / settings contract で回帰する。
- [x] 5.2 macOS / Windows / Linux 風 preset を Storybook contract test で回帰する。
- [x] 5.3 hover / fullscreen hover visibility を state / event contract で回帰する。
- [x] 5.4 light / dark theme を theme token / visual coverage contract で回帰する。

## 6. Storybook ページ

- [x] 6.1 `Selection > WindowControlButtonGroup` ノードを追加する。
- [x] 6.2 preset「macOS 風」「Windows 風」「Linux 風」「fullscreen hover」「Close only」を実装する。
- [x] 6.3 settings で position / size / controls / visibility を切替えできるようにする。

## 7. ドキュメント

- [x] 7.1 `docs/architecture/ui-separation/owned-ui-task-map.md` に WindowControlButtonGroup 行を追加する。
- [x] 7.2 window controls dispatch / draggable region は KUC core contract として保持し、repo 外 runtime 側の責務文書は KUC active tree から除外する。

## 8. 品質ゲート / DoD

- [x] 8.1 `cargo test -p katana-ui-core` をパスする。
- [x] 8.2 `cargo clippy -p katana-ui-core --all-targets -- -D warnings` をパスする。
- [x] 8.3 `openspec validate 19-add-title-bar-window-chrome --strict` をパスする。
- [x] 8.4 core contract gate と自動回帰 gate をパスする。
