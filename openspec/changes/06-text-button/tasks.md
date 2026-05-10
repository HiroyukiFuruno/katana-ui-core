# Tasks — 06-text-button

## 1. 実装

- [x] 1.1 `composite/button/text/types.rs` に `TextButtonProps` / `Variant` / `Tone` / `Size` を定義
- [x] 1.2 `composite/button/text/view.rs` に state 別スタイル解決ヘルパを実装
- [x] 1.3 `loading` 時に `text_alpha=128` でラベル半透明を表現
- [x] 1.4 `composite/button/text/mod.rs` で `TextButton` builder + `ResolvedTextButton` を公開

## 2. テスト

- [x] 2.1 各 variant × tone × size の組合せでスタイル解決が破綻しないテスト
- [x] 2.2 `disabled=true` で bg が None、text_color が text_disabled になるテスト
- [x] 2.3 `loading=true` で text_alpha が 128 になるテスト

## 3. Storybook

- [x] 3.1 `storybook/src/pages/text_button.rs` を追加し `pages/mod.rs` に登録
- [x] 3.2 ページ内表示
  - [x] 各 variant × tone のサンプル
  - [x] disabled / loading の状態
  - [x] light / dark 追従

## 4. 完了確認

- [x] 4.1 `cargo check -p katana-ui-widget`
- [x] 4.2 storybook-check 通過
- [x] 4.3 ast-lint 通過
