# Tasks — 13-search-box

## 1. 実装

- [x] 1.1 `composite/input/search/types.rs` に `SearchBoxProps` を定義
- [x] 1.2 `composite/input/search/view.rs` を実装。`TextInput` を内部利用、Esc / Enter のキー処理
- [x] 1.3 `composite/input/search/mod.rs` で公開 API を整理

## 2. テスト

- [x] 2.1 Esc で value が空文字に戻ることのテスト
- [x] 2.2 Enter で `on_submit` が呼ばれることのテスト
- [x] 2.3 value 非空時のみ clear ボタンが trailing に出ることのテスト

## 3. Storybook

- [x] 3.1 `storybook/src/pages/search_box.rs` を追加し `pages/mod.rs` に登録
- [x] 3.2 ページ内表示
  - [x] 空 / 入力済み
  - [x] disabled
  - [x] サイズトークン違い
  - [x] Esc / Enter のライブ確認セル
  - [x] light / dark 追従

## 4. 完了確認

- [x] 4.1 `cargo check -p katana-ui-widget`
- [x] 4.2 `just storybook` で search_box ページが想定通り表示
- [x] 4.3 ast-lint 通過（`composite/input/search` から `composite/input/text` のみを参照）
