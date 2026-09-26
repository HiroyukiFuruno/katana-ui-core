# 変更履歴

このプロジェクトの注目すべき変更はすべてこのファイルに記録されます。

形式は [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) に基づき、
バージョニングは [Semantic Versioning](https://semver.org/spec/v2.0.0.html) に準拠します。

<!-- next-header -->

## [Unreleased]

## [0.3.16] - 2026-09-26

### 変更

- Storybook の development dependency を `katana-markdown-model` 0.2.3 へ更新し、互換性のある lockfile 解決を更新した。

## [0.3.15] - 2026-09-24

- corrective release。release evidence は `docs/release/v0.3.15.md` に記録した。

### 修正

- viewport clipping 前の文書座標 interaction hit 収集を復旧し、深くスクロールした accordion action が到達可能な状態を維持した（Issue #52）。
- `area.x` が非ゼロの場合の horizontal viewport origin 処理を修正した（Issue #52）。

## [0.3.13] - 2026-09-23

### 修正

- platform text rendering の決定性を維持するため、Linux emoji catalog と runtime font input を pin した。

## [0.3.12] - 2026-09-22

### 修正

- canvas blit boundary と release evidence identity check を修正した（Issue #62）。
- public consumer 向けに `PlatformTextFaceSelection::CandidateChain` と決定的な font selection を追加した。
- viewport hit-collection cutoff と highlight alpha-compositing の挙動を復旧した。

## [0.3.11] - 2026-09-13

### 追加

- document mark 向けに、既存の default color を維持する opt-in の `text-highlight-background` theme token を追加した（Issue #52）。
- `heading-4`、`heading-5`、`heading-6` 向けに、既存および未設定の role metrics を維持する独立した opt-in typography override を追加した（Issue #52）。
- Unicode evidence failure を consumer が型別に扱える `IssuedConsumerArtifactPlan::execute_next_with_evidence_error` と `ConsumerArtifactPlanExecutionError` を追加し、既存の `execute_next` caller の error contract を維持した（Issue #57）。

### 修正

- candidate font face を weight/style/stretch 差異を越えて維持し、system fallback へ暗黙に切り替えないようにした（Issue #52）。
- wrap された link action region を表示行に維持し、scrolled document/viewport coordinate を含めて整合させた（Issue #52）。
- document text と button node/action hit region の fractional bottom edge を含め、logical layout cursor を変更しないようにした（Issue #52）。
- fixed-height document row と hit region を同一の fractional vertical center offset で配置した（Issue #52）。
- span measurement から rich-line rendering まで inline-code と monospace font 選択を維持した（Issue #52）。
- public hover helper が visible/scrolled text を wrap しても fractional document row position を維持した（Issue #52）。
- hover rendering と action hit の fractional scroll visibility を一致させ、partial visible hover surface の後続 child を維持した（Issue #52）。
- partial scrolling 中も absolute media control を表示し、ScrollArea offset を一度だけ適用した（Issue #52）。

## [0.3.10] - 2026-09-13

### 修正

- legacy document-typography coordinate の回帰を修正した。
- KUC の Unicode evidence pin resolver を接続し、consumer artifact が host policy injection なしで実行できるようにした。

## [0.3.9] - 2026-09-12

- corrective consumer-boundary release を公開した。public API と verification の詳細は `docs/release/v0.3.9.md` に記録した。

## [0.3.8] - 2026-09-09

- unified consumer-contract release を公開した。公開範囲と検証詳細は `docs/release/v0.3.8.md` に記録した。

## [0.3.7] - 2026-09-05

### 追加

- role-specific document typography と、renderer、Storybook-host、surface-host、measurement、hit-geometry の統合を追加した。

## [0.3.6] - 2026-09-05

### 追加

- 公開 raster host API の `UiTreeCanvasRenderer`、`UiTreeStorybookHost`、`UiTreeSurfaceHost` に、canonical proportional/monospace face を型付きで opt-in 選択する機能を追加した（Issue #41）。

### 修正

- 既存の `System` generic-family default と emoji 契約を維持したまま、registry-only consumer が path / Git override なしで platform candidate policy を再現できるようにした。

### 変更

- 直接・推移依存を監査し、release gate 前に `Cargo.lock` の互換性がある解決を更新した。

## [0.3.5] - 2026-09-04

### 追加

- registry consumer が必要とする additive かつ framework-neutral な `raster_host` API と、KDV 文書描画用の `raster-host` feature を公開した（Issue #35、#37）。
- 型付き per-side custom grid border と ScrollArea/Grid の原子的 layout 契約を公開 `katana-ui-core` crate から提供した。

### 修正

- path / Git dependency override を使わない registry-only の KDV 互換性を復旧しつつ、Storybook の legacy root facade 互換性を維持した。

## [0.3.4] - 2026-09-02

### 追加

- KUC-issued frame の raster や paint plan を公開せず、可変 viewport の full-motion sequence を一つの固定 canvas GIF/MP4 artifact へ正規化する additive な opaque writer を追加した（Issue #34）。
- source viewport 寸法と PNG hash、正規化 source/decode hash、root provenance、Unicode/IME/hit-test/AccessKit evidence を結合する専用の versioned manifest を追加した。

### 変更

- 完全な release quality gate を維持したまま、互換性のある推移的依存を更新した。

### 修正

- 既存の固定寸法 `write_opaque` 契約を維持しつつ、resize stage を consumer 側の composite や input 書き換えなしで新しい可変 viewport 経路へ渡せるようにした。

## [0.3.2] - 2026-08-29

### 追加

- KUC 所有の hit target で解決する pan、smooth scroll、pinch/trackpad zoom、fullscreen 状態伝播の型付き gesture 契約を追加した。
- 下流 editor host 向け retained full-editor root projection と cross-platform text-raster 証跡を追加した。
- Issue 起点の依存更新証跡 hook と、公開後の branch/worktree cleanup 自動化を追加した。

### 変更

- KUC の全未解決 release 要件を一つの patch release に統合し、互換性のある直接・推移依存と lockfile を更新した。
- crates.io 公開を tag-bound retry を含む GitHub Actions 上に限定し、local registry login を release flow から除外した。

### 修正

- 実 `egui::RawInput` の pointer-resolution 回帰を追加し、除外や閾値変更なしで strict line/function coverage 要件を維持した。
- 旧 opaque host token の挙動を保持し、明示 command-family identity は versioned envelope だけに適用した。

## [0.3.1] - 2026-08-28

### 追加

- 決定論的 layout、color emoji、grapheme hit-test、cache contract を持つ framework-neutral な platform text/SVG raster runtime を追加した。
- text、toolbar、floating toolbar、search、context menu、IME、accessibility を統合する generic text-surface/command-chrome model と optional KUC-owned egui adapter を追加した。
- 既存の公開 presentation struct literal に required field を追加せず、versioned token envelope で host-projected opaque command-family identity を追加した。

### 変更

- release quality gate を変更せず、互換性のある直接・推移依存を更新した。
- 決定論的 font と motion artifact の前提を含む全 publishable runtime/adapter crate へ strict Linux coverage を拡張した。

### 修正

- legacy host-token decode/render の挙動を維持しつつ、明示的に同一 family が投影された場合と未知 envelope version を fail closed にした。

## [0.3.0] - 2026-08-02

### 修正

- 汎用グリッドの罫線表示設定を公開モデルから型付き描画プロパティまで保持し、既存利用側には従来どおり表示する互換デフォルトを適用した。

## [0.2.0] - 2026-07-30

### 追加

- 固定・可変 track、固定行・固定列、表示範囲に限定した cell materialization、結合 cell、型付き cell appearance を備える format-neutral な 2 次元 virtualized grid を追加した。
- public KUC API に、型付き pointer hit-test、keyboard navigation、active cell、range selection を追加した。
- 文書 format semantics や framework 固有依存を持たない KDV `v0.4.0` 向け public consumer contract を追加した。

### 変更

- release gate 前に互換性のある直接・推移依存を最新化した。

## [0.1.1] - 2026-06-24

### 追加

- `UiContextMenuItem` に typed host action と task state payload を追加し、host が item id 文字列を解析せず context menu 選択を扱えるようにした。
- Storybook host query から、描画済み context menu item hit を `UiHostActionPlan` として解決できるようにした。

### 修正

- Storybook crate を内部用のままに戻し、release publish 対象を公開 crate の `katana-ui-core` のみに戻した。

<!-- next-url -->
