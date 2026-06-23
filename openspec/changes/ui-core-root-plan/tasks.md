# Tasks — ui-core-root-plan

> Note: この change は KUC の親設計と依存境界を固定する。01〜24 の atoms / molecules と部品カタログの実装完了は `openspec/changes/establish-kuc-atoms-molecules-catalog/` で判定する。

## 1. 前提と境界の固定

- [x] 1.1 P0-B-007: `docs/adr/0002-katana-ui-core-rename.md` の「決定 6」と `docs/architecture/ui-separation/root-plan-source.md` の「1. KUC の責務」を根拠として、`README.md` と `docs/ui-separation-plan.md` に `katana-ui-core` はフレームワーク非依存（framework-neutral）UI Core であることを明記する。
- [x] 1.2 P0-B-008: `README.md` と `docs/ui-separation-plan.md` に framework-native runtime / renderer は `crates/katana-ui-core` の core dependency ではないことを明記する。
- [x] 1.3 P0-B-009: `docs/architecture/ui-separation/root-plan-source.md` の「2. 公開 API 境界」と P0-B-009 を根拠として、KUC active workspace は core、Storybook、consumer app に限定することを明記する。
- [x] 1.4 P0-B-010: `docs/architecture/ui-separation/root-plan-source.md` の P0-B-010 と `docs/ui-separation-plan.md` の前提欄を根拠として、framework-native runtime / renderer は新規 core API に入れない方針を明記する。
- [x] 1.5 P0-B-014: `Cargo.toml`、`README.md`、`docs/directory-structure.md` に external runtime / renderer crate を active workspace に含めないことを明記する。
- [x] 1.6 P0-B-015: `docs/adr/0002-katana-ui-core-rename.md` の「決定 4」を根拠として、`storybook/Cargo.toml`、`README.md`、`docs/directory-structure.md` に Storybook crate 名を `katana-ui-core-storybook` として明記する。
- [x] 1.7 P0-B-017: `docs/adr/0002-katana-ui-core-rename.md` の「決定 6」と `docs/architecture/ui-separation/root-plan-source.md` の「1. KUC の責務」を根拠として、`README.md` と `docs/ui-separation-plan.md` に UI Core 責務として runtime / window / surface を含める。
- [x] 1.8 P0-B-019: `docs/adr/0002-katana-ui-core-rename.md` の「過去 OpenSpec changes / 履歴文書」を根拠として、`openspec/changes/README.md` と新規 change の説明では `katana-ui-core` 表記を使い、archive 済み OpenSpec の `katana-ui-widget` 表記は履歴として残す。

## 2. Workspace 再構成

- [x] 2.1 P1-A-001: root `Cargo.toml` の current members を確認し、現状を implementation note に残す。
- [x] 2.2 P1-A-002: `crates/katana-ui-core` を core crate として再定義する。
- [x] 2.3 P1-A-003: external runtime / renderer crate を active workspace に追加しない方針へ固定する。
- [x] 2.4 P1-A-004: `crates/katana-ui-core-storybook` を追加する。
- [x] 2.5 P1-A-005: `workspace.dependencies` を core deps と outside-core deps に分け、`docs/dependency-policy.md` に `dependency`、`allowed in core`、`allowed outside core`、`reason`、`verification command` 列を持つ分類表を作る。
- [x] 2.6 P1-A-006: `framework runtime crate` / `framework renderer crate` を KUC active dependency から除外する。
- [x] 2.7 P1-A-007: core crate の package description を framework-neutral な説明へ変更する。
- [x] 2.8 P1-A-008: README から Adapter 前提の説明を削除する。
- [x] 2.9 P1-A-009: README に core policy 節を追加する。必須項目は framework-neutral core、core dependency 禁止、Storybook 経路、release gate、`docs/dependency-policy.md` へのリンク。
- [x] 2.10 P1-A-010: release metadata は core crate の publish と consumer / Storybook gate に限定する。

## 3. Core module skeleton

