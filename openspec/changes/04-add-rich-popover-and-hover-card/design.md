# Design — HoverCard + Popover enhancements

## 目的

リッチコンテンツのホバー / フォーカス表示を `HoverCard` molecule として独立させ、`Popover` には arrow / slots / focus management 等の追加 option を入れる。両者が共有する placement engine を抽出して `Tooltip` / `ContextMenu` / `Menu` / `MenuButton` / `SelectBox` / `ComboBox` も同じ engine で動かす。

## 採用方針

### 1. HoverCard

- trigger: `pointer-enter`、`focus`、`programmatic`
- open delay と close delay を別々に持つ（default: 500ms / 200ms）
- pointer がカード本体に入ったら close delay を一時停止
- 内容: `heading`, `body`, `footer`, `actions` の 4 slot
- arrow（吹き出しの矢印）: 表示有無、サイズ、tone
- focus 内部要素（actions ボタン等）に届いた場合は keep_open（強制継続表示）
- accessibility: `aria-describedby` 相当を anchor に bind

### 2. Popover 追加 option

- `arrow: ArrowSpec`（kind, size, tone）
- `slots: PopoverSlots { heading, body, footer, actions }`
- `focus_management: FocusManagement = None | FirstInteractive | NodeId(UiNodeId)`
- `keep_open_on_inner_focus: bool`
- `auto_flip_priority: Vec<Placement>`

既存の `open` / `anchor` / `placement` / `offset` / `width` / `outside_click` / `Esc` は維持。

### 3. 共通 placement engine

```text
PlacementRequest {
  anchor: AnchorKind,                 // Node, VirtualRect, Pointer
  preferred: Placement,
  priority: Vec<Placement>,           // ordered fallbacks
  offset: f32,
  panel_size: Size,
  viewport: Rect,
  clamp_margin: f32,
}

PlacementResult {
  placement_used: Placement,
  position: Point,
  arrow_offset: Option<f32>,          // when arrow is enabled
  clamped: bool,
}
```

純関数として実装し、`Tooltip` / `Popover` / `HoverCard` / `ContextMenu` / `Menu` / `MenuButton` / `SelectBox` / `ComboBox` で共有する。

### 4. arrow alignment

- arrow は panel の anchor 側に表示し、anchor 中央に揃える
- 矢印が panel 端を超える場合は panel 端から `clamp_margin` 離した位置に置く
- placement flip 時は arrow も反対側に再配置

### 5. pointer follow（HoverCard 専用）

- option として `pointer_follow: bool`（default false）
- true の場合、anchor は pointer 座標になり、pointer 移動に追随する
- close trigger は「pointer が anchor から離れた閾値距離以上、close delay 経過後」

### 6. focus management

- `FirstInteractive`: open 時に内部の最初の interactive 要素にフォーカスを移す
- `NodeId(id)`: 指定 node にフォーカス
- `None`: 何もしない（既定）
- close 時は open 前のフォーカス holder に必ずリターン

## 代替案と却下理由

| 代替 | 却下理由 |
| --- | --- |
| `Tooltip` を拡張して HoverCard を兼ねる | `Tooltip` は text 限定の単純表示の責務にとどめたい。actions / slots / arrow を入れると契約が肥大化する。 |
| `Popover` に hover trigger option を追加 | open trigger の責務が pointer enter / pointer-click / programmatic / focus / right-click と複雑化し、event routing と close reason が散らかる。 |
| 各 disclosure molecule が独自に placement を持つ | 6 つの molecule で同じ edge flip / arrow alignment ロジックが重複し、修正コストとバグが増える。 |

## Out of scope

- 複数 popover の global manager（stacking, z-index 管理）：v2 以降
- 自動コンテンツ measure（測定→再配置）：consumer が measured size を渡す前提を維持
- アニメーション（fade / scale）：`add-animation-primitives-18` で扱う

## 影響範囲

- `crates/katana-ui-core/src/interaction/placement.rs` 新設
- `crates/katana-ui-core/src/molecule/disclosure/` に `hover_card.rs` を追加
- 既存 `Popover` / `Tooltip` / `ContextMenu` / `Menu` / `MenuButton` / `SelectBox` / `ComboBox` を共通 placement engine に切替え
- Storybook に HoverCard preset 追加、Popover の slots / arrow / focus management preset 追加
