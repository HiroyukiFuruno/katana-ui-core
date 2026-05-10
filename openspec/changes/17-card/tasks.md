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
