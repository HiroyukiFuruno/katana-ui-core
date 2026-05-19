## Why

KUC は `Tabs` molecule（segmented / preset 切替えとしての水平 tabs）を持つが、`katana` の document / workspace tab が要求する「閉じるボタン付き / dirty 表示 / ドラッグ並べ替え / グループ表示 / drop indicator / overflow nav / 右クリック context menu / ghost preview」を満たさない。`katana-chat-ui` も session や履歴候補を tab 状に切り替える需要がある。

ただし KUC が `Workspace` や `Document` の意味を持ってはならない。
この change は、画面全体の tab bar（organism）ではなく、domain-free な closeable tab strip molecule を定義する。
`Tabs` を拡張すると segmented 用途と closeable tab 用途の option / state が混ざるため、別 molecule として分ける。

## What Changes

- `widget::molecules` に `CloseableTabStrip` molecule を追加する。
- `CloseableTab` は `id`, `title`, `icon`, `dirty`, `pinned`, `closeable`, `tone`, `tooltip`, `group_id`, `accessibility_label` を typed option で持つ。
- TabGroup（複数 tab の集合）を `TabGroup`（id, label, color, collapsed）として持つ。
- ドラッグ並べ替えは `02-add-drag-drop-primitive` の DragSource / DropTarget を使い、KUC 内で完結する。tab 間 / グループ間 / グループ内移動を区別する。
- overflow（表示しきれない tab）は overflow menu（右端ボタン）に格納し、`ContextMenu`（`01-add-context-menu`）または `MenuButton` を流用する。
- 右クリックで `ContextMenu` を開けるようにする。ただし menu item の意味は consumer が渡す。
- Ghost preview（ドラッグ中の半透明 tab 描画）と drop indicator（before / after / inside-group / new-group）を持つ。
- キーボード操作（Ctrl/Cmd+Tab, Ctrl/Cmd+Shift+Tab, close request, 1〜9 で n 番目を選択）を event として契約に含める。実際の document close は consumer が判断する。

## Capabilities

### New Capabilities

- `kuc-closeable-tab-strip`: CloseableTabStrip molecule の option / action / event / state / preset / preview / settings / 自動テスト / 画像回帰 / Storybook ページの完了条件を定義する。

### Modified Capabilities

- `kuc-widget-layer`: `Tabs`（segmented）と `CloseableTabStrip`（closeable / draggable / grouped）の責務境界を明記し、workspace / document / session の domain 語を public API に入れないことを保証する。

## Impact

- `crates/katana-ui-core/src/molecule/selection/closeable_tab_strip/` を追加する。
- 既存 `Tabs` molecule の Storybook ページは「segmented」と明示し、closeable tab 用途を切り出す。
- `02-add-drag-drop-primitive` と `01-add-context-menu` に依存する（dependency 順は本 change が後）。
- consumer (`katana`) は workspace / document tab を、`katana-chat-ui` は session tab 相当を、この molecule と自前 domain state の組み合わせで実装する。
