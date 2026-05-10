# Tasks — 12-text-input

## 1. 実装

- [x] 1.1 `composite/input/text/types.rs` に `TextInputProps` / `TrailingSlot` を定義
- [x] 1.2 `composite/input/text/view.rs` を実装。leading icon / trailing slot の配置、focus-ring、invalid 状態
- [x] 1.3 `composite/input/text/mod.rs` で公開 API を整理

## 2. テスト

- [x] 2.1 `value` と `on_change` の双方向反映テスト
- [x] 2.2 `invalid` のスタイル切替テスト
- [x] 2.3 disabled / readonly の挙動テスト

## 3. Storybook

- [x] 3.1 `storybook/src/pages/text_input.rs` を追加し `pages/mod.rs` に登録
- [x] 3.2 ページ内表示
  - [x] placeholder のみ
  - [x] leading icon あり / なし
  - [x] trailing: clear / custom icon / spinner / none
  - [x] disabled / readonly / invalid
  - [x] light / dark 追従

## 4. 完了確認

- [x] 4.1 `cargo check -p katana-ui-widget`
- [x] 4.2 `just storybook` で text_input ページが想定通り表示
- [x] 4.3 ast-lint 通過
