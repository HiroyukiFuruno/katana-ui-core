# Tasks — 22-rgba-color-picker

## 1. 実装

- [/] 1.1 `composite/selector/color_picker/types.rs` に `ColorPickerRgbaProps` を定義する
- [/] 1.2 RGBA 各チャンネルを変更できる runtime state を実装する
- [/] 1.3 選択中の色をプレビューへ即時反映する view を実装する
- [/] 1.4 `on_change(Color)` で利用側へ RGBA 値を通知する
- [/] 1.5 disabled / readonly の操作不可状態を実装する
- [/] 1.6 `composite/selector/mod.rs` から公開する

## 2. テスト

- [/] 2.1 R/G/B/A の各チャンネル変更で `Color` が更新されるテスト
- [/] 2.2 disabled / readonly では `on_change` が呼ばれないテスト
- [/] 2.3 alpha 値が preview 表示へ反映されるテスト

## 3. Storybook

- [/] 3.1 `storybook/src/pages/color_picker_rgba.rs` を追加する
- [/] 3.2 live widget で RGBA 値を変更できるようにする
- [/] 3.3 選択中の色プレビューと `rgba(r, g, b, a)` 表示を置く
- [/] 3.4 readonly / disabled / alpha 違いの表示例を置く

## 4. 完了確認

- [x] 4.1 `RUSTFLAGS="-D warnings" cargo test -p katana-ui-widget`
- [x] 4.2 `just storybook-check`
- [x] 4.3 `just ast-lint`

## ユーザーフィードバック

- [/] ColorPicker は katana の `InlineColorPicker` / `LabeledColorPicker` に合わせ、常時展開された数値操作ではなく、色ボタンを押して RGB/RGBA 編集面を開く構成にする。
- [/] 画像1枚目の katana GUI を忠実に再現する。透明チェッカー背景、`U8` 表示、スポイト、R/G/B/A 数値欄、合成方式（Blending）の Normal / Additive、彩度・明度の面、色相スライダー、alpha スライダー、ドラッグハンドル、ポップ表示の角丸パネルを含める。
- [/] Storybook に画像1枚目相当の live widget を置き、色ボタンを開いた状態で各操作が `Color` と preview に反映されることを確認できるようにする。

## 5. 再実装 gate

- [x] 5.1 `proposal.md`、`design.md`、`specs/rgba-color-picker/spec.md` の受け入れ条件に沿って GUI 要素を実装する。
- [/] 5.2 `ColorPickerValue` 相当の型を定義し、Color と blending mode を同時に扱えるようにする。
- [/] 5.3 色面、色相 slider、alpha slider、数値欄のどれを操作しても同じ state と preview に反映されるようにする。
- [/] 5.4 `InlineColorPicker`、`LabeledColorPicker`、`ColorPickerRgba` の責務境界を docs と Storybook で確認できるようにする。
- [/] 5.5 dark mode で panel、text、border、SVG icon、slider handle の配色が theme token に追従することを確認する。
