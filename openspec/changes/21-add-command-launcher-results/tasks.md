# Tasks — 21-add-command-launcher-results

## 1. 設計確定

- [x] 1.1 `CommandResultRow` の typed option を確定する。
- [x] 1.2 `CommandPalette` 既存 API と `CommandResultRow` の互換 path を決める。
- [x] 1.3 keyboard contract と disabled row の扱いを確定する。
- [x] 1.4 modal palette / inline results / slash launcher の責務境界を `design.md` に固定する。

## 2. 中核実装

- [x] 2.1 `molecule/structured` に `CommandResultRow` を追加する。
- [x] 2.2 `CommandPalette` に query / highlighted_index / result row / virtualization option を追加する。
- [x] 2.3 action として `SetQuery` / `MoveHighlight` / `SelectHighlighted` / `Execute` / `Close` を追加する。
- [x] 2.4 event として `QueryChanged` / `ResultHighlighted` / `ResultExecuted` / `Closed` を追加する。
- [x] 2.5 `ShortcutCombo` を result row の右側表示として受け取れるようにする。
- [x] 2.6 `VirtualizationConfig` を大量結果に適用できるようにする。

## 3. 自動テスト

- [x] 3.1 query 変更で `QueryChanged` が発火し、highlight が先頭候補へ移ることを検証する。
- [x] 3.2 Arrow Up / Down / Home / End の移動を検証する。
- [x] 3.3 disabled row の execute が無視され、disabled reason が props に残ることを検証する。
- [x] 3.4 Enter で `ResultExecuted { id }` が発火することを検証する。
- [x] 3.5 virtualization 中も highlighted row が表示範囲に保持されることを検証する。

## 4. 自動回帰

- [x] 4.1 command palette / inline search results / slash launcher の主要 preset を Storybook contract で回帰する。
- [x] 4.2 disabled row / provider group / shortcut badge / secondary label の表示を render contract で回帰する。
- [x] 4.3 virtualized results の先頭 / 中間 / 末尾 highlight を virtual range / settings contract で回帰する。
- [x] 4.4 light / dark theme を theme token / visual coverage contract で回帰する。

## 5. Storybook ページ

- [x] 5.1 preset「command palette」「search results」「slash launcher」「disabled rows」「virtualized results」を追加する。
- [x] 5.2 settings で query、highlight、row count、provider group、shortcut 表示を切替えできるようにする。
- [x] 5.3 state に query、highlighted row、virtual range、disabled reason を表示する。
- [x] 5.4 action / event log に query 変更、highlight、execute、close を表示する。
- [x] 5.5 quality に keyboard contract、virtualized highlight、disabled execution guard の検証結果を表示する。

## 6. ドキュメント

- [x] 6.1 `SearchControlStrip` が必要な検索 option / replace controls は `22-add-search-control-strip` に分離することを明記する。
- [x] 6.2 `CommandPalette` は modal / app command registry / domain action を持たないことを docs に明記する。

## 7. 品質ゲート / DoD

- [x] 7.1 `openspec validate 21-add-command-launcher-results --strict` をパスする。
- [x] 7.2 `cargo test -p katana-ui-core` をパスする。
- [x] 7.3 `cargo clippy -p katana-ui-core -p katana-ui-core-storybook --all-targets -- -D warnings` をパスする。
- [x] 7.4 自動回帰 / 入力回帰 / 静的検査の CI gate をパスする。
