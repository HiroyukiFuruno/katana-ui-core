## Why

3〜5択の排他選択（表示モード / レイアウト切替 / フィルタ）には、Tab とは別の「セグメント化トグル」が適切。`../katana/crates/katana-ui/src/widgets/toggle/segmented_toggle.rs` の adapter 実装を Adapter 向けに再構成する。

## What Changes

- `composite/selector/segmented/` に `SegmentedToggle<K>` widget を提供（`K` は `Eq + Clone`）。
- props: `value: K`、`options: Vec<(K, Segment)>`（`Segment` は `Label(String)` または `Icon(IconSource, String)` の片方）、`on_change: Fn(K)`、`size`、`disabled`、`a11y_label`。
- 選択中セグメントは theme accent 背景。非選択は surface。

## Capabilities

### New Capabilities

- `widget-segmented-toggle`: ジェネリック K で排他選択を扱う水平セグメントトグル。

## Impact

- 表示モード切替（list / grid）、ソート方向切替、フィルタ等で使用。
