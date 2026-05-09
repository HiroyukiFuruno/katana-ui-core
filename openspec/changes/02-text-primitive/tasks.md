# Tasks — 02-text-primitive

## 1. 実装

- [ ] 1.1 `primitive/text/types.rs` に `TextRole` enum と `TextProps` を定義
- [ ] 1.2 `primitive/text/view.rs` に `Text` view 関数を実装（theme から typography を解決）
- [ ] 1.3 `primitive/text/mod.rs` で公開 API を整理

## 2. テスト

- [ ] 2.1 `TextRole` ごとに theme から正しい `TextStyle` が解決されるユニットテスト
- [ ] 2.2 `color_override` 指定時に theme 既定色を上書きすることのテスト

## 3. Storybook

- [ ] 3.1 `storybook/src/pages/text.rs` を追加し、`pages/mod.rs` に登録
- [ ] 3.2 ページ内表示
  - [ ] 全 `TextRole` のサンプル一覧
  - [ ] `color_override` を使った色違い例
  - [ ] `max_lines` で省略される長文例
  - [ ] light / dark 切替の追従

## 4. 完了確認

- [ ] 4.1 `cargo check -p katana-ui-widget`
- [ ] 4.2 `just storybook` で text ページが想定通り表示
- [ ] 4.3 ast-lint 通過
