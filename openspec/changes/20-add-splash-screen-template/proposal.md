## Why

`katana` の `views/splash.rs`、`katana-chat-ui` のセッション初期化中の chrome 表示、各 sibling crate の起動時画面など、「アプリ起動の数百ms〜数秒の間に表示する splash 画面」需要が共通している。logo + アプリ名 + version + loading + 進行 progress + 失敗時 retry の組み合わせは似通っているが、現状 consumer ごとに別々に実装されている。

## What Changes

- `widget::molecules` に `SplashScreen` molecule を追加する:
  - option:
    - `logo: Option<SvgIcon | ImageSource>`
    - `title: String`
    - `subtitle: Option<String>`
    - `version: Option<String>`
    - `status: SplashStatus = Idle | Loading { progress: Option<f32>, label: Option<String> } | Error { message, retry }`
    - `background: Option<SplashBackground>`（Solid token / Gradient / Image）
    - `size: SplashSize = Embedded | Window`
  - action: `Retry` / `Cancel`
  - event: `SplashRetried` / `SplashCancelled` / `SplashStatusChanged`
  - state: status, callback_log

## Capabilities

### New Capabilities

- `kuc-splash-screen`: SplashScreen molecule の完了条件を定義する。

## Impact

- `crates/katana-ui-core/src/molecule/structured/splash_screen.rs` 新設。
- consumer (`katana` splash、各 sibling 起動画面) は KUC molecule に統一可能。
- background option には theme tokens を使う。
