# UI Core root plan source

作成日: 2026-05-17
用途: `katana-ui-core` repo 内だけで `ui-core-root-plan` を実装するためのローカル根拠文書。

このファイルは KUC 実装者が repo 外を見に行かなくて済むよう、UI 分離計画のうち `katana-ui-core` に必要な部分だけを repo 内に固定したもの。
実装時は次の repo 内ファイルだけを読む。

- `docs/architecture/ui-separation/root-plan-source.md`
- `docs/adr/0002-katana-ui-core-rename.md`
- `docs/ui-separation-plan.md`
- `openspec/changes/ui-core-root-plan/`

## 1. KUC の責務

`katana-ui-core` はフレームワーク非依存（framework-neutral）な UI Core とする。
画面部品（widget）集だけではなく、起動、窓、描画面、描画モデル、テーマ、イベント、変換層契約を持つ。
JSX / TSX 互換ではなく、純 Rust の部品（component）合成 API として React に近い使い心地を目指す。
component 構造、component 内部状態（state）、見た目設定（style）は分離し、CSS のように後から class / rule / declaration を差し替えられるようにする。
UI ごとの状態（state）は component 内部 model で管理する。
同じ種類・同じ label の UI が複数あっても、`UiNodeId` と `UiStateId` は一意にする。
旧 Floem 実装は参照資料として扱い、新しい UI は同等範囲 + runtime / window / surface などの +α をゼロから作り直す。
この状態管理制約は `kal` 本体や `kal.json` には足さず、repo-local の `scripts/assert-kuc-state-ownership.py` を `just ast-lint` から呼んで検査する。
純 Rust component、内部 state、後付け style の 3 点を KUC core で満たせない場合、独自実装を続けるより GPUI など既存 UI framework を base に必要部品だけを作る判断へ戻す。

KUC は以下を持つ。

- `runtime`: `Application`, `ApplicationBuilder`, `AppConfig`, `AppHandle`, `AppLifecycle`, `RuntimeAdapter`
- `window`: `Window`, `WindowId`, `WindowConfig`, `WindowEvent`, `WindowCommand`, `WindowManager`, `DisplayInfo`
- `surface`: `Surface`, `FrameHandle`, `PaintRequest`, `SurfaceMetrics`
- `atom`: Text / Icon / Button / Input / Checkbox / Radio / Badge / Divider / Spacer
- `molecule`: Card / List / Menu / Tooltip / Modal / Tabs / Toolbar / SplitPane / FormField / Breadcrumb
- `layout`: Row / Column / Stack / Grid / DockSlot / SizePolicy
- `theme`: ColorToken / FontToken / SpacingToken / RadiusToken / ShadowToken / ThemeSnapshot
- `event`: UiEvent / PointerEvent / KeyboardEvent / FocusEvent / CommandEvent
- `render_model`: UiNode / UiNodeId / UiNodeKind / UiProps / UiTree / UiTreeDiff / UiCommand
- `component`: Component / StyledComponent / ComponentTree
- `style`: StyleSheet / StyleRule / StyleDeclaration / StyleProperty / StyleValue
- `accessibility`: accessibility DTOs
- `adapter_contract`: WidgetAdapter / RenderContext / EventSink / HostHandle

## 2. 公開 API 境界

KUC の公開 API は Floem View / GPUI Element / egui Ui を返してはならない。
利用側は KUC 所有の DTO / trait を使う。
変換層（adapter）が KUC の `UiTree` / `UiNode` / `ThemeSnapshot` / `EventSink` を各 UI framework に変換する。

依存禁止:

- `crates/katana-ui-core` は `floem` に依存しない。
- `crates/katana-ui-core` は `gpui` に依存しない。
- `crates/katana-ui-core` は `egui` に依存しない。
- `crates/katana-ui-core` は `katana-*` domain crate に依存しない。

## 3. Runtime / Window / Surface の中立化範囲

標準 API に入れるもの:

- title
- size
- min_size / max_size
- close
- focus
- fullscreen
- multi-window
- icon
- decorations
- move
- minimize / maximize / restore
- display change
- logical size / scale factor / dpi

標準 API に入れないもの:

- platform menu
- IME
- drag & drop

標準 API に入れないものは `adapter_contract` の拡張として扱う。

## 4. Phase 依存

