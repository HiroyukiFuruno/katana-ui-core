## Why

メニュー項目 / コマンドパレット / ヘルプダイアログなどでキーボードショートカットを表示する小要素。`../katana/crates/katana-ui/src/widgets/toggle/key_cap.rs` の役割（キー名 1 つを「キートップ風」に描く）を Adapter に移植する。OS による修飾キー記号（macOS: `⌘ ⌥ ⌃ ⇧` / others: `Ctrl Alt Shift`）の表記分岐を内部で吸収する。

## What Changes

- `composite/indicator/key_cap/` に `KeyCap` / `KeyCombo` widget を提供。
- `KeyCap`: 単一キーの矩形表示。props: `key: KeyLabel`、`size`、`tone`（`Neutral` / `Subtle`）。
- `KeyCombo`: `Vec<KeyLabel>` を `+` または OS 慣習区切りで並べる。
- `KeyLabel` は enum で modifier を表現（`Cmd`, `Ctrl`, `Shift`, `Alt`, `Option`, `Super`, `Char(char)`, `Named(NamedKey)` 等）。OS 判定は `cfg!(target_os = "macos")` で内部分岐。

## Capabilities

### New Capabilities

- `widget-key-cap`: キーボードショートカット表示。OS による修飾キー表記の差異を内部吸収。

## Impact

- ヘルプダイアログ / メニュー / コマンドパレット候補の右端表示で利用。
