# Tasks — 16-key-cap

## 1. 実装

- [x] 1.1 `composite/indicator/key_cap/types.rs` に `KeyLabel` / `NamedKey` / `KeyCapProps` / `KeyComboProps` を定義
- [x] 1.2 `composite/indicator/key_cap/ops.rs` に OS ごとの表示文字列解決ロジック
- [x] 1.3 `composite/indicator/key_cap/view.rs` に `KeyCap` / `KeyCombo` の view を実装
- [x] 1.4 `composite/indicator/key_cap/mod.rs` で公開 API を整理

## 2. テスト

- [x] 2.1 macOS / non-macOS で modifier 表示が切り替わるテスト（`cfg(test)` で flag 注入）
- [x] 2.2 `Named(F1)` / `Char('a')` / `Cmd + Shift + Char('p')` の代表的シリアライズ確認

## 3. Storybook

- [x] 3.1 `storybook/src/pages/key_cap.rs` を追加し `pages/mod.rs` に登録
- [x] 3.2 ページ内表示
  - [x] 単一キー / 修飾あり組合せの代表例
  - [x] OS 切替トグル（モック）で表示が変わるデモ
  - [x] サイズトークン違い
  - [x] light / dark 追従

## 4. 完了確認

- [x] 4.1 `cargo check -p katana-ui-widget`
- [x] 4.2 `just storybook` で key_cap ページが想定通り表示
- [x] 4.3 ast-lint 通過

## ユーザーフィードバック

- [/] ast-lint の file-length / type-separation が発火した場合、単に `view()` を別ファイルへ逃がして完了扱いにしない。対象 widget の型定義・状態管理・style resolve・event 処理・Storybook ライブセルまで含めて責務境界を再設計し、なぜその分割が妥当かを確認する。
