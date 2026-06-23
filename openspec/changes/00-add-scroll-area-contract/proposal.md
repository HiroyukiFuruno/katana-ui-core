## Why

`katana`、KDV、KLE、`katana-chat-ui` は、どれもスクロールできる領域を前提にしている。
画面上では、本文、目次、検索結果、履歴、設定、Storybook の preview / inspector などが、それぞれ独立して縦または横へ動く。

KUC には `ScrollArea` layout model があるが、現状は単なる子要素入れ物に近く、次を option だけでは補えない。

- 縦 / 横 / 両方向の scroll axis
- 現在 offset、viewport size、content size
- scrollbar visibility / placement
- 外部からの scroll command
- scroll event、edge 到達 event
- nested scroll area の state 分離

これが曖昧なままだと、KDV / KLE は viewer / editor 内部の scroll を自前実装するだけでなく、周辺 panel や search result、TOC、diagnostics でも scroll state を重複実装することになる。

## What Changes

- `ScrollArea` を KUC の layout foundation として要件化する。
- `ScrollAreaOptions` を typed にする。
  - `axis`
  - `offset`
  - `viewport_extent`
  - `content_extent`
  - `scrollbar_visibility`
  - `scrollbar_placement`
  - `edge_threshold`
- action を定義する。
  - `ScrollTo`
  - `ScrollBy`
  - `ScrollIntoView`
  - `SetScrollbarVisibility`
- event を定義する。
  - `Scrolled`
  - `ScrollEdgeReached`
  - `ScrollCommandRejected`
- KDV / KLE の本文 viewer / editor そのものは KUC に入れない。KUC は周辺 UI や component composition に使う scroll container だけを提供する。

## Capabilities

### New Capabilities

- `kuc-scroll-area-contract`: ScrollArea の option、action、event、state、keyboard、scrollbar、Storybook、DoD を定義する。

### Modified Capabilities

- `kuc-widget-layer`: layout foundation として `ScrollArea` を明示し、list / tree / panel / command results / diagnostics が共通の scroll contract を参照できるようにする。

## Impact

- `crates/katana-ui-core/src/layout/` の `ScrollArea` を typed contract に拡張する。
- `crates/katana-ui-core/src/render_model/` に scroll props / event を追加する。
- `02-add-drag-drop-primitive` の autoscroll、`16-add-virtualized-list-and-tree` の scroll state、Storybook panel scroll の基盤になる。
- KDV / KLE の scroll synchronization policy は consumer 側に残す。
