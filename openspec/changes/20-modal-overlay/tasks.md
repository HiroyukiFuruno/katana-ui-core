# Tasks — 20-modal-overlay

## 1. 実装

- [x] 1.1 `layout/modal/types.rs` に `ModalProps` / `Size` を定義
- [ ] 1.2 `layout/modal/ops.rs` にフォーカストラップ / Esc / backdrop 検知ロジック
  - [x] Esc / backdrop の close 判定
  - [x] open/close に伴う focus transition 判定
  - [/] Tab 移動を含む focus trap と trigger への実フォーカス復帰
- [x] 1.3 `layout/modal/view.rs` に overlay + dialog の view を実装
- [x] 1.4 `layout/modal/mod.rs` で公開 API を整理

## 2. テスト

- [x] 2.1 Esc キーで `on_close` が呼ばれるテスト
- [x] 2.2 backdrop クリックで `on_close` が呼ばれる / 抑制設定で呼ばれないテスト
- [/] 2.3 開閉トグルでフォーカスが内部 → 呼び出し元へ正しく戻るテスト

- [x] 3.1 `storybook/src/pages/modal_overlay.rs` を追加し `pages/mod.rs` に登録
- [x] 3.2 ページ内表示
  - [x] 開閉トリガボタン + 各 size のサンプル
  - [x] title / footer slot 使用例（confirm / form / detail の 3 種）
  - [x] dismiss_on_backdrop=false の例
  - [x] light / dark 追従

## 4. 完了確認

- [x] 4.1 `cargo check -p katana-ui-widget`
- [x] 4.2 `just storybook` で modal_overlay ページが想定通り表示
- [x] 4.3 ast-lint 通過

## ユーザーフィードバック

- [/] ast-lint の file-length / type-separation が発火した場合、単に `view()` を別ファイルへ逃がして完了扱いにしない。対象 widget の型定義・状態管理・style resolve・event 処理・Storybook ライブセルまで含めて責務境界を再設計し、なぜその分割が妥当かを確認する。
- [ ] 作り直し対象とする。Modal は別ウィンドウで開くものとして定義し、同一ウィンドウ内 overlay は Modal ではなく別コンポーネントとして扱う。
- [ ] Modal は親ウィンドウから独立した native window を開き、閉じる、Esc、focus return、親との相互作用抑制を API として定義する。
- [ ] 既存の同一ウィンドウ内 overlay 実装は `OverlayDialog` など別名への分離または廃止を検討し、互換性影響を design に明記する。
- [ ] Storybook では別ウィンドウとして開く modal と、同一ウィンドウ overlay の違いが分かる構成にする。
