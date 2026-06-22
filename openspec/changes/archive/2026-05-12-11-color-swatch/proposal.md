## Why

タグ色 / カテゴリ色 / アクセント色などを「事前定義された色パレットから 1 色選ぶ」ケースは多い。フル機能のカラーピッカー (HSV/RGB スライダ) ではなく、**指定パレットからの選択 swatch grid** を最小機能として提供する。`../katana/crates/katana-ui/src/widgets/color_picker/` の役割は adapter の汎用 picker そのものだが、KUW では「事前定義パレット選択」のスコープに絞る（自由色入力は YAGNI）。

## What Changes

- `composite/selector/color/` に `ColorSwatch` widget を提供。
- props: `value: ColorToken`、`palette: Vec<ColorToken>`、`on_change: Fn(ColorToken)`、`size`、`disabled`、`a11y_label`。
- `ColorToken` は `theme/color` で扱う色トークン型を再利用。
- 選択中はリング枠でハイライト。

## Capabilities

### New Capabilities

- `widget-color-swatch`: 事前定義パレットからの単一色選択 widget。

## Impact

- タグ色設定 / カテゴリ設定で利用。
- 自由色入力が必要になった時点で別 change として `ColorPicker` を追加する。
