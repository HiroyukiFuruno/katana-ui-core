# Tasks — 14-tooltip

## 1. 実装

- [x] 1.1 `composite/indicator/tooltip/types.rs` に `TooltipProps` / `Placement` を定義
- [x] 1.2 `composite/indicator/tooltip/view.rs` を実装。hover/focus 検知、delay、画面端反転
- [x] 1.3 `composite/indicator/tooltip/mod.rs` で公開 API を整理（ラッパ関数として export）

## 2. テスト

- [x] 2.1 hover delay の経過後に表示されるテスト
- [x] 2.2 focus でも表示されるテスト
- [x] 2.3 画面端で placement が反転するテスト

## 3. Storybook

- [x] 3.1 `storybook/src/pages/tooltip.rs` を追加し `pages/mod.rs` に登録
- [x] 3.2 ページ内表示
  - [x] 4 placement の例
  - [x] 短い文字列 / 長い文字列（max_width で改行）
  - [x] アイコンボタンに添えた使用例
  - [x] light / dark 追従

## 4. 完了確認

- [x] 4.1 `cargo check -p katana-ui-widget`
- [x] 4.2 `just storybook` で tooltip ページが想定通り表示
- [x] 4.3 ast-lint 通過