```text
P0 (governance / naming / ADR)
  ↓
P1 (KUC neutral core)
  ↓ provides: render model / theme token / event model / adapter contract / runtime / window / surface
P4 (katana-ui composition)
  prereq: P1 output
```

KUC repo の `ui-core-root-plan` は P0-B / P0-C / P1 / P4-0 のうち、KUC に閉じて実施できる作業だけを扱う。
KatanA 本体、KDV、KLE、KCF、KDR、KMM、KCU の実装変更はこの repo では行わない。

## 5. P0-B: naming / ADR tasks

- P0-B-007: `katana-ui-core` は framework-neutral core とする。
- P0-B-008: Floem は adapter 対象であり core dependency ではない。
- P0-B-009: GPUI は adapter 対象であり core dependency ではない。
- P0-B-010: egui は新規 core API に入れない。
- P0-B-011: `katana-ui-widget` を `katana-ui-core` に rename する方針を ADR-0002 に記録する。
- P0-B-012: GitHub repo 名を `katana-ui-core` に rename する。
- P0-B-013: Cargo crate 名を `katana-ui-core` に変更する。
- P0-B-014: adapter crate を `katana-ui-core-floem` / `katana-ui-core-egui` / `katana-ui-core-gpui` に rename する。
- P0-B-015: storybook crate を `katana-ui-core-storybook` に rename する。
- P0-B-016: Justfile / scripts / release の旧名参照を更新する。
- P0-B-017: README / CONTRIBUTING に UI Core 責務を明記する。
- P0-B-018: 関連 repo の dependency 表記追従は KUC repo 内では実装しない。KUC 側では `docs/external-followups.md` に「repo 名、変更対象ファイル、期待する dependency 表記、KUC 側の根拠 task ID」を記録する。
- P0-B-019: 過去 OpenSpec changes / handoff / tmp 文書の `katana-ui-widget` 表記は履歴として残す。新規 OpenSpec changes は `katana-ui-core` 表記を使う。

## 6. P0-C: dependency policy tasks

- P0-C-002: core から `floem` 依存を禁止する guardrail を作る。
- P0-C-003: core から `gpui` 依存を禁止する guardrail を作る。
- P0-C-009: `workspace.dependencies` の shared version policy を整理する。KUC core が参照してよい dependency と adapter crate だけが参照してよい dependency を `docs/dependency-policy.md` に表で記録する。
- P0-C-010: adapter crate の optional feature policy を整理する。feature 名、default 有無、core compile への影響、release gate を `docs/dependency-policy.md` に表で記録する。

KUC repo では P0-C-004 以降の KDV / KLE / KatanA 側 guardrail は実装しない。
必要な追従は `docs/external-followups.md` に repo 名、変更対象ファイル、期待する dependency 表記、KUC 側の根拠 task ID を記録する。

## 7. P1-A: workspace restructuring

- P1-A-001: root `Cargo.toml` の current members を確認する。
- P1-A-002: `crates/katana-ui-core` を core crate として再定義する。
- P1-A-003: `crates/katana-ui-core-floem` を追加する。
- P1-A-004: `crates/katana-ui-core-storybook` を追加する。
- P1-A-005: `workspace.dependencies` の neutral deps と adapter-specific deps を分け、`docs/dependency-policy.md` に分類表を作る。分類表は `dependency`, `allowed in core`, `allowed in adapter`, `reason`, `verification command` 列を持つ。
- P1-A-006: `floem` / `floem_reactive` / `floem_renderer` を adapter crate dependency に移す。
- P1-A-007: core crate の package description を framework-neutral に変更する。
- P1-A-008: README の Floem 前提文言を削除する。
- P1-A-009: README に adapter policy 節を追加する。必須項目は primary adapter、compatibility adapter、core dependency 禁止、Storybook 経路、release gate、詳細文書 `docs/compat-adapters.md` へのリンク。
- P1-A-010: release metadata に adapter crate を含める。

## 8. P1-B: core module skeleton

