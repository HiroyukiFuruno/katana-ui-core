# Tasks — 07-icon-text-button

## 1. 実装

- [ ] 1.1 `composite/button/icon_text/types.rs` に `IconTextButtonProps` / `IconPosition` を定義
- [ ] 1.2 `composite/button/icon_text/view.rs` を実装。`Icon` primitive と `Text` primitive を組み合わせ、間隔は `theme/spacing`
- [ ] 1.3 `loading` 時は icon を spinner に置換しラベルは半透明
- [ ] 1.4 `composite/button/icon_text/mod.rs` で公開 API を整理
- [ ] 1.5 `TextButton` / `SvgButton` と variant/tone/size の意味を揃える

## 2. テスト

- [ ] 2.1 icon_position が leading / trailing の双方で正しく並ぶテスト
- [ ] 2.2 disabled / loading の挙動テスト

## 3. Storybook

- [ ] 3.1 `storybook/src/pages/icon_text_button.rs` を追加し `pages/mod.rs` に登録
- [ ] 3.2 ページ内表示
  - [ ] leading / trailing icon
  - [ ] variant × tone × size のグリッド
  - [ ] disabled / loading
  - [ ] light / dark 追従

## 4. 完了確認

- [ ] 4.1 `cargo check -p katana-ui-widget`
- [ ] 4.2 `just storybook` で icon_text_button ページが想定通り表示
- [ ] 4.3 ast-lint 通過（同 composite サブカテゴリ内 `button` 配下のみで完結していること）
