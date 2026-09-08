## Why

公開済み `katana-ui-core` v0.3.7 では、KDV の registry-only canonical crop が 91/95 に留まり、四辺別 grid border の下流受入も未完了である。さらに、consumer-defined full-editor artifact の公開契約、HiDPI の text layout、毎フレーム同期する scenario session の検索 Close が未解決で、consumer が独自補正や状態管理を持たずに KUC を利用できない。

これらを個別の公開版へ分割すると、同じ public raster host、artifact、session 契約を下流が繰り返し更新することになる。v0.3.8 で全5 Issueの互換性を保った解決を一つの release line に集約する。

## What Changes

- 公開 raster host の document typography、計測、描画、hit-test を同一の logical/physical metrics に固定し、KDV/KatanA canonical crop 95/95 と registry-only consumer contract を回復する。
- 四辺別 grid border API の互換性を保持し、KDV の registry-only thin projection で consumer acceptance を検証可能にする。
- KUC 所有の physical stage、opaque receipt、AccessKit、artifact を出力する versioned consumer-defined full-editor artifact plan を公開する。
- TextSurface gutter と Diagnostics の HiDPI 計測、描画、wrap/clip の scale 変換を一致させる。
- scenario session が search の close/reopen、options、replace mode、result position を毎フレームの projection に保持するようにする。
- `katana-ui-core` の公開版を一度だけ v0.3.8 に更新し、統合品質・release gate、Draft PR review、公開後の registry consumer verification を同じ release line で行う。

## Capabilities

### New Capabilities

- `consumer-defined-full-editor-artifact-plan`: consumer-defined generic leaf を KUC-owned physical stage と opaque evidence へ写像する公開契約。
- `hidpi-text-raster-layout`: text raster の physical texture と logical layout bounds を一貫して扱う計測・描画契約。
- `scenario-session-search-state`: synchronized scenario session の検索表示と操作状態を保持する契約。
- `registry-raster-host-consumer-acceptance`: framework-neutral raster host と四辺別 grid border を registry-only consumer が採用する受入契約。

### Modified Capabilities

- なし。既存の main spec には raster host、artifact、scenario session の requirement spec がないため、本変更で明示的な公開契約を追加する。

## Impact

- `crates/katana-ui-core/src/raster_host/`、`render_model`、text raster、TextSurface、Diagnostics、text command surface の公開 API と実装。
- `katana-ui-core` の `raster-host` / `egui` feature consumer、private Storybook、KDV および KLE の registry-only adoption。
- OpenSpec の既存 #35/#37 handoff と、KUC #40/#43/#44 の requirements、回帰テスト、v0.3.8 release evidence。
