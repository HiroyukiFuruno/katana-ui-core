# Tasks — 21-popover

## 1. 実装

- [ ] 1.1 `layout/popover/types.rs` に `PopoverProps` / `Placement` / `AnchorRef` を定義
- [ ] 1.2 `layout/popover/ops.rs` に位置決め / 自動反転 / outside-click 検知 / Esc 検知を実装
- [ ] 1.3 `layout/popover/view.rs` に overlay レイヤーへ配置する view を実装
- [ ] 1.4 `layout/popover/mod.rs` で公開 API を整理

## 2. テスト

- [ ] 2.1 placement ごとに位置が正しく算出されるテスト
- [ ] 2.2 画面端での自動反転テスト
- [ ] 2.3 outside-click / Esc で `on_close` が呼ばれるテスト

## 3. 既存 widget の置換（追従）

- [ ] 3.1 `composite/selector/select/view.rs` の暫定 popover を `layout/popover` に差し替え（API 変更なし）
- [ ] 3.2 `composite/indicator/tooltip/view.rs` の暫定 popup を `layout/popover` に差し替え
- [ ] 3.3 select_box / tooltip の Storybook ページで「リファクタ前後の見た目同等」回帰確認

## 4. Storybook

- [ ] 4.1 `storybook/src/pages/popover.rs` を追加し `pages/mod.rs` に登録
- [ ] 4.2 ページ内表示
  - [ ] 4 placement の配置確認
  - [ ] 画面端での反転デモ
  - [ ] 子要素にメニュー風 (`TextButton` のリスト) を入れた合成サンプル
  - [ ] outside-click / Esc 動作のライブセル
  - [ ] light / dark 追従

## 5. 完了確認

- [ ] 5.1 `cargo check -p katana-ui-widget`
- [ ] 5.2 `just storybook` で popover ページが想定通り表示。select_box / tooltip ページも回帰なし
- [ ] 5.3 ast-lint 通過
