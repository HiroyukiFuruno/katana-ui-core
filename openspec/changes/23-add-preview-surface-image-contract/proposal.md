## Why

KDV v0.2.0 は、HTML / PDF / PNG / JPG と同等の preview surface image を viewer 本文の主描画結果として扱う。
KDV 側では Markdown / table / code / math / diagram を RGBA surface に描画できるが、現行 KUC の `UiTree` / `PaintRequest` にはその surface を中立に載せる `UiNodeKind` と props がない。

このままだと KDV は KMM node label 列へ fallback するか、`MissingCapability(PreviewSurfaceImage)` で止まるしかない。
KUC は Markdown viewer 本文そのものを所有しないが、consumer が作った opaque な RGBA preview surface を adapter へ同じ意味で渡す契約は必要である。

## What Changes

- `UiNodeKind::ImageSurface` を KUC render model に追加する。
- `UiImageSurfaceProps` を追加し、fingerprint、width、height、RGBA payload、content scale、fit、accessibility label を持たせる。
- viewer 検索 hit highlight を surface 上の rect overlay として渡す `UiImageSurfaceHighlight` を追加する。
- `UiNode` builder と `atom::ImageSurface` から image surface node を構築できるようにする。
- egui / floem / gpui adapter plan が image surface descriptor と highlight rect を受け取る契約テストを追加する。

## Capabilities

### New Capabilities

- `kuc-preview-surface-image-contract`: KDV owned preview surface image を KUC の中立 render model と adapter plan に載せる。

### Modified Capabilities

- `kuc-widget-layer`: KDV の本文 viewer は KDV 所有のまま、KUC は opaque surface image primitive と overlay rect 契約だけを提供する。

## Impact

- `crates/katana-ui-core/src/render_model/` に image surface props と render plan descriptor を追加する。
- `crates/katana-ui-core/src/atom/` に `ImageSurface` atom を追加する。
- `katana-ui-core-egui`、`katana-ui-core-floem`、`katana-ui-core-gpui` の surface bridge / render plan に image surface descriptor を追加する。
- KUC core は `katana-document-viewer`、KMM、Markdown AST、export pipeline には依存しない。
