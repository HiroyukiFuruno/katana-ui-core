# Tasks — 08-toggle

## 1. 実装

- [x] 1.1 `composite/selector/toggle/types.rs` に `ToggleProps` を定義
- [x] 1.2 `composite/selector/toggle/view.rs` に view を実装。state 変化時のアニメーション
- [x] 1.3 `composite/selector/toggle/mod.rs` で公開 API を整理

## 2. テスト

- [x] 2.1 `value` 反転で `on_change` が呼ばれるテスト
- [x] 2.2 `disabled=true` で操作が無効化されるテスト

## 3. Storybook

- [x] 3.1 `storybook/src/pages/toggle.rs` を追加し `pages/mod.rs` に登録
- [x] 3.2 ページ内表示
  - [x] on / off 既定状態
  - [x] disabled
  - [x] サイズトークン違い
  - [x] light / dark 追従
  - [x] ライブセル（Floem signal で実際にトグル動作確認）

## 4. 完了確認

- [x] 4.1 `cargo check -p katana-ui-widget`
- [x] 4.2 `just storybook` で toggle ページが想定通り表示
- [x] 4.3 ast-lint 通過
