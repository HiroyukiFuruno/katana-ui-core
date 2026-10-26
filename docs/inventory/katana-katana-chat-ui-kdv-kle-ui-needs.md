# 外部UI観測（履歴）とKUC汎用UI要件への翻訳

作成日: 2026-05-20

> この文書の外部repository名、path、画面責務、利用元は、当時の外部観測を再現可能にするための履歴証拠であり、KUCの現行実装要件・公開API・release gateではない。現行要件は下記の汎用 atoms / molecules 表へ翻訳し、外部repositoryの採用・受入・Issue状態をKUCの完了条件へ持ち込まない。

## 結論

KUC が持つ対象は、最小部品（atoms）と複合部品（molecules）までに限定する。
画面全体の構造（organisms）、画面ひな形（templates）、本文エディター、本文プレビュー、チャット画面全体は、利用側が KUC 部品を組み合わせて実装する。

そのため本棚卸しでは、利用側画面をそのまま KUC に移すのではなく、利用側が自前で画面を組むために必要な KUC の atoms / molecules だけを抽出する。

## 外部観測の読み取り元（履歴証拠。現行要件ではない）

### katana

- `crates/katana-ui/src/views/app_frame/ui.rs`
- `crates/katana-ui/src/views/app_frame/types.rs`
- `crates/katana-ui/src/views/top_bar/workspace_tab_bar*.rs`
- `crates/katana-ui/src/views/top_bar/tab_bar/**/*.rs`
- `crates/katana-ui/src/views/top_bar/search.rs`
- `crates/katana-ui/src/views/top_bar/status_bar.rs`
- `crates/katana-ui/src/views/app_frame/sidebar/explorer/**/*.rs`
- `crates/katana-ui/src/views/app_frame/tab_toolbar.rs`
- `crates/katana-ui/src/views/app_frame/central_content.rs`
- `crates/katana-ui/src/views/panels/editor/**/*.rs`
- `crates/katana-ui/src/views/panels/preview/**/*.rs`
- `crates/katana-ui/src/views/panels/toc/render.rs`
- `crates/katana-ui/src/views/panels/problems/**/*.rs`
- `crates/katana-ui/src/views/panels/explorer/**/*.rs`
- `crates/katana-ui/src/views/panels/tree/**/*.rs`
- `crates/katana-ui/src/settings/**/*.rs`
- `crates/katana-ui/src/views/modals/**/*.rs`
- `crates/katana-ui/src/views/diff_viewer/**/*.rs`

### katana-chat-ui

- `crates/katana-chat-ui/src/surface/model.rs`
- `crates/katana-chat-ui/src/surface/build.rs`
- `crates/katana-chat-ui/src/surface/composer.rs`
- `crates/katana-chat-ui/src/surface/message.rs`
- `crates/katana-chat-ui/src/render_model.rs`
- `crates/katana-chat-ui/src/message.rs`
- `crates/katana-chat-ui/src/input/attachment.rs`
- `crates/katana-chat-ui/src/input/path_drop.rs`
- `crates/katana-chat-ui/src/adapter_ui/**/*.rs`
- `crates/katana-chat-ui/src/usage.rs`
- `crates/katana-chat-ui-adapter/src/widget/**/*.rs`
- `crates/katana-chat-ui-adapter/src/**/*.rs`
- `crates/katana-chat-ui-adapter/src/**/*.rs`

### KDV / KLE

- `katana-document-viewer/openspec/project.md`
- `katana-document-viewer/docs/ui-separation-plan.md`
- `katana-document-viewer/openspec/changes/v0-1-0-document-preview-extraction/specs/markdown-preview-component/spec.md`
- `katana-language-editor/openspec/project.md`
- `katana-language-editor/docs/ui-separation-plan.md`
- `katana-language-editor/openspec/changes/v0-1-0-language-editor-extraction/specs/language-editor-*.md`

## 汎用境界へ翻訳する対象外画面

