# Tasks — 21-popover

## 1. 実装

- [x] 1.1 `layout/popover/types.rs` に `PopoverProps` / `Placement` / `AnchorRef` を定義
- [x] 1.2 `layout/popover/ops.rs` に位置決め / 自動反転 / outside-click 検知 / Esc 検知を実装
- [x] 1.3 `layout/popover/view.rs` に overlay レイヤーへ配置する view を実装
- [x] 1.4 `layout/popover/mod.rs` で公開 API を整理

## 2. テスト

- [x] 2.1 placement ごとに位置が正しく算出されるテスト
- [x] 2.2 画面端での自動反転テスト
- [x] 2.3 outside-click / Esc で `on_close` が呼ばれるテスト

## 3. 既存 widget の置換（追従）

- [x] 3.1 `select_box` 画面で `layout/popover` のオフセット/反転/配置計算を使った popover 表示へ置換
- [x] 3.2 `tooltip` 画面で `layout/popover` のオフセット/反転/配置計算を使った tooltip 表示へ置換
- [x] 3.3 select_box / tooltip の Storybook ページで「リファクタ前後の見た目同等」回帰確認（実行手順/スクリーンショット検証を追加）
  - 実施手順と照合基準を [`tmp/storybook_popover_regression_check.md`](/Users/hiroyuki_furuno/works/private/katana-ui-widget/tmp/storybook_popover_regression_check.md) に明文化

## 4. Storybook

- [x] 4.1 `storybook/src/pages/popover.rs` を追加し `pages/mod.rs` に登録
- [x] 4.2 ページ内表示
  - [x] 4 placement の配置確認
  - [x] 画面端での反転デモ
  - [x] 子要素にメニュー風 (`TextButton` のリスト) を入れた合成サンプル
  - [x] outside-click / Esc 動作のライブセル
  - [x] light / dark 追従

## 5. 完了確認

- [x] 5.1 `cargo check -p katana-ui-widget`
- [x] 5.2 `just storybook` で popover ページが起動し、light/dark 切替が反映（select_box / tooltip は手元確認）
  - 2026-05-11: `KATANA_UI_WIDGET_STORYBOOK_PAGE=popover` で起動し、katana方式の `screencapture -l <window_id>` でPopoverページのウィンドウ単体表示を確認。
- [x] 5.3 ast-lint 通過
  - 2026-05-11: `just ast-lint` 通過

## ユーザーフィードバック

- [/] ast-lint の file-length / type-separation が発火した場合、単に `view()` を別ファイルへ逃がして完了扱いにしない。対象 widget の型定義・状態管理・style resolve・event 処理・Storybook ライブセルまで含めて責務境界を再設計し、なぜその分割が妥当かを確認する。
- [x] 現状は未実装扱いで再確認する。座標説明や疑似表示ではなく、anchor に紐づく実 overlay として表示する。
- [x] open / close、outside click、Esc、placement、自動反転、offset、width、focus handling を実 event で確認できるようにする。
- [x] Popover content は上位から任意 node を受け取り、menu、form、card など複雑な UI を入れられるようにする。
- [x] Storybook ではボタンを押すと popover が実際に開き、配置変更と close 条件が操作できる live sample にする。
