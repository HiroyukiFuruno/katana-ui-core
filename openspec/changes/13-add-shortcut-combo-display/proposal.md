## Why

`katana` の `widgets/shortcut.rs`、command palette のショートカット表示、各 toolbar action のキー表示、`katana-chat-ui` の slash launcher エントリ説明など、KeyCap を「組み合わせ（Combo）」で並べる需要が広く存在する。`KeyCap` atom は単独のキー表示だけで、`Cmd+Shift+P` のようなコンボ表示と「修飾キー+主キーの並び」「区切り（+ / 　／ 矢印）」「macOS と Windows / Linux の platform 別表示」を typed に表現しない。

加えて「キーボードショートカット早見表（cheatsheet）」widget が欲しいケースが繰り返し発生する（command palette、settings、`?` ホットキー表示）。

## What Changes

- `widget::atoms` に `ShortcutCombo` atom を追加する:
  - option:
    - `combo: KeyCombo`（modifiers + key）
    - `separator: Plus | Space | Arrow | None`
    - `platform_display: Auto | MacOS | Windows | Linux`
    - `size: Compact | Default | Large`
    - `tone: Neutral | Muted | Accent`
  - action: none
  - event: none
  - state: resolved cap sequence、`UiStateId`
- `widget::molecules` に `ShortcutCheatsheet` molecule を追加する:
  - option: groups（カテゴリ ごとの shortcut 集合）, query（フィルタ）, group_layout（Two-Column | One-Column）
  - action: SetQuery / SelectShortcut
  - event: ShortcutSelected / QueryChanged
- `KeyCap` atom は 1 つのキー表示の責務を維持し、複数キー / コンボは `ShortcutCombo` に分離する。

## Capabilities

### New Capabilities

- `kuc-shortcut-combo-atom`: ShortcutCombo atom の完了条件を定義する。
- `kuc-shortcut-cheatsheet`: ShortcutCheatsheet molecule の完了条件を定義する。

### Modified Capabilities

- `kuc-widget-layer`: `KeyCap`（1 キー）と `ShortcutCombo`（コンボ）と `ShortcutCheatsheet`（一覧）の責務境界を明記する。

## Impact

- `crates/katana-ui-core/src/atom/shortcut_combo.rs` 新設。
- `crates/katana-ui-core/src/molecule/shortcut_cheatsheet.rs` 新設。
- `KeyCap` ページに「複数キーは ShortcutCombo」リンクを追加。
- consumer (`katana` shortcut display、command palette、toolbar) は新 atom で統一可能になる。
