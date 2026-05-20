# katana / katana-chat-ui / KDV / KLE UI 要求棚卸し

作成日: 2026-05-20

## 結論

KUC が持つ対象は、最小部品（atoms）と複合部品（molecules）までに限定する。
画面全体の構造（organisms）、画面ひな形（templates）、本文エディター、本文プレビュー、チャット画面全体は、KDV / KLE / katana-chat-ui / katana など利用側が KUC 部品を組み合わせて実装する。

そのため本棚卸しでは、利用側画面をそのまま KUC に移すのではなく、利用側が自前で画面を組むために必要な KUC の atoms / molecules だけを抽出する。

## 読み取り元

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
- `crates/katana-chat-ui/src/vendor_ui/**/*.rs`
- `crates/katana-chat-ui/src/usage.rs`
- `crates/katana-chat-ui-floem/src/widget/**/*.rs`
- `crates/katana-chat-ui-egui/src/**/*.rs`
- `crates/katana-chat-ui-gpui/src/**/*.rs`

### KDV / KLE

- `katana-document-viewer/openspec/project.md`
- `katana-document-viewer/docs/ui-separation-plan.md`
- `katana-document-viewer/openspec/changes/v0-1-0-document-preview-extraction/specs/markdown-preview-component/spec.md`
- `katana-language-editor/openspec/project.md`
- `katana-language-editor/docs/ui-separation-plan.md`
- `katana-language-editor/openspec/changes/v0-1-0-language-editor-extraction/specs/language-editor-*.md`

## 対象外にする画面

| 対象 | 画面上でどう見えるか | 何を操作するものか | KUC の扱い |
| --- | --- | --- | --- |
| 本文エディター | 行番号、カーソル、選択範囲、診断マーカー付きの大きな編集面 | 文書本文を編集する | KLE が実装する。KUC は周辺の `TextArea`、`ContextMenu`、`Toolbar`、`HoverCard`、`DiagnosticsList` を提供する |
| 本文プレビュー | Markdown / 図表 / 画像 / 表を表示するスクロール面 | 文書の閲覧、選択、スクロール、リンク操作 | KDV が実装する。KUC は `TreeView`、`Toolbar`、`Popover`、`StatusBar`、`EmptyState` などを提供する |
| チャット画面全体 | header + message list + composer の縦積み画面 | AI への入力、履歴閲覧、提供元切替 | katana-chat-ui が実装する。KUC は入力、チップ、メニュー、カード、進捗表示などを提供する |
| アプリの shell | title bar、sidebar、main、status bar を含む画面全体 | アプリ全体のナビゲーション | katana / KDV / KLE / chat 側が組む。KUC は小部品のみ |
| splash 画面テンプレート | 起動ロゴ、version、loading、retry を画面中央に出す | 起動状態を示す | template としては KUC 対象外。KUC は `Progress`、`Banner`、`EmptyState`、`Skeleton` を提供する |

## KUC が持つべき atoms / molecules