- [x] 3.1 P1-B-001: `atom` module を作る。
- [x] 3.2 P1-B-002: `molecule` module を作る。
- [x] 3.3 P1-B-003: `layout` module を neutral 化する。
- [x] 3.4 P1-B-004: `theme` module を neutral 化する。
- [x] 3.5 P1-B-005: `event` module を作る。
- [x] 3.6 P1-B-006: `render_model` module を作る。
- [x] 3.7 P1-B-013: `runtime` module を作る。
- [x] 3.8 P1-B-014: `window` module を作る。
- [x] 3.9 P1-B-015: `surface` module を作る。
- [x] 3.10 P1-B-007: `accessibility` module を作る。
- [x] 3.11 P1-B-008: `adapter_contract` module を作る。
- [x] 3.12 P1-B-009: `primitive` module を `atom` へ段階移行する。
- [x] 3.13 P1-B-010: `composite` module を `molecule` へ段階移行する。
- [x] 3.14 P1-B-011: `adapter_view` module を core から削除する。
- [x] 3.15 P1-B-012: `overlay_lifecycle` module を adapter へ移す。

## 4. 中立 UI model

- [x] 4.1 P1-C-001: `ColorToken` を定義する。
- [x] 4.2 P1-C-002: `FontToken` を定義する。
- [x] 4.3 P1-C-003: `SpacingToken` を定義する。
- [x] 4.4 P1-C-004: `RadiusToken` を定義する。
- [x] 4.5 P1-C-005: `ShadowToken` を定義する。
- [x] 4.6 P1-C-006: `BorderToken` を定義する。
- [x] 4.7 P1-C-007: `ZIndexToken` を定義する。
- [x] 4.8 P1-C-008: `ThemeSnapshot` を定義する。
- [x] 4.9 P1-C-009: `ThemeId` を定義する。
- [x] 4.10 P1-C-010: light theme fixture を作る。
- [x] 4.11 P1-C-011: dark theme fixture を作る。
- [x] 4.12 P1-C-012: theme serialization test を作る。
- [x] 4.13 P1-C-013: theme diff test を作る。
- [x] 4.14 P1-D-001: `SizePolicy` を定義する。
- [x] 4.15 P1-D-002: `Length` を定義する。
- [x] 4.16 P1-D-003: `EdgeInsets` を定義する。
- [x] 4.17 P1-D-004: `Alignment` を定義する。
- [x] 4.18 P1-D-005: `Row` model を定義する。
- [x] 4.19 P1-D-006: `Column` model を定義する。
- [x] 4.20 P1-D-007: `Stack` model を定義する。
- [x] 4.21 P1-D-008: `Grid` model を定義する。
- [x] 4.22 P1-D-009: `ScrollArea` model を定義する。
- [x] 4.23 P1-D-010: `SplitPane` model を定義する。
- [x] 4.24 P1-D-011: layout snapshot test を作る。
- [x] 4.25 P1-D-012: layout serialization test を作る。
- [x] 4.26 P1-E-001: `Text` atom を定義する。
- [x] 4.27 P1-E-002: `Icon` atom を定義する。
- [x] 4.28 P1-E-003: `Button` atom を定義する。
- [x] 4.29 P1-E-004: `Input` atom を定義する。
- [x] 4.30 P1-E-005: `Checkbox` atom を定義する。
- [x] 4.31 P1-E-006: `Radio` atom を定義する。
- [x] 4.32 P1-E-007: `Badge` atom を定義する。
- [x] 4.33 P1-E-008: `Divider` atom を定義する。
- [x] 4.34 P1-E-009: `Spacer` atom を定義する。
- [x] 4.35 P1-E-010: disabled state を atom 共通に追加する。
- [x] 4.36 P1-E-011: focusable state を atom 共通に追加する。
- [x] 4.37 P1-E-012: accessibility label を atom 共通に追加する。
- [x] 4.38 P1-E-013: atom render model snapshot を作る。
- [x] 4.39 P1-F-001: `Card` molecule を定義する。
- [x] 4.40 P1-F-002: `List` molecule を定義する。
- [x] 4.41 P1-F-003: `Menu` molecule を定義する。
- [x] 4.42 P1-F-004: `Tooltip` molecule を定義する。
- [x] 4.43 P1-F-005: `Modal` molecule を定義する。
- [x] 4.44 P1-F-006: `Tabs` molecule を定義する。
- [x] 4.45 P1-F-007: `Toolbar` molecule を定義する。
- [x] 4.46 P1-F-008: `FormField` molecule を定義する。
- [x] 4.47 P1-F-009: `Breadcrumb` molecule を定義する。
- [x] 4.48 P1-F-010: molecule event routing を定義する。
- [x] 4.49 P1-F-011: molecule snapshot test を作る。
- [x] 4.50 P1-G-001: `UiEvent` を定義する。
- [x] 4.51 P1-G-002: `PointerEvent` を定義する。
- [x] 4.52 P1-G-003: `KeyboardEvent` を定義する。
- [x] 4.53 P1-G-004: `FocusEvent` を定義する。
- [x] 4.54 P1-G-005: `CommandEvent` を定義する。
- [x] 4.55 P1-G-006: `UiNodeId` を event target に使う。
- [x] 4.56 P1-G-007: event bubbling policy を定義する。必須項目は parent traversal order、stop propagation、disabled node の扱い、nested molecule の伝播、serialization test。
- [x] 4.57 P1-G-008: event capture policy を定義する。必須項目は root-to-target order、target 到達前 cancellation、capture listener の登録単位、bubbling との実行順、ordering test。
- [x] 4.58 P1-G-009: event serialization test を作る。
- [x] 4.59 P1-G-010: event ordering test を作る。
- [x] 4.60 P1-H-001: `UiNodeId` を定義する。
- [x] 4.61 P1-H-002: `UiNodeKind` を定義する。
- [x] 4.62 P1-H-003: `UiProps` を定義する。
- [x] 4.63 P1-H-004: `UiNode` を定義する。
- [x] 4.64 P1-H-005: `UiTree` を定義する。
- [x] 4.65 P1-H-006: `UiTreeDiff` を定義する。
- [x] 4.66 P1-H-007: `UiCommand` を定義する。
- [x] 4.67 P1-H-008: `RenderContext` を定義する。
- [x] 4.68 P1-H-009: render model snapshot test を作る。
- [x] 4.69 P1-H-010: render model no-framework compile test を作る。

