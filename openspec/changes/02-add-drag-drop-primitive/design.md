# Design — drag & drop primitive

## 目的

drag & drop（DnD）を framework-neutral な event / interaction / atom / molecule の集合として KUC に持たせ、explorer・tab bar・attachment などの consumer が再実装せずに使える状態にする。

## 採用方針

### 1. drag data は typed + tag のハイブリッド

```text
DragData {
  tag: &'static str,         // 例: "katana/file-list", "katana-ui-core/tab-id"
  payload: serde_json::Value, // 構造体は consumer 側で deserialize
  metadata: HashMap<String, String>, // 表示用 (icon, label, count)
}
```

- core は payload の中身を解釈しない。`tag` で互換性をチェックし、drop target の `accept` callback が判定する。
- OS native の text/url/file 等は adapter が `DragData` に変換するか、escape hatch を通す。

### 2. drop target の判定モデル

- `DropTarget` は次を持つ:
  - `accept(data: &DragData, position: Point) -> DropAcceptance`
  - `on_enter` / `on_over` / `on_leave` / `on_drop`
  - `auto_scroll: AutoScrollPolicy`（edge zone size、加速度、無効化）
- `DropAcceptance`:
  - `Reject`
  - `Accept { effect: DropEffect, indicator: DropIndicator }`
- 1 つの target 上で「項目間に挿入」「項目内に投入」を区別するため、position に応じて `indicator` を変える（before / after / inside / none）

### 3. drop indicator atom

- `DropIndicator` は描画 model だけを持ち、actual paint は adapter 側
- option: kind（Line / Outline / Glow）、orientation（Horizontal / Vertical）、tone、anchor rect
- atom の `state` に「現在表示中か」「どの target にバインドされているか」を保持

### 4. keyboard drag

- target がキーボードドラッグ可能（`keyboard_draggable: true`）のとき:
  - Space または Enter で pick up（DragStart 発火 + announce）
  - 矢印キーで focus 移動（focus 先が DropTarget なら DragEnter / DragOver 発火）
  - Space または Enter で Drop
  - Esc で DragCancel
- screen reader announcement は `accessibility` module の API を経由（adapter が読み上げる）

### 5. autoscroll

- DropTarget が scrollable ancestor を持つ場合、edge zone 内に pointer / keyboard focus が留まると autoscroll を開始
- edge zone size と最大速度は `AutoScrollPolicy` でカスタム可能、デフォルトは theme token から取得
- core は target → scroll request だけ生成し、実 scroll は adapter（あるいは consumer の `ScrollArea` model）に委ねる

### 6. Esc / pointer cancel

- DragStart 発火後、Esc 押下、または window 失活、または adapter からの cancel 通知で `DragCancel` → `DragEnd { committed: false }` を順に発火
- drop indicator は immediately clear
- focus は drag source へ復帰

### 7. native OS DnD

- core API は OS payload 形式（HTML5 DataTransfer、NSDraggingInfo）を持たない
- adapter は OS DnD イベントを `DragData` + tag に変換する責務を持つ（共通 tag は `os/file-list`、`os/url`、`os/text`）
- consumer は tag を見て interpret する

## 代替案と却下理由

| 代替 | 却下理由 |
| --- | --- |
| すべての DnD を consumer 側に丸投げ | explorer / tab bar / attachment で挙動差が積み上がる。入力回帰 / 画像回帰の対象から漏れる。 |
| Drag を `PointerEvent` の長押し + delta で表現する | pointer / drag の責務が混在し、drop target / indicator / autoscroll を後付けで足す必要があり API が散らかる。 |
| HTML5 DataTransfer 風 API をそのまま core に持つ | environment-specific 型が漏れ、framework-neutrality を破る。 |

## Out of scope

- Touch jesture（pinch / rotate）：別 change に分離
- Multi-item drag（複数選択を 1 つの drag に束ねる）：consumer 側で `DragData.payload` に複数 ID を入れる前提とし、core は同じ DragData 1 つしか扱わない
- Drag-and-edit（drag を続けながら inline rename 等）

## 影響範囲

- `event` / `interaction` / `atom` / `molecule` モジュールを跨ぐため、event bubbling / capture policy も更新する。
- external runtime boundary に neutral contract を追加する。
- Storybook playground の追加（reorder / file drop / tab reorder / attachment drop）。
