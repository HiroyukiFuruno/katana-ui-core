## Why

`katana` の editor / preview 分割、KDV の TOC / viewer 分割、KLE の find / editor 周辺分割、Storybook の navigation / preview / inspector 分割は、どれも「2つの領域を境界線で分け、比率を変えられる UI」を必要とする。

KUC には `SplitPane` があるが、現状は layout model として存在するだけで、次を完了条件として固定できていない。

- 2 pane contract
- horizontal / vertical axis
- min / max / reset ratio
- drag resize と keyboard resize
- resize event
- persistence は consumer 責務であること
- `CollapsiblePanel` / `AppShell` との境界

この要件が弱いままだと、KDV / KLE / `katana` がそれぞれ別の splitter を作り、panel layout の操作感と検証が割れる。

## What Changes

- `SplitPane` を KUC の layout foundation molecule として要件化する。
- `SplitPaneOptions` を typed にする。
  - `axis`
  - `ratio`
  - `min_ratio`
  - `max_ratio`
  - `reset_ratio`
  - `handle_size`
  - `resize_mode`
- action を定義する。
  - `SetRatio`
  - `ResizeBy`
  - `ResetRatio`
  - `StartResize`
  - `EndResize`
- event を定義する。
  - `RatioChanged`
  - `ResizeStarted`
  - `ResizeEnded`
  - `ResizeRejected`
- application shell や viewer/editor 同期は持たない。

## Capabilities

### New Capabilities

- `kuc-split-pane-contract`: SplitPane の option、action、event、state、keyboard、Storybook、DoD を定義する。

### Modified Capabilities

- `kuc-widget-layer`: symmetric split は `SplitPane`、single sidebar collapse は `CollapsiblePanel`、app shell は consumer 責務として分離する。

## Impact

- `crates/katana-ui-core/src/layout/split_pane.rs` の public contract を強化する。
- `15-add-collapsible-sidebar-shell` の `CollapsiblePanel` と重複しない境界を固定する。
- KDV / KLE / `katana` は 2 pane layout を KUC `SplitPane` で組める。