## 5. Runtime / Window / Surface

- [x] 5.1 P1-L-001: `Application` と `ApplicationBuilder` を定義する。
- [x] 5.2 P1-L-002: `AppConfig` を定義する。
- [x] 5.3 P1-L-003: `AppHandle` を定義する。
- [x] 5.4 P1-L-004: `AppLifecycle` event を定義する。
- [x] 5.5 P1-L-005: `RuntimeAdapter` trait を定義する。
- [x] 5.6 P1-L-006: `Window` / `WindowId` を定義する。
- [x] 5.7 P1-L-007: `WindowConfig` を定義する。
- [x] 5.8 P1-L-008: `WindowEvent` enum を定義する。
- [x] 5.9 P1-L-009: `WindowCommand` enum を定義する。
- [x] 5.10 P1-L-010: `WindowManager` を定義する。
- [x] 5.11 P1-L-011: `DisplayInfo` DTO を定義する。
- [x] 5.12 P1-L-012: `Surface` / `FrameHandle` / `PaintRequest` / `SurfaceMetrics` を定義する。
- [x] 5.13 P1-L-013: Noop adapter で runtime / window / surface の framework 非依存 snapshot test を作る。
- [x] 5.14 P1-L-014: runtime / window / surface public API が adapter 型を返さないことを script で検査する。
- [x] 5.15 P1-L-015: external runtime / renderer 実装を KUC active workspace から除外する。
- [x] 5.16 P1-L-016: KUC active docs / release gate は external runtime / renderer の機能差異を管理しない。
- [x] 5.17 P1-L-017: platform menu / IME / drag & drop の escape hatch を `adapter_contract` 拡張として定義する。

## 6. Consumer app contract

注: UI をゼロから確立する現在段階では KUC core と consumer app contract を先に固める。external runtime / renderer は KUC active workspace の完了範囲に含めない。

