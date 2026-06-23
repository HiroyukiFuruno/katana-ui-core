# 変更履歴

このプロジェクトの注目すべき変更はすべてこのファイルに記録されます。

形式は [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) に基づき、
バージョニングは [Semantic Versioning](https://semver.org/spec/v2.0.0.html) に準拠します。

<!-- next-header -->

## [Unreleased]

## [0.1.1] - 2026-06-24

### 追加

- `UiContextMenuItem` に typed host action と task state payload を追加し、host が item id 文字列を解析せず context menu 選択を扱えるようにした。
- Storybook host query から、描画済み context menu item hit を `UiHostActionPlan` として解決できるようにした。

### 修正

- Storybook crate を内部用のままに戻し、release publish 対象を公開 crate の `katana-ui-core` のみに戻した。

<!-- next-url -->
