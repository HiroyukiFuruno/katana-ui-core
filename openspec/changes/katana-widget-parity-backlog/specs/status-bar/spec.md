# StatusBar Widget Spec

## 概要

severity アイコン付きステータスメッセージ + アクションボタンを表示する水平バー widget。

## 出典

- `../katana/crates/katana-ui/src/views/top_bar/status_bar.rs`

## 階層配置

`layout/status_bar`

## 依存

- Icon (03)
- Badge (15) — severity 表示

## API 概要（TBD）

- `Severity`: Error | Warning | Success | Info
- `StatusBar`: message, severity, leading_slot (node), trailing_slot (node), on_action