| 優先 | UI 群 | 画面上でどう見えるか | 何を操作するものか | 主な利用元 | 現状 / gap |
| --- | --- | --- | --- | --- | --- |
| 00 | scroll area contract | 長い内容を独立して縦 / 横に動かす領域 | offset、scrollbar、外部 scroll command、edge 到達を扱う | KDV viewer 周辺、KLE find 周辺、katana panels、chat history、Storybook | `ScrollArea` は typed axis / offset / extent / event / scrollbar 契約へ移管済み |
| 00 | split pane contract | 左右または上下の 2 領域を境界線で分ける | 境界線 drag、keyboard resize、比率 reset | katana editor-preview、KDV TOC-viewer、Storybook panels | `SplitPane` はあるが 2 pane contract、ratio clamp、event、`CollapsiblePanel` との境界が不足 |
| 01 | 文脈メニュー（context menu） | 右クリック位置やボタン付近に出る縦メニュー | action、submenu、toggle、radio、shortcut 表示を選ぶ | katana tab / explorer / editor 周辺、chat output | `ContextMenu` はあるが、shared overlay / placement と consumer preset の整理が必要 |
| 02 | drag and drop primitive | ドラッグ中の影、drop 線、drop 領域 | 並べ替え、添付 drop、ツリー移動 | katana tab / explorer、chat attachment | 現行 KUC に共通 DnD model が不足 |
| 03 | closeable tab strip | 横並びの tab、close、dirty dot、overflow | 文書・session の切替、閉じる、並べ替え | katana document tabs / workspace tabs | `Tabs` は segmented 用で、closeable / draggable / grouped tab には option 不足。workspace domain は入れない |
| 04 | overlay placement / popover / hover card | 対象の近くに浮く説明・詳細・小パネル | hover / focus / click で詳細を開閉 | diagnostics hover、image/diagram controls、vendor tooltip | `Popover` / `Tooltip` はあるが rich content、delay、arrow、pointer-follow、shared placement が不足 |
| 05 | toolbar overflow / action rail | 横並びの icon button 群と、入り切らない分の menu | command 実行、表示切替、overflow 操作 | editor 周辺 toolbar、preview side rail、chat composer footer | `Toolbar` はあるが overflow partition、priority、roving focus が不足 |
| 06 | multiline text input | 複数行の入力欄、placeholder、IME 下線 | chat composer、簡易メモ、form 長文入力 | chat composer、settings、KLE 周辺入力 | `Input` はあるが `TextArea` atom が不足。KLE 本文 editor そのものは対象外 |
| 07 | chip / attachment chip / chip group | 丸い pill、アイコン、ラベル、削除ボタン、進捗 | 添付、filter、tag、paste preview を表す | chat attachment tray、explorer filter、diagnostic filter | KUC に Chip 系 atom / molecule が不足 |
| 08 | diagnostics list | severity icon、message、location、fix button の一覧 | 問題を選ぶ、修正候補を実行する | katana problems、KLE diagnostics、chat tool output | `List` / `TreeView` では severity / location / action の typed contract が不足 |
| 09 | empty state | 空の領域に icon、heading、説明、action を表示 | 何もない状態から次 action へ誘導 | explorer empty、history empty、search empty | KUC に EmptyState molecule が不足 |
| 10 | inline banner | 画面内上部に残る警告・成功・情報表示 | 閉じる、詳細を見る、再試行する | settings warning、provider missing、save error | toast / modal / status bar と違う persistent inline 表示が不足 |
| 11 | toast stack | 画面端に一時通知が積まれる | 保存通知、完了通知、エラー通知 | katana save、chat agent events | 単一 `NotificationToast` はあるが queue / dedupe / pause-on-hover が不足 |
| 12 | status segment / progress meter | 下部 status bar、usage 円グラフ、progress 表示 | 状態確認、対象 segment の action | katana status、chat usage、export progress | `StatusBar` / `ProgressBar` はあるが multi segment と circular / ring meter が不足 |
| 13 | shortcut combo / cheatsheet | `Cmd + Shift + P` のようなキー列 | shortcut 表示、検索、選択 | command palette、toolbar tooltip、settings | `KeyCap` 単体では combo / platform 差分が不足。現行追加は妥当 |
| 14 | sectioned settings form | 左カテゴリ、右に section と入力 rows | 設定値の変更、reset、検索 | katana settings、chat settings、KLE/KDV config | `SettingsList` はあるが control kind、dirty、filter、reset の契約を強める必要 |
| 15 | collapsible / resizable panel | 折りたためる横パネル、hover overlay、resize handle | サイドパネルの開閉・固定・幅変更 | explorer sidebar、TOC panel、history panel | AppShell は対象外。panel molecule としてだけ扱う |
| 16 | virtualized list / tree | 大量 row のうち見える範囲だけ描画 | scroll、focus row 維持、selection | explorer、TOC、diagnostics、history、command results | 現行追加は妥当だが対象 molecule へ統一 option を付ける必要 |
| 17 | skeleton loader | 読み込み中に灰色の形だけを表示 | loading 中の layout shift を抑える | explorer、preview、chat history | 現行追加は妥当。motion 依存は optional にする |
| 18 | motion primitives | fade / slide / scale / shimmer の数値 token | 開閉、hover、loading の動き | overlay、toast、skeleton | 現行追加は妥当。reduced-motion を contract 化する |
| 19 | window control button group | close / minimize / maximize の小ボタン群 | window command を通知する | katana title area、chat header | title bar / window chrome 全体は対象外。button group molecule まで |
| 20 | startup state composition | loading / error / retry を持つ小さな状態面 | 起動・初期化状態を表示する | app splash、session loading | splash template は対象外。KUC は state panel か既存 molecule の組合せを提供 |
| 21 | command launcher / search results | 上部入力欄と、icon / label / shortcut 付き結果一覧 | command 検索、slash command、履歴検索 | katana command palette、chat slash launcher | `CommandPalette` はあるが result row、provider、keyboard selection、shortcut badge が不足 |
| 22 | search control strip | 検索欄の横に match case / whole word / regex / 前後移動 / 件数が並ぶ | 検索条件、前後移動、replace request を consumer へ通知する | katana search modal、KLE find/replace、KDV viewer search、chat history search | `SearchBox` はあるが検索 option、件数、replace、navigation event が不足 |

