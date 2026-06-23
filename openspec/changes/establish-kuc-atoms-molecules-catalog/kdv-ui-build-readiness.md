# KDV UI 構築 readiness plan

作成日: 2026-05-27

## 結論

KDV の `v0.2.0` Markdown viewer UI の周辺 UI は、現行の KUC OpenSpec 計画で構築できる。
追加で、KDV が生成した HTML / PDF / PNG / JPG 相当の RGBA preview surface を KUC `UiTree` に載せる契約は `23-add-preview-surface-image-contract` で扱う。

KUC が提供するのは atoms / molecules / layout / panel / event / state / theme / font の契約までとする。
Markdown 本文 viewer、TOC panel、hit-test metadata、画像・図形操作の意味、scroll sync、PDF page viewer は KDV 側が実装する。
KUC は KDV owned surface を opaque な `ImageSurface` として adapter へ渡す。

## 読み取り元

- `../katana-document-viewer/README.md`
- `../katana-document-viewer/docs/ui-separation-plan.md`
- `../katana-document-viewer/openspec/project.md`
- `../katana-document-viewer/openspec/changes/v0-2-0-markdown-viewer-kuc-integration/{proposal.md,design.md,tasks.md}`
- `../katana-document-viewer/openspec/changes/v0-2-0-markdown-viewer-kuc-integration/specs/markdown-viewer-kuc-integration/spec.md`
- `openspec/changes/README.md`
- `docs/inventory/katana-katana-chat-ui-kdv-kle-ui-needs.md`
- `openspec/changes/ui-core-root-plan/`
- `openspec/changes/establish-kuc-atoms-molecules-catalog/`

## KDV UI に必要なものと既存 KUC 計画

| KDV 側で画面上に必要なもの | KUC が提供する必要があるもの | 既存 KUC 計画 | 判定 |
| --- | --- | --- | --- |
| Markdown 本文を表示する大きな viewer 面 | `Panel`、`ScrollArea`、theme / font、pointer / keyboard event、`UiTree` / `UiNode`、`ImageSurface` | `ui-core-root-plan`、`establish-kuc-atoms-molecules-catalog`、`00-add-scroll-area-contract`、`23-add-preview-surface-image-contract`、`storybook-page-panel`、`storybook-page-scroll-area` | 周辺 UI は既存計画で足りる。本文描画そのものは KDV が持ち、結果だけ `ImageSurface` に載せる |
| HTML / PDF / PNG / JPG と同等の preview surface image | `UiNodeKind::ImageSurface`、`UiImageSurfaceProps`、RGBA payload、content scale、fit、accessibility label、highlight rect overlay、adapter render plan descriptor | `23-add-preview-surface-image-contract` | 新規契約が必要。KMM label fallback は不可 |
| TOC と本文の 2 ペイン構成 | `SplitPane`、`CollapsiblePanel`、`TreeView` または `List`、`ScrollArea`、virtualization | `00-add-split-pane-contract`、`15-add-collapsible-sidebar-shell`、`16-add-virtualized-list-and-tree`、`storybook-page-tree-view`、`storybook-page-list` | 既存計画で足りる。TOC item の正本は KDV/KMM が持つ |
| TOC click で viewer command を返す | selection / navigation event、state id、keyboard selection、scroll command | `00-add-scroll-area-contract`、`16-add-virtualized-list-and-tree`、`establish-kuc-atoms-molecules-catalog/specs/kuc-widget-layer` | 既存計画で足りる。editor scroll の副作用は KatanA / KDV host 側 |
| rendered node の hover highlight と選択 | pointer / focus event、theme token、`HoverCard`、`Popover`、placement engine | `04-add-rich-popover-and-hover-card`、`01-add-context-menu`、`establish-kuc-atoms-molecules-catalog/core-foundation-contract.md` | 既存計画で足りる。node id / source range / rect mapping は KDV 側 |
| 画像・図形の open / copy / fit 入口 | `Toolbar`、icon / button、`HoverCard`、`Popover`、`ContextMenu`、placement engine | `05-add-toolbar-overflow`、`04-add-rich-popover-and-hover-card`、`01-add-context-menu`、`storybook-page-toolbar` | 既存計画で足りる。画像・図形の artifact と command 意味は KDV 側 |
| unresolved metadata の警告・詳細表示 | `Banner`、`DiagnosticsList`、`EmptyState`、`Tooltip` / `Popover`、theme | `10-add-inline-banner-alert`、`08-add-diagnostics-list`、`09-add-empty-state`、`04-add-rich-popover-and-hover-card` | 既存計画で足りる。文言と診断内容は KDV が渡す |
| viewer 検索 UI | `SearchControlStrip`、検索結果 row、list / virtualization、keyboard navigation | `22-add-search-control-strip`、`21-add-command-launcher-results`、`16-add-virtualized-list-and-tree` | 既存計画で足りる。検索実行と match range は KDV 側 |
| export / PDF 事前確認 viewer の周辺 UI | `Toolbar`、`StatusBar`、`ProgressBar`、`ScrollArea`、`SearchControlStrip` | `05-add-toolbar-overflow`、`12-add-multi-segment-status-bar`、`00-add-scroll-area-contract`、`22-add-search-control-strip` | 既存計画で足りる。PDF page model と export pipeline は KDV 側 |
| viewer 設定と interaction config | `SettingsList`、`FormField`、`Toggle`、`Checkbox`、`Radio`、typed action / event / state | `14-add-sectioned-settings-form`、`storybook-page-form-field`、`storybook-page-toggle`、`storybook-page-checkbox`、`storybook-page-radio` | 既存計画で足りる。`ViewerInteractionConfig` は KDV が定義する |

