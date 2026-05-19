# Tasks — 22-add-search-control-strip

## 1. 設計確定

- [x] 1.1 `SearchControlStrip` と `SearchBox` の責務境界を確定する。
- [x] 1.2 `SearchOptions`（match case / whole word / regex）を typed にする。
- [x] 1.3 replace mode の Hidden / Visible / Disabled を確定する。
- [x] 1.4 result count / active index の表示契約を確定する。

## 2. 中核実装

- [x] 2.1 `SearchControlStrip` molecule を追加する。
- [x] 2.2 `SearchControlStripAction` に search query / option / navigation / replace 系 action を追加する。
- [x] 2.3 `SearchControlStripEvent` に query changed / option changed / navigation requested / replace requested を追加する。
- [x] 2.4 option toggle icon button には tooltip と accessibility label を必須にする。
- [x] 2.5 `CommandPalette` / `CommandResultRow` と組み合わせても state id が衝突しないようにする。

## 3. 自動テスト

- [x] 3.1 query 変更で `SearchQueryChanged` が発火することを検証する。
- [x] 3.2 option toggle で typed option と event が更新されることを検証する。
- [x] 3.3 previous / next が `SearchNavigationRequested` を発火することを検証する。
- [x] 3.4 replace mode が Hidden のとき replace action が発火しないことを検証する。
- [x] 3.5 result count / active index の表示が 0 件、1 件、複数件で安定することを検証する。

## 4. 画像回帰

- [ ] 4.1 compact / expanded / disabled / invalid regex の主要 preset を回帰する。
- [ ] 4.2 match case / whole word / regex toggle の on / off を回帰する。
- [ ] 4.3 result count 0 / 1 / many、active index あり / なしを回帰する。
- [ ] 4.4 light / dark theme を回帰する。

## 5. Storybook ページ

- [x] 5.1 `Molecules > SearchControlStrip` を追加する。
- [x] 5.2 preset「workspace search」「editor find」「editor replace」「viewer search」「history search」を追加する。
- [ ] 5.3 settings で query、option、replace mode、result count、active index を切替えできるようにする。
- [ ] 5.4 state に query、options、replace value、result summary を表示する。
- [ ] 5.5 action / event log に query、option、navigation、replace を表示する。
- [ ] 5.6 quality に option label、state id、result count、event contract の検証結果を表示する。

## 6. ドキュメント

- [x] 6.1 `SearchBox` は simple query input、`SearchControlStrip` は検索操作 row として docs に明記する。
- [ ] 6.2 `docs/inventory/katana-katana-chat-ui-kdv-kle-ui-needs.md` の SearchControlStrip 行と同期する。

## 7. 品質ゲート / DoD

- [x] 7.1 `openspec validate 22-add-search-control-strip --strict` をパスする。
- [x] 7.2 `cargo test -p katana-ui-core` をパスする。
- [x] 7.3 `cargo clippy -p katana-ui-core -p katana-ui-core-storybook --all-targets -- -D warnings` をパスする。
- [ ] 7.4 画像回帰 / 入力回帰 / 静的検査の CI gate をパスする。
