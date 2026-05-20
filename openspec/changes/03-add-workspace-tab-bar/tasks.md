# Tasks — 03-add-workspace-tab-bar

## 1. 設計確定

- [x] 1.1 `CloseableTab` / `TabGroup` の typed option を確定する。
- [x] 1.2 overflow menu のしきい値判定アルゴリズム（measured width + remaining width）を確定する。
- [x] 1.3 context menu 項目（close / close others / close right / pin / move to group / new group）を確定する。
- [x] 1.4 ドラッグ accept ルール（pinned vs unpinned、グループ collapsed auto-expand）を確定する。

## 2. 中核実装

- [x] 2.1 `molecule/structured/workspace_tab_bar/` に domain-free alias として `CloseableTabStrip` 型を作る。
- [x] 2.2 `options.rs` で typed option を実装する。
- [x] 2.3 `actions.rs` で `SelectTab` / `CloseTab` / `PinTab` / `UnpinTab` / `MoveTab` / `MoveToGroup` / `ToggleGroupCollapse` / `OpenOverflow` / `ConfirmClose` を実装する。
- [x] 2.4 `events.rs` で `TabSelected` / `TabCloseRequested` / `TabClosed` / `TabReordered` / `GroupCollapseChanged` / `OverflowOpened` を実装する。
- [x] 2.5 `state.rs` で active_tab_id / overflow_visible / drag_in_progress / pending_close_confirm を持つ親 state、子 tab の `UiStateId` 分離を実装する。
- [x] 2.6 `overflow.rs` でしきい値判定を純関数として実装する。
- [x] 2.7 `keyboard.rs` で `Cmd/Ctrl+Tab`、`Cmd/Ctrl+W`、`Cmd/Ctrl+1..9` の操作を実装する。

## 3. 依存連携

- [x] 3.1 `02-add-drag-drop-primitive` の DragSource / DropTarget / DropIndicator / DragPreview を使う。
- [x] 3.2 `01-add-context-menu` の `ContextMenu` を tab 右クリック / group header 右クリックで使う。

## 4. 公開境界

- [x] 4.1 `widget::molecules` の re-export に `CloseableTabStrip` / `CloseableTab` / `TabGroup` を追加する。
- [x] 4.2 `Tabs` molecule と `CloseableTabStrip` の用途差を docs へ明記する。

## 5. 自動テスト

- [x] 5.1 overflow しきい値判定が measured width に基づき正しく hidden tab を計算することを純関数で検証する。
- [x] 5.2 pinned tab が左端固定であり、unpinned が pinned 領域へ移動できないことを検証する。
- [x] 5.3 dirty tab close で `TabCloseRequested` → `ConfirmClose` action → `TabClosed` の順を検証する。
- [x] 5.4 ドラッグ並べ替えで `TabReordered { from, to }` が正しく発火することを検証する。
- [x] 5.5 group 内 / 間 / 新規グループ作成のドロップが正しく state に反映することを検証する。
- [x] 5.6 キーボード `Cmd/Ctrl+1..9` で n 番目の visible tab が active になることを検証する。
- [x] 5.7 子 tab の `UiStateId` がユニークであり、親 state と独立であることを検証する。

## 6. 数値化された描画契約

- [x] 6.1 default / overflow / pinned 混在 / dirty 混在の表示を render model 契約で回帰する。
- [x] 6.2 ドラッグ中の ghost + drop indicator（before / after / inside-group / new-group）を DragPreview / DropTarget / event 契約で回帰する。
- [x] 6.3 group collapsed / expanded、group color tone を state / render model 契約で回帰する。
- [x] 6.4 light / dark theme での tab tone（Default / Accent / Warning / Danger / Muted）を render model 契約で回帰する。

## 7. Storybook ページ

- [x] 7.1 `Selection > CloseableTabStrip` ノードを catalog に追加する。
- [x] 7.2 preset「default」「overflow」「pinned」「groups」「dirty」「dragging」を実装する。
- [x] 7.3 settings で tab 追加 / 削除 / pin / dirty 切替え / group 切替えを行えるようにする。
- [x] 7.4 event log と callback log を表示する。

## 8. ドキュメント

- [x] 8.1 `docs/architecture/ui-separation/owned-ui-task-map.md` に CloseableTabStrip 行を追加する。
- [x] 8.2 `docs/widget-extraction-policy.md` に Tabs / CloseableTabStrip の責務境界を追記する。

## 9. 品質ゲート / DoD

- [x] 9.1 `cargo test -p katana-ui-core` をパスする。
- [x] 9.2 `cargo clippy -p katana-ui-core --all-targets -- -D warnings` をパスする。
- [x] 9.3 `openspec validate 03-add-workspace-tab-bar --strict` をパスする。
- [x] 9.4 数値化された描画契約 / 入力回帰の CI gate をパスする。