- [x] 6.1 P1-I-001: external runtime / renderer crate を workspace member から除外する。
- [x] 6.2 P1-I-002: framework-native view 変換は KUC active release gate に含めない。
- [x] 6.3 P1-I-003: `examples/kuc-consumer-app` で public API だけを使う汎用 app shell を構築する。
- [x] 6.4 P1-I-004: consumer app shell の主要 node kind を `UiTree` / action / event / state 契約で検査する。
- [x] 6.5 P1-I-005: core dependency boundary と public API neutral guard で framework leak を検出する。
- [x] 6.6 P1-K-001: `examples/kuc-consumer-app` を workspace member として維持する。
- [x] 6.7 P1-K-002: consumer app は input、search、select、combo、text area、image surface、scroll、split pane、tabs、toolbar を組み合わせる。
- [x] 6.8 P1-K-003: consumer app は text input、textarea resize、scroll、split pane、tab close/add/pin/group/context command を action / event / state で検証する。
- [x] 6.9 P1-K-004: `consumer-app-contract` を `kuc-guardrails` と `release-readiness-check` の前提にする。
- [x] 6.10 P1-K-005: Storybook は core-only catalog として扱い、consumer readiness の完了根拠にはしない。

## 7. Quality gate update

- [x] 7.1 P1-J-001: core crate が `adapter` を含まないことを script で検査する。
- [x] 7.2 P1-J-002: core crate が `adapter` を含まないことを script で検査する。
- [x] 7.3 P1-J-003: core crate が `katana-*` domain crate を含まないことを script で検査する。
- [x] 7.4 P1-J-004: `just check` に dependency leak guard を追加する。
- [x] 7.5 P1-J-005: Storybook gate を `katana-ui-core` core-only 対象に変更する。
- [x] 7.6 P1-J-006: release dry-run に core crate を含める。
- [x] 7.7 P1-J-007: release dry-run は core crate、Storybook、consumer smoke を対象にする。
- [x] 7.8 P1-J-008: `README.md` の core policy 節に、framework-neutral core、core dependency 禁止、Storybook 経路、release gate の参照先を追加する。
- [x] 7.9 P8-A-001: `docs/architecture/ui-separation/root-plan-source.md`、`docs/ui-separation-plan.md`、`openspec/changes/ui-core-root-plan/tasks.md` の task ID drift を検出する検査を準備する。
- [x] 7.10 `git diff --check -- openspec/changes/ui-core-root-plan docs/ui-separation-plan.md README.md Cargo.toml crates storybook scripts Justfile` が通る。

## 8. P4-0 との接続

- [x] 8.1 P4-0-001: out-of-tree runtime / renderer 候補の比較 ADR は KUC active tree では管理しない。
- [x] 8.2 P4-0-002: KUC active tree は comparison ADR ではなく core public API と dependency boundary のみを管理する。
- [x] 8.3 P4-0-003: KUC が保証するのは external runtime / renderer が消費できる中立 public API までとする。
- [x] 8.4 P4-0-004: primary 以外の repo 外 runtime / renderer 一覧は KUC active tree では管理しない。
- [x] 8.5 P4-0-005: external runtime / renderer の最低品質 gate は KUC active tree では管理しない。
- [x] 8.6 P4-0-006: external runtime / renderer の failure は KUC core release を止めない。
- [x] 8.7 P4-0-007: primary 切り替え時の旧 primary 降格 flow は KUC active tree では管理しない。
- [x] 8.8 この change の実装着手前に `./scripts/openspec validate ui-core-root-plan --strict` を通す。
- [x] 8.9 実装完了時に `just check`、consumer app contract、Storybook gate、release dry-run を通す。

## 9. User Review Phase

