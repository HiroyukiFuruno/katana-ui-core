## Why

`katana` の command palette と検索 modal、`katana-chat-ui` の slash launcher、chat history 検索は、どれも「入力欄 + 結果一覧 + 選択中 row + shortcut 表示 + 実行 action」を持つ。

KUC には `CommandPalette` molecule があるが、現状は単純な command item 集合に近く、次を option だけでは補えない。

- 結果 row の icon、主 label、補助 label、右寄せ shortcut badge
- provider ごとの source / group / disabled reason
- query 変更、highlight 移動、Enter 実行、Esc close の typed action / event
- slash command のような「`/` 起点で開く小型 launcher」と、中央 modal palette の同一 contract
- 大量結果に対する virtualization と keyboard selection の保持

## What Changes

- `CommandPalette` を domain-free な command launcher molecule として拡張する。
- `SearchControlStrip` が必要な検索 option / replace controls は `22-add-search-control-strip` に分離し、本 change は result row と keyboard selection に集中する。
- `CommandResultRow` を typed item として追加する。
  - `id`
  - `label`
  - `secondary_label`
  - `icon`
  - `shortcut`
  - `provider_id`
  - `group_id`
  - `disabled`
  - `disabled_reason`
- action を定義する。
  - `SetQuery`
  - `MoveHighlight`
  - `SelectHighlighted`
  - `Execute`
  - `Close`
- event を定義する。
  - `QueryChanged`
  - `ResultHighlighted`
  - `ResultExecuted`
  - `Closed`
- `13-add-shortcut-combo-display` の `ShortcutCombo` と、`16-add-virtualized-list-and-tree` の `VirtualizationConfig` を利用できるようにする。

## Capabilities

### New Capabilities

- `kuc-command-launcher-results`: command launcher / search result list の option、action、event、state、keyboard contract を定義する。

### Modified Capabilities

- `kuc-widget-layer`: `CommandPalette` は domain command を実行しない。実行対象は event として consumer へ返す。

## Impact

- `crates/katana-ui-core/src/molecule/structured/command_palette` または既存 `structured` module を拡張する。
- `CommandItem` を破壊せず、`CommandResultRow` への移行 path を用意する。
- Storybook に preview / settings / state / event / action / quality を持つ command launcher page を追加する。
- consumer (`katana` command palette、search modal、`katana-chat-ui` slash launcher / history search) は KUC の launcher molecule を組み合わせて使える。
- chat session、workspace command registry、KLE / KDV の実 command provider は KUC に入れない。
