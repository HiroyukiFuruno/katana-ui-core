# Design — StartupStatePanel composition

## 目的

アプリ起動時や初期化中の状態表示を、KUC の atoms / molecules で組めるようにする。
KUC は splash 画面テンプレートを公開しない。

## 採用方針

### 1. status

```text
StartupState =
  | Idle
  | Loading { progress: Option<u8>, label: Option<String> }
  | Error { message: String, retry: bool }
```

- `Idle`: アニメーションのみ
- `Loading`: progress=None で indeterminate、Some(f32) で determinate
- `Error`: error メッセージと retry button

### 2. composition

```text
StartupStatePanel =
  heading + body + optional version
  optional icon
  ProgressBar or LoadingDots
  optional retry / cancel actions
```

### 3. キーボード / accessibility

- Error 時、Enter で retry / Esc で cancel を event として返す
- role=status（Loading / Idle）、role=alert（Error）
- live region announce: 状態変化時に label を読み上げ

### 4. アニメーション

- `18-add-animation-primitives` の MotionSpec を使える
- reduced-motion で loading animation を無効化できる

## 代替案と却下理由

| 代替 | 却下理由 |
| --- | --- |
| SplashScreen template を KUC に入れる | template / page に近く、KUC の atoms / molecules 境界を超える。 |
| `EmptyState` だけで済ませる | loading / progress / retry / live region の契約が不足する。 |

## Out of scope

- 起動シーケンスのライフサイクル管理
- full-screen / window / centered layout
- logo placement / background image / brand template
- マルチウィンドウの splash 同期

## 影響範囲

- consumer は startup template を自前で組む
- KUC は状態表示の atoms / molecules と event contract だけを統一する
