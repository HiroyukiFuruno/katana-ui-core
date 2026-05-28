# Tasks — 23-add-preview-surface-image-contract

## 1. 設計確定

- [x] 1.1 KDV viewer 本文は KDV 所有の RGBA surface とし、KUC は Markdown / KMM を解釈しない境界を `design.md` に固定する。
- [x] 1.2 preview surface image の props、fit、content scale、accessibility label、highlight rect を確定する。
- [x] 1.3 egui / floem / gpui adapter は同じ descriptor を受け取るだけにし、framework 型を KUC core へ入れない。

## 2. 中核実装

- [x] 2.1 `UiNodeKind::ImageSurface` を追加する。
- [x] 2.2 `UiImageSurfaceProps`、`UiImageSurfaceFit`、`UiImageSurfaceHighlight`、`UiImageSurfaceValidationError` を追加する。
- [x] 2.3 `UiProps` と `UiNode` builder に image surface props を追加する。
- [x] 2.4 `atom::ImageSurface` から RGBA surface node を作れるようにする。
- [x] 2.5 `UiImageSurfaceRenderPlan` を追加し、adapter plan が surface descriptor を受け取れるようにする。

## 3. 自動テスト

- [x] 3.1 RGBA payload length が `width * height * 4` と一致しない場合に fail fast する contract test を追加する。
- [x] 3.2 `ImageSurface` atom と `UiNode` builder が surface props と highlight rect を保持する contract test を追加する。
- [x] 3.3 egui adapter が image surface descriptor と highlight rect を受け取る test を追加する。
- [x] 3.4 floem adapter が image surface descriptor と highlight rect を受け取る test を追加する。
- [x] 3.5 gpui adapter が image surface descriptor と highlight rect を受け取る test を追加する。

## 4. ドキュメント

- [x] 4.1 `openspec/changes/README.md` の KDV readiness 記述を issue #1 の新前提に合わせる。
- [x] 4.2 `kdv-ui-build-readiness.md` に preview surface image primitive を追加する。
- [x] 4.3 `docs/inventory/katana-katana-chat-ui-kdv-kle-ui-needs.md` に ImageSurface gap を追加する。

## 5. 品質ゲート / DoD

- [x] 5.1 `cargo test -p katana-ui-core image_surface --locked` をパスする。
- [x] 5.2 `cargo test -p katana-ui-core-egui image_surface --locked` をパスする。
- [x] 5.3 `cargo test -p katana-ui-core-floem image_surface --locked` をパスする。
- [x] 5.4 `cargo test -p katana-ui-core-gpui image_surface --locked` をパスする。
