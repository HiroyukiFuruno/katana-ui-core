# Design — 35-add-core-raster-host-boundary

## Ownership

KUC は generic UI tree の rasterization、presentation、座標変換、hit-test を所有する。
KDV は document model、preview scene、surface input を所有するが、KUC の描画実装を複製しない。

## Public feature

```text
raster-host = [text-raster, svg-raster, dep:image]
```

`raster-host` は core-only feature であり、`egui`、`eframe`、`winit` を feature dependency に含めない。

## Public API

`katana_ui_core::raster_host` は少なくとも次を公開する。

- raster canvas と RGBA blit request
- `UiTreeStorybookHost` / `UiTreeCanvasRenderer`
- presentation と render area
- UI node に対する hit rect、action hit、interaction target

public host は `UiNode` を同じ render-plan へ変換し、private Storybook wrapper はその host に interactive adapter を重ねるだけにする。

## Compatibility

既存 private Storybook の利用箇所は、移動した型を re-export して source compatibility を保つ。
KDV は KUC の公開 release から同じ型を直接 import する。

## Verification strategy

1. `raster-host` 単独で compile し、dependency tree に GUI runtime がないことを検査する。
2. 同一 `UiNode` の public host と private wrapper で raster / hit-test outcome が一致する unit test を持つ。
3. private Storybook の既存 test / compile を通す。
4. KUC 公開後、KDV を registry-only KUC に切り替え、boundary gate と KatanA canonical crop 95/95 を通す。
