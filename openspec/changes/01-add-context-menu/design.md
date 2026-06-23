# Design — ContextMenu molecule

## 目的

pointer 座標 / 仮想 anchor / 既存ノード anchor から開ける文脈メニュー（context menu）を `widget::molecules::ContextMenu` として確立する。既存 `Menu` molecule（anchor 起点のトリガー＋パネル）を残し、両者の責務を分離する。

## 採用方針

### 1. anchor 戦略は 3 種類を typed enum で持つ

```text
ContextMenuAnchor =
  | Pointer { x: f32, y: f32 }
  | VirtualRect { origin: Point, size: Size }
  | NodeId(UiNodeId)
```

Pointer 起動を一級市民として持つ。これは編集器の右クリックや tab bar の右クリックで必要な挙動である。`NodeId` 起動は `MenuButton` 系の anchored 起動と互換にする。

### 2. 項目モデル

- `ContextMenuItem` enum: `Action`, `Toggle`, `Radio`, `Submenu`, `Section`, `Divider`
- `Action`: label / leading icon / shortcut key cap / disabled / destructive / accessibility label
- `Toggle` / `Radio`: checked / radio group id
- `Submenu`: 子 `Vec<ContextMenuItem>` と open delay
- `Section`: header label とその下に並ぶ items
- `Divider`: tone（neutral / emphasis）

`Menu` molecule の `ChoiceItem` と互換シリアライズできる subset を提供する（migration コストを下げる）。

### 3. 配置とエッジフリップ

- 基本配置は `placement: Placement = AnchorBelowStart`
- 画面領域からはみ出る場合は priority list（`BelowStart`, `BelowEnd`, `AboveStart`, `AboveEnd`, `RightStart`, `LeftStart`）で順次フリップ
- submenu は `RightStart` を基本とし、はみ出れば `LeftStart` にフリップ
- 最小幅 = anchor 幅 or 設定値、最大高 = 画面残り高 - 余白、超過時は内部スクロール
- pointer 起動時は anchor サイズ 0 として扱い、ポインタ座標を起点に配置する

### 4. キーボードナビゲーション契約

- Open: 既定で先頭の enabled item を highlight
- ↑↓: enabled item を循環移動（disabled / divider / section header はスキップ）
- → on Submenu: submenu を開き、最初の enabled item を highlight
- ← on Submenu: 親に戻り、submenu を閉じる
- Enter / Space: 確定（Toggle は state を反転）
- Esc: 閉じてフォーカスを起動元に戻す
- Home / End: 先頭 / 末尾の enabled item
- Type-ahead: 1秒以内の同一プレフィックス入力で label 先頭マッチへジャンプ

### 5. 状態 / イベント

- 親 state: `open`, `anchor`, `placement_used`, `highlighted_path: Vec<usize>`, `pending_submenu`, `callback_log`
- 子 submenu の `UiStateId` は親と別に持つ
- core event:
  - `ContextMenuOpened { anchor, placement_used }`
  - `ContextMenuClosed { reason: Escape | OutsideClick | Selected | FocusReturn }`
  - `ContextMenuItemHighlighted { path }`
  - `ContextMenuItemSelected { path, command }`
  - `ContextMenuSubmenuOpened { path }`
  - `ContextMenuSubmenuClosed { path }`

### 6. focus return

閉じた時、開く前のフォーカス holder にフォーカスを戻す。`open` を programmatic に切り替えた場合は呼び出し側がリターン先を指定できる API を持つ。

### 7. `Menu` molecule との責務境界

- `Menu`: anchor element 内で開閉する標準メニュー。`MenuButton` / `SelectBox` のパネル側で使う。
- `ContextMenu`: pointer / 仮想 rect / 既存 node のいずれかから開ける右クリック相当のメニュー。selection の確定はホスト側コマンドに委ねる。
- 共通 model（`ChoiceItem` / `MenuCommand`）は subset を共有し、互換変換できる layer を持つ。

## 代替案と却下理由

| 代替 | 却下理由 |
| --- | --- |
| 既存 `Menu` に `pointer_anchor` option を増設する | option が肥大化し、 anchor がノードに紐付かない場合の focus return / Esc / placement 戦略が他オプションと衝突する。 |
| `Popover` molecule の上に context menu logic を被せる | `Popover` は generic な panel で、項目モデル / キーボードナビ / submenu 契約を持たず、契約境界が曖昧になる。 |
| consumer 側に丸投げ（KUC は anchor 起点 menu だけを提供） | KatanA / chat-ui の各画面で reimpl が発生し、入力 / 画像回帰の対象から漏れ、入力品質ゲートを満たせない。 |

## Out of scope

- macOS / Windows / Linux のネイティブメニュー連携。これは `adapter_contract` の escape hatch（platform menu）に委ねる。
- アクセラレーターキーの OS グローバル登録。consumer 側責務。
- ドラッグでの項目選択（press-drag-release pattern）は v2 以降に延期。

## 影響範囲

- `crates/katana-ui-core/src/molecule/selection/` 配下に `context_menu.rs` を新設する。
- 既存 `Menu` molecule の Storybook ページから「右クリック起動」用 preset を ContextMenu 側に移管する。
- `katana-ui-core-storybook` の TreeView 末尾に `ContextMenu` ノードを追加する。
- migration ガイドを `docs/architecture/ui-separation/owned-ui-task-map.md` の追加 UI 表に反映する。
