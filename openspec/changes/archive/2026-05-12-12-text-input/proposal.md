## Why

単行テキスト入力（フォームフィールド / 設定値編集 / 検索の前提）は最頻出。Adapter の生 input view を直に使うと placeholder / clear / disabled / readonly / error 表示が呼び出し側ごとにバラつくため、統一 widget として固定する。

## What Changes

- `composite/input/text/` に `TextInput` widget を提供。
- props: `value: String`、`on_change: Fn(String)`、`placeholder: Option<String>`、`leading_icon: Option<IconSource>`、`trailing: TrailingSlot`、`size`、`disabled`、`readonly`、`invalid: bool`、`a11y_label`。
- `TrailingSlot` は `None` / `ClearButton` / `Custom(Icon)` / `Spinner`（loading 表示）の enum。
- `invalid=true` で枠と focus-ring が danger 色に切り替わる（メッセージ表示は本 widget の責務外、消費側で `Text` を添える）。

## Capabilities

### New Capabilities

- `widget-text-input`: 単行テキスト入力。leading icon / trailing slot / 状態（disabled/readonly/invalid/loading）を統一。

## Impact

- 13 (search-box) は `TextInput` の trailing=ClearButton + leading=search icon の合成で実装される（13 側で `TextInput` を直接使う）。
- フォーム widget が今後増える場合は同じ `size` / `invalid` 規約で拡張する。
