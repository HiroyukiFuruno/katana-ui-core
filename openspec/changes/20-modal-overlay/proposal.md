## Why

確認ダイアログ / 入力ダイアログ / 詳細表示など、画面全体を覆って前面に表示する UI が必要。`../katana/crates/katana-ui/src/widgets/modal/` の役割を Floem に移植。背景 dimmer / Esc 閉じ / 外側クリック閉じ / フォーカストラップ等の慣習を吸収する。

## What Changes

- `layout/modal/` に `Modal` widget を提供。
- props: `open: bool`、`on_close: Fn()`、`title: Option<String>`、`size`（`Sm` / `Md` / `Lg` / `Custom(width pt)`）、`dismiss_on_backdrop: bool`（既定 true）、`dismiss_on_esc: bool`（既定 true）、`children: View`、`footer: Option<View>`。
- a11y: 開いた瞬間にフォーカスを内部に移し、閉じたら呼び出し元に戻す。フォーカストラップ。
- 背景 dimmer は `theme/color` の overlay トークン。
- 内部実装は最小の overlay 重ね描画。21 (popover) 完了後にレイヤー管理を共通化する追従 task を 21 側に置く。

## Capabilities

### New Capabilities

- `widget-modal-overlay`: 背景 dimmer + フォーカストラップ + Esc/backdrop 閉じを統一したダイアログ枠。

## Impact

- 確認 / 入力 / 詳細 / 設定など、消費側が `children` に好きな view を入れて使える。
- title / footer slot を持たせることで「ヘッダ + 本体 + フッタ」の典型構造を簡潔に書ける。
