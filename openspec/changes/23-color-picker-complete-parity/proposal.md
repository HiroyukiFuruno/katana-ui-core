# ColorPicker 完全版への作り直し

> Archive candidate: ColorPicker の KUC 実装要件は `openspec/changes/establish-kuc-atoms-molecules-catalog/` へ移管する。この change は要件移管後に archive 候補として扱う。

## 目的

`ColorPickerRgba` を、色見本ではなく RGB / RGBA を実際に編集できる色選択 UI として成立させる。

## 背景

現状の実装は、画面上では色選択パネルに見えるが、操作導線、前面表示、閉じる挙動、表示位置、色面の見た目に不備がある。
KatanA 側では `egui` の色選択 UI により実用できる挙動が存在するため、Floem 側でも同等の体験を提供する。

## 範囲

- RGB / RGBA の値を編集できる ColorPicker
- 開閉可能な前面パネル
- 外側クリック、Esc、閉じるボタンによる閉じる挙動
- パネルの重なり順と表示位置の改善
- 色面、色相、透明度、合成方式の操作
- Storybook で実操作できるサンプル

## 範囲外

- OS 標準カラーピッカーとの連携
- スポイト機能の実 OS 取得
- `egui` コンポーネントの直接埋め込み
