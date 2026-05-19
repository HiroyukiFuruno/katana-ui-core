# Design — 21-add-command-launcher-results

## 方針

画面全体の command palette を KUC に入れない。
KUC は、入力欄と結果 row を結びつける molecule contract だけを提供する。

consumer は次を自前で持つ。

- command registry
- provider selection
- 実行権限
- domain action
- modal / side panel / slash launcher の配置

KUC は次を持つ。

- query
- result row model
- highlighted row
- keyboard action
- result execution event
- shortcut 表示
- disabled reason 表示
- virtualization option

## Model

```rust
pub struct CommandResultRow {
    pub id: String,
    pub label: String,
    pub secondary_label: Option<String>,
    pub icon: Option<String>,
    pub shortcut: Option<ShortcutCombo>,
    pub provider_id: Option<String>,
    pub group_id: Option<String>,
    pub disabled: bool,
    pub disabled_reason: Option<String>,
}

pub struct CommandLauncher {
    pub query: String,
    pub rows: Vec<CommandResultRow>,
    pub highlighted_index: Option<usize>,
    pub virtualization: Option<VirtualizationConfig>,
}
```

## Keyboard

- Arrow Up / Down: highlighted row を移動する。
- Home / End: 先頭 / 末尾へ移動する。
- Enter: highlighted row を実行 event として consumer へ返す。
- Esc: close event を consumer へ返す。
- disabled row は highlight できるが execute はできない。理由表示は `disabled_reason` で行う。

## Variants

| variant | 用途 | KUC の責務 |
| --- | --- | --- |
| `ModalPalette` | 中央に開く command palette | row / query / keyboard contract だけ |
| `InlineSearchResults` | search modal の結果 pane | row / selection / virtualization |
| `SlashLauncher` | composer 直下の小型 launcher | row / query / keyboard contract だけ |

配置、背景 overlay、modal wrapper は consumer が選ぶ。

## Non-goals

- command registry は実装しない。
- workspace / file / chat / editor command の意味は持たない。
- modal window や app-level launcher template は提供しない。
- KLE / KDV の command provider は KUC に入れない。
