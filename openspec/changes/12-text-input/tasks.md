# Tasks — 12-text-input

## 1. 実装

- [ ] 1.1 `composite/input/text/types.rs` に `TextInputProps` / `TrailingSlot` を定義
- [ ] 1.2 `composite/input/text/view.rs` を実装。leading icon / trailing slot の配置、focus-ring、invalid 状態
- [ ] 1.3 `composite/input/text/mod.rs` で公開 API を整理

## 2. テスト

- [ ] 2.1 `value` と `on_change` の双方向反映テスト
- [ ] 2.2 `invalid` のスタイル切替テスト
- [ ] 2.3 disabled / readonly の挙動テスト

## 3. Storybook

- [ ] 3.1 `storybook/src/pages/text_input.rs` を追加し `pages/mod.rs` に登録
- [ ] 3.2 ページ内表示
  - [ ] placeholder のみ
  - [ ] leading icon あり / なし
  - [ ] trailing: clear / custom icon / spinner / none
  - [ ] disabled / readonly / invalid
  - [ ] light / dark 追従

## 4. 完了確認

- [ ] 4.1 `cargo check -p katana-ui-widget`
- [ ] 4.2 `just storybook` で text_input ページが想定通り表示
- [ ] 4.3 ast-lint 通過
