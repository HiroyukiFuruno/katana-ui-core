## Why

KUC は `NotificationToast` molecule を持つが、現状は単一 toast の表示モデルに留まり、複数 toast が連続発生したときの「スタッキング（stacking）」「重複排除（deduplication）」「位置（top-left / top-right / bottom-right 等）」「同時表示上限」「キュー管理」「pause-on-hover」を持たない。

`katana` の保存通知、`katana-chat-ui` のエージェントイベント通知、`katana-markdown-linter` の lint 完了通知などで、複数 toast が前後して発生する。consumer ごとにキュー管理を作っており、挙動の揺れと記述コストが大きい。

## What Changes

- `widget::molecules` に `ToastStackManager` molecule を追加する:
  - option:
    - `position: TopStart | TopCenter | TopEnd | BottomStart | BottomCenter | BottomEnd`
    - `max_visible: usize`
    - `dedup_strategy: None | ById | ByIdAndSeverity`
    - `default_duration_ms: u64`
    - `pause_on_hover: bool`
    - `stack_gap: f32`
    - `enter_direction`, `exit_direction`（slide direction）
  - action: `Enqueue(Toast)` / `Dismiss(id)` / `DismissAll` / `Pause` / `Resume`
  - event: `ToastShown` / `ToastTimedOut` / `ToastDismissed` / `ToastQueued` / `ToastReplaced`
  - state: `visible: VecDeque<ActiveToast>`, `queued: VecDeque<PendingToast>`, `paused: bool`
- `NotificationToast` を ToastStackManager の child として組み合わせる（既存単一 toast 用 API は維持）。
- toast 内 actions（button）は `Button` atom を子に持つ。

## Capabilities

### New Capabilities

- `kuc-toast-stack-manager`: ToastStackManager molecule の option / action / event / state / preset / preview / settings / 自動テスト / 数値化された描画契約 / Storybook ページの完了条件を定義する。

### Modified Capabilities

- `kuc-widget-layer`: `NotificationToast`（単一 toast）と `ToastStackManager`（複数 toast の編成）の責務境界を明記する。

## Impact

- `crates/katana-ui-core/src/molecule/disclosure/toast.rs` を更新し、`molecule/disclosure/toast_stack.rs` を新設する。
- consumer は単一 toast の場合は `NotificationToast`、複数 toast 管理が必要な場合は `ToastStackManager` を使う。
- adapter は overlay 描画と pointer event の hit testing 責務を持つ。
