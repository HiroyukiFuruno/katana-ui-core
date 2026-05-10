# TreeView Widget Spec

## 概要

階層データの展開・折り畳み・選択・アクティブ表示を扱う汎用ツリー widget。

## 出典

- `../katana/crates/katana-ui/src/views/panels/explorer/` (ファイルツリー)
- `../katana/crates/katana-ui/src/views/panels/toc/render.rs` (目次ツリー)
- `../katana/crates/katana-ui/src/settings/settings_tree.rs` (設定ツリー)

## 階層配置

`layout/tree`

## 依存

- Accordion (18) — 折り畳みパターンの上位構造
- Icon (03) — item icon 表示

## API 概要（TBD）

- `TreeItem`: label, icon (Option), indent_level, expanded, active, disabled
- `TreeView`: items, on_select, on_expand, on_collapse, virtual_scroll, show_indent_lines
