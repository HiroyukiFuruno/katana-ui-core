## Why

KUC は `Tabs` molecule（segmented / preset 切替えとしての水平 tabs）を持つが、`katana` の workspace tab bar が要求する「閉じるボタン付き / ドラッグ並べ替え / グループ表示 / drop indicator / overflow nav / 右クリック context menu / ghost preview」を満たさない。`katana-chat-ui` の session history パネルも複数 session を tab 様 UI で並べる需要があるが、現状 KUC には対応 widget がない。

`Tabs` を拡張すると segmented 用途と workspace 用途の option / state が混ざり、画像 / 入力回帰の対象が膨張する。両用途は別 molecule として分けるのが妥当である。

## What Changes

- `widget::molecules` に `WorkspaceTabBar` molecule を追加する。
- `WorkspaceTab` は `id`, `title`, `icon`, `dirty`, `pinned`, `closeable`, `tone`, `tooltip`, `group_id`, `accessibility_label` を typed option で持つ。
- TabGroup（複数 tab の集合）を `WorkspaceTabGroup`（id, label, color, collapsed）として持つ。
- ドラッグ並べ替えは `02-add-drag-drop-primitive` の DragSource / DropTarget を使い、KUC 内で完結する。tab 間 / グループ間 / グループ内移動を区別する。
- overflow（表示しきれない tab）は overflow menu（右端ボタン）に格納し、`ContextMenu`（`01-add-context-menu`）または `MenuButton` を流用する。
- 右クリックで `ContextMenu` を開く（close / close others / close right / pin / unpin / move to new group）。
- Ghost preview（ドラッグ中の半透明 tab 描画）と drop indicator（before / after / inside-group / new-group）を持つ。
- キーボード操作（Ctrl/Cmd+Tab, Ctrl/Cmd+Shift+Tab, Cmd+W close, Cmd+1〜9 で n 番目を選択）を契約に含める。

## Capabilities

### New Capabilities

- `kuc-workspace-tab-bar`: WorkspaceTabBar molecule の option / action / event / state / preset / preview / settings / 自動テスト / 画像回帰 / Storybook ページの完了条件を定義する。

### Modified Capabilities

- `kuc-widget-layer`: `Tabs`（segmented）と `WorkspaceTabBar`（workspace）の責務境界を明記し、両者を別 molecule として公開することを保証する。

## Impact

- `crates/katana-ui-core/src/molecule/structured/` または `selection/` に `workspace_tab_bar/` を追加する。
- 既存 `Tabs` molecule の Storybook ページは「segmented」と明示し、workspace 用途を切り出す。
- `02-add-drag-drop-primitive` と `01-add-context-menu` に依存する（dependency 順は本 change が後）。
- consumer (`katana`) は workspace_tab_bar.rs を KUC 版に置き換える前提で migration ガイドが必要になる。
