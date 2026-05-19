## Why

`Popover` / `HoverCard` / `ContextMenu` / `Modal` / `NotificationToast` / `ToastStackManager` / `Banner` / `Skeleton` / `Accordion` / `DragPreview` 等の molecule は、表示・消失・遷移の挙動が consumer / adapter によって揺れている。KUC 内で「reduced-motion 設定」「アニメーション curve トークン」「fade / slide / scale / shimmer の 4 primitive」を共通化していないため、accessibility（前庭機能の問題への配慮、reduced-motion 尊重）も保証できない。

## What Changes

- `theme` module に animation tokens を追加する:
  - `MotionDurationToken`（Instant / Fast / Default / Slow）
  - `MotionEasingToken`（Linear / Standard / Emphasized / Decelerate / Accelerate）
  - `MotionDistanceToken`（Compact / Default / Spacious）
- `interaction` module に `MotionPolicy` を追加する:
  - `reduced_motion: ReducedMotionPolicy = Respect | Force | Ignore`
  - `disable_in: Vec<MotionContext>`（特定 context での無効化）
- 共通アニメーション primitive を `interaction/motion.rs` に追加する:
  - `Fade { from, to }`
  - `Slide { from, distance, direction }`
  - `Scale { from, to, origin }`
  - `Shimmer { speed, direction }`
- 各 molecule（Popover / HoverCard / ContextMenu / Modal / NotificationToast / ToastStackManager / Banner / Skeleton / Accordion / DragPreview）に `motion: MotionSpec` を typed option として追加する。
- accessibility OS 設定 (`prefers-reduced-motion`) を adapter から受け取る経路を確立する。

## Capabilities

### New Capabilities

- `kuc-motion`: animation tokens / MotionPolicy / 4 primitive / reduced-motion respect の完了条件を定義する。

### Modified Capabilities

- `kuc-widget-layer`: 各 disclosure / overlay / loading molecule が `motion: MotionSpec` を共通受け入れし、reduced-motion を統一的に尊重することを明記する。
- `kuc-core-foundation`: theme に motion tokens が含まれることを明記する。

## Impact

- `crates/katana-ui-core/src/theme/` に motion tokens を追加する。
- `crates/katana-ui-core/src/interaction/motion.rs` を新設する。
- 各 molecule に motion option を追加する（default は theme から導出）。
- adapter は OS の `prefers-reduced-motion` を runtime callback で報告する責務を持つ。
