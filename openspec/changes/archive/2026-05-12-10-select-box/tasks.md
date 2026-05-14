# Tasks — 10-select-box

## 1. 実装

- [x] 1.1 `composite/selector/select/types.rs` に `SelectBoxProps<K>` を定義
- [x] 1.2 `composite/selector/select/ops.rs` に開閉状態管理を実装
- [x] 1.3 `composite/selector/select/view.rs` にトリガ + 一覧パネルを実装。画面端での上展開フォールバック
- [x] 1.4 `composite/selector/select/mod.rs` で公開 API を整理

## 2. テスト

- [x] 2.1 開閉トグルの状態遷移テスト
- [x] 2.2 選択時に `on_change` が呼ばれパネルが閉じるテスト
- [x] 2.3 disabled 時にトリガが反応しないテスト

## 3. Storybook

- [x] 3.1 `storybook/src/pages/select_box.rs` を追加し `pages/mod.rs` に登録
- [x] 3.2 ページ内表示
  - [x] 短い options / 長い options（スクロールが必要なケース）
  - [x] placeholder 表示
  - [x] disabled
  - [x] light / dark 追従

## 4. 完了確認

- [x] 4.1 `cargo check -p katana-ui-widget`
- [x] 4.2 `just storybook` で select_box ページが想定通り表示
- [x] 4.3 ast-lint 通過

## ユーザーフィードバック

- [/] ast-lint の file-length / type-separation が発火した場合、単に `view()` を別ファイルへ逃がして完了扱いにしない。対象 widget の型定義・状態管理・style resolve・event 処理・Storybook ライブセルまで含めて責務境界を再設計し、なぜその分割が妥当かを確認する。
- [/] Storybook の SelectBox ページから resolved 値で trigger/options を再現した疑似サンプルを除去し、実際の `SelectBox::view` で live / readonly / open / long list を確認できる構成にする。
