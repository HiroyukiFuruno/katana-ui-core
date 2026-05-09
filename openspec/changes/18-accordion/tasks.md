# Tasks — 18-accordion

## 1. 実装

- [ ] 1.1 `layout/accordion/types.rs` に `AccordionProps` / `IndicatorPosition` を定義
- [ ] 1.2 `layout/accordion/view.rs` に展開・折り畳みのアニメーションを実装
- [ ] 1.3 `layout/accordion/mod.rs` で公開 API を整理

## 2. テスト

- [ ] 2.1 expanded toggle で `on_toggle` が呼ばれるテスト
- [ ] 2.2 disabled 時にクリックが反応しないテスト

## 3. Storybook

- [ ] 3.1 `storybook/src/pages/accordion.rs` を追加し `pages/mod.rs` に登録
- [ ] 3.2 ページ内表示
  - [ ] 既定 / 展開済み
  - [ ] indicator 位置違い 3 種
  - [ ] 子要素に `Text` / `TextInput` / `Card` を入れた合成例
  - [ ] disabled
  - [ ] light / dark 追従

## 4. 完了確認

- [ ] 4.1 `cargo check -p katana-ui-widget`
- [ ] 4.2 `just storybook` で accordion ページが想定通り表示
- [ ] 4.3 ast-lint 通過
