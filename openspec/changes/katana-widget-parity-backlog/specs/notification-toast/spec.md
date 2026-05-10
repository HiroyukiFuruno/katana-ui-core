# NotificationToast Widget Spec

## 概要

severity 付きメッセージの一時表示 widget。自動消去、手動 dismiss、スタック表示を扱う。

## 出典

- `../katana/crates/katana-ui/src/views/top_bar/status_bar.rs` (severity pattern)
- `../katana/crates/katana-ui/src/app_state.rs` (StatusType enum)

## 階層配置

`layout/toast`

## 依存

- Icon (03)
- Modal (20) / Popover (21) とは独立した overlay layer

## API 概要（TBD）

- `Severity`: Error | Warning | Success | Info
- `Toast`: message, severity, duration (Option), action_label (Option), on_dismiss, on_action
- `ToastStack`: position (TopRight | BottomRight | TopCenter ...), max_visible, toasts
