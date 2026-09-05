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

- [x] 3.1 KUC v0.3.7用に、本文・H1〜H3のfont size、line height、baseline位置を独立指定するframework-neutralなtyped APIを追加する。無効値はtheme由来metricsへfail closedする。
- [x] 3.2 public canvas / Storybook / surface host、raster draw、scroll/layout測定、node/action hitが同一final typographyを使うようにする。raster幅とnode hitのline-boxを同時に検証する回帰を追加する。
- [x] 3.3 KUC v0.3.7のformat、lint、raster-host contract、focused / workspace test、release gateを通す。review修正後のdocument accordion open/closed typography回帰を含むLinux strict coverageはfunctions 14,035/14,035・lines 148,054/148,054、`rtk proxy just VERSION=v0.3.7 release-check`のpackage verify、publish dry-run、single-public-crate scope、未公開確認まで通過済み。
- [/] 3.4 KUC v0.3.7をGitHub Releaseとcrates.ioへ公開し、registry artifactを確認する。release/v0.3.7のDraft PR、review、required CI、merge、publish workflowが必要。
- [/] 3.5 KDVがexact public KUC v0.3.7の`raster-host`を採用し、path/git overrideなしでboundary gateとKatanA canonical crop 95/95を通す。公開artifact後のKDV/KatanA consumer gateが必要。
