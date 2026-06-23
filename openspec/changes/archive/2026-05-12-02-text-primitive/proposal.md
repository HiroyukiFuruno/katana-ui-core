## Why

Adapter の生 `label()` / `text()` を直接呼ぶと、フォント役割（body / caption / heading 等）と色が呼び出し側ごとにバラバラになる。`Text` primitive を導入し、`theme/typography` の役割を引数で選ばせる形にすることで、見た目の一貫性と将来のフォント差し替えを 1 箇所で担保する。

## What Changes

- `primitive/text/` に `Text` widget を提供。
- `Text::new(content).role(TextRole::Body)` のような fluent API。`role`、`color_override`、`max_lines`、`align` を持つ。
- `TextRole` は `theme/typography` の役割と 1:1 対応（独自増設しない）。
- `view.rs` で Adapter の text view にトークンを反映。

## Capabilities

### New Capabilities

- `widget-text-primitive`: typography 役割と color トークンを統一インタフェースで適用するテキスト表示 widget。

## Impact

- composite/layout 層からは `Text` を経由してテキストを描画する規約となる。
- 02 以降の合成 widget で `Text` を直接使う / 包む形で利用する。
