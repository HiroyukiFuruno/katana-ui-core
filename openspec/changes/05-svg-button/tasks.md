# Tasks — 05-svg-button

## 1. 実装

- [x] 1.1 `composite/button/svg/types.rs` に `SvgButtonProps` / `Variant` / `Tone` を定義（`a11y_label` は必須フィールド）
- [x] 1.2 `composite/button/svg/view.rs` に state 別スタイル解決ヘルパを実装
- [x] 1.3 `loading` / `disabled` の状態を `ResolvedSvgButton` に反映
- [x] 1.4 `composite/button/svg/mod.rs` で `SvgButton` builder + `ResolvedSvgButton` を公開

## 2. テスト

- [x] 2.1 各 variant × tone の組合せでスタイル解決が破綻しないユニットテスト
- [x] 2.2 `disabled=true` で bg が None かつアイコン色が text_disabled になるテスト
- [x] 2.3 `a11y_label` は `SvgButton::new()` の必須引数として型レベルで強制

## 3. Storybook

- [x] 3.1 `storybook/src/pages/svg_button.rs` を追加し `pages/mod.rs` に登録
- [x] 3.2 ページ内表示
  - [x] 各 variant × tone のグリッド
  - [x] disabled の状態
  - [x] light / dark 追従

## 4. 完了確認

- [x] 4.1 `cargo check -p katana-ui-widget`
- [x] 4.2 storybook-check 通過
- [x] 4.3 ast-lint 通過