以下は特定製品の実装担当を定める表ではなく、KUCが公開する部品境界を示す現行の汎用分類である。大きな画面・本文・shellは利用側が組み、KUCは再利用可能なleaf contractだけを提供する。

| 対象 | 画面上でどう見えるか | 何を操作するものか | KUC の扱い |
| --- | --- | --- | --- |
| 本文エディター | 行番号、カーソル、選択範囲、診断マーカー付きの大きな編集面 | 文書本文を編集する | 利用側が実装する。KUC は周辺の `TextArea`、`ContextMenu`、`Toolbar`、`HoverCard`、`DiagnosticsList` の汎用contractを提供する |
| 本文プレビュー | 文書・図表・画像・表を表示するスクロール面 | 閲覧、選択、スクロール、リンク操作 | 利用側が実装する。KUC は `ImageSurface`、`TreeView`、`Toolbar`、`Popover`、`StatusBar`、`EmptyState` などの汎用contractを提供する |
| チャット画面全体 | header + message list + composer の縦積み画面 | 入力、履歴閲覧、提供元切替 | 利用側が実装する。KUC は入力、チップ、メニュー、カード、進捗表示などの汎用contractを提供する |
| アプリの shell | title bar、sidebar、main、status bar を含む画面全体 | アプリ全体のナビゲーション | 利用側が組む。KUC は小部品とstate/action/event contractだけを提供する |
| splash 画面テンプレート | 起動ロゴ、version、loading、retry を画面中央に出す | 起動状態を示す | template としては KUC 対象外。KUC は `Progress`、`Banner`、`EmptyState`、`Skeleton` を提供する |

## KUC が持つべき汎用 atoms / molecules（現行設計の根拠）

以下の表の「主な利用用途」は観測時点の分類例であり、KUCが依存する特定hostや外部pathを意味しない。現状 / gap は作成時点の棚卸しであり、現在の未実装判定ではない。現行の完了判定は公開API、contract、回帰、guardで行う。

