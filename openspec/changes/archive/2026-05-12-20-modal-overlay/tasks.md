# Tasks — 20-modal-overlay

## 1. 実装

- [x] 1.1 `layout/modal/types.rs` に `ModalProps` / `Size` を定義
- [x] 1.2 `layout/modal/ops.rs` にフォーカストラップ / Esc / backdrop 検知ロジック
  - [x] Esc / backdrop の close 判定
  - [x] open/close に伴う focus transition 判定
  - [x] Tab 移動を含む focus trap と trigger への実フォーカス復帰
- [x] 1.3 `layout/modal/view.rs` に overlay + dialog の view を実装
- [x] 1.4 `layout/modal/mod.rs` で公開 API を整理
- [x] 1.5 `Modal` をネイティブ窓 API 化し、同一ウィンドウ overlay は `OverlayDialog` として扱う
- [x] 1.6 `Modal::view` の挙動を native window 側に寄せる

## 2. テスト

- [x] 2.1 Esc キーで `on_close` が呼ばれるテスト
- [x] 2.2 backdrop クリックで `on_close` が呼ばれる / 抑制設定で呼ばれないテスト
- [x] 2.3 開閉トグルでフォーカスが内部 → 呼び出し元へ正しく戻るテスト

- [x] 3.1 `storybook/src/pages/modal_overlay.rs` を追加し `pages/mod.rs` に登録
- [x] 3.2 ページ内表示
  - [x] 開閉トリガボタン + 各 size のサンプル
  - [x] title / footer slot 使用例（confirm / form / detail の 3 種）
  - [x] dismiss_on_backdrop=false の例
  - [x] light / dark 追従
- [x] 3.3 `OverlayDialog` を同一ページ内で Modal と分離して見える UI と説明文で整理する
- [x] 3.4 `Modal` 導線と `OverlayDialog` を別名で併記し、Storybook 上で「別ウィンドウ起動」と「同一ウィンドウ表示」を明確に分かる構成にする

## 4. 完了確認

- [x] 4.1 `cargo check -p katana-ui-widget`
- [x] 4.2 `just storybook` で `modal_overlay` ページを開き、「Modal 起動」操作で別ネイティブウィンドウが開くことを目視で確認すること
  - 2026-05-11: `KATANA_UI_WIDGET_STORYBOOK_PAGE=modal-overlay` で起動し、katana方式の `screencapture -l <window_id>` でModal / OverlayDialog導線を含むページ表示を確認。
- [x] 4.3 `OverlayDialog` の見た目/説明と `Modal` の見た目/説明を同ページで確認し、同一ウィンドウと別ウィンドウの役割差を明示的に担保すること
- [x] 4.4 `Modal` クローズ、Esc、外側クリック、focus return の確認手順を 1 画面内にまとめ、再現可能にすること
- [x] 4.5 `cargo test` で `Modal` / `OverlayDialog` の差分が壊れていないことを確認すること
- [x] 4.6 ast-lint 通過

## ユーザーフィードバック

- [/] ast-lint の file-length / type-separation が発火した場合、単に `view()` を別ファイルへ逃がして完了扱いにしない。対象 widget の型定義・状態管理・style resolve・event 処理・Storybook ライブセルまで含めて責務境界を再設計し、なぜその分割が妥当かを確認する。
- [/] 作り直し対象とする。Modal は別ウィンドウで開くものとして定義し、同一ウィンドウ内 overlay は Modal ではなく別コンポーネントとして扱う。
- [/] Modal は親ウィンドウから独立した native window を開き、閉じる、Esc、focus return、親との相互作用抑制を API として定義する。
- [/] 既存の同一ウィンドウ内 overlay 実装は `OverlayDialog` など別名への分離または廃止を検討し、互換性影響を design に明記する。
- [/] Storybook では別ウィンドウとして開く modal と、同一ウィンドウ overlay の違いが分かる構成にする。
