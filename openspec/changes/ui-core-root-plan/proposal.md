## Why

既存の OpenSpec 変更単位（OpenSpec change）は、主に旧 UI 前提の画面部品（widget）抽出を扱っている。
一方で親計画（root plan）の UI 分離計画では、`katana-ui-core` を画面部品だけでなく、起動・窓・描画面まで含むフレームワーク非依存（framework-neutral）な UI Core として再定義している。

この変更単位（change）は、root 側の計画を `katana-ui-core` 側の実装入口として固定する。
仕様の再解釈はせず、`docs/architecture/ui-separation/root-plan-source.md` と `docs/ui-separation-plan.md` の KUC 担当分を OpenSpec の実装単位へ展開する。

## What Changes

- `katana-ui-core` の親計画を OpenSpec 変更単位（OpenSpec change）として追加する。
- 中核 crate（core crate）を framework-native runtime / renderer 直接依存（hard dependency）から切り離し、`runtime` / `window` / `surface` / `atom` / `molecule` / `layout` / `theme` / `event` / `render_model` / `accessibility` / `adapter_contract` を持つ UI Core として定義する。
- framework-specific UI は中核（core）の依存ではなく、KUC active workspace の外側で中立 contract を消費する責務として扱う。
- Storybook は `katana-ui-core` の中核（core）model だけで動かし、framework-native runtime / renderer 経由にはしない。
- root 計画の task ID (`P1-*`, `P4-0-*`) を OpenSpec task に展開し、実装 runner が迷わない粒度にする。
- 既存の個別画面部品（widget）変更単位（change）は削除しない。履歴と既存部品の根拠として残し、この親 change が Phase 1 の優先境界を定義する。
- **BREAKING**: 中核（core）の公開 API（public API）は Adapter View / Adapter Element / adapter Ui などの framework 型を返さない前提に移行する。

## Capabilities

### New Capabilities

- `ui-core-architecture`: KUC の責務、モジュール構成、依存禁止、公開 API のフレームワーク非依存性を定義する。
- `adapter-boundary`: framework-specific UI などの framework 実装を中核（core）から分離し、KUC active workspace の外側に置く境界を定義する。
- `runtime-window-surface`: `Application`、`Window`、`Surface` を含む起動・窓・描画面の中立 API（neutral API）を定義する。
- `migration-quality-gates`: framework-specific UI / Katana domain の依存漏れを防ぐ検査、Storybook、リリース検査（release gate）、差分ずれ（drift）検出を定義する。

### Modified Capabilities

- なし。

## Impact

- `crates/katana-ui-core` の公開 API と依存方針が変わる。
- `katana-ui-core-storybook` と `examples/kuc-consumer-app` の整理が必要になる。
- `README.md`、`docs/ui-separation-plan.md`、`docs/directory-structure.md`、`docs/release.md`、release metadata、CI / Justfile gate が影響を受ける。
- 既存の `primitive` / `composite` / `layout` / `theme` は削除前提ではなく、`atom` / `molecule` / 中立モデル（neutral model）へ段階移行する。
- root 側の必要情報は `docs/architecture/ui-separation/root-plan-source.md` にコピー済み。実装者は repo 外の文書を読まず、KUC repo 内の文書だけを入口にする。