| 優先 | UI 群 | 画面上でどう見えるか | 何を操作するものか | 主な利用用途 | 現状 / gap（作成時点） |
| --- | --- | --- | --- | --- | --- |
| 00 | scroll area contract | 長い内容を独立して縦 / 横に動かす領域 | offset、scrollbar、外部 scroll command、edge 到達を扱う | 本文・検索結果・履歴・パネルのスクロール領域 | `ScrollArea` の typed axis / offset / extent / event / scrollbar 契約を棚卸し |
| 00 | split pane contract | 左右または上下の 2 領域を境界線で分ける | 境界線 drag、keyboard resize、比率 reset | 編集・プレビュー・ナビゲーション・検査パネル | `SplitPane` の pane slot、ratio clamp、typed action / event、handle propsを棚卸し |
| 01 | 文脈メニュー（context menu） | 右クリック位置やボタン付近に出る縦メニュー | action、submenu、toggle、radio、shortcut 表示を選ぶ | タブ・ツリー・本文・出力周辺 | `ContextMenu` のshared overlay / placementと利用側presetを棚卸し |
| 02 | drag and drop primitive | ドラッグ中の影、drop 線、drop 領域 | 並べ替え、添付 drop、ツリー移動 | タブ・ツリー・添付・並べ替え | 共通 DnD model の要否を棚卸し |
| 03 | closeable tab strip | 横並びの tab、close、dirty dot、overflow | 項目・session の切替、閉じる、並べ替え | 文書・workspace・sessionのタブ領域 | `Tabs`との差分とdomain-free tab strip境界を棚卸し |
| 04 | overlay placement / popover / hover card | 対象の近くに浮く説明・詳細・小パネル | hover / focus / click で詳細を開閉 | 診断・画像・図表・ツールチップ | `Popover` / `Tooltip` のrich content、delay、arrow、pointer-followを棚卸し |
| 05 | toolbar overflow / action rail | 横並びの icon button 群と、入り切らない分の menu | command 実行、表示切替、overflow 操作 | 本文・プレビュー・入力欄のaction rail | `Toolbar` のoverflow partition、priority、roving focusを棚卸し |
| 06 | multiline text input | 複数行の入力欄、placeholder、IME 下線 | composer、簡易メモ、form 長文入力 | 入力欄・設定・検索周辺 | `Input`との差分と`TextArea` atomの境界を棚卸し |
| 07 | chip / attachment chip / chip group | 丸い pill、アイコン、ラベル、削除ボタン、進捗 | 添付、filter、tag、paste preview を表す | 添付一覧・filter・tag・paste preview | KUCにChip系atom / moleculeが不足（作成時点） |
| 08 | diagnostics list | severity icon、message、location、fix button の一覧 | 問題を選ぶ、修正候補を実行する | 問題一覧・検査結果・出力一覧 | `List` / `TreeView`のseverity / location / action contractを棚卸し |
| 09 | empty state | 空の領域に icon、heading、説明、action を表示 | 何もない状態から次 action へ誘導 | 空の一覧・履歴・検索結果 | KUCにEmptyState moleculeが不足（作成時点） |
| 10 | inline banner | 画面内上部に残る警告・成功・情報表示 | 閉じる、詳細を見る、再試行する | 設定警告・未設定・保存エラー | persistent inline表示のcontractが不足（作成時点） |
| 11 | toast stack | 画面端に一時通知が積まれる | 完了通知、エラー通知、進捗通知 | 保存・実行・同期の通知領域 | 単一 `NotificationToast`との差分をqueue / dedupe / pause-on-hoverで棚卸し |
| 12 | status segment / progress meter | 下部 status bar、usage 円グラフ、progress 表示 | 状態確認、対象 segment の action | 状態表示・使用量・進捗領域 | `StatusBar` / `ProgressBar`のmulti segmentとring meterを棚卸し |
| 13 | shortcut combo / cheatsheet | `Cmd + Shift + P` のようなキー列 | shortcut 表示、検索、選択 | command検索・toolbar tooltip・設定 | `KeyCap`単体との差分をcombo / platform表現で棚卸し |
| 14 | sectioned settings form | 左カテゴリ、右に section と入力 rows | 設定値の変更、reset、検索 | 設定画面・構成フォーム | `SettingsList`のcontrol kind、dirty、filter、reset contractを棚卸し |
| 15 | collapsible / resizable panel | 折りたためる横パネル、hover overlay、resize handle | サイドパネルの開閉・固定・幅変更 | ナビゲーション・目次・履歴パネル | AppShellは対象外。panel moleculeとしてだけ扱う |
| 16 | virtualized list / tree | 大量 row のうち見える範囲だけ描画 | scroll、focus row 維持、selection | ファイル・目次・診断・履歴・検索結果 | 統一optionの要否を棚卸し（作成時点） |
| 17 | skeleton loader | 読み込み中に灰色の形だけを表示 | loading 中の layout shift を抑える | 一覧・preview・履歴のloading表示 | motion依存をoptionalにする方針を棚卸し |
| 18 | motion primitives | fade / slide / scale / shimmer の数値 token | 開閉、hover、loading の動き | overlay、toast、skeleton | 現行追加は妥当。reduced-motion を contract 化する |
| 19 | window control button group | close / minimize / maximize の小ボタン群 | window command を通知する | window controlを配置するheader領域 | title bar / window chrome全体は対象外。button group moleculeまで |
| 20 | startup state composition | loading / error / retry を持つ小さな状態面 | 起動・初期化状態を表示する | 起動・初期化・再試行の状態面 | splash templateは対象外。KUCはstate panelか既存moleculeの組合せを提供 |
| 21 | command launcher / search results | 上部入力欄と、icon / label / shortcut 付き結果一覧 | command 検索、履歴検索、項目選択 | command検索・履歴・候補一覧 | result row、group、keyboard selection、shortcut badge、disabled guard、virtualized highlightをKUC contractへ整理。domain registryは利用側 |
| 22 | search control strip | 検索欄の横に match case / whole word / regex / 前後移動 / 件数が並ぶ | 検索条件、前後移動、replace requestを利用側へ通知する | 検索・置換・履歴検索のcontrol row | `SearchBox`との差分であるoption、件数、replace、navigation eventを棚卸し |
| 23 | preview surface image | 描画済みのpreview surfaceを表示する領域 | fit、scale、検索highlight rectをadapterへ渡す | preview・exportのsurface表示 | `UiTree` / `PaintRequest`のRGBA `ImageSurface`契約を棚卸し |

