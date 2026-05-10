## Why

ColorPicker は katana の設定画面で使われる重要な入力部品だが、現状の `ColorPickerRgba` は画像1枚目の GUI と比べて操作要素が不足している。
単なる RGBA 数値操作では katana から移植できないため、画像1枚目を受け入れ基準として再定義する。

## What Changes

- ColorPicker を画像1枚目のようなポップ型 GUI として作り直す。
- 透明チェッカー、色 preview、`U8` 表示、スポイト、R/G/B/A 値、合成方式、色面、色相 slider、alpha slider、ドラッグハンドルを実装対象にする。
- RGB mode と RGBA mode を分け、RGBA mode では alpha と blending を扱う。
- `InlineColorPicker`、`LabeledColorPicker`、`ColorPickerRgba` の関係を整理し、katana の用途に合わせた API を提供する。
- Storybook では画像1枚目相当の操作を live widget として確認できるようにする。

## Capabilities

### New Capabilities

- `rgba-color-picker`: katana の色選択 GUI を再現する ColorPicker。

### Modified Capabilities

- なし。

## Impact

- `crates/katana-ui-widget/src/composite/selector/color_picker` の API と view を再設計する。
- Storybook の ColorPicker ページを、画像1枚目相当の live sample に置き換える。
- ColorSwatch とは別の入力部品として責務を分離する。
