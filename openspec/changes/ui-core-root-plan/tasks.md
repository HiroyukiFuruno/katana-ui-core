# Tasks — ui-core-root-plan

> Note: この change は KUC の親設計と依存境界を固定する。01〜24 の atoms / molecules と部品カタログの実装完了は `openspec/changes/establish-kuc-atoms-molecules-catalog/` で判定する。

## 1. 前提と境界の固定

- [x] 1.1 P0-B-007: `docs/adr/0002-katana-ui-core-rename.md` の「決定 6」と `docs/architecture/ui-separation/root-plan-source.md` の「1. KUC の責務」を根拠として、`README.md` と `docs/ui-separation-plan.md` に `katana-ui-core` はフレームワーク非依存（framework-neutral）UI Core であることを明記する。
- [x] 1.2 P0-B-008: `docs/adr/0002-katana-ui-core-rename.md` の「決定 3」と「理由」を根拠として、`README.md` と `docs/ui-separation-plan.md` に Floem は変換層（adapter）対象であり、`crates/katana-ui-core` の core dependency ではないことを明記する。
- [x] 1.3 P0-B-009: `docs/architecture/ui-separation/root-plan-source.md` の「2. 公開 API 境界」と P0-B-009 を根拠として、`docs/ui-separation-plan.md` に GPUI は変換層（adapter）対象であり、`crates/katana-ui-core` の core dependency ではないことを明記する。
- [x] 1.4 P0-B-010: `docs/architecture/ui-separation/root-plan-source.md` の P0-B-010 と `docs/ui-separation-plan.md` の前提欄を根拠として、`README.md` と `docs/ui-separation-plan.md` に egui は新規 core API に入れず、互換変換層（compatibility adapter）だけで扱う方針を明記する。
- [x] 1.5 P0-B-014: `docs/adr/0002-katana-ui-core-rename.md` の「決定 3」を根拠として、`Cargo.toml`、`README.md`、`docs/directory-structure.md` に adapter crate 名を `katana-ui-core-floem` / `katana-ui-core-egui` / `katana-ui-core-gpui` として明記する。
- [x] 1.6 P0-B-015: `docs/adr/0002-katana-ui-core-rename.md` の「決定 4」を根拠として、`storybook/Cargo.toml`、`README.md`、`docs/directory-structure.md` に Storybook crate 名を `katana-ui-core-storybook` として明記する。
- [x] 1.7 P0-B-017: `docs/adr/0002-katana-ui-core-rename.md` の「決定 6」と `docs/architecture/ui-separation/root-plan-source.md` の「1. KUC の責務」を根拠として、`README.md` と `docs/ui-separation-plan.md` に UI Core 責務として runtime / window / surface を含める。
- [x] 1.8 P0-B-019: `docs/adr/0002-katana-ui-core-rename.md` の「過去 OpenSpec changes / 履歴文書」を根拠として、`openspec/changes/README.md` と新規 change の説明では `katana-ui-core` 表記を使い、archive 済み OpenSpec の `katana-ui-widget` 表記は履歴として残す。

## 2. Workspace 再構成

- [x] 2.1 P1-A-001: root `Cargo.toml` の current members を確認し、現状を implementation note に残す。
- [x] 2.2 P1-A-002: `crates/katana-ui-core` を core crate として再定義する。
- [x] 2.3 P1-A-003: `crates/katana-ui-core-floem` を追加する。
- [x] 2.4 P1-A-004: `crates/katana-ui-core-storybook` を追加する。
- [x] 2.5 P1-A-005: `workspace.dependencies` を neutral deps と adapter-specific deps に分け、`docs/dependency-policy.md` に `dependency`、`allowed in core`、`allowed in adapter`、`reason`、`verification command` 列を持つ分類表を作る。
- [x] 2.6 P1-A-006: `floem` / `floem_reactive` / `floem_renderer` を adapter crate dependency に移す。
- [x] 2.7 P1-A-007: core crate の package description を framework-neutral な説明へ変更する。
- [x] 2.8 P1-A-008: README から Floem 前提の説明を削除する。
- [x] 2.9 P1-A-009: README に adapter policy 節を追加する。必須項目は primary adapter、compatibility adapter、core dependency 禁止、Storybook 経路、release gate、`docs/compat-adapters.md` へのリンク。
- [x] 2.10 P1-A-010: release metadata に core crate と adapter crate を含める。

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
- [x] 3.14 P1-B-011: `floem_view` module を core から削除する。
- [x] 3.15 P1-B-012: `overlay_lifecycle` module を Floem adapter へ移す。

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
- [x] 5.15 P1-L-015: Floem adapter で runtime / window / surface を実装する。
- [x] 5.16 P1-L-016: egui / gpui 互換 adapter で runtime / window / surface skeleton を作る。
- [x] 5.17 P1-L-017: platform menu / IME / drag & drop の escape hatch を `adapter_contract` 拡張として定義する。

