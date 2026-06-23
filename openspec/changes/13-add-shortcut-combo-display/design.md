# Design — ShortcutCombo + ShortcutCheatsheet

## 目的

複数キーの組み合わせ表示と早見表（cheatsheet）を KUC で標準化する。

## 採用方針

### 1. KeyCombo 型

```text
KeyCombo {
  modifiers: KeyModifiers,   // bitflags: Cmd, Ctrl, Alt, Shift, Meta
  key: KeyKind,              // KeyKind::Char('S'), KeyKind::Named(NamedKey::Enter)
}
```

NamedKey は Enter / Esc / F1..F12 / Tab / Space / Backspace / Arrow* / Home / End / PageUp / PageDown / Plus / Minus 等を列挙。

### 2. platform_display

- `Auto`: OS 検出（adapter から受信）に従う
- `MacOS`: ⌘ / ⌥ / ⌃ / ⇧ などの記号を使う
- `Windows`: `Ctrl`, `Win`, `Alt`, `Shift` の英字
- `Linux`: `Ctrl`, `Super`, `Alt`, `Shift` の英字

### 3. separator

- `Plus`: `Cmd+Shift+P`
- `Space`: `Cmd Shift P`
- `Arrow`: `Cmd → Shift → P`
- `None`: 区切りなし（macOS スタイル）

macOS は default で `None`、Windows / Linux は default で `Plus`。

### 4. ShortcutCheatsheet

- 上部に検索ボックス（query）
- 下部に groups（カテゴリ + items）
- group_layout: `Two-Column`（左カテゴリ右items）または `One-Column`（カテゴリ accordion）
- 各 item: label + ShortcutCombo

### 5. accessibility

- ShortcutCombo に accessibility_label optional
- 自動生成: 「Command + Shift + P」のようなテキスト（platform_display に応じて）

## 代替案と却下理由

| 代替 | 却下理由 |
| --- | --- |
| `KeyCap` に modifiers option を追加 | 1 キーと複数キーの責務が混ざる。preset が膨張する。 |
| 文字列 `"Cmd+Shift+P"` で渡す | typed でない。修飾キーや separator や platform 切替えが推測になりテストしづらい。 |

## Out of scope

- グローバルキー登録の OS API：consumer 責務
- accelerator マッチ：`add-toolbar-overflow-05` 側 / consumer の listener 側
- 動的レンダリングの長さ調整：consumer 責務

## 影響範囲

- `KeyCap` の責務縮小
- consumer の shortcut 表示を統一できる
