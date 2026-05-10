# Tasks — 11-color-swatch

## 1. 実装

- [x] 1.1 `composite/selector/color/types.rs` に `ColorSwatchProps` を定義
- [x] 1.2 `composite/selector/color/view.rs` を実装。grid レイアウト、選択リング表示
- [x] 1.3 `composite/selector/color/mod.rs` で公開 API を整理

## 2. テスト

- [x] 2.1 選択切替のテスト
- [x] 2.2 disabled 時に操作が反応しないテスト

## 3. Storybook

- [x] 3.1 `storybook/src/pages/color_swatch.rs` を追加し `pages/mod.rs` に登録
- [x] 3.2 ページ内表示
  - [x] 6 色 / 12 色のパレット
  - [x] サイズトークン違い
  - [x] disabled
  - [x] light / dark 追従

## 4. 完了確認

- [x] 4.1 `cargo check -p katana-ui-widget`
- [x] 4.2 `just storybook` で color_swatch ページが想定通り表示
- [x] 4.3 ast-lint 通過

## ユーザーフィードバック

- [/] ast-lint の file-length / type-separation が発火した場合、単に `view()` を別ファイルへ逃がして完了扱いにしない。対象 widget の型定義・状態管理・style resolve・event 処理・Storybook ライブセルまで含めて責務境界を再設計し、なぜその分割が妥当かを確認する。
- [/] Storybook の ColorSwatch ページから resolved 値を四角で再現した疑似サンプルを除去し、実際の `ColorSwatch::view` で live / readonly / size を確認できる構成にする。
- [/] Live widget の選択色を preview と `rgba(r, g, b, a)` 表示へ反映し、選択操作の意味が画面上で分かるようにする。
- [/] ColorSwatch とは別に、RGBA 各チャンネルを編集できる ColorPickerRgba を追加する（詳細は `openspec/changes/22-rgba-color-picker/tasks.md`）。
- [/] katana で使えるように、swatch cell を角丸四角と完全な円で切り替えられる `SwatchShape` を追加する。
