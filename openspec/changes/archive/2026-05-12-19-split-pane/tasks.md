# Tasks — 19-split-pane

## 1. 実装

- [x] 1.1 `layout/split/types.rs` に `SplitPaneProps` / `Direction` を定義
- [x] 1.2 `layout/split/ops.rs` にドラッグ中の ratio 計算 / min 制約クランプを実装
- [x] 1.3 `layout/split/view.rs` に view を実装（ハンドルの hover/active 装飾、cursor 変更）
- [x] 1.4 `layout/split/mod.rs` で公開 API を整理

## 2. テスト

- [x] 2.1 ratio 更新と min 制約クランプのユニットテスト
- [x] 2.2 direction Horizontal / Vertical の双方で同じ仕様が成立することのテスト
- [x] 2.3 double-click で 50/50 復帰するテスト

## 3. Storybook

- [x] 3.1 `storybook/src/pages/split_pane.rs` を追加し `pages/mod.rs` に登録
- [x] 3.2 ページ内表示
  - [x] Horizontal / Vertical サンプル
  - [x] 入れ子（3 ペイン）の合成例
  - [x] min 制約デモ
  - [x] light / dark 追従

## 4. 完了確認

- [x] 4.1 `cargo check -p katana-ui-widget`
- [x] 4.2 `just storybook` で split_pane ページが想定通り表示
- [x] 4.3 ast-lint 通過

## ユーザーフィードバック

- [/] ast-lint の file-length / type-separation が発火した場合、単に `view()` を別ファイルへ逃がして完了扱いにしない。対象 widget の型定義・状態管理・style resolve・event 処理・Storybook ライブセルまで含めて責務境界を再設計し、なぜその分割が妥当かを確認する。
