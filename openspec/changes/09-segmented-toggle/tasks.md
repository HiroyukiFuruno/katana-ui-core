# Tasks — 09-segmented-toggle

## 1. 実装

- [x] 1.1 `composite/selector/segmented/types.rs` に `SegmentedToggleProps<K>` / `Segment` を定義
- [x] 1.2 `composite/selector/segmented/view.rs` を実装。横並び + 選択ハイライトのアニメーション
- [x] 1.3 `composite/selector/segmented/mod.rs` で公開 API を整理

## 2. テスト

- [x] 2.1 `K` enum を渡して選択切替が反映されるテスト
- [x] 2.2 `options` が空の場合のフォールバック挙動

## 3. Storybook

- [x] 3.1 `storybook/src/pages/segmented_toggle.rs` を追加し `pages/mod.rs` に登録
- [x] 3.2 ページ内表示
  - [x] Label のみ / Icon のみ / Icon+Label のサンプル各 1
  - [x] 2/3/5 セグメントのバリエーション
  - [x] disabled
  - [x] light / dark 追従

## 4. 完了確認

- [x] 4.1 `cargo check -p katana-ui-widget`
- [x] 4.2 `just storybook` で segmented_toggle ページが想定通り表示
- [x] 4.3 ast-lint 通過
