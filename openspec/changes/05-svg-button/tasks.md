# Tasks — 05-svg-button

## 1. 実装

- [ ] 1.1 `composite/button/svg/types.rs` に `SvgButtonProps` / `Variant` / `Tone` を定義（`a11y_label` は `String` 必須フィールド）
- [ ] 1.2 `composite/button/svg/view.rs` に view を実装。state 別 (default/hover/active/focus/disabled/loading) のスタイルを theme から解決
- [ ] 1.3 `loading` 時は `Icon` を `Spinner` に置換。サイズは props.size に追従
- [ ] 1.4 `composite/button/svg/mod.rs` で公開 API を整理

## 2. テスト

- [ ] 2.1 各 variant × tone の組合せでスタイル解決が破綻しないユニットテスト
- [ ] 2.2 `disabled=true` で `on_click` が呼ばれないテスト
- [ ] 2.3 `a11y_label` 未指定をコンパイルエラーで弾く API 設計確認

## 3. Storybook

- [ ] 3.1 `storybook/src/pages/svg_button.rs` を追加し `pages/mod.rs` に登録
- [ ] 3.2 ページ内表示
  - [ ] 各 variant × tone のグリッド
  - [ ] サイズトークン違い
  - [ ] disabled / loading の状態
  - [ ] hover / active / focus の動作（マウス / キーボード操作で確認可能なライブセル）
  - [ ] light / dark 追従

## 4. 完了確認

- [ ] 4.1 `cargo check -p katana-ui-widget`
- [ ] 4.2 `just storybook` で svg_button ページが想定通り表示
- [ ] 4.3 ast-lint 通過（`composite/button/svg` から他の `composite/<X>` を参照していないこと）
