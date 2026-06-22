# Design — Animation primitives + reduced-motion

## 目的

UI の遷移を統一する animation tokens と 4 primitive、reduced-motion 配慮を core に持たせる。

## 採用方針

### 1. theme tokens

```text
MotionDurationToken = Instant(0ms) | Fast(120ms) | Default(200ms) | Slow(320ms)
MotionEasingToken = Linear | Standard | Emphasized | Decelerate | Accelerate
MotionDistanceToken = Compact(4px) | Default(8px) | Spacious(16px)
```

light / dark theme で同じ値（motion はテーマ非依存だが、 token として参照可能）。

### 2. MotionSpec

```text
MotionSpec {
  duration: MotionDurationToken,
  easing: MotionEasingToken,
  distance: MotionDistanceToken,
  primitive: MotionPrimitive,
  delay: Option<MotionDurationToken>,
}

MotionPrimitive =
  | Fade { from: f32, to: f32 }
  | Slide { distance: MotionDistanceToken, direction: MotionDirection }
  | Scale { from: f32, to: f32, origin: ScaleOrigin }
  | Shimmer { speed: ShimmerSpeed, direction: ShimmerDirection }
```

### 3. reduced-motion policy

```text
ReducedMotionPolicy = Respect | Force | Ignore
```

- `Respect`（デフォルト）: OS / adapter 設定に従う
- `Force`: 常に reduced（test / debug 用）
- `Ignore`: 常に full animation（テスト / 特殊用途）

reduced 時、すべての primitive は `Instant` 扱いになる。Shimmer は無効化。

### 4. context-aware 無効化

`MotionPolicy.disable_in: Vec<MotionContext>` で特定 context（例: storybook test mode、ある molecule type）でアニメーションを切る hook を持つ。テストの再現性確保のため。

### 5. 各 molecule への組み込み

| molecule | motion default (open/show) |
| --- | --- |
| Popover | Fade + slight Slide |
| HoverCard | Fade + Slide(small) |
| ContextMenu | Fade + Scale(0.95→1.0) |
| Modal | Fade + Scale(0.96→1.0) |
| NotificationToast | Slide in (position 由来) + Fade |
| ToastStackManager | 個々 toast の Slide + Fade |
| Banner | Slide down + Fade |
| Skeleton | Shimmer or Pulse（既 atom） |
| Accordion | Height transition + Fade |
| DragPreview | Scale(1.0→0.96) + Fade(1.0→0.8) on pickup |

### 6. adapter contract

- adapter は OS の `prefers-reduced-motion` を読み取り、runtime に通知する責務
- core contract gate に reduced-motion query を追加

### 7. test 用

- Storybook の test mode では `disable_in = [Storybook]` を入れて全アニメーション無効
- 自動テストは Force = Reduced で挙動を回帰可能にする

## 代替案と却下理由

| 代替 | 却下理由 |
| --- | --- |
| 各 molecule が独自 animation を持つ | 揃わない、reduced-motion が molecule 毎に確認必要、画像回帰の根拠も散らかる。 |
| アニメーションは adapter 任せ | core の挙動契約に持てない、accessibility 保証が adapter 依存になる。 |
| アニメーションを完全に除外 | overlay / disclosure の UX 品質が下がる、業界ベースライン未満。 |

## Out of scope

- spring animation：v2 以降（duration token に統一できないため）
- 物理ベースの drag animation：別 change
- 4 primitive を超えるカスタム curve：consumer が `Custom` motion を渡せるよう v2 で検討

## 影響範囲

- すべての overlay / disclosure / loading molecule に motion option を追加（default は既存挙動を破壊しない transition のみ）
- external runtime boundary に reduced-motion query を追加
- 自動テストで Force=Reduced による安定回帰
