# Tasks — 18-accordion

## 1. 実装

- [x] 1.1 `layout/accordion/types.rs` に `AccordionProps` / `IndicatorPosition` を定義
- [x] 1.2 `layout/accordion/view.rs` に展開・折り畳みのアニメーションを実装
- [x] 1.3 `layout/accordion/mod.rs` で公開 API を整理

## 2. テスト

- [x] 2.1 expanded toggle で `on_toggle` が呼ばれるテスト
- [x] 2.2 disabled 時にクリックが反応しないテスト

## 3. Storybook

- [x] 3.1 `storybook/src/pages/accordion.rs` を追加し `pages/mod.rs` に登録
- [x] 3.2 ページ内表示
  - [x] 既定 / 展開済み
  - [x] indicator 位置違い 3 種
  - [x] 子要素に `Text` / `TextInput` / `Card` を入れた合成例
  - [x] disabled
  - [x] light / dark 追従

## 4. 完了確認

- [x] 4.1 `cargo check -p katana-ui-widget`
- [x] 4.2 `just storybook` で accordion ページが想定通り表示
- [x] 4.3 ast-lint 通過

## ユーザーフィードバック

- [/] ast-lint の file-length / type-separation が発火した場合、単に `view()` を別ファイルへ逃がして完了扱いにしない。対象 widget の型定義・状態管理・style resolve・event 処理・Storybook ライブセルまで含めて責務境界を再設計し、なぜその分割が妥当かを確認する。
- [ ] 現状は未実装扱いで再確認する。単純な開閉だけでなく、katana の tree 表現を参考にした tree mode を追加する。
- [ ] default open / default closed、controlled / uncontrolled、disabled、複数 item の同時展開可否を API で指定できるようにする。
- [ ] 展開を発火するクリック領域を、アイコン＋文字、アイコンのみ、文字のみ、行全体から選択できるようにする。
- [ ] tree mode ではネスト、垂直線、現在選択中の行、hover 行、開閉アイコン、子要素の indent を表現できるようにする。
- [ ] 開閉時のアニメーションを実装し、reduced motion 時は安全に無効化できるようにする。
- [ ] Storybook に単純 accordion、tree accordion、default open / closed、クリック領域違い、アニメーション確認を追加する。
