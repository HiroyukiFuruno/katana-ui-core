# Tasks — 11-color-swatch

## 1. 実装

- [ ] 1.1 `composite/selector/color/types.rs` に `ColorSwatchProps` を定義
- [ ] 1.2 `composite/selector/color/view.rs` を実装。grid レイアウト、選択リング表示
- [ ] 1.3 `composite/selector/color/mod.rs` で公開 API を整理

## 2. テスト

- [ ] 2.1 選択切替のテスト
- [ ] 2.2 disabled 時に操作が反応しないテスト

## 3. Storybook

- [ ] 3.1 `storybook/src/pages/color_swatch.rs` を追加し `pages/mod.rs` に登録
- [ ] 3.2 ページ内表示
  - [ ] 6 色 / 12 色のパレット
  - [ ] サイズトークン違い
  - [ ] disabled
  - [ ] light / dark 追従

## 4. 完了確認

- [ ] 4.1 `cargo check -p katana-ui-widget`
- [ ] 4.2 `just storybook` で color_swatch ページが想定通り表示
- [ ] 4.3 ast-lint 通過
