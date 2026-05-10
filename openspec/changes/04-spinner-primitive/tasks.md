# Tasks — 04-spinner-primitive

## 1. 実装

- [x] 1.1 `primitive/spinner/types.rs` に `SpinnerProps`/`SpinnerSize` を定義
- [x] 1.2 `primitive/spinner/view.rs` に arc SVG 生成ヘルパを実装（angle_deg でフレーム位置指定）
- [x] 1.3 `primitive/spinner/mod.rs` で `Spinner` builder + `ResolvedSpinner` を公開
- [x] 1.4 `reduced_motion` フラグで角度を 0 に固定（prefers-reduced-motion 相当）

## 2. テスト

- [x] 2.1 デフォルト props が panic なく解決されることのテスト
- [x] 2.2 `reduced_motion=true` 時に角度違いでも同一 SVG が返るテスト

## 3. Storybook

- [x] 3.1 `storybook/src/pages/spinner.rs` を追加し `pages/mod.rs` に登録
- [x] 3.2 ページ内表示
  - [x] 各サイズトークンの spinner を並べる
  - [x] accent / danger 色での色違い
  - [x] reduced-motion トグルによる挙動デモ
  - [x] light / dark 追従

## 4. 完了確認

- [x] 4.1 `cargo check -p katana-ui-widget`
- [x] 4.2 storybook-check 通過
- [x] 4.3 ast-lint 通過
