# 変更履歴

このプロジェクトの注目すべき変更はすべてこのファイルに記録されます。

形式は [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) に基づき、
バージョニングは [Semantic Versioning](https://semver.org/spec/v2.0.0.html) に準拠します。

<!-- next-header -->

## [Unreleased]

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