## KDV viewer 本文を KUC に入れない理由

- `MarkdownViewer`、`DocumentPreview`、`TocPanel`、`ImageDiagramControls`、`PdfPageViewer` は KDV の利用側 organism / page であり、KUC public API に入れない方針と一致しない。
- KDV が不足として挙げている `ScrollArea`、`SplitPane`、`SearchControlStrip` は既に KUC change として存在する。
- hover / media 操作 / diagnostics / settings は、既存の `HoverCard`、`Popover`、`Toolbar`、`DiagnosticsList`、`Banner`、`SettingsList` を組み合わせれば domain-free に表現できる。
- KDV の hit-test metadata、KMM node id、source range、artifact、diagnostics、viewer command は KUC ではなく KDV の責務である。
- `23-add-preview-surface-image-contract` は本文 viewer を KUC に移す change ではなく、KDV が描画済みの opaque surface と highlight rect を adapter に渡すための契約である。

## KDV UI 実装時の順序

1. `katana-document-viewer-kuc` で KUC の `ThemeSnapshot`、font role、`UiTree` / `UiNode`、event / state を受ける境界を作る。
2. 本文 viewer を KDV owned RGBA surface として描画し、`ImageSurface` node で KUC `UiTree` に載せる。
3. TOC は KMM AST 由来の heading list から作り、KUC `TreeView` または `List` を使って `SplitPane` / `CollapsiblePanel` に載せる。
4. hover / selection / media controls は KDV の hit-test 結果を入力にして、KUC `HoverCard` / `Popover` / `Toolbar` / `ContextMenu` で表示する。
5. unresolved metadata は KDV diagnostics を KUC `Banner` / `DiagnosticsList` / `Popover` へ渡す。
6. viewer search と export preview は、KUC `SearchControlStrip` / `StatusBar` / `Toolbar` を組み合わせ、検索・export の実処理は KDV に残す。

## KDV 着手前に確認する KUC 側 gate

- `ui-core-root-plan` と `establish-kuc-atoms-molecules-catalog` の OpenSpec validation が通る。
- `23-add-preview-surface-image-contract` の OpenSpec validation と image surface contract tests が通る。
- `storybook-page-*` leaf change の harness DoD が current tasks と矛盾していない。
- `establish-kuc-atoms-molecules-catalog` の 6.8 は、KDV 向け新機能ではなく Storybook harness guard の拡張であり、`window_interaction` の required page 接続検査まで完了済みである。
- KUC core が `katana-document-viewer` など domain crate へ依存しないことを guard で確認する。
