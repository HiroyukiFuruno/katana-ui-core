# Tasks — 01-add-context-menu

## 1. 設計確定

- [ ] 1.1 `ContextMenuAnchor` enum（Pointer / VirtualRect / NodeId）を設計し、`Menu` molecule との互換 subset を確定する。
- [ ] 1.2 `ContextMenuItem` enum（Action / Toggle / Radio / Submenu / Section / Divider）の typed option を確定する。
- [ ] 1.3 placement priority list とエッジフリップアルゴリズムを `design.md` に基づき確定する。
- [ ] 1.4 キーボードナビゲーション契約（↑↓ Enter Esc Home End Type-ahead）を確定する。

## 2. 中核実装

- [ ] 2.1 `crates/katana-ui-core/src/molecule/selection/context_menu/mod.rs` を新設し、`ContextMenu` 型を作る。
- [ ] 2.2 `options.rs` で typed option（anchor / items / placement / min_width / max_height / submenu_open_delay）を実装する。
- [ ] 2.3 `actions.rs` で `Open` / `Close` / `Highlight` / `Activate` / `OpenSubmenu` / `CloseSubmenu` / `TypeAhead` を実装する。
- [ ] 2.4 `events.rs` で `ContextMenuOpened` / `ContextMenuClosed` / `ContextMenuItemHighlighted` / `ContextMenuItemSelected` / `ContextMenuSubmenuOpened` / `ContextMenuSubmenuClosed` を実装する。
- [ ] 2.5 `state.rs` で 親 state（open / anchor / placement_used / highlighted_path / pending_submenu / callback_log）と submenu 子 state を `UiStateId` 分離で実装する。
- [ ] 2.6 `placement.rs` でエッジフリップを純関数として実装する（viewport size と anchor から `Placement` を決定）。
- [ ] 2.7 `keyboard.rs` でキーボードナビゲーション state 遷移を純関数として実装する（disabled / divider / section をスキップ）。

## 3. 公開境界

- [ ] 3.1 `widget::molecules` の re-export に `ContextMenu` / `ContextMenuAnchor` / `ContextMenuItem` を追加する。
- [ ] 3.2 `Menu` molecule の責務縮小説明をドキュメント文字列で明示する。
- [ ] 3.3 `ChoiceItem` ↔ `ContextMenuItem::Action` の互換変換 helper を提供する。

## 4. 自動テスト

- [ ] 4.1 anchor 3 種類で `Open` action が成立し、placement_used が viewport に収まることを単体テストで検証する。
- [ ] 4.2 エッジフリップが priority list 順で発火することを `placement.rs` 純関数で検証する。
- [ ] 4.3 キーボードナビゲーションが disabled / divider / section をスキップすることを純関数で検証する。
- [ ] 4.4 submenu open / close が親 state を壊さないこと、子 `UiStateId` が一意であることを検証する。
- [ ] 4.5 Esc / 外側クリック / 選択での close reason が `ContextMenuClosed.reason` に乗ることを検証する。
- [ ] 4.6 focus return が起動元 / 呼び出し側指定の双方で機能することを検証する。
- [ ] 4.7 Type-ahead プレフィックスマッチ（複数文字 / タイムアウト）を検証する。

## 5. 画像回帰

- [ ] 5.1 anchor=Pointer・anchor=Node・anchor=VirtualRect の非空描画を回帰する。
- [ ] 5.2 submenu 展開後、placement flip 発火、最大高超過時の内部スクロールを回帰する。
- [ ] 5.3 light / dark theme での divider / section / destructive / disabled / checked / radio の見た目を回帰する。

## 6. Storybook ページ

- [ ] 6.1 `crates/katana-ui-core-storybook/src/catalog/molecules/` に `context_menu.rs` を新設する。
- [ ] 6.2 preset として「編集器右クリック」「explorer 空領域」「tab bar」「message 行」「leading icon + shortcut」を実装する。
- [ ] 6.3 settings inspector で anchor / placement / 各 item kind の切替えと callback log 表示を実装する。
- [ ] 6.4 catalog TreeView に `Selection > ContextMenu` ノードを追加する。

## 7. ドキュメント / 互換

- [ ] 7.1 `docs/architecture/ui-separation/owned-ui-task-map.md` の追加 UI 表に ContextMenu 行を追加する。
- [ ] 7.2 `docs/widget-extraction-policy.md` に anchor 起動と pointer 起動の責務境界を追記する。
- [ ] 7.3 `openspec/changes/README.md` の優先順位表に本 change を載せる。

## 8. 品質ゲート / DoD

- [ ] 8.1 `cargo test -p katana-ui-core` をローカルでパスする。
- [ ] 8.2 `cargo clippy -p katana-ui-core -p katana-ui-core-storybook --all-targets -- -D warnings` をパスする。
- [ ] 8.3 `openspec validate 01-add-context-menu --strict` をパスする。
- [ ] 8.4 画像回帰 / 入力回帰 / 静的検査の CI gate をパスする。
