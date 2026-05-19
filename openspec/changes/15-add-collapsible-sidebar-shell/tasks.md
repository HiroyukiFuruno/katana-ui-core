# Tasks — 15-add-collapsible-sidebar-shell

## 1. 設計確定

- [x] 1.1 `PanelMode` 4 種類と挙動を確定する。
- [x] 1.2 `ResizableWidth` の min / max / default / persist_id を確定する。
- [ ] 1.3 `AppShell` を KUC 公開対象外にし、consumer が shell を組む境界を確定する。

## 2. 中核実装

- [x] 2.1 `molecule/structured/collapsible_panel.rs` に `CollapsiblePanel` を実装する。
- [ ] 2.2 `AppShell` / page / template を public API に追加しない guard を用意する。
- [x] 2.3 option / action / event / state を実装する。
- [x] 2.4 `widget::molecules` の re-export に `CollapsiblePanel` を追加する。

## 3. 連携

- [ ] 3.1 sidebar resize に layout primitive を使う。
- [x] 3.2 consumer が `Grid` / `SplitPane` と組み合わせられる props を公開する。
- [x] 3.3 panel の content slot に `SideMenu` / `TreeView` / `Toolbar` 等を embed 可能とする。

## 4. 自動テスト

- [x] 4.1 mode 4 種類の遷移（Expanded ↔ IconOnly ↔ Collapsed ↔ FloatingOverlay）を検証する。
- [x] 4.2 width drag が min / max に clamp されることを検証する。
- [x] 4.3 pinned=false + expand_on_hover でホバーで一時展開、離脱で元に戻ることを検証する。
- [x] 4.4 FloatingOverlay 時に z-index が上層に来ること、main の width に影響しないことを検証する。
- [x] 4.5 persist_id があるとき、width 変更が consumer callback を経て報告されることを検証する。
- [x] 4.6 consumer shell に main width 算出を委ね、KUC panel state が必要な幅情報を返すことを検証する。
- [x] 4.7 ToggleExpand action と accelerator hook の整合を検証する。

## 5. 画像回帰

- [ ] 5.1 sidebar mode 4 種類を回帰する。
- [ ] 5.2 sidebar 幅の drag 中、drag 終了、default 復帰を回帰する。
- [ ] 5.3 FloatingOverlay 表示時の overlay 描画を回帰する。
- [ ] 5.4 consumer shell preset ではなく、panel 単体と child slot 組合せを回帰する。
- [ ] 5.5 light / dark theme を回帰する。

## 6. Storybook ページ

- [x] 6.1 `Structured > CollapsiblePanel` ノードを追加する。
- [ ] 6.2 `Structured > AppShell` ノードは追加しない。
- [ ] 6.3 preset「Explorer panel」「Chat history panel」「TOC panel」「Floating overlay」「IconOnly」を実装する。
- [ ] 6.4 settings で mode / width / pinned / expand_on_hover / resize_handle を切替えできるようにする。

## 7. ドキュメント

- [ ] 7.1 `docs/architecture/ui-separation/owned-ui-task-map.md` に CollapsiblePanel 行を追加する。
- [ ] 7.2 README に panel molecule と consumer shell の責務境界を追記する。

## 8. 品質ゲート / DoD

- [x] 8.1 `cargo test -p katana-ui-core` をパスする。
- [x] 8.2 `cargo clippy -p katana-ui-core --all-targets -- -D warnings` をパスする。
- [x] 8.3 `openspec validate 15-add-collapsible-sidebar-shell --strict` をパスする。
- [ ] 8.4 画像 / 入力回帰 CI gate をパスする。
