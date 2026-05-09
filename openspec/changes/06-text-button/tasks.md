# Tasks — 06-text-button

## 1. 実装

- [ ] 1.1 `composite/button/text/types.rs` に `TextButtonProps` / `Variant` / `Tone` / `Size` を定義
- [ ] 1.2 `composite/button/text/view.rs` に view を実装。state 別スタイルを theme から解決
- [ ] 1.3 `loading` 時のラベル半透明 + 先頭 `Spinner` を実装
- [ ] 1.4 `composite/button/text/mod.rs` で公開 API を整理

## 2. テスト

- [ ] 2.1 各 variant × tone × size の組合せでスタイル解決が破綻しないテスト
- [ ] 2.2 `disabled=true` で `on_click` が呼ばれないテスト
- [ ] 2.3 `loading` でラベルがクリック不能になるテスト

## 3. Storybook

- [ ] 3.1 `storybook/src/pages/text_button.rs` を追加し `pages/mod.rs` に登録
- [ ] 3.2 ページ内表示
  - [ ] 各 variant × tone × size のグリッド
  - [ ] disabled / loading
  - [ ] long label / short label の見え方
  - [ ] light / dark 追従

## 4. 完了確認

- [ ] 4.1 `cargo check -p katana-ui-widget`
- [ ] 4.2 `just storybook` で text_button ページが想定通り表示
- [ ] 4.3 ast-lint 通過