## 既存 UI では option で補えない差分

| 既存 UI | 補えない差分 | 必要な change |
| --- | --- | --- |
| `ScrollArea` | axis、offset、viewport/content extent、scrollbar、外部 scroll command、edge event は typed contract へ移管済み | `00-add-scroll-area-contract` |
| `SplitPane` | 2 pane contract、min/max/reset ratio、drag/keyboard resize event、persistence 境界は `00-add-split-pane-contract` へ移管済み | `00-add-split-pane-contract` |
| `Tabs` | segmented 切替と closeable / dirty / draggable / grouped tab は状態が違う | `03-add-workspace-tab-bar` を domain-free tab strip に修正 |
| `MenuButton` / `Menu` | pointer 座標起点、submenu path、outside click reason、type-ahead を持てない | `01-add-context-menu` |
| `Popover` / `Tooltip` | rich content、hover delay、arrow、pointer-follow、shared placement が不足 | `04-add-rich-popover-and-hover-card` |
| `Toolbar` | action の優先度、幅測定、overflow menu、roving focus が不足 | `05-add-toolbar-overflow` |
| `Input` | 複数行、IME composition、submit shortcut、auto height が不足 | `06-add-multiline-text-input` |
| `Badge` | dismiss、status、thumbnail、progress、chip group overflow が不足 | `07-add-chip-and-attachment-chip` |
| `List` / `TreeView` | severity、location、fix action、bulk action、filter chip が typed でない | `08-add-diagnostics-list` |
| `StatusBar` / `ProgressBar` | segment action、ring / pie meter、usage tooltip が不足 | `12-add-multi-segment-status-bar` |
| `KeyCap` | modifier + key の platform 表示、検索可能 cheatsheet が不足 | `13-add-shortcut-combo-display` |
| `CommandPalette` | result row / provider group / keyboard selection / shortcut badge / virtualized highlight は `21-add-command-launcher-results` で対応済み。provider dispatch と app command registry は domain 側に残す | `21-add-command-launcher-results` |
| `SearchBox` | match case / whole word / regex、前後移動、件数、replace controls は `SearchControlStrip` の typed state / action / event / Storybook settings へ移管済み | `22-add-search-control-strip` |
| なし | 利用側が描画済みの RGBA preview surface、content scale、fit、accessibility label、検索 highlight rectを`UiTree` / integration planに載せる契約がない | `23-add-preview-surface-image-contract` |

## 利用側が組む organisms / templates

| 利用側 | KUC 部品を使って組むもの |
| --- | --- |
| preview利用側 | preview surface、navigation panel、image / diagram overlay controls、export side panel。ただし描画済み RGBA payloadはKUC `ImageSurface` nodeに載せられる |
| editor利用側 | editor surface、gutter、selection、diagnostic decoration、find bar |
| conversation利用側 | chat root、message thread、composer、adapter controls、history panel |
| application利用側 | app frame、workspace shell、title area、dashboard、splash screen |

KUC はこれらを直接提供しない。
KUC は、上記 organisms / templates を構築するための atoms / molecules と、その state / action / event / rendering contract を提供する。
