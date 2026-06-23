# OpenSpec Changes — katana-ui-core

この一覧は `katana-ui-core` repo 内の OpenSpec 変更単位（OpenSpec change）だけを扱う。
実装者は repo 外の sibling repository を直接読まない。
root 計画の根拠は `openspec/changes/ui-core-root-plan/` と `docs/architecture/ui-separation/root-plan-source.md` にコピー済みの内容を使う。
`katana` / `katana-chat-ui` / KDV / KLE の UI 層から読んだ追加根拠は、実装前に repo-local の棚卸しとして `docs/inventory/katana-katana-chat-ui-kdv-kle-ui-needs.md` へ固定する。

## 新規計画

| order | change | 役割 | 着手条件 |
| --- | --- | --- | --- |
| root | `ui-core-root-plan` | KUC をフレームワーク非依存（framework-neutral）UI Core として再定義し、runtime / window / surface / adapter 境界を固定する親 change | この change を先に読む |
| implementation | `establish-kuc-atoms-molecules-catalog` | Storybook menu 77 page を KUC の atoms / molecules と Storybook harness へ移管し、自動品質ゲートを正本化する親 change | `ui-core-root-plan` の境界を確認後に読む |

KDV の Markdown viewer UI 構築に必要な周辺 UI は既存 change を使う。
ただし KDV が生成した HTML / PDF / PNG / JPG 相当の RGBA preview surface を KUC `UiTree` に載せる契約は `23-add-preview-surface-image-contract` で扱う。
対応表は `establish-kuc-atoms-molecules-catalog/kdv-ui-build-readiness.md` を正本にする。

## 現行 Storybook 実装キュー

現行の実装単位は、番号無しの `storybook-page-*` leaf change に統一する。
次に着手する順番は change 名ではなく、`establish-kuc-atoms-molecules-catalog/storybook-menu-priority-order.md` の `SB-001`〜`SB-077` で管理する。

この運用は「番号無しで統一、優先順を別途一覧で可視化」を採用する。
理由は、Storybook menu page の安定 ID と実装順を分離し、順序を組み換えるときに change rename を不要にするためである。

| queue | 正本 |
| --- | --- |
| leaf change 一覧 | `openspec/changes/establish-kuc-atoms-molecules-catalog/storybook-menu-change-split.md` |
| 優先順位 | `openspec/changes/establish-kuc-atoms-molecules-catalog/storybook-menu-priority-order.md` |
| readiness audit | `docs/storybook-consumer-readiness-audit.md` |

次に作業する change は、優先順位表のうち leaf change DoD が未完了で最小の `SB-*` とする。
ユーザーが `次をすすめて`、`continue`、`次` のように抽象的に依頼した場合は、`rtk proxy python3 scripts/next-storybook-page-change.py --json` を実行し、返った `change` を現在の OpenSpec leaf change として扱う。
`NN-add-*` や archive 済み 01〜24 は入力元であり、現在の着手キューではない。

## 履歴入力元 — `NN-add-*` 系列

`katana` / `katana-chat-ui` / KDV / KLE の利用側画面を KUC の atoms / molecules で組めるようにするための、優先度順 (`01` が最優先) の機能追加 change。
prefix の数字は優先度を表すもので、`archive/2026-05-25-18-accordion` などの旧 archive change（命名がそのまま prefix 番号を持つ）とは別系列。
旧系列は機能名そのもの (`18-accordion`) を archive した履歴、現行系列は `NN-add-*` 形式で識別する。
現在は Storybook menu leaf change の入力元として扱い、新規実装の着手順には使わない。

KUC の公開対象は最小部品（atoms）と複合部品（molecules）までとする。
画面全体の構造（organisms）、画面ひな形（templates）、本文エディター、本文プレビュー、チャット画面全体は KUC に入れない。
KDV (`katana-document-viewer`) / KLE (`katana-language-editor`) は、KUC や他の部品を利用して個別に実装する利用側である。

