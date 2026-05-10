## Why

katana と katana-chat-ui で繰り返し使われる UI が widget 化されておらず、Storybook 上にも利用者視点の部品契約が不足している。
このまま個別実装を進めると、見せかけのサンプルや再利用できない UI が増えるため、先に汎用 widget の対象と要件を固定する。

## What Changes

- ProgressBar、Tabs、Breadcrumb、SideMenu、SelectionList、SlideControl、DynamicArrayEditor、AlignCenterWrapper を新規 widget 化対象として定義する。
- katana / katana-chat-ui を横断して、複数回利用されている UI を洗い出し、widget 化対象に追加する task を作る。
- 既存の `12-text-input`、`13-search-box`、`14-tooltip`、`17-card`、`18-accordion`、`20-modal-overlay`、`21-popover`、`22-rgba-color-picker` は、この backlog の前提として再実装・再検証が必要な既存 scope として扱う。
- Tabs は content あり / なしの両方を扱い、content なしの場合は callback による外部 UI 連動を可能にする。
- SideMenu は左右配置、幅指定、hover 展開、SVG アイコン配列、アイコンごとの callback、アイコンからの pop 表示を扱う。
- 画像2枚目のような section label、選択行、色付きマーカー、もっと表示、数値 slider を含む設定リスト表現を定義する。
- 画像3枚目のような配列を動的に追加 / 削除 / 並び替えできる UI を定義する。

### 横断調査で追加された widget 候補（2026-05-11）

以下は katana (`katana-ui/src/`) と katana-chat-ui (`katana-chat-ui-floem/src/widget/`) のソースコード横断調査で発見された、widget 化すべき汎用 UI 部品である。README 除外リストの「汎用化が見えた段階で追加」方針に基づき、ドメインロジックを分離した上で汎用構造として定義する。

- **TreeView**: 階層データの展開・折り畳み表示。katana の explorer (`views/panels/explorer/`)、TOC (`views/panels/toc/`)、settings tree (`settings/settings_tree.rs`) で繰り返し利用されるパターン。ファイルツリー、設定ツリー、目次など広く使える。
- **ComboBox**: テキスト入力 + ドロップダウン選択。katana の `widgets/combo_box/` で定義済みだが widget crate に抽出されていない。SelectBox (10) の上位互換として基本フォーム widget に位置付ける。
- **MenuButton**: ボタンクリックでドロップダウンメニューを開く widget。katana の `widgets/menu_button/` で定義済みで、breadcrumbs やコンテキストメニューで利用される。
- **CommandPalette**: 検索入力 + フィルタ可能な結果リスト + キーボードナビゲーション。katana の `views/modals/command_palette.rs` が実装。ドメイン（provider / payload）を分離すれば、汎用の「検索可能リストオーバーレイ」として再利用可能。
- **StatusBar**: アイコン付きステータスメッセージ + アクションボタンを表示する水平バー。katana の `views/top_bar/status_bar.rs` が実装。severity（error / warning / success / info）に応じた配色とアイコンを扱う。
- **Toolbar**: SVG アイコンアクションの水平配置。katana-chat-ui の `widget/toolbar.rs` が実装。identity セクション + action セクションに分かれる汎用パターン。
- **LoadingDots**: アニメーション付きドットインジケーター。katana-chat-ui の `widget/thinking_indicator.rs` が実装。Spinner (04) とは異なる「テキスト横の点滅ドット」パターンで、非同期処理の汎用表示に使える。
- **NotificationToast**: ステータスメッセージの一時表示。katana の StatusBar と status type (`Error / Warning / Success / Info`) パターンから抽出。時間経過で自動消去するトースト通知。

## Capabilities

### New Capabilities

- `progress-bar`: 進捗値、未確定進捗、ラベル、色、サイズを持つ progress 表示。
- `tabs`: タブ見出し、選択状態、content slot、callback 連動を持つ tab UI。
- `breadcrumb`: 階層パスを表示し、各 crumb の click callback を扱うパンくずリスト。
- `side-menu`: 左右配置、幅制御、hover 展開、SVG icon action、pop 表示を持つサイドメニュー。
- `selection-list`: 画像2枚目のような section label、選択行、色付き marker、もっと表示を扱うリスト。
- `slide-control`: 最小値 / 最大値 / step / 小数 / 整数 / 対象 binding を扱う slider UI。
- `dynamic-array-editor`: 画像3枚目のような配列 item の追加、削除、編集、並び替えを扱う UI。
- `align-center-wrapper`: katana の AlignCenter のように子要素を中央揃えする wrapper。
- `widget-inventory-audit`: katana / katana-chat-ui から widget 化漏れを洗い出す監査。
- `tree-view`: 階層データの展開・折り畳み・選択・アクティブ表示を持つツリー widget。katana explorer / TOC / settings tree 由来。
- `combo-box`: テキスト入力 + ドロップダウン選択。入力によるフィルタリング、自由入力許可 / 不許可を扱う。katana combo_box 由来。
- `menu-button`: ボタンクリックでドロップダウンメニューを開く widget。framed / unframed、任意メニュー content を扱う。katana menu_button 由来。
- `command-palette`: 検索入力 + フィルタ可能な結果リスト + キーボードナビゲーション（↑↓Enter/Esc）を持つオーバーレイ。katana command_palette 由来。
- `status-bar`: severity アイコン付きメッセージ + アクションボタン + 右寄せ情報を表示する水平バー。katana status_bar 由来。
- `toolbar`: SVG アイコンアクションの水平配置。leading / trailing セクション分離、gap 制御を扱う。katana-chat-ui toolbar 由来。
- `loading-dots`: アニメーション付きドットインジケーター。dot 数、サイズ、アニメーション速度、ラベル付きを扱う。katana-chat-ui thinking_indicator 由来。
- `notification-toast`: severity 付きメッセージの一時表示。自動消去、手動 dismiss、スタック表示を扱う。katana status type パターン由来。

### Modified Capabilities

- なし。

## Impact

- `crates/katana-ui-widget/src/composite` と `crates/katana-ui-widget/src/layout` に新規 widget が追加される。
- `storybook/src/pages` に各 widget の live sample が追加される。
- katana / katana-chat-ui で個別実装している UI の移植候補が整理される。
- OpenSpec の完了判定は、`tasks.md` の checkbox だけでなく、Storybook 上の実操作と widget API の再利用性を確認してから行う。
