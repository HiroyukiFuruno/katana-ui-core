## Why

設定パネル / FAQ / セクション分割で「ヘッダクリックで本体を折り畳む」UI が要る。
root plan 適用後は repo 外の実装を直接読まず、必要な挙動を `docs/inventory/accordion.md` にコピーしてから KUC の中立 model と adapter 経由で実装する。
`docs/inventory/accordion.md` が未作成の間は、この change 単独で実装を開始しない。

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
