# Tasks — 17-card

## 1. 実装

- [x] 1.1 `layout/card/types.rs` に `CardProps` / `Variant` を定義
- [x] 1.2 `layout/card/view.rs` を実装。variant ごとの境界線 / 影 / 背景を theme から解決
- [x] 1.3 `layout/card/mod.rs` で公開 API を整理

## 2. テスト

- [x] 2.1 各 variant のスタイル解決が破綻しないテスト
- [x] 2.2 `interactive=true` の hover/active 状態テスト

## 3. Storybook

- [x] 3.1 `storybook/src/pages/card.rs` を追加し `pages/mod.rs` に登録
- [x] 3.2 ページ内表示
  - [x] 各 variant
  - [x] padding バリエーション
  - [x] interactive あり / なし
  - [x] 子要素として `Text` / `Badge` / `TextButton` を入れた合成サンプル
  - [x] light / dark 追従

## 4. 完了確認

- [x] 4.1 `cargo check -p katana-ui-widget`
- [x] 4.2 `just storybook` で card ページが想定通り表示
- [x] 4.3 ast-lint 通過

## ユーザーフィードバック

- [/] ast-lint の file-length / type-separation が発火した場合、単に `view()` を別ファイルへ逃がして完了扱いにしない。対象 widget の型定義・状態管理・style resolve・event 処理・Storybook ライブセルまで含めて責務境界を再設計し、なぜその分割が妥当かを確認する。
- [ ] 現状は未実装扱いで再確認する。Card は単なる枠ではなく、複雑な UI 自身を子要素として配置できる構成にする。
- [ ] header / body / footer / actions / 任意 content slot を持ち、TextInput、Button、Badge、Accordion などの複合 widget を内部に置けることを Storybook で確認する。
- [ ] interactive Card の click / hover / focus は、内部の button や input の操作を壊さない責務分離にする。
