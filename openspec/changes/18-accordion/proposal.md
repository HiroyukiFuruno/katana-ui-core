## Why

設定パネル / FAQ / セクション分割で「ヘッダクリックで本体を折り畳む」UI が要る。`../katana/crates/katana-ui/src/widgets/accordion/` の役割を Floem に移植。アニメーション付き高さ可変を素直な API で提供する。

## What Changes

- `layout/accordion/` に `Accordion` widget を提供。
- props: `header: View`（クロージャ）、`expanded: bool`、`on_toggle: Fn(bool)`、`disabled`、`indicator`（chevron icon の表示位置 `Leading` / `Trailing` / `None`）、`children: View`（クロージャ）。
- 高さアニメーションは `theme/spacing` の固定 duration トークンを参照。
- 単一の Accordion のみを対象（複数項目を排他的に開閉する `AccordionGroup` は YAGNI、必要時に別 change）。

## Capabilities

### New Capabilities

- `widget-accordion`: 単一セクションの折り畳みコンテナ。chevron 配置と展開アニメーションを統一。

## Impact

- 設定 UI、ヘルプセクション、リスト項目の詳細展開などで利用。
