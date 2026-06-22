# Design — 23-add-preview-surface-image-contract

## 方針

KUC は viewer 本文を解釈しない。
KDV が生成した RGBA surface を `ImageSurface` node として `UiTree` に載せ、adapter が texture 化できるだけの中立 descriptor を受け取る。

## Surface props

`UiImageSurfaceProps` は次を持つ。

- `fingerprint`: 同じ surface を識別する consumer 由来の fingerprint。
- `width` / `height`: RGBA payload の pixel extent。
- `rgba`: adapter が texture 化する中立 payload。
- `content_scale`: consumer が渡す scale。初期値は 100。
- `fit`: `Original` / `Contain` / `Cover` / `Stretch`。
- `accessibility_label`: screen reader と fallback 表示用の label。
- `highlight_rects`: surface 座標系の overlay rect。

RGBA payload は `width * height * 4` bytes と一致しない場合に `UiImageSurfaceValidationError` で fail fast する。

## Overlay

検索 hit は KDV viewer state 由来なので、KUC は検索計算をしない。
KUC は `UiImageSurfaceHighlight` で `rect`、`current`、`label` だけを保持し、adapter へ同じ descriptor として渡す。

## Adapter boundary

external renderer は `UiImageSurfaceRenderPlan` を受け取る。
この plan は RGBA bytes 本体を複製せず、fingerprint、extent、byte length、fit、accessibility label、highlight rect を持つ。
実 runtime が texture 化するか、plan に保持するかは external renderer の責務である。

## 非対象

- Markdown display-list / block model。
- KMM node id、source range、hit-test metadata。
- PDF page model、export pipeline。
- KDV viewer runtime、TOC、検索 engine、scroll sync。
