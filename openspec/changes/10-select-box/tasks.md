# Tasks — 10-select-box

## 1. 実装

- [ ] 1.1 `composite/selector/select/types.rs` に `SelectBoxProps<K>` を定義
- [ ] 1.2 `composite/selector/select/ops.rs` に開閉状態管理を実装
- [ ] 1.3 `composite/selector/select/view.rs` にトリガ + 一覧パネルを実装。画面端での上展開フォールバック
- [ ] 1.4 `composite/selector/select/mod.rs` で公開 API を整理

## 2. テスト

- [ ] 2.1 開閉トグルの状態遷移テスト
- [ ] 2.2 選択時に `on_change` が呼ばれパネルが閉じるテスト
- [ ] 2.3 disabled 時にトリガが反応しないテスト

## 3. Storybook

- [ ] 3.1 `storybook/src/pages/select_box.rs` を追加し `pages/mod.rs` に登録
- [ ] 3.2 ページ内表示
  - [ ] 短い options / 長い options（スクロールが必要なケース）
  - [ ] placeholder 表示
  - [ ] disabled
  - [ ] light / dark 追従

## 4. 完了確認

- [ ] 4.1 `cargo check -p katana-ui-widget`
- [ ] 4.2 `just storybook` で select_box ページが想定通り表示
- [ ] 4.3 ast-lint 通過