- P1-B-001: `atom` module を作る。
- P1-B-002: `molecule` module を作る。
- P1-B-003: `layout` module を neutral 化する。
- P1-B-004: `theme` module を neutral 化する。
- P1-B-005: `event` module を作る。
- P1-B-006: `render_model` module を作る。
- P1-B-013: `runtime` module を作る。
- P1-B-014: `window` module を作る。
- P1-B-015: `surface` module を作る。
- P1-B-007: `accessibility` module を作る。
- P1-B-008: `adapter_contract` module を作る。
- P1-B-009: `primitive` module を `atom` へ段階移行する。
- P1-B-010: `composite` module を `molecule` へ段階移行する。
- P1-B-011: `floem_view` module を core から削除する。
- P1-B-012: `overlay_lifecycle` module を Floem adapter へ移す。

## 9. P1-C to P1-H: neutral model

この節は P1-C / P1-D / P1-E / P1-F / P1-G / P1-H の詳細 task を圧縮して記録する。

Theme:

- ColorToken / FontToken / SpacingToken / RadiusToken / ShadowToken / BorderToken / ZIndexToken / ThemeSnapshot / ThemeId
- light / dark fixture
- serialization test
- diff test

Layout:

- SizePolicy / Length / EdgeInsets / Alignment
- Row / Column / Stack / Grid / ScrollArea / SplitPane
- snapshot test
- serialization test

Atom:

- Text / Icon / Button / Input / Checkbox / Radio / Badge / Divider / Spacer
- disabled state
- focusable state
- accessibility label
- render model snapshot

Molecule:

- Card / List / Menu / Tooltip / Modal / Tabs / Toolbar / FormField / Breadcrumb
- event routing
- snapshot test

Event model:

- UiEvent / PointerEvent / KeyboardEvent / FocusEvent / CommandEvent
- UiNodeId target
- bubbling policy: parent traversal order、stop propagation、disabled node の扱い、nested molecule の伝播、serialization test
- capture policy: root-to-target order、target 到達前 cancellation、capture listener の登録単位、bubbling との実行順、ordering test
- serialization test
- ordering test

Render model:

- UiNodeId / UiNodeKind / UiProps / UiNode / UiTree / UiTreeDiff / UiCommand / RenderContext
- snapshot test
- no-framework compile test

## 10. P1-I: primary adapter candidate

Floem は最初に整備する primary adapter 候補。
P4-0 で primary が確定するまでは、Floem は互換 adapter に降格可能な扱いにする。

- P1-I-001: `katana-ui-core-floem` crate を作る。
- P1-I-002: core の `UiTree` を Floem view に変換する adapter skeleton を作る。
- P1-I-003: `Text` adapter を実装する。
- P1-I-004: `Button` adapter を実装する。
- P1-I-005: `Input` adapter を実装する。
- P1-I-006: `Row` / `Column` adapter を実装する。
- P1-I-007: `Tabs` adapter を実装する。
- P1-I-008: `Toolbar` adapter を実装する。
- P1-I-009: `SplitPane` adapter を実装する。
- P1-I-010: overlay lifecycle guard を Floem adapter 側に移す。
- P1-I-011: menu button contract を Floem adapter 側に移す。
- P1-I-012: adapter compile test を作る。

## 11. P1-K: compatibility adapter

primary に選ばれていない framework 向けに互換 adapter を併設する。
目的は外部利用者が既存環境へ KUC を差し込めるようにすること。
品質 gate は primary より緩くできるが、core crate への依存漏れは禁止する。
ただし UI をゼロから確立する現在段階では、KUC core と primary adapter 候補を先に固める。
egui / GPUI 互換 crate の作成は後続段階に送る。

- P1-K-001: `katana-ui-core-egui` 互換 adapter crate を新設する。
- P1-K-002: `katana-ui-core-gpui` 互換 adapter crate を新設する。
- P1-K-003: 各互換 adapter で `UiTree` から framework view への変換 skeleton を作る。最低ラインは Text / Button / Row / Column。
- P1-K-004: 各互換 adapter の対応 widget / 未対応機能 / fallback を `docs/compat-adapters.md` に表で明記し、README からリンクする。
- P1-K-005: 各互換 adapter に opt-in feature gate を設定し、core compile に引き込まれないことを保証する。
- P1-K-006: 各互換 adapter の最低品質 gate を CI に追加する。最低ラインは compile test。Storybook は `katana-ui-core` の core-only 確認だけを必須にする。
- P1-K-007: 互換 adapter の release が primary release を止めない条件を `docs/compat-adapters.md` の Release blocking rule に明記し、CI / release script はその条件を参照する。
- P1-K-008: 互換 adapter の support 範囲と SemVer policy を `docs/compat-adapters.md` と `docs/release.md` に記録する。

