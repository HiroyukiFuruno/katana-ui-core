# katana-ui-core — UI 分離計画 抜粋

作成日: 2026-05-17
更新日: 2026-05-17 (rename: `katana-ui-widget` → `katana-ui-core` / ADR-0002 / runtime/window/surface 追加)
repo-local source: [`docs/architecture/ui-separation/root-plan-source.md`](architecture/ui-separation/root-plan-source.md)

> **注**: この repo での作業は repo 外の文書を読まずに完結させる。root 計画のうち KUC に必要な根拠は [`root-plan-source.md`](architecture/ui-separation/root-plan-source.md) と [`ADR-0002`](adr/0002-katana-ui-core-rename.md) にコピー済み。

## このファイルの位置付け

本ファイルは KUC 担当分を repo 内で実装するための作業入口である。task ID は [`root-plan-source.md`](architecture/ui-separation/root-plan-source.md) と同一に保つ。作業者は repo 外の親ディレクトリや sibling repository を読まない。

`ui-core-root-plan` は親設計の正本であり、01〜24 の部品実装完了を意味しない。
atoms / molecules と Storybook の実装正本は [`openspec/changes/establish-kuc-atoms-molecules-catalog/`](../openspec/changes/establish-kuc-atoms-molecules-catalog/) とする。
このファイルには root architecture と依存境界だけを置き、部品ごとの option / action / event / state / preset / test は新 change 側で管理する。

`ScrollArea` は KUC の layout foundation として、axis、offset、viewport/content extent、scrollbar visibility / placement、scroll command、edge event、nested state identity を typed contract で持つ。
KDV / KLE の本文 viewer / editor scroll policy や editor-preview 同期は KUC に入れず、利用側がこの scroll container を組み合わせて実装する。
KDV が描画済みの HTML / PDF / PNG / JPG 相当 preview surface は `ImageSurface` node として `UiTree` に載せるが、Markdown display-list、KMM node id、検索 engine、PDF page model は KUC に入れない。

## Repository の役割

`katana-ui-core` (KUC、旧 `katana-ui-widget`) は **framework-neutral な UI Core** として位置付ける。

- 独自 UI 表現 (Component model / DSL) を持つ。
- **runtime / window / surface API も core が持つ** (Application::new().window(...).run() のような起動 entry / multi-window / fullscreen / icon を neutral 化)。
- framework-specific UI / native-renderer は **adapter (出力先)** として後ろに置く。core crate に framework 依存を持ち込まない。
- atoms / molecules / layout primitive / theme token / event model / render model / accessibility / adapter contract を提供する。
- KatanA 固有の概念を持たない (KDV / KLE / KMM 等の domain crate に依存しない)。

