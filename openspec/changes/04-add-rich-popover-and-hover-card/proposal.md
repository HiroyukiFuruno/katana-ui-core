## Why

`katana` editor の `diagnostics_hover` / `diagnostics_popup` / `code_block_menu` / explorer の `breadcrumb` ホバー、`katana-chat-ui` の adapter capability ホバー説明など、リッチな内容を anchor に紐付けて表示するホバー UI 需要は広範に存在する。既存 `Popover` molecule は open / close / outside click / Esc / placement 制御を持つが、次が不足している:

- ホバー（pointer enter / focus）でのホバーカード起動（open delay / close delay / pointer follow）
- リッチコンテンツ用の typed slot（heading / body / footer / actions）
- 矢印（arrow / tail）描画 model
- pointer がポップオーバー内に入った時の延長動作
- focus 移動先がポップオーバー内の interactive 要素の場合の focus management

`Tooltip` は text 限定で、リッチコンテンツや action を持てない。`Popover` を拡張すると open trigger と内容モデルの責務が肥大化するため、`HoverCard` を独立 molecule として追加し、`Popover` には arrow / focus management の追加 option を入れる。

## What Changes

- `widget::molecules` に `HoverCard` molecule を追加する（hover / focus trigger、open / close delay、pointer follow、arrow、rich content slot）。
- 既存 `Popover` molecule に追加 option を入れる:
  - `arrow`（表示有無、サイズ、tone）
  - `slots`（heading / body / footer / actions の typed slot）
  - `focus_management`（none / first-interactive / specified node）
  - `keep_open_on_inner_focus`
  - `auto_flip_priority`（list 化）
- `HoverCard` / `Popover` 共通の placement engine（priority list、anchor follow、edge flip、viewport clamp）を `interaction/placement.rs` として共有化する。
- `Popover` / `HoverCard` / `Tooltip` / `ContextMenu` で共通利用できる anchor / placement / arrow API を整備する。

## Capabilities

### New Capabilities

- `kuc-hover-card`: HoverCard molecule の option / action / event / state / preset / preview / settings / 自動テスト / 数値化された描画契約 / Storybook ページの完了条件を定義する。
- `kuc-placement-engine`: anchor + placement priority + edge flip + arrow alignment の共通 API を定義する。

### Modified Capabilities

- `kuc-widget-layer`: `Popover` の追加 option（arrow / slots / focus_management）と、`HoverCard` の責務分離を明記する。

## Impact

- `crates/katana-ui-core/src/molecule/disclosure/` に `hover_card.rs` を追加し、`Popover` の option を拡張する。
- `crates/katana-ui-core/src/interaction/placement.rs` を共有 module として新設する。
- 既存 `Tooltip` / `Popover` / `ContextMenu` / `Menu` / `MenuButton` / `SelectBox` / `ComboBox` のパネル配置を共通 placement engine に寄せる。
- Storybook に HoverCard 用 preset、Popover の arrow / slots / focus management の追加 preset を入れる。