| priority | change | 主 capability | 主な依存 |
| --- | --- | --- | --- |
| 00 | `00-add-scroll-area-contract` | `kuc-scroll-area-contract` | — |
| 00 | `00-add-split-pane-contract` | `kuc-split-pane-contract` | — |
| 01 | `01-add-context-menu` | `kuc-context-menu` | — |
| 02 | `02-add-drag-drop-primitive` | `kuc-drag-drop` | 00-scroll |
| 03 | `03-add-workspace-tab-bar` | `kuc-closeable-tab-strip` | 01, 02 |
| 04 | `04-add-rich-popover-and-hover-card` | `kuc-hover-card`, `kuc-placement-engine` | — |
| 05 | `05-add-toolbar-overflow` | `kuc-toolbar-overflow`, `kuc-action-rail` | 01, 04 |
| 06 | `06-add-multiline-text-input` | `kuc-text-area-atom` | — |
| 07 | `07-add-chip-and-attachment-chip` | `kuc-chip-atom`, `kuc-attachment-chip` | 02 (opt-in reorder) |
| 08 | `08-add-diagnostics-list` | `kuc-diagnostics-list` | 07 (filter chip) |
| 09 | `09-add-empty-state` | `kuc-empty-state` | — |
| 10 | `10-add-inline-banner-alert` | `kuc-banner` | — |
| 11 | `11-add-toast-stack-manager` | `kuc-toast-stack-manager` | — |
| 12 | `12-add-multi-segment-status-bar` | `kuc-status-bar-segments`, `kuc-progress-meter` | 04 |
| 13 | `13-add-shortcut-combo-display` | `kuc-shortcut-combo-atom`, `kuc-shortcut-cheatsheet` | — |
| 14 | `14-add-sectioned-settings-form` | `kuc-settings-list` | 09 |
| 15 | `15-add-collapsible-sidebar-shell` | `kuc-collapsible-panel` | 04 |
| 16 | `16-add-virtualized-list-and-tree` | `kuc-virtualization` | 00-scroll |
| 17 | `17-add-skeleton-loader` | `kuc-skeleton-atom`, `kuc-skeleton-cluster` | — |
| 18 | `18-add-animation-primitives` | `kuc-motion` | — |
| 19 | `19-add-title-bar-window-chrome` | `kuc-window-control-button-group` | — |
| 20 | `20-add-splash-screen-template` | `kuc-startup-state-composition` | 09, 10, 17, 18 |
| 21 | `21-add-command-launcher-results` | `kuc-command-launcher-results` | 13, 16 |
| 22 | `22-add-search-control-strip` | `kuc-search-control-strip` | 05, 13, 21 |
| 23 | `23-add-preview-surface-image-contract` | `kuc-preview-surface-image-contract` | 00-scroll, 22 |

## NN-add-* 系列の横断 DoD

各 change は、完了条件（DoD）として次を満たす。

- proposal / design / tasks / specs が揃っている。
- specs は `## ADDED Requirements` または `## MODIFIED Requirements` を持ち、各 Requirement に Scenario を持つ。
- tasks は `設計確定`、`中核実装`、`自動テスト`、`自動回帰`、`Storybook ページ`、`ドキュメント`、`品質ゲート / DoD` を持つ。該当しない項目は省略せず「対象外」と明記する。
- Storybook は静的見本ではなく、preview、settings、state、event、action、quality を表示する。
- passive な atom / molecule でも、action / event が `none` であることを Storybook と contract に明記する。
- 正しさの根拠は Storybook 目視ではなく、自動テスト、数値化された layout / rendering contract、入力回帰、静的検査、guard とする。
- `openspec validate <change> --strict`、`cargo test -p katana-ui-core`、`cargo clippy -p katana-ui-core -p katana-ui-core-storybook --all-targets -- -D warnings`、数値化された layout / rendering contract、入力回帰、静的検査の CI gate を通す。
- KUC public API に organisms / templates / pages / app shell / title bar / splash template / viewer body / editor body / chat root を入れない。

## 進行ルール

- 新規作業では `katana-ui-core` / `KUC` 表記を使う。
- 旧リポジトリ名や旧略称は archive 済み change の履歴説明だけに残す。
- repo 外の実装挙動が必要な場合は、先に `docs/inventory/<topic>.md` へ画面・操作・入力・出力・状態遷移をコピーしてから実装する。
- 中核 crate（core crate）は `adapter` / `adapter` / `adapter` を直接依存に持たない。
- 画面フレームワーク（UI framework）固有の型は KUC active workspace に入れない。
- KUC 公開 API は atoms / molecules までに限定し、organisms / templates / pages を公開 widget として追加しない。
- `WorkspaceTabBar`、`AppShell`、`TitleBar`、`SplashScreen` のような draft 名が残る change は、実装前に domain-free な atoms / molecules へ粒度を落とす。
- Storybook は KUC の部品を実画面で触ってフィードバックするための画面であり、KUC の中立 model と専用 surface で表示する。
- 部品の正しさは Storybook や手動操作ではなく、自動テスト、数値化された layout / rendering contract、入力回帰、静的検査を主根拠にする。
- 依存境界は `docs/dependency-policy.md` と `docs/directory-structure.md` を基準にする。

