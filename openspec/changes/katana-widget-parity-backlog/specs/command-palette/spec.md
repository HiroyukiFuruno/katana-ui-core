# CommandPalette Widget Spec

## 概要

検索入力 + フィルタ可能な結果リスト + キーボードナビゲーションを持つオーバーレイ widget。

## 出典

- `../katana/crates/katana-ui/src/views/modals/command_palette.rs`

## 階層配置

`layout/command_palette`

## 依存

- Modal (20)
- TextInput (12)

## API 概要（TBD）

- `PaletteResult`: label, icon (Option), shortcut (Option), score, payload
- `CommandPalette`: on_search, on_select, on_dismiss, results, keyboard_nav (↑↓ Enter Esc)
- Provider trait で検索ロジックを外部注入
