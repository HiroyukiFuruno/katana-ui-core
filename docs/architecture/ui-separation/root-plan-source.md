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
旧 Adapter 実装は参照資料として扱い、新しい UI は同等範囲 + runtime / window / surface などの +α をゼロから作り直す。
この状態管理制約は `kal` 本体や `kal.json` には足さず、repo-local の `scripts/assert-kuc-state-ownership.py` を `just ast-lint` から呼んで検査する。
純 Rust component、内部 state、後付け style の 3 点を KUC core で満たせない場合、独自実装を続けるより Adapter など既存 UI framework を base に必要部品だけを作る判断へ戻す。

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

KUC の公開 API は Adapter View / Adapter Element / adapter Ui を返してはならない。
利用側は KUC 所有の DTO / trait を使う。
変換層（adapter）が KUC の `UiTree` / `UiNode` / `ThemeSnapshot` / `EventSink` を各 UI framework に変換する。

依存禁止:

- `crates/katana-ui-core` は `adapter` に依存しない。
- `crates/katana-ui-core` は `adapter` に依存しない。
- `crates/katana-ui-core` は `adapter` に依存しない。
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
- P0-B-008: framework-native runtime / renderer は core dependency ではない。
- P0-B-009: KUC active workspace は core、Storybook、consumer app に限定する。
- P0-B-010: framework-native runtime / renderer は新規 core API に入れない。
- P0-B-011: `katana-ui-widget` を `katana-ui-core` に rename する方針を ADR-0002 に記録する。
- P0-B-012: GitHub repo 名を `katana-ui-core` に rename する。
- P0-B-013: Cargo crate 名を `katana-ui-core` に変更する。
- P0-B-014: external runtime / renderer crate 名は KUC active workspace では管理しない。
- P0-B-015: storybook crate を `katana-ui-core-storybook` に rename する。
- P0-B-016: Justfile / scripts / release の旧名参照を更新する。
- P0-B-017: README / CONTRIBUTING に UI Core 責務を明記する。
- P0-B-018: 関連 repo の dependency 表記追従は KUC repo 内では実装しない。KUC 側では `docs/external-followups.md` に「repo 名、変更対象ファイル、期待する dependency 表記、KUC 側の根拠 task ID」を記録する。
- P0-B-019: 過去 OpenSpec changes / handoff / tmp 文書の `katana-ui-widget` 表記は履歴として残す。新規 OpenSpec changes は `katana-ui-core` 表記を使う。

## 6. P0-C: dependency policy tasks

- P0-C-002: core から `adapter` 依存を禁止する guardrail を作る。
- P0-C-003: core から `adapter` 依存を禁止する guardrail を作る。
- P0-C-009: `workspace.dependencies` の shared version policy を整理する。KUC core が参照してよい dependency と outside core に限定する dependency を `docs/dependency-policy.md` に表で記録する。
- P0-C-010: core feature policy を整理する。feature 名、default 有無、core compile への影響、release gate を `docs/dependency-policy.md` に表で記録する。

KUC repo では P0-C-004 以降の KDV / KLE / KatanA 側 guardrail は実装しない。
必要な追従は `docs/external-followups.md` に repo 名、変更対象ファイル、期待する dependency 表記、KUC 側の根拠 task ID を記録する。

## 7. P1-A: workspace restructuring

- P1-A-001: root `Cargo.toml` の current members を確認する。
- P1-A-002: `crates/katana-ui-core` を core crate として再定義する。
- P1-A-003: external runtime / renderer crate を active workspace に追加しない方針へ固定する。
- P1-A-004: `crates/katana-ui-core-storybook` を追加する。
- P1-A-005: `workspace.dependencies` の neutral deps と adapter-specific deps を分け、`docs/dependency-policy.md` に分類表を作る。分類表は `dependency`, `allowed in core`, `allowed in adapter`, `reason`, `verification command` 列を持つ。
- P1-A-006: `framework runtime crate` / `framework renderer crate` を KUC active dependency から除外する。
- P1-A-007: core crate の package description を framework-neutral に変更する。
- P1-A-008: README の Adapter 前提文言を削除する。
- P1-A-009: README に core policy 節を追加する。必須項目は framework-neutral core、core dependency 禁止、Storybook 経路、release gate、詳細文書 `docs/dependency-policy.md` へのリンク。
- P1-A-010: release metadata は core crate の publish と consumer / Storybook gate に限定する。

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
- P1-B-011: `adapter_view` module を core から削除する。
- P1-B-012: `overlay_lifecycle` module を adapter へ移す。

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

