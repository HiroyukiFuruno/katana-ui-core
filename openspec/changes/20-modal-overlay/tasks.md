# Tasks — 20-modal-overlay

## 1. 実装

- [ ] 1.1 `layout/modal/types.rs` に `ModalProps` / `Size` を定義
- [ ] 1.2 `layout/modal/ops.rs` にフォーカストラップ / Esc / backdrop 検知ロジック
- [ ] 1.3 `layout/modal/view.rs` に overlay + dialog の view を実装
- [ ] 1.4 `layout/modal/mod.rs` で公開 API を整理

## 2. テスト

- [ ] 2.1 Esc キーで `on_close` が呼ばれるテスト
- [ ] 2.2 backdrop クリックで `on_close` が呼ばれる / 抑制設定で呼ばれないテスト
- [ ] 2.3 開閉トグルでフォーカスが内部 → 呼び出し元へ正しく戻るテスト

## 3. Storybook

- [ ] 3.1 `storybook/src/pages/modal_overlay.rs` を追加し `pages/mod.rs` に登録
- [ ] 3.2 ページ内表示
  - [ ] 開閉トリガボタン + 各 size のサンプル
  - [ ] title / footer slot 使用例（confirm / form / detail の 3 種）
  - [ ] dismiss_on_backdrop=false の例
  - [ ] light / dark 追従

## 4. 完了確認

- [ ] 4.1 `cargo check -p katana-ui-widget`
- [ ] 4.2 `just storybook` で modal_overlay ページが想定通り表示
- [ ] 4.3 ast-lint 通過
