# Tasks — 08-add-diagnostics-list

## 1. 設計確定

- [ ] 1.1 `DiagnosticsItem` / `DiagnosticAction` / `DiagnosticsGroup` の typed model を確定する。
- [ ] 1.2 `group_by` enum と sort 規則を確定する。
- [ ] 1.3 fix preview の embed 方法（`CodeDiffSnapshot`）を確定する。
- [ ] 1.4 bulk action の dry run preview / 結果 event を確定する。

## 2. 中核実装

- [ ] 2.1 `molecule/structured/diagnostics_list/mod.rs` を新設し型を作る。
- [ ] 2.2 `options.rs` / `actions.rs` / `events.rs` / `state.rs` を分離して実装する。
- [ ] 2.3 group / sort / filter を純関数として実装する。
- [ ] 2.4 fix preview の expand / collapse、子 `UiStateId` 分離を実装する。
- [ ] 2.5 bulk fix preview を `ModalOverlay` 連携で実装する。
- [ ] 2.6 キーボード操作（↑↓ ← → Enter Space F8 Shift+F8）を実装する。

## 3. 連携

- [ ] 3.1 fix preview に既存 `CodeDiff` molecule を使う。
- [ ] 3.2 severity filter に `Chip`（`07-add-chip-and-attachment-chip`）を使う。
- [ ] 3.3 empty state を `empty_slot: Option<UiTree>` として受け取れるようにする。
- [ ] 3.4 loading state を `loading_slot: Option<UiTree>` として受け取れるようにする。

## 4. 自動テスト

- [ ] 4.1 group_by 切替えで items の再 group 化が正しく行われることを純関数で検証する。
- [ ] 4.2 severity filter 切替えで visible_count が正しく変動することを検証する。
- [ ] 4.3 fix preview expand / collapse、apply fix の event 発火を検証する。
- [ ] 4.4 bulk fix preview → apply → BulkFixApplied{applied, skipped} を検証する。
- [ ] 4.5 キーボードナビゲーション（↑↓ ← → Enter Space）と accelerator (F8 / Shift+F8) を検証する。
- [ ] 4.6 子 `UiStateId` 独立性を検証する。
- [ ] 4.7 sort_by 切替えで stable な順序が得られることを検証する。

## 5. 画像回帰

- [ ] 5.1 group_by 4 種類 × severity 混在 × expanded / collapsed の表示を回帰する。
- [ ] 5.2 fix preview expand（CodeDiff embed）を回帰する。
- [ ] 5.3 severity filter chip row、bulk action bar を回帰する。
- [ ] 5.4 empty / loading 状態を回帰する。
- [ ] 5.5 light / dark theme での severity color を回帰する。

## 6. Storybook ページ

- [ ] 6.1 `Structured > DiagnosticsList` ノードを追加する。
- [ ] 6.2 preset「lint result」「editor inline」「tool result」「empty」「loading」「bulk fix」を実装する。
- [ ] 6.3 settings で group_by / sort_by / severity_filter / bulk_action / fix_preview の切替えを実装する。

## 7. ドキュメント

- [ ] 7.1 `docs/architecture/ui-separation/owned-ui-task-map.md` に DiagnosticsList 行を追加する。

## 8. 品質ゲート / DoD

- [ ] 8.1 `cargo test -p katana-ui-core` をパスする。
- [ ] 8.2 `cargo clippy -p katana-ui-core --all-targets -- -D warnings` をパスする。
- [ ] 8.3 `openspec validate 08-add-diagnostics-list --strict` をパスする。
- [ ] 8.4 画像 / 入力回帰の CI gate をパスする。
