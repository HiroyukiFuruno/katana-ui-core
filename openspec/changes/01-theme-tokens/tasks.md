# Tasks — 01-theme-tokens

## 1. 実装

- [ ] 1.1 `theme/color/palette.rs` に生 RGB 定数を定義（外部に `pub` しない）
- [ ] 1.2 `theme/color/mod.rs` に意味的トークン構造体 `ColorTokens` を定義
- [ ] 1.3 `theme/spacing/mod.rs` に `SpacingTokens` を定義（`xxs`〜`xxl`）
- [ ] 1.4 `theme/typography/mod.rs` に `TypographyTokens` と役割別 `TextStyle` を定義
- [ ] 1.5 `theme/mod.rs` に `Theme` をまとめ、`Theme::default_light()` / `Theme::default_dark()` を実装
- [ ] 1.6 Floem の reactive context に Theme を注入するヘルパ `provide_theme` / `use_theme` を提供

## 2. テスト

- [ ] 2.1 light/dark でトークン値が異なることのスナップショットテスト
- [ ] 2.2 `use_theme` がコンテキスト未注入時に既定値を返すことのテスト

## 3. Storybook

- [ ] 3.1 `storybook/src/pages/theme_tokens.rs` を追加
- [ ] 3.2 ページ内で以下を表示
  - [ ] 色トークン一覧（スウォッチ + トークン名）
  - [ ] spacing スケール（実際の余白を矩形で可視化）
  - [ ] typography スタイル一覧（サンプル文字列付き）
- [ ] 3.3 Storybook グローバルに **light/dark 切替トグル** を実装し、本ページで切替が反映されることを確認

## 4. ドキュメント

- [ ] 4.1 `docs/directory-structure.md` に「数値・色を直書きせず theme トークンを参照する」規約を追記
- [ ] 4.2 `kuw-workflow-guide` skill に同規約を追記

## 5. 完了確認

- [ ] 5.1 `cargo check -p katana-ui-widget` が通る
- [ ] 5.2 `just storybook` 起動 → theme_tokens ページが light/dark で破綻なく表示される
- [ ] 5.3 ast-lint が通る（`primitive`/`composite`/`layout` から `theme` への参照は許可、逆方向の混入が無いこと）
