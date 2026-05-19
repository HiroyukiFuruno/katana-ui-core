## Why

`katana` の splash、`katana-chat-ui` のセッション初期化中表示、KDV / KLE の初期 loading では、loading / error / retry / version などの状態表示が必要になる。

ただし splash 画面テンプレートは KUC の公開対象ではない。
KUC は画面ひな形（templates）を持たず、利用側が `EmptyState`、`Banner`、`ProgressBar`、`Skeleton`、`Button` を組み合わせて startup state を構築できる contract だけを持つ。

## What Changes

- `widget::molecules` に `StartupStatePanel` molecule を追加する:
  - option:
    - `heading: String`
    - `body: Option<String>`
    - `version_label: Option<String>`
    - `state: Idle | Loading { progress: Option<u8>, label: Option<String> } | Error { message: String, retry: bool }`
    - `icon: Option<Icon>`
    - `actions: Vec<ButtonSpec>`
  - action: `Retry` / `Cancel`
  - event: `StartupRetried` / `StartupCancelled` / `StartupStateChanged`
  - state: state, callback_log
- full-screen layout、background image、logo placement、起動 lifecycle は consumer が持つ。

## Capabilities

### New Capabilities

- `kuc-startup-state-composition`: StartupStatePanel molecule と既存 atoms / molecules の組合せ条件を定義する。

## Impact

- `crates/katana-ui-core/src/molecule/structured/startup_state_panel.rs` 新設、または `EmptyState` / `Banner` / `ProgressBar` の composition contract として実装する。
- consumer (`katana` splash、各 sibling 起動画面) は KUC atoms / molecules を組み合わせて template を自前実装する。
- background / full-screen / logo layout option は KUC に入れない。
