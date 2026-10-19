## Why

Card の固定16pxインデントを利用側で指定できず、本文の原点と幅を両立できない（Issue #83）。既定の挙動を維持して、Cardごとの明示的設定を提供する。

## What Changes

- UiCommonProps に省略可能な child_indent と builder を追加する。
- raster host と Storybook の描画・測定・hit/scroll へ一貫して適用する。
- v0.4.2 として公開し、旧JSON互換と数値契約を検証する。

## Capabilities

### New Capabilities

- `card-child-indent`: Card の子原点の公開設定と互換性。

### Modified Capabilities

なし。

## Impact

public render model、core raster host、Storybook。KUC 内で完結し、外部consumerの編集・採用を完了条件にしない。