## 6. Adapter migration

注: UI をゼロから確立する現在段階では KUC core と primary adapter 候補を先に固める。6.13 以降の egui / GPUI 互換 adapter は本実装ではなく、skeleton / compile gate / support policy までを完了範囲にする。

- [x] 6.1 P1-I-001: `katana-ui-core-floem` crate を作る。
- [x] 6.2 P1-I-002: core の `UiTree` を Floem view に変換する adapter skeleton を作る。
- [x] 6.3 P1-I-003: `Text` adapter を実装する。
- [x] 6.4 P1-I-004: `Button` adapter を実装する。
- [x] 6.5 P1-I-005: `Input` adapter を実装する。
- [x] 6.6 P1-I-006: `Row` / `Column` adapter を実装する。
- [x] 6.7 P1-I-007: `Tabs` adapter を実装する。
- [x] 6.8 P1-I-008: `Toolbar` adapter を実装する。
- [x] 6.9 P1-I-009: `SplitPane` adapter を実装する。
- [x] 6.10 P1-I-010: overlay lifecycle guard を Floem adapter 側に移す。
- [x] 6.11 P1-I-011: menu button contract を Floem adapter 側に移す。
- [x] 6.12 P1-I-012: Floem adapter compile test を作る。
- [x] 6.13 P1-K-001: `katana-ui-core-egui` 互換 adapter crate を新設する。
- [x] 6.14 P1-K-002: `katana-ui-core-gpui` 互換 adapter crate を新設する。
- [x] 6.15 P1-K-003: 各互換 adapter で Text / Button / Row / Column の `UiTree` 変換 skeleton を作る。
- [x] 6.16 P1-K-004: `README.md` に互換 adapter ごとの対応 widget、未対応機能、fallback を表で明記し、詳細が長くなる場合は `docs/compat-adapters.md` へ分離して README からリンクする。
- [x] 6.17 P1-K-005: 互換 adapter を opt-in feature gate にし、core compile に引き込まれないことを保証する。
- [x] 6.18 P1-K-006: 互換 adapter の最低品質 gate を compile test に限定して CI に追加する。Storybook は `katana-ui-core` の core-only 確認だけを必須にし、adapter 経由にはしない。
- [x] 6.19 P1-K-007: 互換 adapter の失敗が primary release を止めない条件を `docs/release.md` に明記し、CI / release script はその条件を参照して判定する。
- [x] 6.20 P1-K-008: 互換 adapter の support 範囲と SemVer policy を `docs/compat-adapters.md` に記録し、`README.md` と `docs/release.md` から参照できる形にする。

## 7. Quality gate update

- [x] 7.1 P1-J-001: core crate が `floem` を含まないことを script で検査する。
- [x] 7.2 P1-J-002: core crate が `gpui` を含まないことを script で検査する。
- [x] 7.3 P1-J-003: core crate が `katana-*` domain crate を含まないことを script で検査する。
- [x] 7.4 P1-J-004: `just check` に dependency leak guard を追加する。
- [x] 7.5 P1-J-005: Storybook gate を `katana-ui-core` core-only 対象に変更する。
- [x] 7.6 P1-J-006: release dry-run に core crate を含める。
- [x] 7.7 P1-J-007: release dry-run に Floem adapter crate を含める。
- [x] 7.8 P1-J-008: `README.md` の adapter policy 節に、primary adapter、compatibility adapter、core dependency 禁止、Storybook 経路、release gate の参照先を追加する。
- [x] 7.9 P8-A-001: `docs/architecture/ui-separation/root-plan-source.md`、`docs/ui-separation-plan.md`、`openspec/changes/ui-core-root-plan/tasks.md` の task ID drift を検出する検査を準備する。
- [x] 7.10 `git diff --check -- openspec/changes/ui-core-root-plan docs/ui-separation-plan.md README.md Cargo.toml crates storybook scripts Justfile` が通る。

