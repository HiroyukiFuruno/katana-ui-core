# Design — 00-add-scroll-area-contract

## 方針

`ScrollArea` は、画面全体の viewer / editor を実装する部品ではない。
KUC は、子要素を持つ scroll container と、その scroll state / command / event contract を提供する。

consumer は次を自前で持つ。

- viewer / editor 本文の scroll policy
- editor-preview 同期
- active heading 計算
- document node hit-test
- storage への scroll position 保存

KUC は次を持つ。

- scroll axis
- offset
- viewport / content extent
- scrollbar visibility / placement
- scroll actions
- scroll events
- nested state identity
- keyboard scroll mapping

## Model

```rust
pub enum ScrollAxis {
    Vertical,
    Horizontal,
    Both,
}

pub enum ScrollbarVisibility {
    Auto,
    Always,
    Hidden,
}

pub enum ScrollbarPlacement {
    Reserved,
    Overlay,
}

pub struct ScrollAreaOptions {
    pub axis: ScrollAxis,
    pub offset_x: f32,
    pub offset_y: f32,
    pub viewport_width: f32,
    pub viewport_height: f32,
    pub content_width: f32,
    pub content_height: f32,
    pub scrollbar_visibility: ScrollbarVisibility,
    pub scrollbar_placement: ScrollbarPlacement,
    pub edge_threshold: f32,
}
```

## Actions / Events

- `ScrollTo { x, y }`: 指定 offset へ移動する。
- `ScrollBy { dx, dy }`: 現在 offset から相対移動する。
- `ScrollIntoView { target_rect }`: 子要素の矩形が見えるように移動する。
- `SetScrollbarVisibility { visibility }`: scrollbar 表示方針を変更する。
- `Scrolled { x, y }`: scroll 後の offset を通知する。
- `ScrollEdgeReached { edge }`: top / right / bottom / left へ到達したことを通知する。
- `ScrollCommandRejected { reason }`: axis 不一致や範囲外 command を通知する。

## Keyboard

- PageUp / PageDown は viewport 高さ基準で移動する。
- Home / End は axis に応じて先頭 / 末尾へ移動する。
- horizontal scroll が有効な場合、Shift + wheel または adapter が渡す horizontal delta を受け取れる。

## Non-goals

- KDV viewer / KLE editor の本文 scroll 実装は持たない。
- editor-preview scroll sync は持たない。
- scroll position の永続化は持たない。
- OS / framework 固有の scrollbar 描画 API は adapter が持つ。
