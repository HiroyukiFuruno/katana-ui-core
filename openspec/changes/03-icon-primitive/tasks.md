# Tasks — 03-icon-primitive

## 1. 実装

- [x] 1.1 `primitive/icon/types.rs` に `IconSource` / `IconSize` / `IconProps` を定義
- [x] 1.2 `primitive/icon/mod.rs` に `Icon` builder + `ResolvedIcon` を実装（floem の svg() ビューに委譲）
- [x] 1.3 公開 API を整理
- [x] 1.4 サイズは `theme/spacing` トークン or 明示 pt の双方を受けられるようにする

## 2. テスト

- [x] 2.1 有効な SVG bytes が非空の content を返すことのユニットテスト
- [x] 2.2 不正な UTF-8 バイト入力時にパニックせず空コンテンツを返す挙動の確認

## 3. Storybook

- [x] 3.1 `storybook/src/pages/icon.rs` を追加し `pages/mod.rs` に登録
- [x] 3.2 ページ内表示
  - [x] 同梱サンプル SVG（チェックマーク・矢印・x など 3 種をページ内 const として持たせる）
  - [x] 各サイズトークンでの並び表示
  - [x] 色トークンを差し替えて色変化を確認
  - [x] light / dark 追従

## 4. 完了確認

- [x] 4.1 `cargo check -p katana-ui-widget`
- [x] 4.2 storybook-check 通過
- [x] 4.3 ast-lint 通過
