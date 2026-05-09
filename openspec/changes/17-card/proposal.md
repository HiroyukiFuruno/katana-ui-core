## Why

「枠 + padding + 角丸 + 影」の最小コンテナはダッシュボード / リスト項目 / フォームのまとまり等で頻出。各所で `container().style(...)` を直書きするとスタイルがバラつくため、`Card` widget として固定する。

## What Changes

- `layout/card/` に `Card` widget を提供。
- props: `variant`（`Plain`, `Elevated`, `Outlined`）、`padding`（theme spacing トークンの選択）、`interactive: bool`（hover/active 装飾を有効化）、`on_click: Option<Fn()>`、`children: View`。
- `interactive=true` かつ `on_click` 指定時は a11y 上 button として振舞う。

## Capabilities

### New Capabilities

- `widget-card`: 枠 + padding + 角丸の最小コンテナ。3 variant とインタラクティブ化を統一。

## Impact

- 18 (accordion) / 19 (split) / 20 (modal) のヘッダ・本体パネル等で内部利用される想定（必要に応じて）。
