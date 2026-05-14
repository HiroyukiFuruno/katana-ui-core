## Why

`SvgButton` を含む icon-only 要素には文脈説明が必要で、hover/focus 時に小さな注釈を浮かせる Tooltip が定型として要る。各 widget が独自実装すると遅延 / 位置 / スタイルがバラつくため、汎用 widget として固定する。

## What Changes

- `composite/indicator/tooltip/` に `Tooltip` widget（ラッパ関数）を提供。
- API: `tooltip(label, placement, anchor_view)` のように、対象 view を受けて装飾された view を返す。
- props: `label: String`、`placement`（`Top` / `Bottom` / `Start` / `End`、既定 Top）、`delay_ms`（既定 400）、`max_width`。
- 表示は内部で軽量 popup（layout/popover に依存しない最小実装、画面端での反転は実装する）。21 完了後にリファクタする方針は select-box と同様。
- a11y: hover だけでなく focus 時にも表示。

## Capabilities

### New Capabilities

- `widget-tooltip`: hover/focus トリガで注釈を表示する装飾 widget。

## Impact

- 主に icon-only 要素（SvgButton）と組み合わせて利用される。
