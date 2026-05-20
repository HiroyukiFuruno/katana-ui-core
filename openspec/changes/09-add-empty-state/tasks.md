# Tasks — 09-add-empty-state

## 1. 設計確定

- [x] 1.1 EmptyState の option（icon / illustration / heading / body / actions / tone / size / alignment）を確定する。
- [x] 1.2 action 子要素（`Button` atom）の typed 持ち方を確定する。

## 2. 中核実装

- [x] 2.1 `molecule/empty_state/` を新設する。
- [x] 2.2 option / action / event / state を実装する。
- [x] 2.3 `widget::molecules` の re-export に `EmptyState` を追加する。

## 3. 連携

- [x] 3.1 DiagnosticsList / SelectionList / TreeView / CommandPalette / SearchBox の empty 表示に embed 可能であることを契約に明記する。

## 4. 自動テスト

- [x] 4.1 primary / secondary action 押下で `EmptyStateEvent::Actioned { id, action_id }` が発火することを検証する。
- [x] 4.2 heading 必須、icon と illustration の排他、body の optional を検証する。
- [x] 4.3 tone × size × alignment の組合せが layout snapshot で安定であることを検証する。
- [x] 4.4 accessibility live region announce の payload を検証する。

## 5. 数値化された描画 / 入力契約

- [x] 5.1 tone 5 種 × size 3 種 × alignment 2 種の主要 subset を layout snapshot / theme token contract で検証する。
- [x] 5.2 icon-only / illustration-only / actions あり / actions なし を render tree contract で検証する。
- [x] 5.3 light / dark theme は tone token contract と Storybook theme gate で検証する。

## 6. Storybook ページ

- [x] 6.1 `Surface > EmptyState` ノードを追加する。
- [x] 6.2 preset「explorer empty」「search no result」「diagnostics clean」「history empty」「error fallback」を実装する。
- [x] 6.3 settings で tone / size / alignment / actions を切替えできるようにする。

## 7. ドキュメント

- [x] 7.1 `docs/architecture/ui-separation/owned-ui-task-map.md` に EmptyState 行を追加する。

## 8. 品質ゲート / DoD

- [x] 8.1 `cargo test -p katana-ui-core` をパスする。
- [x] 8.2 `cargo clippy -p katana-ui-core --all-targets -- -D warnings` をパスする。
- [x] 8.3 `openspec validate 09-add-empty-state --strict` をパスする。
- [x] 8.4 数値化された描画契約 / 入力回帰の CI gate をパスする。
