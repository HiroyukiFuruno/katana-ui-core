# Tasks — 35-add-core-raster-host-boundary

Issue: <https://github.com/HiroyukiFuruno/katana-ui-core/issues/35>

## 1. Public core boundary

- [x] 1.1 `katana-ui-core` に `raster-host` feature と framework-neutral な module export を追加する。
- [x] 1.2 private Storybook の canvas、presentation、UI tree renderer、hit-test の中立部分を public core へ移動する。
- [x] 1.3 private Storybook wrapper が public host を再利用し、interactive GUI adapter だけを所有するようにする。

## 2. Contracts

- [x] 2.1 `raster-host` feature 単独の compile contract を追加する。
- [x] 2.2 dependency graph に `egui`、`eframe`、`winit` がないことを fail-closed で検査する。
- [x] 2.3 public host と private wrapper の raster / hit-test parity test を追加する。

## 3. Verification and release handoff

- [/] 3.1 KUC format、lint、focused / workspace test、release gate を通す。ローカル format / raster-host contract / full strict coverage は通過済みで、release gate は release/v0.3.4 PR の最新 HEAD で実行待ち。
- [/] 3.2 KUC v0.3.4 を GitHub Release と crates.io へ公開し、registry artifact を確認する。release/v0.3.4 の統合・公開 workflow 待ち。
- [/] 3.3 KDV が exact public KUC v0.3.4 の `raster-host` を採用し、path/git override なしで boundary gate と KatanA canonical crop 95/95 を通す。公開 artifact 後の KDV/KatanA consumer gate 待ち。