詳細: [`root-plan-source.md` 1. KUC の責務](architecture/ui-separation/root-plan-source.md#1-kuc-の責務)

## 担当 Phase

- **Phase 1**: KUC neutral core 化 (本 repo のメイン作業)
- **P4-0**: Primary adapter 選定 (KatanA 側で決定するが、本 repo の release / 品質ゲートに直結)
- **横断**: P0 (governance / naming / ADR)

依存グラフ抜粋: [`root-plan-source.md` 4. Phase 依存](architecture/ui-separation/root-plan-source.md#4-phase-依存)

```
P0 → P1 (本 repo)
       ↓ provides: render model / theme token / event model / adapter contract
     P4 (katana-ui composition がこの output を消費)
```

## Phase 1 概要

目的: 既存の Adapter 前提 crate を framework-neutral UI Core に再定義する。

設計原則 (master 5.1.1):

- Framework-neutral
- Katana domain-neutral
- State-light
- Render-model oriented
- Adapter contract first
- Theme token first
- Accessibility DTO を最初から持つ

## Task list (master 抜粋)

### P1-A. Workspace restructuring

- [x] P1-A-001: `katana-ui-core` root Cargo.toml の current members を確認する。
- [x] P1-A-002: `crates/katana-ui-core` を core crate として再定義する。
- [x] P1-A-003: external runtime / renderer crate を active workspace に追加しない方針へ固定する。
- [x] P1-A-004: `crates/katana-ui-core-storybook` を追加する。
- [x] P1-A-005: root `workspace.dependencies` の neutral deps と adapter-specific deps を分けて整理する。`docs/dependency-policy.md` に `dependency`、`allowed in core`、`allowed in adapter`、`reason`、`verification command` 列を持つ分類表を作る。
- [x] P1-A-006: `framework runtime crate` / `framework renderer crate` を KUC active dependency から除外する。
- [x] P1-A-007: core crate の package description を「framework-neutral」に変更する。
- [x] P1-A-008: Adapter 前提の README 文言を削除する。
- [x] P1-A-009: README に core policy 節を追加する。必須項目は framework-neutral core、core dependency 禁止、Storybook 経路、release gate、`docs/dependency-policy.md` へのリンク。
- [x] P1-A-010: release metadata は core crate、Storybook、consumer smoke のみを対象にする。

### P1-B. Core module skeleton

- [x] P1-B-001: `atom` module を作る。
- [x] P1-B-002: `molecule` module を作る。
- [x] P1-B-003: `layout` module を neutral 化する。
- [x] P1-B-004: `theme` module を neutral 化する。
- [x] P1-B-005: `event` module を作る。
- [x] P1-B-006: `render_model` module を作る。
- [x] P1-B-007: `accessibility` module を作る。
- [x] P1-B-008: `adapter_contract` module を作る。
- [x] P1-B-009: `primitive` module を `atom` へ段階移行する。
- [x] P1-B-010: `composite` module を `molecule` へ段階移行する。
- [x] P1-B-011: `adapter_view` module を core から削除する。
- [x] P1-B-012: `overlay_lifecycle` module を adapter へ移す。

### P1-C. Theme tokens

- [x] P1-C-001: `ColorToken` を定義する。
- [x] P1-C-002: `FontToken` を定義する。
- [x] P1-C-003: `SpacingToken` を定義する。
- [x] P1-C-004: `RadiusToken` を定義する。
- [x] P1-C-005: `ShadowToken` を定義する。
- [x] P1-C-006: `BorderToken` を定義する。
- [x] P1-C-007: `ZIndexToken` を定義する。
- [x] P1-C-008: `ThemeSnapshot` を定義する。
- [x] P1-C-009: `ThemeId` を定義する。
- [x] P1-C-010: light theme fixture を作る。
- [x] P1-C-011: dark theme fixture を作る。
- [x] P1-C-012: theme serialization test を作る。
- [x] P1-C-013: theme diff test を作る。

### P1-D. Layout primitives

- [x] P1-D-001: `SizePolicy` を定義する。
- [x] P1-D-002: `Length` を定義する。
- [x] P1-D-003: `EdgeInsets` を定義する。
- [x] P1-D-004: `Alignment` を定義する。
- [x] P1-D-005: `Row` model を定義する。
- [x] P1-D-006: `Column` model を定義する。
- [x] P1-D-007: `Stack` model を定義する。
- [x] P1-D-008: `Grid` model を定義する。
- [x] P1-D-009: `ScrollArea` model を定義する。
- [x] P1-D-010: `SplitPane` model を定義する。
  - `SplitPane` は `first` / `second` の 2 pane slot、typed action / event、ratio clamp、reset、handle props を KUC が持つ。
  - ratio の保存、app shell、折りたたみ sidebar、editor-preview sync は consumer 側の責務とする。
- [x] P1-D-011: layout snapshot test を作る。
- [x] P1-D-012: layout serialization test を作る。

### P1-E. Atom widgets

- [x] P1-E-001: `Text` atom を定義する。
- [x] P1-E-002: `Icon` atom を定義する。
- [x] P1-E-003: `Button` atom を定義する。
- [x] P1-E-004: `Input` atom を定義する。
- [x] P1-E-005: `Checkbox` atom を定義する。
- [x] P1-E-006: `Radio` atom を定義する。
- [x] P1-E-007: `Badge` atom を定義する。
- [x] P1-E-008: `Divider` atom を定義する。
- [x] P1-E-009: `Spacer` atom を定義する。
- [x] P1-E-010: disabled state を atom 共通に追加する。
- [x] P1-E-011: focusable state を atom 共通に追加する。
- [x] P1-E-012: accessibility label を atom 共通に追加する。
- [x] P1-E-013: atom render model snapshot を作る。

### P1-F. Molecule widgets

- [x] P1-F-001: `Card` molecule を定義する。
- [x] P1-F-002: `List` molecule を定義する。
- [x] P1-F-003: `Menu` molecule を定義する。
- [x] P1-F-004: `Tooltip` molecule を定義する。
- [x] P1-F-005: `Modal` molecule を定義する。
- [x] P1-F-006: `Tabs` molecule を定義する。
- [x] P1-F-007: `Toolbar` molecule を定義する。
- [x] P1-F-008: `FormField` molecule を定義する。
- [x] P1-F-009: `Breadcrumb` molecule を定義する。
- [x] P1-F-010: molecule event routing を定義する。
- [x] P1-F-011: molecule snapshot test を作る。

### P1-G. Event model

- [x] P1-G-001: `UiEvent` を定義する。
- [x] P1-G-002: `PointerEvent` を定義する。
- [x] P1-G-003: `KeyboardEvent` を定義する。
- [x] P1-G-004: `FocusEvent` を定義する。
- [x] P1-G-005: `CommandEvent` を定義する。
- [x] P1-G-006: `UiNodeId` を event target に使う。
- [x] P1-G-007: event bubbling policy を定義する。必須項目は parent traversal order、stop propagation、disabled node の扱い、nested molecule の伝播、serialization test。
- [x] P1-G-008: event capture policy を定義する。必須項目は root-to-target order、target 到達前 cancellation、capture listener の登録単位、bubbling との実行順、ordering test。
- [x] P1-G-009: event serialization test を作る。
- [x] P1-G-010: event ordering test を作る。

### P1-H. Render model

- [x] P1-H-001: `UiNodeId` を定義する。
- [x] P1-H-002: `UiNodeKind` を定義する。
- [x] P1-H-003: `UiProps` を定義する。
- [x] P1-H-004: `UiNode` を定義する。
- [x] P1-H-005: `UiTree` を定義する。
- [x] P1-H-006: `UiTreeDiff` を定義する。
- [x] P1-H-007: `UiCommand` を定義する。
- [x] P1-H-008: `RenderContext` を定義する。
- [x] P1-H-009: render model snapshot test を作る。
- [x] P1-H-010: render model no-framework compile test を作る。

### P1-I. External runtime boundary

KUC は external runtime / renderer を active workspace に持たない。
KUC が提供するのは `UiTree`、render model、event、action、state、theme、runtime / window / surface の中立契約であり、framework-native view 変換はこの repository の成果物に含めない。

- [x] P1-I-001: external runtime / renderer crate を workspace member から除外する。
- [x] P1-I-002: framework-native view 変換は KUC active release gate に含めない。
- [x] P1-I-003: `examples/kuc-consumer-app` で public API だけを使う汎用 app shell を構築する。
- [x] P1-I-004: consumer app shell の主要 node kind を `UiTree` / action / event / state 契約で検査する。
- [x] P1-I-005: core dependency boundary と public API neutral guard で framework leak を検出する。

### P1-K. Consumer app contract

KUC の利用可否は external runtime の有無ではなく、汎用 Rust app が KUC public API だけで desktop app 相当の UI tree と interaction を構築できることで判定する。

- [x] P1-K-001: `examples/kuc-consumer-app` を workspace member として維持する。
- [x] P1-K-002: consumer app は input、search、select、combo、text area、image surface、scroll、split pane、tabs、toolbar を組み合わせる。
- [x] P1-K-003: consumer app は text input、textarea resize、scroll、split pane、tab close/add/pin/group/context command を action / event / state で検証する。
- [x] P1-K-004: `consumer-app-contract` を `kuc-guardrails` と `release-readiness-check` の前提にする。
- [x] P1-K-005: Storybook は core-only catalog として扱い、consumer readiness の完了根拠にはしない。

### P1-L. Runtime / Window / Surface API

KUC を framework-neutral UI Core として完成させるために、起動 entry / window 管理 / 描画 surface の neutral API を整備する。framework-native runtime / renderer はこの neutral API を外部で消費する。

neutral 化の粒度は **「中」**: title / size / close / focus / fullscreen / multi-window / icon を共通サポートする。platform menu / IME / drag&drop は adapter 経由 escape hatch (`adapter_contract` 拡張) で対応。

- [x] P1-L-001: `Application` を定義する (`Application::new() -> ApplicationBuilder`、`run(self) -> AppExitCode`)。
- [x] P1-L-002: `AppConfig` を定義する (識別子 / persistence path / locale / accessibility option)。
- [x] P1-L-003: `AppHandle` を定義する (`spawn_window` / `dispatch_command` / `current_windows`)。
- [x] P1-L-004: `AppLifecycle` event (`Started` / `Suspended` / `Resumed` / `ShuttingDown`) を定義する。
- [x] P1-L-005: `RuntimeAdapter` trait を定義する (event loop を adapter に委譲)。
- [x] P1-L-006: `Window` / `WindowId` を定義する。
- [x] P1-L-007: `WindowConfig` を定義する (title / size / min_size / max_size / icon / decorations / fullscreen)。
- [x] P1-L-008: `WindowEvent` enum を定義する (Close / Resize / Move / Focus / Minimize / Maximize / Restore / DisplayChanged)。
- [x] P1-L-009: `WindowCommand` enum を定義する (SetTitle / SetSize / SetPosition / Focus / Minimize / Maximize / Close / Fullscreen)。
- [x] P1-L-010: `WindowManager` を定義する (multi-window 作成 / iteration / 1 window 終了でアプリ終了するかの policy)。
- [x] P1-L-011: `DisplayInfo` DTO を定義する (multi-monitor read-only 情報)。
- [x] P1-L-012: `Surface` / `FrameHandle` / `PaintRequest` / `SurfaceMetrics` を定義する。
- [x] P1-L-013: runtime / window / surface module に対する framework 非依存 snapshot test を作る (Noop adapter で起動できることを確認)。
- [x] P1-L-014: runtime / window / surface module の public API が adapter 型を返さないことを script で検査する。
- [x] P1-L-015: external runtime / renderer 実装を KUC active workspace から除外する。
- [x] P1-L-016: KUC active docs / release gate は external runtime / renderer の機能差異を管理しない。
- [x] P1-L-017: platform menu / IME / drag&drop の escape hatch を `adapter_contract` 拡張として定義する (KUC 標準 API には入れない)。

### P1-J. Quality gate update

- [x] P1-J-001: core crate が `adapter` を含まないことを script で検査する。
- [x] P1-J-002: core crate が `adapter` を含まないことを script で検査する。
- [x] P1-J-003: core crate が `katana-*` domain crate を含まないことを script で検査する。
- [x] P1-J-004: `just check` に dependency leak guard を追加する。
- [x] P1-J-005: Storybook gate を `katana-ui-core` core-only 対象に変更する。
- [x] P1-J-006: release dry-run に core crate を含める。
- [x] P1-J-007: release dry-run は core crate、Storybook、consumer smoke を対象にする。
- [x] P1-J-008: README に core policy 節を追加する。内容は framework-neutral core、Storybook、release gate、dependency boundary に限定し、詳細は `docs/dependency-policy.md` へリンクする。

## repo 外 runtime との接点

KUC active tree は framework-neutral core として維持する。
repo 外 runtime / renderer の選定、比較、release blocking は KUC の active docs と release gate に含めない。
KUC が保証するのは `UiTree`、theme、event、state、action、layout、render model、adapter contract の public API と自動テストである。

## 前提 (depends on) / 出力 (provides)

- **前提 (P0 完了)**:
  - `katana-ui-core` を framework-neutral core とする ADR 記録 (P0-B-007)
  - framework-native runtime / renderer を新規 core API に入れない方針 (P0-B-010)
  - dependency leak guard (P0-C-002, P0-C-003)

- **出力 (Phase 1 完了で他 Phase に提供するもの)**:
  - `UiTree` / `UiNode` / `UiNodeKind` / `UiProps` (render model)
  - `ThemeSnapshot` / 各 token (theme)
  - `UiEvent` / `PointerEvent` / `KeyboardEvent` / `FocusEvent` / `CommandEvent` (event model)
  - atom (Text / Icon / Button / Input / Checkbox / Radio / Badge / Divider / Spacer)
  - molecule (Card / List / Menu / Tooltip / Modal / Tabs / Toolbar / FormField / SplitPane 等)
  - external runtime / renderer が消費できる中立 contract trait

## Done criteria

本 repo に関する [`root-plan-source.md` 15. KUC done criteria](architecture/ui-separation/root-plan-source.md#15-kuc-done-criteria) の該当項目:

- [x] `katana-ui-core` core が external runtime / renderer なしで compile できる
- [x] core crate が framework-native runtime / renderer dependency を含まない (P1-J-001〜002 script 通過)
- [x] core crate が `katana-*` domain crate を含まない (P1-J-003)
- [x] active workspace は core、Storybook、consumer app のみである
- [x] external runtime / renderer は KUC release gate の対象外である

上記は core architecture の完了条件であり、部品ごとの実装完了条件ではない。
01〜24 の UI は `establish-kuc-atoms-molecules-catalog` の自動テスト、数値化された layout / rendering contract、入力回帰、Storybook 条件を満たすまで完了にしない。

## drift 検出

- 本ファイルの task ID は [`root-plan-source.md`](architecture/ui-separation/root-plan-source.md) と完全一致させる。
- task の追加・削除・変更時は、まず `root-plan-source.md` を更新し、その後に本ファイルと `openspec/changes/ui-core-root-plan/tasks.md` を更新する。
- P8-A-001 の CI script は `root-plan-source.md`、本ファイル、`ui-core-root-plan/tasks.md` の task ID 一致を検査する。

## 参照リンク

- [UI Core root plan source](architecture/ui-separation/root-plan-source.md)
- [ADR-0002: katana-ui-core rename](adr/0002-katana-ui-core-rename.md)
- [既存 docs/widget-extraction-policy.md](widget-extraction-policy.md)
- [既存 docs/directory-structure.md](directory-structure.md)
- [既存 docs/release.md](release.md)