- [x] 9.1 ユーザーFB: `OpenSpec task complete` を `旧UI同等+α完了` と同一視しない。旧Storybook対象と現core catalogの差分を `docs/architecture/ui-separation/ui-core-parity-gap.md` に記録する。
- [x] 9.2 ユーザーFB: Storybook gate は story 数だけでなく、各UIが placeholder ではない最低構造を持つことを検査する。（`scripts/storybook-requirement-gate.sh` / `scripts/assert-storybook-page-layout.py` / `visual_renderer_covers_required_ui_without_fallback` に固定）
- [x] 9.3 ユーザーFB: `同等+α` の証拠として、旧UI対象ごとの core model story、状態一意性、最低構造、Adapter非経由を同じ検査で確認する。（`scripts/assert-storybook-consumer-contract.py` / `scripts/assert-kuc-state-ownership.py` / `scripts/assert-core-public-api-neutral.sh` に固定）
- [x] 9.4 ユーザーFB: 9.1-9.3 を通した後に `just check`、`just storybook-regression`、`./scripts/openspec validate ui-core-root-plan --strict` を再実行する。（2026-05-29 に `just check` と `just storybook-regression` と `./scripts/openspec validate ui-core-root-plan --strict` を再実行済み）
- [x] 9.5 ユーザーFB: Storybook は CLI サマリではなく、`katana-ui-core` の panel で描画する前提にする。（`crates/katana-ui-core-storybook/src/panel.rs` / `crates/katana-ui-core-storybook/src/visual/render.rs` に固定）
- [x] 9.6 ユーザーFB: Storybook panel は theme 設定を必須とし、左ナビ panel と右プレビュー panel の両方で `ThemeSnapshot` を受け取ることを gate で確認する。（`crates/katana-ui-core-storybook/src/panel.rs` / `crates/katana-ui-core-storybook/src/panel/panel_verify.rs` / `summary_reports_panel_theme_and_style_gates` に固定）
- [x] 9.7 ユーザーFB: `Panel` 自体が `ThemeSnapshot` を受け取って `UiNode` に theme id を渡すことを core contract test と OpenSpec spec に固定する。（`crates/katana-ui-core/src/panel/mod.rs` / `crates/katana-ui-core/tests/core_contract.rs` / `openspec/changes/ui-core-root-plan/specs/ui-core-architecture/spec.md` に固定）
- [x] 9.8 ユーザーFB: `storybook-requirement-gate` は `panel_theme_configured=true` を必須 marker とし、panel theme 未設定を成功扱いにしない。（`scripts/storybook-requirement-gate.sh` と `summary_reports_panel_theme_and_style_gates` に固定）
- [x] 9.9 ユーザーFB: 静的HTML export や CLI summary を Storybook 完了根拠にしない。KUC の中核（core）UIと表示枠（panel）で動く可視 Storybook を作り、そこでスクリーンショット確認する。（`crates/katana-ui-core-storybook/src/main.rs` / `crates/katana-ui-core-storybook/src/visual/render.rs` に固定し、2026-05-29 に `just storybook-visual-snapshot` で `target/storybook-panel.png` を生成済み）
- [x] 9.10 ユーザーFB: KUC は JSX / TSX 互換ではなく、純 Rust の部品（component）合成 API として React に近い使い心地を目指す。（`crates/katana-ui-core/src/component.rs` / `crates/katana-ui-core/tests/generic_rust_app_contract.rs` / `examples/kuc-consumer-app/src/lib.rs` に固定）
- [x] 9.11 ユーザーFB: 見た目設定（style）は CSS のように後付けで差し替え可能にし、component 構造と内部 state から分離する。（`crates/katana-ui-core/src/theme/mod.rs` / `crates/katana-ui-core/src/state.rs` / `crates/katana-ui-core/tests/generic_rust_app_layout_contract.rs` に固定）
- [x] 9.12 ユーザーFB: 9.10 / 9.11 を満たせない場合、KUC 独自実装ではなく Adapter など既存 UI framework base へ戻す Go / No-Go 条件を design に明記する。（`openspec/changes/ui-core-root-plan/design.md` / `docs/architecture/ui-separation/root-plan-source.md` に固定）
- [x] 9.13 ユーザーFB: Retina / HiDPI 対応を利用側に意識させず、Canvas/Text/Window/Presentation の基盤契約として固定し、対応内容を自動テスト・`-D warnings` ビルド・`storybook` release 起動で確認済み。（`crates/katana-ui-core-storybook/src/visual/text_antialias_tests.rs` と `just storybook-visual-snapshot` に固定）
- [x] 9.14 ユーザーFB: Panel Storybook の `Panel In Panel` は静的なスクロールバー描画を禁止し、内側 Panel の scroll state、Inspector の scrollbar visibility 設定、preset ごとの明確な描画差分を自動テストで固定する。（`crates/katana-ui-core-storybook/src/visual/panel_in_panel_behavior_tests.rs` / `crates/katana-ui-core-storybook/src/visual/panel_in_panel_state_tests.rs` に固定）
- [x] 9.15 ユーザーFB: Storybook 全体の右端スクロールバーは既定で非表示にし、Panel 部品の scrollbar on/off は Storybook 共通操作ではなく Inspector の Panel 設定で切り替える。（`crates/katana-ui-core-storybook/src/visual/panel_scroll_panel_contract_tests.rs` / `crates/katana-ui-core-storybook/src/visual/panel_options.rs` に固定）
- [x] 9.16 ユーザーFB: Panel Storybook は `active panel` を state として持ち、`nav` / `preview` / `details` ごとの scroll offset と scrollbar visibility を分離し、Inspector 操作と wheel 入力が active または hit した panel だけに反映されることを自動テストで固定する。（`crates/katana-ui-core-storybook/src/visual/panel_scroll_interaction_tests.rs` / `crates/katana-ui-core-storybook/src/visual/panel_scroll_panel_interaction_tests.rs` / `crates/katana-ui-core-storybook/src/visual/panel_screen_state.rs` に固定）
- [x] 9.17 ユーザーFB: Panel は全 UI を描画する根本基盤として扱い、Storybook の中央 playground は小さな見せかけ wireframe ではなく、root viewport / child panel composition / clipping / independent scroll / Inspector targeted state を実操作で確認できる表現にする。（`crates/katana-ui-core-storybook/src/visual/panel_scroll_layout_contract_tests.rs` / `crates/katana-ui-core-storybook/src/visual/visual_tests.rs` に固定）
- [x] 9.18 ユーザーFB: Storybook UI ごとの構築ルールを skill 化し、theme / preset / Inspector option / state-event-action / 実操作確認を必須構成として固定する。（`.agents/skills/storybook-ui-harness/SKILL.md` / `scripts/storybook_ui_harness_assertions.py` に固定）
- [x] 9.19 ユーザーFB: Storybook の option 網羅と見せかけ preset を人間の注意に頼らず検出するため、internal ast-lint に UI option contract / preset count / Inspector route の構造制約を追加する。（`scripts/assert-storybook-ui-harness.py` / `crates/katana-ui-core-storybook/src/visual/visual_inspector_option_contract_tests.rs` / `crates/katana-ui-core-storybook/src/visual/visual_inspector_preset_follow_tests.rs` に固定）
- [x] 9.20 ユーザーFB: Storybook 起動時に `button` 固定で始めず、状態・preset・Inspector が見える代表ページとして `text-input` を初期表示にする。（`DEFAULT_STORYBOOK_PAGE = "text-input"` と `default_storybook_page_is_representative_input_playground` に固定）
- [x] 9.21 ユーザーFB: text-input は `readonly` / `placeholder` / 左 icon slot 予約 / SVG icon / 内部 SVG icon button callback を Storybook preset と core render model 契約の両方で扱う。（`openspec/changes/storybook-page-text-input/tasks.md`、`visual_interaction_text_input*_tests`、`atom_props_contract`、`storybook-ui-harness` guard に固定）
- [x] 9.22 ユーザーFB: button / text-button / svg-button / icon-text-button は hover 時に visible border を描き、Storybook native window では clickable hit area 上で hand cursor に切り替える。button atom の共通 default cursor も `UiCursor::Pointer` にする。
- [x] 9.23 ユーザーFB: Storybook preset tab は表示幅を超える場合に横スクロールし、外部 / 内部の preset 選択時に active tab が見える位置へ自動追従することを layout / interaction test で固定する。
- [x] 9.24 ユーザーFB: Storybook `tabs` は Katana workspace tab と同等に、追加 / close / move / group / pin / unpin / overflow を live control、preset、state/action/event、自動テストで固定する。