## Archived / superseded input sources

| change | 現在の扱い |
| --- | --- |
| `archive/2026-05-25-katana-widget-parity-backlog` | 01〜24 と追加 UI の入力元。要件は `establish-kuc-atoms-molecules-catalog` へ移管済みで、実装正本にはしない。 |
| `archive/2026-05-25-ui-core-interaction-visual-parity` | 旧 interaction / visual gate の入力元。Storybook 目視寄りの完了扱いは現行品質ゲートへ移管済み。 |
| `archive/2026-05-25-18-accordion` | Accordion 要件の履歴入力元。現行実装単位は `storybook-page-accordion`。 |
| `archive/2026-05-25-23-color-picker-complete-parity` | ColorPicker parity 要件の履歴入力元。現行実装単位は `storybook-page-color-picker-rgba`。 |
| `archive/2026-05-25-24-code-diff` | CodeDiff 要件の履歴入力元。現行実装単位は `storybook-page-code-diff`。 |

## Archive change group

以下は旧画面部品（widget）抽出 change 群である。
履歴として残すが、新規実装の優先境界は `ui-core-root-plan` に従う。
01〜24 の現在の実装正本は `establish-kuc-atoms-molecules-catalog` である。

| # | change | 旧階層 | 現在の扱い |
| --- | --- | --- | --- |
| 00 | bootstrap-dev-environment | meta | 履歴。repo-local skill / docs の初期化記録 |
| 01 | theme-tokens | theme | KUC `theme` へ継承 |
| 02 | text-primitive | primitive/text | KUC `atom::Text` へ移行対象 |
| 03 | icon-primitive | primitive/icon | KUC `atom::Icon` へ移行対象 |
| 04 | spinner-primitive | primitive/spinner | KUC `atom` へ移行対象 |
| 05 | svg-button | composite/button/svg | KUC `atom::Button` / `molecule` へ再分類 |
| 06 | text-button | composite/button/text | KUC `atom::Button` へ移行対象 |
| 07 | icon-text-button | composite/button/icon_text | KUC `atom::Button` へ移行対象 |
| 08 | toggle | composite/selector/toggle | KUC `atom` / `molecule` へ再分類 |
| 09 | segmented-toggle | composite/selector/segmented | KUC `molecule::Tabs` との重複を確認 |
| 10 | select-box | composite/selector/select | KUC `molecule::Menu` との重複を確認 |
| 11 | color-swatch | composite/selector/color | KUC `atom` / `molecule` へ再分類 |
| 12 | text-input | composite/input/text | KUC `atom::Input` へ移行対象 |
| 13 | search-box | composite/input/search | KUC `atom::Input` か consumer crate へ再分類 |
| 14 | tooltip | composite/indicator/tooltip | KUC `molecule::Tooltip` へ移行対象 |
| 15 | badge | composite/indicator/badge | KUC `atom::Badge` へ移行対象 |
| 16 | key-cap | composite/indicator/key_cap | KUC `atom` へ移行対象 |
| 17 | card | layout/card | KUC `molecule::Card` へ移行対象 |
| 18 | accordion | layout/accordion | KUC `molecule` へ移行対象 |
| 19 | split-pane | layout/split | KUC layout model へ移行対象 |
| 20 | modal-overlay | layout/modal | KUC `molecule::Modal` へ移行対象 |
| 21 | popover | layout/popover | KUC `molecule` へ移行対象 |
| 22 | rgba-color-picker | composite/selector/color_picker | KUC 採用条件を満たす場合だけ移行 |
| 23 | color-picker-complete-parity | composite/selector/color_picker | KUC 採用条件を満たす場合だけ移行 |
| 24 | code-diff | composite/code_diff | domain を持たない文字列 diff として KUC 候補 |

## Domain exclusion

次は KUC core に入れない。

- markdown rendering panel
- AI adapter control
- chat composer
- linter result list
- workspace file tree
- editor gutter / ruler
- document preview / TOC
- application shell
- title bar / window chrome
- splash screen template
- chat root / message thread / composer
- application title bar / status bar / command palette の domain logic

入れるか迷う UI は `docs/widget-extraction-policy.md` の採用条件で判定する。
