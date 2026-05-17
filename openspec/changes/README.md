# OpenSpec Changes — katana-ui-core

この一覧は `katana-ui-core` repo 内の OpenSpec 変更単位（OpenSpec change）だけを扱う。
実装者は repo 外の sibling repository を直接読まない。
root 計画の根拠は `openspec/changes/ui-core-root-plan/` と `docs/architecture/ui-separation/root-plan-source.md` にコピー済みの内容を使う。

## 新規計画

| order | change | 役割 | 着手条件 |
| --- | --- | --- | --- |
| root | `ui-core-root-plan` | KUC をフレームワーク非依存（framework-neutral）UI Core として再定義し、runtime / window / surface / adapter 境界を固定する親 change | この change を先に読む |
| implementation | `establish-kuc-atoms-molecules-catalog` | 01〜24 を KUC の atoms / molecules と部品カタログへ移管し、自動品質ゲートを正本化する change | `ui-core-root-plan` の境界を確認後に読む |

## 進行ルール

- 新規作業では `katana-ui-core` / `KUC` 表記を使う。
- 旧リポジトリ名や旧略称は archive 済み change の履歴説明だけに残す。
- repo 外の実装挙動が必要な場合は、先に `docs/inventory/<topic>.md` へ画面・操作・入力・出力・状態遷移をコピーしてから実装する。
- 中核 crate（core crate）は `floem` / `egui` / `gpui` を直接依存に持たない。
- 画面フレームワーク（UI framework）固有の型は変換層 crate（adapter crate）に閉じる。
- Storybook は KUC の部品カタログであり、KUC の中立 model と専用 visual surface で表示する。
- 部品の正しさは Storybook 目視ではなく、自動テスト、画像回帰、入力回帰、静的検査を主根拠にする。
- 依存境界は `docs/dependency-policy.md` と `docs/directory-structure.md` を基準にする。

## Superseded / archive candidates

| change | 現在の扱い |
| --- | --- |
| `katana-widget-parity-backlog` | 01〜24 と追加 UI の入力元。要件は `establish-kuc-atoms-molecules-catalog` へ移管し、実装正本にはしない。 |
| `ui-core-interaction-visual-parity` | 旧 interaction / visual gate の入力元。Storybook 目視寄りの完了扱いは新 change の品質ゲートへ移す。 |
| `18-accordion` | 要件移管後の archive 候補。 |
| `23-color-picker-complete-parity` | 要件移管後の archive 候補。 |
| `24-code-diff` | 要件移管後の archive 候補。 |

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
- AI vendor control
- chat composer
- linter result list
- workspace file tree
- editor gutter / ruler
- document preview / TOC
- application title bar / status bar / command palette の domain logic

入れるか迷う UI は `docs/widget-extraction-policy.md` の採用条件で判定する。
