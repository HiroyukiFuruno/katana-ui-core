# Design — SplashScreen molecule

## 目的

アプリ起動時の splash 画面を統一テンプレートで提供する。

## 採用方針

### 1. status

```text
SplashStatus =
  | Idle
  | Loading { progress: Option<f32>, label: Option<String> }
  | Error { message: String, retry: bool }
```

- `Idle`: アニメーションのみ
- `Loading`: progress=None で indeterminate、Some(f32) で determinate
- `Error`: error メッセージと retry button

### 2. background

```text
SplashBackground =
  | Solid(ColorToken)
  | Gradient { from: ColorToken, to: ColorToken, direction: GradientDirection }
  | Image { source: ImageSource, opacity: f32 }
```

### 3. size

- `Embedded`: 親 container 内に配置
- `Window`: 全画面、layout は中央寄せ

### 4. キーボード / accessibility

- splash 表示中、Enter で retry（Error 時）/ Esc で cancel
- role=status（Loading / Idle）、role=alert（Error）
- live region announce: 状態変化時に label を読み上げ

### 5. アニメーション

- `add-animation-primitives-18` の MotionSpec を使って logo の fade-in / pulse
- reduced-motion で fade-in は Instant、pulse は無効化

## 代替案と却下理由

| 代替 | 却下理由 |
| --- | --- |
| `EmptyState` を splash 代わりに使う | EmptyState は「空表示」の責務。アプリ起動の loading / error / retry / version を持たない。 |
| `Modal` をフルスクリーン化する | Modal は overlay 概念。アプリ起動 splash は overlay ではなく root view。 |

## Out of scope

- 起動シーケンスのライフサイクル管理：consumer 責務
- スプラッシュ消滅のトランジション：consumer がアニメーションを叩く
- マルチウィンドウの splash 同期：consumer 責務

## 影響範囲

- consumer の splash 実装を統一
- 動作は KUC `Application` / `Window` の起動シーケンスと並走
