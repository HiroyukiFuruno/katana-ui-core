# Design — WorkspaceTabBar molecule

## 目的

ドキュメント / セッション / view を表す closeable / draggable / groupable な水平 tab strip を KUC molecule として提供する。`Tabs`（segmented control）とは別の molecule にする。

## 採用方針

### 1. データモデル

```text
WorkspaceTabBar {
  groups: Vec<WorkspaceTabGroup>,
  tabs: Vec<WorkspaceTab>,
  active_tab_id: Option<TabId>,
  overflow_menu_visible: bool,
}

WorkspaceTab {
  id: TabId,
  title: String,
  icon: Option<SvgIcon>,
  dirty: bool,
  pinned: bool,
  closeable: bool,
  tone: TabTone,            // Default / Accent / Warning / Danger / Muted
  tooltip: Option<String>,
  group_id: Option<GroupId>,
  accessibility_label: Option<String>,
}

WorkspaceTabGroup {
  id: GroupId,
  label: String,
  color: ColorToken,
  collapsed: bool,
}
```

- `pinned` tabs は左端固定、close button を隠す
- `dirty` は modified indicator（小さな dot）と close button hover での confirm
- `group_id = None` の tab は「グループ未所属」として表示

### 2. drag & drop

- DragSource: tab 単体（`tag = "katana-ui-core/workspace-tab"`, payload に `TabId`）
- DropTarget: tab 間（before / after）、グループ内（inside）、グループ間、新規グループ作成（far right）
- pinned tab は移動の anchor として使えるが、unpinned tab は pinned 領域に挿入できない（accept で reject）
- グループ collapsed 時はドロップ可能だが、ホバーで自動展開（500ms delay）

### 3. overflow

- 表示領域に収まらない tab は右端の overflow ボタンに集約
- 「現在表示中」「隠れている」を visually 区別
- overflow ボタンを押すと `MenuButton`（または `ContextMenu`）が開き、隠れている tab の list を表示
- list 内では「アイコン + タイトル + dirty indicator + close」が並ぶ

### 4. context menu

`ContextMenu` を使って次を提供:

- `Close`
- `Close Others`
- `Close to the Right`
- `Close All`
- `Pin` / `Unpin`
- `Move to New Group`
- `Move to Group >` (submenu)
- `Rename Group`（group header の右クリック）
- `Collapse Group` / `Expand Group`

### 5. キーボード

- `Cmd/Ctrl + Tab`: 次の tab に進む（pinned 含む）
- `Cmd/Ctrl + Shift + Tab`: 前の tab
- `Cmd/Ctrl + W`: active tab を close（dirty なら confirm dispatch）
- `Cmd/Ctrl + 1`〜`9`: n 番目の visible tab を active
- `Cmd/Ctrl + 0`: 最後の tab
- `Esc`: ドラッグ中なら cancel

### 6. dirty / confirm

- close 時、`dirty = true` の tab は `CloseRequested` event を発火（実 close はホスト判断）
- `CloseConfirmed` action で実際に削除
- 半確認 UI（modal / popover）は KUC に持たない（`ModalOverlay` を使う前提）

### 7. ghost preview

- ドラッグ中、`DragPreview` molecule（`02-add-drag-drop-primitive`）を使って半透明 tab を描画
- preview には title + icon + dirty dot を含む
- drag preview 自身の z-index は overflow / context menu より下

### 8. accessibility

- `WorkspaceTabBar` 自体は role=tablist 相当
- 各 tab は role=tab 相当、active は aria-selected=true 相当
- group header は role=group 相当、collapsed は aria-expanded で表現
- announce: 「Tab 3 of 7, Article.md, modified」のような形式（locale テンプレート）

## 代替案と却下理由

| 代替 | 却下理由 |
| --- | --- |
| `Tabs` molecule に `closeable` / `draggable` を増設 | segmented preset 切替え用途と option が混在し、画像 / 入力回帰の preset 数が膨張、契約が散らかる。 |
| `SelectionList` + 横並びレイアウトで代用 | drag / drop / overflow / group / pin / context menu の組み合わせが contracted されず、consumer 毎に差が出る。 |
| consumer 側に丸投げ | KatanA tab bar の `tab_bar/{drag,group_header,group_header_popup,tab_context_menu,tab_ghost,drop_indicator}` がそのまま consumer 内に残り、KUC の品質ゲート対象外になる。 |

## Out of scope

- 縦並び tab bar（vertical strip）：別 change に分離
- ドラッグでの切り離し（detach to new window）：adapter 経由 escape hatch
- tab pinning の永続化 ID 管理：consumer 責務

## 影響範囲

- `02-add-drag-drop-primitive` と `01-add-context-menu` に依存
- 既存 `Tabs` molecule は「segmented」用途に絞り、Storybook page タイトルに明記
- consumer (`katana`) の `workspace_tab_bar.rs` を新 molecule で置き換える migration
