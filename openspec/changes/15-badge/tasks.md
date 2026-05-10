# Tasks — 15-badge

## 1. 実装

- [x] 1.1 `composite/indicator/badge/types.rs` に `BadgeProps` / `Tone` / `Variant` / `Size` を定義
- [x] 1.2 `composite/indicator/badge/view.rs` を実装。`Text` primitive と任意の `Icon` primitive を合成
- [x] 1.3 `composite/indicator/badge/mod.rs` で公開 API を整理

## 2. テスト

- [x] 2.1 各 tone × variant の組合せでスタイル解決が破綻しないテスト
- [x] 2.2 leading_icon の有無で width が適切に変わるテスト

## 3. Storybook

- [x] 3.1 `storybook/src/pages/badge.rs` を追加し `pages/mod.rs` に登録
- [x] 3.2 ページ内表示
  - [x] tone × variant × size のグリッド
  - [x] leading_icon あり / なし
  - [x] 数字バッジ / テキストバッジの代表例
  - [x] light / dark 追従

## 4. 完了確認

- [x] 4.1 `cargo check -p katana-ui-widget`
- [x] 4.2 `just storybook` で badge ページが想定通り表示
- [x] 4.3 ast-lint 通過
