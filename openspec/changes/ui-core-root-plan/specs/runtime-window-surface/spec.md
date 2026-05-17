## ADDED Requirements

### Requirement: Application runtime API

KUC MUST `Application`, `ApplicationBuilder`, `AppConfig`, `AppHandle`, `AppLifecycle`, and `RuntimeAdapter` の中立 runtime API を提供する。
runtime API MUST 中核公開 API（core public API）から adapter-native runtime types を公開せず、イベントループ（event loop）を変換層（adapter）へ委譲する。

#### Scenario: app is built through KUC

- **WHEN** consumer が `Application::new().window(...).run()` で application を作る
- **THEN** app identity、persistence path、locale、accessibility options、lifecycle handling を KUC-owned types で設定できる
- **AND** 選択済み変換層（adapter）が `RuntimeAdapter` の背後で framework event loop を持つ

#### Scenario: running app receives commands

- **WHEN** 実行中 application が `AppHandle` 経由で command を受け取る
- **THEN** command は中立 KUC command types 経由で dispatch される
- **AND** window lookup と command routing は framework-native handles を必要としない

### Requirement: Window management API

KUC MUST `Window`, `WindowId`, `WindowConfig`, `WindowEvent`, `WindowCommand`, `WindowManager`, and `DisplayInfo` の中立 window API を提供する。
サポートする中立 window scope MUST title、size、minimum size、maximum size、close、focus、fullscreen、multi-window、decorations、icon、move、minimize、maximize、restore、display change を含む。

#### Scenario: multiple windows are created

- **WHEN** consumer が `WindowManager` で複数窓（multiple windows）を作る
- **THEN** 各 window は stable `WindowId` を受け取る
- **AND** consumer は framework-native window handles なしで current windows を iterate できる

#### Scenario: window command is sent

- **WHEN** consumer が `SetTitle`, `SetSize`, `SetPosition`, `Focus`, `Minimize`, `Maximize`, `Close`, or `Fullscreen` を送る
- **THEN** command は `WindowCommand` として表現される
- **AND** 選択済み変換層（adapter）が framework-specific behavior へ変換する

### Requirement: Surface API

KUC MUST `Surface`, `FrameHandle`, `PaintRequest`, and `SurfaceMetrics` の中立 surface API を提供する。
surface API MUST logical size、scale factor、DPI を neutral DTOs で公開する。

#### Scenario: adapter paints a frame

- **WHEN** 変換層（adapter）が `PaintRequest` を受け取る
- **THEN** KUC-owned types から surface metrics を取得できる
- **AND** 中核 crate（core crate）が framework renderer を知らないまま current `UiTree` を paint できる

### Requirement: platform-specific escape hatch

KUC MUST NOT platform menu、IME、drag & drop を標準中立 API（standard neutral API）へ入れてはならない。
それらの concern MUST `AdapterExtension` を入口にし、具体型は `PlatformMenuRequest`、`ImeRequest`、`DragDropRequest` として表現する。

#### Scenario: IME support is needed

- **WHEN** 変換層（adapter）が IME support を必要とする
- **THEN** `AdapterExtension::Ime(ImeRequest)` 経由で support を実装する
- **AND** 中核標準 API（core standard API）は合意済み neutral scope に限定されたままにする

### Requirement: Noop adapter validation

KUC MUST runtime / window / surface validation 用に無処理の変換層（Noop adapter）または同等の test harness を含める。
test harness MUST UI framework なしで neutral API を構築・実行できることを証明する。

#### Scenario: neutral runtime test runs

- **WHEN** runtime / window / surface tests を Noop adapter で実行する
- **THEN** application creation、window configuration、lifecycle events、surface metrics を Floem / GPUI / egui なしで validate できる
