## Why

SVG ボタンや indicator 系 widget はアイコン表示を必要とする。`../katana/crates/katana-ui/src/icon/` には adapter 向けの大きなレジストリ + ローダ機構があるが、KUW では **registry を持たない最小の Icon primitive**（呼び出し側が SVG bytes / 静的データを渡す形）として始める。registry の必要性が見えた段階で別 change で拡張する（YAGNI）。

## What Changes

- `primitive/icon/` に `Icon` widget を提供。
- 入力は `IconSource`（`SvgBytes(&'static [u8])` / `SvgString(String)`）と `size`（pt or token）と `color`（theme color トークン or 上書き）。
- 内部で SVG をパースして Adapter の描画 API に流す（resvg ベースの最小実装、katana-ui の `svg_loader` を Adapter 向けに書き直し）。
- アイコンセットの提供は本 change の対象外。`Icon::new(IconSource::SvgBytes(MY_ICON))` のように利用側がデータを持ち込む。

## Capabilities

### New Capabilities

- `widget-icon-primitive`: SVG bytes / string を入力に取り、theme の色とサイズトークンを適用して描画する最小アイコン widget。

## Impact

- 05〜16 の composite で `Icon` を組み合わせて使う。
- 将来の icon registry / icon pack 機能は別 change（24+）として切り出す。
