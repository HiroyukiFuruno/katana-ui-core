# Tasks — 18-accordion

## 1. 実装

- [x] 1.1 `layout/accordion/types.rs` に `AccordionProps` / `IndicatorPosition` を定義
- [x] 1.2 `layout/accordion/view.rs` に展開・折り畳みのアニメーションを実装
- [x] 1.3 `layout/accordion/mod.rs` で公開 API を整理

## 2. テスト

- [x] 2.1 expanded toggle で `on_toggle` が呼ばれるテスト
- [x] 2.2 disabled 時にクリックが反応しないテスト

## 3. Storybook

- [x] 3.1 `storybook/src/pages/accordion.rs` を追加し `pages/mod.rs` に登録
- [x] 3.2 ページ内表示
  - [x] 既定 / 展開済み
  - [x] indicator 位置違い 3 種
  - [x] 子要素に `Text` / `TextInput` / `Card` を入れた合成例
  - [x] disabled
  - [x] light / dark 追従

## 4. 完了確認

- [x] 4.1 `cargo check -p katana-ui-widget`
- [x] 4.2 `just storybook` で accordion ページが想定通り表示
- [x] 4.3 ast-lint 通過
