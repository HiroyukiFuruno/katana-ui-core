## Why

Card 本文の描画インデントを保ったまま、同じ stable node の viewport hit 幅を維持する公開契約が必要。Issue: https://github.com/HiroyukiFuruno/katana-ui-core/issues/85

## What Changes

- UiCommonProps に hit_outset_left と builder を追加する。
- node hit と node action hit を viewport / scroll clip より前に拡張する。
- 描画・折返し・layout / scroll measurement は既存設定を使い、hit 拡張では変えない。

## Impact

- render_model の後方互換 serde default、raster host、Storybook hit collector。
- 既存の既定値は 0。新規依存はない。
