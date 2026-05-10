# Tasks — 02-text-primitive

## 1. 実装

- [x] 1.1 `primitive/text/types.rs` に `TextRole` enum と `TextProps` を定義
- [x] 1.2 `primitive/text/view.rs` に resolve helpers を実装（theme から typography を解決）
- [x] 1.3 `primitive/text/mod.rs` で `Text` builder + `ResolvedText` を公開

## 2. テスト

- [x] 2.1 `TextRole` ごとに theme から正しい `TextStyle` が解決されるユニットテスト
- [x] 2.2 `color_override` 指定時に theme 既定色を上書きすることのテスト

## 3. Storybook

- [x] 3.1 `storybook/src/pages/text.rs` を追加し、`pages/mod.rs` に登録
- [x] 3.2 ページ内表示
  - [x] 全 `TextRole` のサンプル一覧
  - [x] `color_override` を使った色違い例
  - [x] `max_lines` で省略される長文例
  - [x] light / dark 切替の追従

## 4. 完了確認

- [x] 4.1 `cargo check -p katana-ui-widget`
- [x] 4.2 storybook-check 通過
- [x] 4.3 ast-lint 通過

## ユーザーフィードバック

- [/] ast-lint の file-length / type-separation が発火した場合、単に `view()` を別ファイルへ逃がして完了扱いにしない。対象 widget の型定義・状態管理・style resolve・event 処理・Storybook ライブセルまで含めて責務境界を再設計し、なぜその分割が妥当かを確認する。