## 8. P4-0 との接続

- [x] 8.1 P4-0-001: primary adapter 候補の比較 ADR は `docs/adr/katana-ui-primary-adapter.md` に作る前提を `docs/ui-separation-plan.md` と `README.md` に明記する。
- [x] 8.2 P4-0-002: `docs/adr/katana-ui-primary-adapter.md` に記録すべき比較基準として、API 安定度、エディタ系適合、移行コスト、Phase 5 整合、外部利用者向け魅力、Storybook / release gate 維持コストを `docs/ui-separation-plan.md` に列挙する。
- [x] 8.3 P4-0-003: primary adapter に選ばれた `katana-ui-core-<primary>` の品質 gate を core と同等にする。
- [x] 8.4 P4-0-004: primary 以外の adapter を互換 adapter として扱う一覧を `docs/compat-adapters.md` に記録する。
- [x] 8.5 P4-0-005: 互換 adapter の最低品質 gate を `docs/compat-adapters.md` と `docs/release.md` に記録する。
- [x] 8.6 P4-0-006: primary adapter の release を互換 adapter の failure が止めない条件を `docs/release.md` に記録する。
- [x] 8.7 P4-0-007: primary 切り替え時の旧 primary 降格 flow を `docs/compat-adapters.md` と `docs/release.md` から参照できるようにする。
- [x] 8.8 この change の実装着手前に `./scripts/openspec validate ui-core-root-plan --strict` を通す。
- [x] 8.9 実装完了時に `just check`、adapter compile test、Storybook gate、release dry-run を通す。

## 9. User Review Phase

- [/] 9.1 ユーザーFB: `OpenSpec task complete` を `旧UI同等+α完了` と同一視しない。旧Storybook対象と現core catalogの差分を `docs/architecture/ui-separation/ui-core-parity-gap.md` に記録する。
- [/] 9.2 ユーザーFB: Storybook gate は story 数だけでなく、各UIが placeholder ではない最低構造を持つことを検査する。
- [/] 9.3 ユーザーFB: `同等+α` の証拠として、旧UI対象ごとの core model story、状態一意性、最低構造、Floem非経由を同じ検査で確認する。
- [/] 9.4 ユーザーFB: 9.1-9.3 を通した後に `just check`、`just storybook-regression`、`./scripts/openspec validate ui-core-root-plan --strict` を再実行する。
- [/] 9.5 ユーザーFB: Storybook は CLI サマリではなく、`katana-ui-core` の panel で描画する前提にする。
- [/] 9.6 ユーザーFB: Storybook panel は theme 設定を必須とし、左ナビ panel と右プレビュー panel の両方で `ThemeSnapshot` を受け取ることを gate で確認する。
- [/] 9.7 ユーザーFB: `Panel` 自体が `ThemeSnapshot` を受け取って `UiNode` に theme id を渡すことを core contract test と OpenSpec spec に固定する。
- [/] 9.8 ユーザーFB: `storybook-requirement-gate` は `panel_theme_configured=true` を必須 marker とし、panel theme 未設定を成功扱いにしない。
- [/] 9.9 ユーザーFB: 静的HTML export や CLI summary を Storybook 完了根拠にしない。KUC の中核（core）UIと表示枠（panel）で動く可視 Storybook を作り、そこでスクリーンショット確認する。
  - 2026-05-17: `crates/katana-ui-core-storybook` に KUC panel tree から PNG snapshot を描く純 Rust visual surface を追加。次に実ウィンドウ起動と画像確認を行う。
- [/] 9.10 ユーザーFB: KUC は JSX / TSX 互換ではなく、純 Rust の部品（component）合成 API として React に近い使い心地を目指す。
- [/] 9.11 ユーザーFB: 見た目設定（style）は CSS のように後付けで差し替え可能にし、component 構造と内部 state から分離する。
- [/] 9.12 ユーザーFB: 9.10 / 9.11 を満たせない場合、KUC 独自実装ではなく GPUI など既存 UI framework base へ戻す Go / No-Go 条件を design に明記する。
