# Tasks — 04-spinner-primitive

## 1. 実装

- [ ] 1.1 `primitive/spinner/types.rs` に `SpinnerProps` を定義
- [ ] 1.2 `primitive/spinner/view.rs` に Floem animation を使った回転描画を実装
- [ ] 1.3 `primitive/spinner/mod.rs` で公開 API を整理
- [ ] 1.4 prefers-reduced-motion 相当（システム設定）を受け取れるなら回転を停止する分岐を入れる

## 2. テスト

- [ ] 2.1 props の既定値テスト
- [ ] 2.2 アニメーションを止めるフラグでフレーム更新が呼ばれないことのテスト（モック注入で確認）

## 3. Storybook

- [ ] 3.1 `storybook/src/pages/spinner.rs` を追加し `pages/mod.rs` に登録
- [ ] 3.2 ページ内表示
  - [ ] 各サイズトークンの spinner を並べる
  - [ ] 各 color トークンでの色違い
  - [ ] reduced-motion 切替時の挙動デモ
  - [ ] light / dark 追従

## 4. 完了確認

- [ ] 4.1 `cargo check -p katana-ui-widget`
- [ ] 4.2 `just storybook` で spinner ページが想定通り表示
- [ ] 4.3 ast-lint 通過
