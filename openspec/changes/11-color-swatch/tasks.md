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
