# Tasks — 03-icon-primitive

## 1. 実装

- [ ] 1.1 `primitive/icon/types.rs` に `IconSource` / `IconSize` / `IconProps` を定義
- [ ] 1.2 `primitive/icon/view.rs` に `Icon` view 関数を実装（resvg で SVG → ピクセル / Floem 描画）
- [ ] 1.3 `primitive/icon/mod.rs` で公開 API を整理
- [ ] 1.4 サイズは `theme/spacing` トークン or 明示 pt の双方を受けられるようにする

## 2. テスト

- [ ] 2.1 SVG bytes が描画用バッファに変換されることのユニットテスト
- [ ] 2.2 不正な SVG 入力時にパニックせずエラーログのみで描画スキップする挙動の確認

## 3. Storybook

- [ ] 3.1 `storybook/src/pages/icon.rs` を追加し `pages/mod.rs` に登録
- [ ] 3.2 ページ内表示
  - [ ] 同梱サンプル SVG（チェックマーク・矢印・x など 3 種をページ内 const として持たせる）
  - [ ] 各サイズトークンでの並び表示
  - [ ] 色トークンを差し替えて色変化を確認
  - [ ] light / dark 追従

## 4. 完了確認

- [ ] 4.1 `cargo check -p katana-ui-widget`
- [ ] 4.2 `just storybook` で icon ページが想定通り表示
- [ ] 4.3 ast-lint 通過
