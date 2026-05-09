# Tasks — 08-toggle

## 1. 実装

- [ ] 1.1 `composite/selector/toggle/types.rs` に `ToggleProps` を定義
- [ ] 1.2 `composite/selector/toggle/view.rs` に view を実装。state 変化時のアニメーション
- [ ] 1.3 `composite/selector/toggle/mod.rs` で公開 API を整理

## 2. テスト

- [ ] 2.1 `value` 反転で `on_change` が呼ばれるテスト
- [ ] 2.2 `disabled=true` で操作が無効化されるテスト

## 3. Storybook

- [ ] 3.1 `storybook/src/pages/toggle.rs` を追加し `pages/mod.rs` に登録
- [ ] 3.2 ページ内表示
  - [ ] on / off 既定状態
  - [ ] disabled
  - [ ] サイズトークン違い
  - [ ] light / dark 追従
  - [ ] ライブセル（Floem signal で実際にトグル動作確認）

## 4. 完了確認

- [ ] 4.1 `cargo check -p katana-ui-widget`
- [ ] 4.2 `just storybook` で toggle ページが想定通り表示
- [ ] 4.3 ast-lint 通過
