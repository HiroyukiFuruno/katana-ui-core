# ComboBox Widget Spec

## 概要

テキスト入力 + ドロップダウン選択を組み合わせた入力 widget。SelectBox (10) の上位互換。

## 出典

- `../katana/crates/katana-ui/src/widgets/combo_box/`

## 階層配置

`composite/input/combo`

## 依存

- TextInput (12)
- Popover (21)

## API 概要（TBD）

- `ComboBoxItem`: label, value
- `ComboBox`: items, selected, on_select, on_input_change, strict_mode, placeholder