## 既存 UI では option で補えない差分

| 既存 UI | 補えない差分 | 必要な change |
| --- | --- | --- |
| `ScrollArea` | axis、offset、viewport/content extent、scrollbar、外部 scroll command、edge event は typed contract へ移管済み | `00-add-scroll-area-contract` |
| `SplitPane` | 2 pane contract、min/max/reset ratio、drag/keyboard resize event、persistence 境界が曖昧 | `00-add-split-pane-contract` |
| `Tabs` | segmented 切替と closeable / dirty / draggable / grouped tab は状態が違う | `03-add-workspace-tab-bar` を domain-free tab strip に修正 |
| `MenuButton` / `Menu` | pointer 座標起点、submenu path、outside click reason、type-ahead を持てない | `01-add-context-menu` |
| `Popover` / `Tooltip` | rich content、hover delay、arrow、pointer-follow、shared placement が不足 | `04-add-rich-popover-and-hover-card` |
| `Toolbar` | action の優先度、幅測定、overflow menu、roving focus が不足 | `05-add-toolbar-overflow` |
| `Input` | 複数行、IME composition、submit shortcut、auto height が不足 | `06-add-multiline-text-input` |
| `Badge` | dismiss、status、thumbnail、progress、chip group overflow が不足 | `07-add-chip-and-attachment-chip` |
| `List` / `TreeView` | severity、location、fix action、bulk action、filter chip が typed でない | `08-add-diagnostics-list` |
| `StatusBar` / `ProgressBar` | segment action、ring / pie meter、usage tooltip が不足 | `12-add-multi-segment-status-bar` |
| `KeyCap` | modifier + key の platform 表示、検索可能 cheatsheet が不足 | `13-add-shortcut-combo-display` |
| `CommandPalette` | result row の icon / secondary text / shortcut badge / provider dispatch が不足 | `21-add-command-launcher-results` |
| `SearchBox` | match case / whole word / regex、前後移動、件数、replace controls は `SearchControlStrip` の typed state / action / event / Storybook settings へ移管済み | `22-add-search-control-strip` |

## 利用側が組む organisms / templates

| 利用側 | KUC 部品を使って組むもの |
| --- | --- |
| KDV | viewer surface、TOC panel、image / diagram overlay controls、export side panel |
| KLE | editor surface、gutter、selection、diagnostic decoration、find bar |
| katana-chat-ui | chat root、message thread、composer、vendor controls、history panel |
| katana | app frame、workspace shell、title area、dashboard、splash screen |

KUC はこれらを直接提供しない。
KUC は、上記 organisms / templates を構築するための atoms / molecules と、その state / action / event / rendering contract を提供する。