## 10. P1-I: external runtime boundary

KUC active workspace は external runtime / renderer crate を持たない。
KUC は framework-native view 変換を成果物にせず、外部実装が消費できる中立 contract を提供する。

- P1-I-001: external runtime / renderer crate を workspace member から除外する。
- P1-I-002: framework-native view 変換は KUC active release gate に含めない。
- P1-I-003: `examples/kuc-consumer-app` で public API だけを使う汎用 app shell を構築する。
- P1-I-004: consumer app shell の主要 node kind を `UiTree` / action / event / state 契約で検査する。
- P1-I-005: core dependency boundary と public API neutral guard で framework leak を検出する。

## 11. P1-K: consumer app contract

KUC の利用可否は、汎用 Rust app が KUC public API だけで desktop app 相当の UI tree と interaction を構築できることで判定する。
Storybook は視覚的な実部品確認の場であり、consumer readiness の完了根拠にはしない。

- P1-K-001: `examples/kuc-consumer-app` を workspace member として維持する。
- P1-K-002: consumer app は input、search、select、combo、text area、image surface、scroll、split pane、tabs、toolbar を組み合わせる。
- P1-K-003: consumer app は text input、textarea resize、scroll、split pane、tab close/add/pin/group/context command を action / event / state で検証する。
- P1-K-004: `consumer-app-contract` を `kuc-guardrails` と `release-readiness-check` の前提にする。
- P1-K-005: Storybook は core-only catalog として扱い、consumer readiness の完了根拠にはしない。

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
- P1-L-015: external runtime / renderer 実装を KUC active workspace から除外する。
- P1-L-016: repo 外 runtime / renderer は KUC core の `RuntimeAdapter` / render model を消費する。機能差異は KUC active tree では管理しない。
- P1-L-017: platform menu / IME / drag & drop の escape hatch を `adapter_contract` 拡張として定義する。

## 13. P1-J: quality gate update

- P1-J-001: core crate が `adapter` を含まないことを script で検査する。
- P1-J-002: core crate が `adapter` を含まないことを script で検査する。
- P1-J-003: core crate が `katana-*` domain crate を含まないことを script で検査する。
- P1-J-004: `just check` に dependency leak guard を追加する。
- P1-J-005: Storybook gate を `katana-ui-core` core-only 対象に変更する。
- P1-J-006: release dry-run に core crate を含める。
- P1-J-007: release dry-run は core crate、Storybook、consumer smoke を対象にする。
- P1-J-008: README に core policy 節を追加する。内容は framework-neutral core、dependency boundary、Storybook、release gate の概要に限定し、詳細は `docs/dependency-policy.md` へリンクする。

## 14. External runtime decision

KUC active tree は external runtime / renderer の選定、比較、品質 gate を管理しない。
この repository の判断対象は、framework-neutral core API、consumer app contract、Storybook core catalog、release guard に限定する。

- P4-0-001: external runtime / renderer の比較 ADR は KUC active tree で管理しない。
- P4-0-002: KUC release gate は external runtime / renderer の成否を参照しない。
- P4-0-003: KUC が保証するのは external runtime / renderer が消費できる中立 public API までとする。

## 15. KUC done criteria

- `katana-ui-core` core が external runtime / renderer なしで compile できる。
- core crate が framework-native runtime / renderer dependency を含まない。
- core crate が `katana-*` domain crate を含まない。
- active workspace は core、Storybook、consumer app のみである。
- external runtime / renderer は KUC release gate の対象外である。
- runtime / window / surface は neutral API として存在する。
- Storybook は `katana-ui-core` core-only で検証し、framework-native runtime / renderer を経由しない。
- release dry-run は core crate のみを publish 対象にする。

## 16. Drift detection

- P8-A-001: `root-plan-source.md`、`docs/ui-separation-plan.md`、`ui-core-root-plan/tasks.md` の task ID drift を検出する。
