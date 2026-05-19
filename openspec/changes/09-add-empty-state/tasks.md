# Tasks — 09-add-empty-state

## 1. 設計確定

- [ ] 1.1 EmptyState の option（icon / illustration / heading / body / actions / tone / size / alignment）を確定する。
- [ ] 1.2 action 子要素（`Button` atom）の typed 持ち方を確定する。

## 2. 中核実装

- [ ] 2.1 `molecule/empty_state.rs` を新設する。
- [ ] 2.2 option / action / event / state を実装する。
- [ ] 2.3 `widget::molecules` の re-export に `EmptyState` を追加する。

## 3. 連携

- [ ] 3.1 DiagnosticsList / SelectionList / TreeView / CommandPalette / SearchBox の empty 表示に embed 可能であることを契約に明記する。

## 4. 自動テスト

- [ ] 4.1 primary / secondary action 押下で `EmptyStateActioned { id }` が発火することを検証する。
- [ ] 4.2 heading 必須、icon と illustration の排他、body の optional を検証する。
- [ ] 4.3 tone × size × alignment の組合せが layout snapshot で安定であることを検証する。
- [ ] 4.4 accessibility live region announce の payload を検証する。

## 5. 画像回帰

- [ ] 5.1 tone 5 種 × size 3 種 × alignment 2 種の主要 subset を回帰する。
- [ ] 5.2 icon-only / illustration-only / actions あり / actions なし を回帰する。
- [ ] 5.3 light / dark theme を回帰する。

## 6. Storybook ページ

- [ ] 6.1 `Surface > EmptyState` ノードを追加する。
- [ ] 6.2 preset「explorer empty」「search no result」「diagnostics clean」「history empty」「error fallback」を実装する。
- [ ] 6.3 settings で tone / size / alignment / actions を切替えできるようにする。

## 7. ドキュメント

- [ ] 7.1 `docs/architecture/ui-separation/owned-ui-task-map.md` に EmptyState 行を追加する。

## 8. 品質ゲート / DoD

- [ ] 8.1 `cargo test -p katana-ui-core` をパスする。
- [ ] 8.2 `cargo clippy -p katana-ui-core --all-targets -- -D warnings` をパスする。
- [ ] 8.3 `openspec validate 09-add-empty-state --strict` をパスする。
- [ ] 8.4 画像回帰の CI gate をパスする。
