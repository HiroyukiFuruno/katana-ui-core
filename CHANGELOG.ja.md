# 変更履歴

このプロジェクトの注目すべき変更はすべてこのファイルに記録されます。

形式は [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) に基づき、
バージョニングは [Semantic Versioning](https://semver.org/spec/v2.0.0.html) に準拠します。

<!-- next-header -->

## [Unreleased]

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
