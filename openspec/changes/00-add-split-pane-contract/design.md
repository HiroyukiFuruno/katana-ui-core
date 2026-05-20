# Design — 00-add-split-pane-contract

## 方針

`SplitPane` は、左右または上下の 2 pane を比率で分ける molecule とする。
アプリ全体の shell、sidebar の icon-only collapse、viewer/editor の同期状態は持たない。

consumer は次を自前で持つ。

- どの pane に何を置くか
- ratio の永続化
- editor-preview sync
- window / app shell layout

KUC は次を持つ。

- 2 pane model
- axis
- ratio clamp
- drag resize
- keyboard resize
- reset
- resize event
- handle rendering props

## Model

```rust
pub enum SplitPaneAxis {
    Horizontal,
    Vertical,
}

pub enum SplitPaneResizeMode {
    PointerOnly,
    KeyboardOnly,
    PointerAndKeyboard,
    Disabled,
}

pub struct SplitPaneOptions {
    pub axis: SplitPaneAxis,
    pub ratio_percent: u8,
    pub min_percent: u8,
    pub max_percent: u8,
    pub reset_percent: u8,
    pub handle_width_px: u8,
    pub resize_mode: SplitPaneResizeMode,
}
```

## Contract

- primary slots は `first` / `second` の 2 つに限定する。
- ratio は `[min_percent, max_percent]` に clamp する。
- pointer drag は `ResizeStarted` → `RatioChanged` → `ResizeEnded` の順で event を出す。
- keyboard resize は consumer が key 入力を `ResizeBy { source: Keyboard }` または `ResetRatio` へ変換し、KUC は axis / step / clamp 後の event を返す。
- persistence は `RatioChanged` を consumer が保存する。

## Boundary

| UI | KUC の扱い |
| --- | --- |
| symmetric 2 pane split | `SplitPane` |
| 折りたたみ sidebar | `CollapsiblePanel` |
| AppShell / page template | consumer |
| editor-preview scroll sync | consumer |

## Non-goals

- 3 pane 以上の dashboard layout は提供しない。
- title bar、status bar、sidebar、main content をまとめた shell は提供しない。
- ratio の保存先は提供しない。
