# Tasks — 13-search-box

## 1. 実装

- [ ] 1.1 `composite/input/search/types.rs` に `SearchBoxProps` を定義
- [ ] 1.2 `composite/input/search/view.rs` を実装。`TextInput` を内部利用、Esc / Enter のキー処理
- [ ] 1.3 `composite/input/search/mod.rs` で公開 API を整理

## 2. テスト

- [ ] 2.1 Esc で value が空文字に戻ることのテスト
- [ ] 2.2 Enter で `on_submit` が呼ばれることのテスト
- [ ] 2.3 value 非空時のみ clear ボタンが trailing に出ることのテスト

## 3. Storybook

- [ ] 3.1 `storybook/src/pages/search_box.rs` を追加し `pages/mod.rs` に登録
- [ ] 3.2 ページ内表示
  - [ ] 空 / 入力済み
  - [ ] disabled
  - [ ] サイズトークン違い
  - [ ] Esc / Enter のライブ確認セル
  - [ ] light / dark 追従

## 4. 完了確認

- [ ] 4.1 `cargo check -p katana-ui-widget`
- [ ] 4.2 `just storybook` で search_box ページが想定通り表示
- [ ] 4.3 ast-lint 通過（`composite/input/search` から `composite/input/text` のみを参照）
