## Why

`katana` の preview pane / explorer / problems panel、`katana-chat-ui` の message list / vendor bar、`katana-markdown-linter` の lint 進行中など、画面 load 中に「形状だけのプレースホルダ（skeleton）」を出したい箇所が広範に存在する。現状は Spinner / LoadingDots や空白で代用しており、layout のがたつき（layout shift）と UX 品質が低い。

## What Changes

- `widget::atoms` に `Skeleton` atom を追加する:
  - option:
    - `shape: Rect | Circle | Line | Text { lines: usize, last_line_ratio: f32 }`
    - `width: SkeletonSize`, `height: SkeletonSize`
    - `radius: Option<RadiusToken>`
    - `tone: Neutral | Muted | Inverted`
    - `animation: None | Pulse | Shimmer | Wave`
    - `accessibility_label: Option<String>`
    - `aspect_ratio: Option<f32>`
- `widget::molecules` に `SkeletonCluster` molecule を追加する:
  - 複数 skeleton をまとめて配置するテンプレート（avatar + 2 lines、card、list-row）
  - preset: card, list-row, message, paragraph, image-card, code-block

## Capabilities

### New Capabilities

- `kuc-skeleton-atom`: Skeleton atom の完了条件を定義する。
- `kuc-skeleton-cluster`: SkeletonCluster molecule の完了条件を定義する。

## Impact

- `crates/katana-ui-core/src/atom/skeleton.rs` 新設。
- `crates/katana-ui-core/src/molecule/skeleton_cluster.rs` 新設。
- DiagnosticsList / SelectionList / TreeView 等の loading 表示で embed 可能。
- アニメーション本体は `add-animation-primitives-18` の reduced-motion 設定に従う。
