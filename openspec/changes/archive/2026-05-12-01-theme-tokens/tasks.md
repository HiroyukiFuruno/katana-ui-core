# Tasks — 01-theme-tokens

## 1. 実装

- [x] 1.1 `theme/color/palette.rs` に生 RGB 定数を定義（外部に `pub` しない）
- [x] 1.2 `theme/color/mod.rs` に意味的トークン構造体 `ColorTokens` を定義
- [x] 1.3 `theme/spacing/mod.rs` に `SpacingTokens` を定義（`xxs`〜`xxl`）
- [x] 1.4 `theme/typography/mod.rs` に `TypographyTokens` と役割別 `TextStyle` を定義
- [x] 1.5 `theme/mod.rs` に `Theme` をまとめ、`Theme::default_light()` / `Theme::default_dark()` を実装
- [x] 1.6 Floem の reactive context に Theme を注入するヘルパ `Theme::provide()` / `Theme::current()` を提供

## 2. テスト

- [x] 2.1 light/dark でトークン値が異なることのスナップショットテスト
- [x] 2.2 `Theme::current()` がコンテキスト未注入時に既定値を返すことのテスト

## 3. Storybook

- [x] 3.1 `storybook/src/pages/theme_tokens.rs` を追加
- [x] 3.2 ページ内で以下を表示
  - [x] 色トークン一覧（スウォッチ + トークン名）
  - [x] spacing スケール（実際の余白を矩形で可視化）
  - [x] typography スタイル一覧（サンプル文字列付き）
- [x] 3.3 Storybook グローバルに **light/dark 切替トグル** を実装し、本ページで切替が反映されることを確認

## 4. ドキュメント

- [x] 4.1 `docs/directory-structure.md` に「数値・色を直書きせず theme トークンを参照する」規約を追記
- [x] 4.2 `kuw-workflow-guide` skill に同規約を追記

## 5. 完了確認

- [x] 5.1 `cargo check -p katana-ui-widget` が通る
- [x] 5.2 `just storybook-check` 通過（theme_tokens ページ追加済）
- [x] 5.3 ast-lint が通る