## 12. P1-L: runtime / window / surface

- P1-L-001: `Application` を定義する。
- P1-L-002: `AppConfig` を定義する。
- P1-L-003: `AppHandle` を定義する。
- P1-L-004: `AppLifecycle` event を定義する。
- P1-L-005: `RuntimeAdapter` trait を定義する。
- P1-L-006: `Window` / `WindowId` を定義する。
- P1-L-007: `WindowConfig` を定義する。
- P1-L-008: `WindowEvent` enum を定義する。
- P1-L-009: `WindowCommand` enum を定義する。
- P1-L-010: `WindowManager` を定義する。
- P1-L-011: `DisplayInfo` DTO を定義する。
- P1-L-012: `Surface` / `FrameHandle` / `PaintRequest` / `SurfaceMetrics` を定義する。
- P1-L-013: Noop adapter で framework 非依存 snapshot test を作る。
- P1-L-014: public API が adapter 型を返さないことを script で検査する。
- P1-L-015: primary adapter で runtime / window / surface を実装する。
- P1-L-016: 互換 adapter で runtime / window / surface を実装し、機能差異を `docs/compat-adapters.md` の adapter 一覧表と各 adapter README に明記する。
- P1-L-017: platform menu / IME / drag & drop の escape hatch を `adapter_contract` 拡張として定義する。

## 13. P1-J: quality gate update

- P1-J-001: core crate が `floem` を含まないことを script で検査する。
- P1-J-002: core crate が `gpui` を含まないことを script で検査する。
- P1-J-003: core crate が `katana-*` domain crate を含まないことを script で検査する。
- P1-J-004: `just check` に dependency leak guard を追加する。
- P1-J-005: Storybook gate を `katana-ui-core` core-only 対象に変更する。
- P1-J-006: release dry-run に core crate を含める。
- P1-J-007: release dry-run に Floem adapter crate を含める。
- P1-J-008: README に adapter policy 節を追加する。内容は `docs/compat-adapters.md` と同じ primary / compatibility / release blocking の概要に限定し、詳細は `docs/compat-adapters.md` へリンクする。

## 14. P4-0: primary adapter decision

KUC 側は primary adapter を独断で決めない。
ただし KUC repo 内の作業者が repo 外を読まなくて済むよう、比較 ADR の作成先と比較基準をこの local source に固定する。

比較 ADR の KUC 側コピー先:

- `docs/adr/katana-ui-primary-adapter.md`

候補:

- A: Floem を primary にする。
- B: GPUI を primary にする。
- C: egui を短期 primary にする。
- D: primary は当面確定せず adapter agnostic で進める。

比較基準:

- API 安定度
- editor 系 UI への適合度
- KatanA 移行コスト
- Phase 5 との整合
- 外部利用者向けの魅力
- Storybook / release gate の維持コスト

P4-0 tasks:

- P4-0-001: 比較 ADR を作る。
- P4-0-002: 比較基準を ADR に明記する。
- P4-0-003: primary adapter 選定結果を `katana-ui-core-<primary>` crate の品質 gate に反映する。
- P4-0-004: 互換 adapter として併設する framework 一覧を確定する。
- P4-0-005: 互換 adapter の品質 gate を CI に追加する。
- P4-0-006: primary adapter の release を互換 adapter breakage が止めない条件を `docs/compat-adapters.md` に記録し、CI / release script はその条件を参照する。
- P4-0-007: primary 切り替え時の旧 primary 降格 flow を ADR に記載する。

## 15. KUC done criteria

- `katana-ui-core` core が Floem なしで compile できる。
- core crate が `floem` / `gpui` / `egui` を含まない。
- core crate が `katana-*` domain crate を含まない。
- Floem は adapter 対象であり core dependency ではない。
- GPUI は adapter 対象であり core dependency ではない。
- egui は compatibility adapter 以外に残らない。
- runtime / window / surface は neutral API として存在する。
- Storybook は `katana-ui-core` core-only で検証し、adapter 経由にしない。
- release dry-run は core と selected primary adapter を含む。

## 16. Drift detection

- P8-A-001: `root-plan-source.md`、`docs/ui-separation-plan.md`、`ui-core-root-plan/tasks.md` の task ID drift を検出する。
