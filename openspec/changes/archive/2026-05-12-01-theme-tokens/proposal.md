## Why

01 以降の widget はすべて **色 / 間隔 / typography** を直書きせず、`theme/` のトークン経由で参照することで、後から light/dark 切替やブランド差し替えを 1 箇所で行えるようにしたい。トークンが存在しないと各 widget が独自の数値を抱え込み、後から統一できなくなる（典型的な AI Slop の蓄積）。

参照元: `../katana/crates/katana-ui/src/theme_bridge/` の役割定義（egui 依存実装はそのまま使えないため、Floem 向けの最小トークンに再構成する）。

## What Changes

- `theme/color/` — 意味的トークン（`bg`, `surface`, `text`, `text_muted`, `border`, `accent`, `accent_muted`, `danger`, `warning`, `success` など）を提供。生の RGB は `theme/color/palette.rs` 内に閉じ込め、外部からは意味名でのみ参照させる。
- `theme/spacing/` — `xxs`, `xs`, `sm`, `md`, `lg`, `xl`, `xxl` の固定スケール。
- `theme/typography/` — `body`, `body_strong`, `caption`, `code`, `heading_*` のフォント役割。`font-family` / `font-size` / `line-height` / `weight` を組で持つ。
- `Theme` 構造体で全トークンをまとめ、`Theme::default_light()` / `Theme::default_dark()` を提供。Floem の reactive コンテキストに 1 つ流す。
- 数値の hard-cord 禁止を `kuw-workflow-guide` skill に追記。違反は ast-lint または CI lint で検出する仕組みを将来 (24 以降) で検討。

## Capabilities

### New Capabilities

- `widget-theme-tokens`: 意味的トークン（color / spacing / typography）と Theme 構造体、light/dark の既定値、Floem コンテキストへの注入手順。

### Modified Capabilities

- なし。

## Impact

- `crates/katana-ui-widget/src/theme/` 配下に実装が追加される。
- 02 以降の全 widget は `theme` を参照するため、本 change が前提となる。
- Storybook には light / dark 切替トグルが組み込まれ、各 widget ページがテーマ切替に追従できることを目視確認できる状態にする。
