# Tasks — 15-badge

## 1. 実装

- [ ] 1.1 `composite/indicator/badge/types.rs` に `BadgeProps` / `Tone` / `Variant` / `Size` を定義
- [ ] 1.2 `composite/indicator/badge/view.rs` を実装。`Text` primitive と任意の `Icon` primitive を合成
- [ ] 1.3 `composite/indicator/badge/mod.rs` で公開 API を整理

## 2. テスト

- [ ] 2.1 各 tone × variant の組合せでスタイル解決が破綻しないテスト
- [ ] 2.2 leading_icon の有無で width が適切に変わるテスト

## 3. Storybook

- [ ] 3.1 `storybook/src/pages/badge.rs` を追加し `pages/mod.rs` に登録
- [ ] 3.2 ページ内表示
  - [ ] tone × variant × size のグリッド
  - [ ] leading_icon あり / なし
  - [ ] 数字バッジ / テキストバッジの代表例
  - [ ] light / dark 追従

## 4. 完了確認

- [ ] 4.1 `cargo check -p katana-ui-widget`
- [ ] 4.2 `just storybook` で badge ページが想定通り表示
- [ ] 4.3 ast-lint 通過
