## Why

KDV Issue #48 の XLSX frame は、セルごとの top/right/bottom/left の style と color を保持している。一方で公開 `katana-ui-core` の `UiGridCellAppearance` には一様な罫線以外を表す型と raster host の描画契約がなく、consumer は fidelity を落とすか owner boundary を越えて独自描画するしかない。

この差分を公開 core に追加し、KDV が registry artifact だけを使う thin projection でセル罫線を再現できるようにする。

## What Changes

- `UiGridCellAppearance` に四辺それぞれの visibility、style、color を保持する公開型を追加する。
- 既存の未指定・一様罫線相当の入力との後方互換性を維持する。
- `raster-host` がセル矩形ごとに四辺を個別に描画し、色と線種の違いを反映する。
- KDV の XLSX frame を用いる consumer が公開 API 経由で描画できることを検証する。

## Capabilities

### New Capabilities

- `grid-cell-per-side-border`: 公開 grid cell の四辺別 border モデルと raster host 描画契約。

### Modified Capabilities

- なし。

## Impact

- `crates/katana-ui-core/src/render_model/typed_grid_types.rs` の公開 serializable API。
- `crates/katana-ui-core/src/raster_host/` の grid cell 描画経路。
- KDV の registry-only consumer projection と XLSX fidelity 回帰。
- KUC `release/v0.3.4` に統合し、追加の release branch は作らない。
