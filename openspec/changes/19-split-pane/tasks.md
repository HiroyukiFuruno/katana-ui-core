# Tasks — 19-split-pane

## 1. 実装

- [ ] 1.1 `layout/split/types.rs` に `SplitPaneProps` / `Direction` を定義
- [ ] 1.2 `layout/split/ops.rs` にドラッグ中の ratio 計算 / min 制約クランプを実装
- [ ] 1.3 `layout/split/view.rs` に view を実装（ハンドルの hover/active 装飾、cursor 変更）
- [ ] 1.4 `layout/split/mod.rs` で公開 API を整理

## 2. テスト

- [ ] 2.1 ratio 更新と min 制約クランプのユニットテスト
- [ ] 2.2 direction Horizontal / Vertical の双方で同じ仕様が成立することのテスト
- [ ] 2.3 double-click で 50/50 復帰するテスト

## 3. Storybook

- [ ] 3.1 `storybook/src/pages/split_pane.rs` を追加し `pages/mod.rs` に登録
- [ ] 3.2 ページ内表示
  - [ ] Horizontal / Vertical サンプル
  - [ ] 入れ子（3 ペイン）の合成例
  - [ ] min 制約デモ
  - [ ] light / dark 追従

## 4. 完了確認

- [ ] 4.1 `cargo check -p katana-ui-widget`
- [ ] 4.2 `just storybook` で split_pane ページが想定通り表示
- [ ] 4.3 ast-lint 通過
