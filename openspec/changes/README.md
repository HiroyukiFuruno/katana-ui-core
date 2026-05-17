# OpenSpec Changes — katana-ui-core v0.1 ロードマップ

`../katana` (egui) と `../katana-chat-ui` (Floem) の中で「Katana domain に依存しない汎用 UI 部品」を抽出し、Floem 前提の共通 widget として独立させるための change 群。

## 進行ルール

- 番号は **0 埋め 2 桁の実施順序**（00 → 24）。前提が後段にあるため上から順に着手する。
- 階層は `theme / primitive / composite{button,selector,input,indicator} / layout`。階層命名はディレクトリのみ、widget 名にカテゴリ語（atom 等）は付けない。
- 依存方向は ast-lint (00 で導入) で機械的に強制: `theme ← primitive ← composite ← layout`。`composite` 配下のサブカテゴリ間（button / selector / input / indicator）は横断不可。同一サブカテゴリ内の sub-widget 間は許可。
- すべての widget change は **Storybook ページ追加** を DoD に含める。Storybook はリポジトリルート `/storybook/` の独立 Cargo プロジェクト（`crates/` 外、workspace member ではない）。
- 抽出元（参考、コードコピーではなく仕様抽出）:
  - `../katana/crates/katana-ui/src/widgets/`
  - `../katana/crates/katana-ui/src/views/layout/`
  - `../katana-chat-ui/crates/katana-chat-ui-floem/src/widget/`

## 一覧

| # | name | 階層 | 備考 |
|---|---|---|---|
| 00 | bootstrap-dev-environment | meta | KML 標準差分の取り込み / ast-lint / Storybook 立上げ / kuw-workflow-guide skill / docs |
| 01 | theme-tokens | theme | color / spacing / typography トークンと light/dark 既定値 |
| 02 | text-primitive | primitive/text | typography 役割を統一する Text |
| 03 | icon-primitive | primitive/icon | SVG bytes / string を渡す最小アイコン (registry なし) |
| 04 | spinner-primitive | primitive/spinner | インデターミネートローディング |
| 05 | svg-button | composite/button/svg | アイコンのみのボタン (variant × tone × state) |
| 06 | text-button | composite/button/text | テキストラベルボタン |
| 07 | icon-text-button | composite/button/icon_text | アイコン + ラベルボタン |
| 08 | toggle | composite/selector/toggle | フラット on/off トグル |
| 09 | segmented-toggle | composite/selector/segmented | 排他選択セグメント |
| 10 | select-box | composite/selector/select | 単一選択ドロップダウン (検索なし) |
| 11 | color-swatch | composite/selector/color | パレット選択 swatch grid |
| 12 | text-input | composite/input/text | 単行入力 (leading icon / trailing slot / invalid) |
| 13 | search-box | composite/input/search | 検索専用入力 (Esc/Enter/clear) |
| 14 | tooltip | composite/indicator/tooltip | hover/focus 注釈 (暫定 popup → 21 で popover 化) |
| 15 | badge | composite/indicator/badge | ステータス / カテゴリラベル |
| 16 | key-cap | composite/indicator/key_cap | キーボードショートカット表示 |
| 17 | card | layout/card | 枠 + padding + 角丸コンテナ |
| 18 | accordion | layout/accordion | 折り畳みセクション |
| 19 | split-pane | layout/split | 2 ペイン可変サイズ分割 |
| 20 | modal-overlay | layout/modal | 全画面ダイアログ + フォーカストラップ |
| 21 | popover | layout/popover | アンカー型 overlay (10 / 14 を内部置換) |
| 22 | rgba-color-picker | composite/selector/color_picker | RGBA 編集できる色選択 UI |
| 23 | color-picker-complete-parity | composite/selector/color_picker | ColorPicker を実用水準へ作り直す追従 change |
| 24 | code-diff | composite/code_diff | 2つのコード文字列を見比べる汎用差分表示 |

## 除外（Katana domain）

以下は本 widget crate のスコープ外。消費側（katana / katana-chat-ui）で自前実装するか、`katana-ui-katana-domain` 等のドメインクレートで管理する。

- markdown 描画 / KMM 連携 (`markdown_hooks`, `markdown` widget)
- AI vendor 制御 (`vendor_ui`, `vendor_controls`, `vendor_control_parts`)
- chat 専用 (`composer/thinking/usage/output_cards`)
- Katana 固有の diff / linter 表示 (`diff_viewer`, `problems`, lint 関連)。ただし、2つのコード文字列だけを見比べる汎用 `CodeDiff` は `24-code-diff` で扱う。
- workspace ファイルツリー / エディタ / プレビュー / TOC など Katana ワークスペース固有
- アプリ frame (`breadcrumbs`, `tab_toolbar`, `title_bar`, `status_bar`, `command_palette` の domain ロジック)

汎用化が見えた段階で、本ロードマップに追加 change として組み込む。

## 命名と粒度のルール

- ディレクトリ配下のファイルが 10 を超えたら **関心事で分割**する（例: 12 個になる `composite/` 直下は `button/` `selector/` `input/` `indicator/` に分割済み）。
- 各 widget モジュールは `mod.rs` / `types.rs` / `ops.rs`（stateful時のみ）/ `view.rs` / `tests.rs` の慣例に従う。
- 数値・色を直書きせず `theme/` トークンを参照する。違反は `kuw-workflow-guide` skill と将来の lint で抑止。
