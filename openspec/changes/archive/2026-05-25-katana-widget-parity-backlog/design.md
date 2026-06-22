## Overview

この change は実装を一気に行うためのものではなく、katana / katana-chat-ui 由来の汎用 UI を widget 化するための backlog を固定する。
各 widget は「見た目だけのサンプル」ではなく、利用側が状態、event、content を制御できる部品として扱う。

## Design Principles

- 画面上で操作できる live widget を Storybook に置く。
- 表示専用の疑似ラベルや resolved 値の説明で完了扱いにしない。
- Storybook では「何を操作するものか」と「操作結果がどこに反映されるか」を同じ画面内で確認できるようにする。
- content を受け取る部品は文字列だけでなく任意 node を受け取れる設計にする。
- icon は SVG を配列または slot として受け取り、各 icon に callback を設定できるようにする。
- dark / light の配色は theme token に追従する。
- disabled / readonly / controlled / uncontrolled の扱いを API と Storybook に明記する。

## Existing Scope Boundary

この backlog は新規 widget の一覧化だけを目的にしない。
既存の `12-text-input`、`13-search-box`、`14-tooltip`、`17-card`、`18-accordion`、`20-modal-overlay`、`21-popover`、`22-rgba-color-picker` は、完了済み checkbox が残っていても再実装 gate を通るまで完了扱いにしない。

既存 widget の再実装 gate:

- 実 widget が Storybook で操作できる。
- 操作結果が同じ画面内の preview / selected value / callback log に反映される。
- theme token による light / dark の見た目差分が確認できる。
- `RUSTFLAGS="-D warnings"`、`just storybook-check`、`just ast-lint` を完了条件に含める。
- ast-lint の file-length / type-separation が出た場合、除外ではなく責務分離で解決する。

## Visual Reference Handling

画像由来の UI は「似た雰囲気」ではなく、操作要素を checklist 化して実装する。
たとえば ColorPicker は、透明チェッカー、色ボタン、R/G/B/A 値、合成方式、色面、色相 slider、alpha slider、ドラッグハンドル、ポップパネルを個別の受け入れ条件に分解する。

## Widget Direction

### ProgressBar

確定値の進捗、未確定の進捗、label 付き表示、色 token、サイズ差分を持つ。
値は `0..=100` の percent と、任意範囲の value / max の両方を扱えるようにする。
Storybook では、進捗値の変更が bar 幅と label の両方に反映されることを確認する。

### Tabs

タブは content あり / なしを分ける。
content ありでは選択 tab に対応する node を表示し、content なしでは選択 callback で外部 UI を更新できるようにする。
katana の tab 実装から、閉じる、overflow、未保存状態の表現を採用する。並び替えはこの change では採用しない。

### Breadcrumb

階層 path を crumb 配列として受け取り、各 crumb は label、任意 icon、disabled、on_click を持つ。
省略表示と separator の変更を扱う。

### SideMenu

KatanA のアクティビティバー（activity rail）/ プレビュー側パネル（preview side panel）と同じく、主表示は細い暗色の縦アイコンバー（icon rail）とする。
左右配置、クリック（click）固定表示、ホバー（hover）一時表示、ホバー遅延、外側へポインターが離れたときの解除（pointer leave）を扱う。
SVGアイコン（SVG icon）配列を受け取り、アイコンごとにクリック処理（click callback）とポップ内容（pop content）を設定する。
左配置では内容パネル（panel）をバー右側へ、右配置ではバー左側へ表示する。
アイコンは上寄せグループ（group）と下寄せグループを持ち、選択中または内容表示中のアイコンはアクティブ背景（active background）で示す。
ポップ表示（pop）はモーダル風（modal）、ポップオーバー風（popover）、領域拡張型を選べるようにする。
SideMenu 自体がポップオーバー / モーダル / 重ね表示（overlay）に依存しすぎないよう、ポップ表示方式は列挙型（enum）と内容スロット（content slot）で切り替える。

### SelectionList

画像2枚目のように、section label、色付き marker、選択行 highlight、もっと表示、補助 control を持つ。
theme preset のような一覧に使える構成にする。

### SlideControl

整数 / 小数、最小値、最大値、step、現在値、単位、表示 format、適用先 binding を扱う。
適用先は HTML の id 的な識別子ではなく、利用側 callback と node 更新の両方を想定する。

### DynamicArrayEditor

画像3枚目のように、item 配列を追加 / 削除 / 編集 / 並び替えできる。
item の表示内容は上位から node として渡せるようにする。
空配列、削除不可 item、最大件数到達、並び替え不可 item を状態として扱う。

### AlignCenterWrapper

katana の AlignCenter と同じ用途で、子要素を縦横中央に置く wrapper とする。
幅、高さ、padding、gap、disabled 時の扱いを指定可能にする。

## 決定事項

- repo 外の対象 path は実装者に読ませない。必要な挙動は `docs/inventory/<widget>.md` にコピーしてから実装する。
- Modal の別ウィンドウ化と同一ウィンドウ overlay の分離は、既存 `20-modal-overlay` の design で扱う。
- 採用判定は 0/1 のみとし、未判定状態を作らない。

---

## 横断調査で追加された widget（2026-05-11）

以下は katana / katana-chat-ui のソースコード横断調査で採用判定を 1 とした汎用 widget である。

### TreeView

katana の explorer (`views/panels/explorer/`)、TOC (`views/panels/toc/render.rs`)、settings tree (`settings/settings_tree.rs`) で繰り返されるパターン。

**出典コード分析:**
- explorer: `dir_entry.rs` / `file_entry.rs` で expand/collapse、click select、hover highlight、indent を個別実装。
- TOC: `render.rs` で `render_leaf_item` / `render_parent_item` に分けて再帰的に階層を描画。active 表示、vertical line、force open を扱う。
- settings_tree: accordion ベースのツリー表示。

**widget 構造:**
- item は label、icon（任意）、indent level、expanded / collapsed、active、disabled を持つ。
- leaf item と parent item を区別し、parent は子の展開・折り畳みを制御する。
- on_select、on_expand、on_collapse callback を持つ。
- virtual scroll に対応し、大量 item でもパフォーマンスを維持する。
- 階層: `layout/tree`。Accordion (18) の上位構造として位置付ける。

### ComboBox

katana の `widgets/combo_box/` で実装済み。SelectBox (10) が検索なし単一選択であるのに対し、ComboBox はテキスト入力によるフィルタリングを加えた上位互換。

**widget 構造:**
- TextInput (12) + Popover (21) の組み合わせ。
- 入力によるフィルタリング、自由入力許可 / 不許可（strict mode）を API として持つ。
- 選択肢は label + value のペア。
- on_select、on_input_change callback を持つ。
- 階層: `composite/input/combo`。TextInput (12)、Popover (21) に依存。

### MenuButton

katana の `widgets/menu_button/` で実装。breadcrumbs の各セグメント (`views/app_frame/breadcrumbs.rs`) や context menu で利用。

**出典コード分析:**
- `MenuButtonOps::show_unframed` として呼び出される framed / unframed の 2 形態。
- ボタンクリックで content（任意 node）を popover / dropdown として表示。

**widget 構造:**
- trigger は任意 label / icon / node を受け取る。
- content は Popover (21) の content slot に相当。
- framed（ボタン枠あり）/ unframed（テキストリンク風）の variant。
- on_open、on_close callback を持つ。
- 階層: `composite/button/menu`。SVG Button (05)、Popover (21) に依存。

### CommandPalette

katana の `views/modals/command_palette.rs` で実装。検索入力 + 結果リスト + キーボードナビゲーション。

**出典コード分析:**
- 600px 幅の overlay。TextEdit + ScrollArea + 結果リスト。
- ↑↓ で選択移動、Enter で実行、Escape で閉じる。
- query prefix (`>`) でモード切り替え（ファイル検索 / コマンド検索）。
- provider パターンでドメインロジック（ファイル検索、コマンド一覧）を分離。

**widget 構造:**
- 検索入力、結果リスト表示、キーボードナビゲーションの汎用構造。
- result item は label、icon（任意）、shortcut（任意）、score、payload を持つ。
- provider trait / callback でドメインロジックを注入。
- Modal (20) の上に構築。
- 階層: `layout/command_palette`。Modal (20)、TextInput (12) に依存。

### StatusBar

katana の `views/top_bar/status_bar.rs` で実装。

**出典コード分析:**
- 左寄せ: severity icon + message text。
- 右寄せ: 進行中 indicator (spinner + filename)、dirty dot。
- severity は Error / Warning / Success / Info の 4 段階で色とアイコンを変える。
- problem count ボタンで ProblemsPanel を toggle。

**widget 構造:**
- leading slot: severity icon + message。
- trailing slot: 任意 node。
- severity enum で配色を制御。
- on_action callback を持つ。
- 階層: `layout/status_bar`。Icon (03)、Badge (15) に依存可能。

### Toolbar

katana-chat-ui の `widget/toolbar.rs` で実装。

**出典コード分析:**
- 左: identity section (provider icon + adapter selector + title)。
- 右: action section (new chat + history)。
- `justify_between` で左右に分離。

**widget 構造:**
- leading slot と trailing slot を任意 node として受け取る。
- gap、alignment、wrap を API として持つ。
- 階層: `layout/toolbar`。依存は最小（テーマトークンのみ）。

### LoadingDots

katana-chat-ui の `widget/thinking_indicator.rs` で実装。

**出典コード分析:**
- 3 つの dot を `h_stack` で並べ、各 dot に animation を適用。
- idle → active でサイズと透明度が変化し、auto_reverse + repeat で点滅。
- label text + dots の組み合わせ。
- trailing（user 側）/ leading（assistant 側）で背景色を変更。

**widget 構造:**
- dot 数、dot サイズ（idle / active）、アニメーション速度を API として持つ。
- label あり / なしの variant。
- tone（配色）を theme token に追従。
- Spinner (04) との棲み分け: Spinner は回転アニメーション、LoadingDots は点滅ドット。
- 階層: `primitive/loading_dots`。依存は最小（テーマトークンのみ）。

### NotificationToast

katana の StatusBar の status type パターン（Error / Warning / Success / Info）と、一般的なトースト通知 UI の融合。

**widget 構造:**
- severity（Error / Warning / Success / Info）に応じた配色とアイコン。
- message text + optional action button。
- 自動消去（duration 指定）+ 手動 dismiss。
- 複数トーストのスタック表示（上 / 下 / 右上など position 指定）。
- on_dismiss、on_action callback を持つ。
- 階層: `layout/toast`。Modal (20) / Popover (21) とは独立した overlay layer。

## Codex-5.3-Spark 実装順

Spark には依存が浅い順に渡す。

1. `AlignCenterWrapper`、`LoadingDots`、`ProgressBar`
2. `Toolbar`、`StatusBar`、`NotificationToast`
3. `SelectionList`、`SlideControl`、`DynamicArrayEditor`
4. `Tabs`、`Breadcrumb`、`TreeView`
5. `MenuButton`、`SideMenu`
6. `ComboBox`
7. `CommandPalette`

各バッチの完了条件:

- widget API、ops、view、Storybook を同じバッチで実装する。
- `just storybook-check` と `RUSTFLAGS="-D warnings" cargo test -p katana-ui-widget` を通す。
- `just ast-lint` が失敗した場合は除外ではなく設計分割で解消する。

## KUC core 再編成（2026-05-17）

この backlog は、旧 `katana-ui-widget` / Adapter Storybook の完了記録をそのまま KUC 完了として扱わない。
archive 済みの 01〜24 は参考資料であり、KUC では `katana-ui-core` の中立モデル（neutral model）として作り直す。

新しい完了条件:

- UI ごとの状態（state）は component 内部で管理し、同じ UI が複数あっても `UiStateId` が一意になる。
- Storybook は `katana-ui-core::panel::Panel` で左ナビ表示枠と右プレビュー表示枠を構成する。
- 表示枠（panel）は `ThemeSnapshot` を必ず受け取り、見た目テーマ（theme）未設定を成功扱いにしない。
- Storybook は framework-specific UI の変換層（adapter）を経由しない。
- gate は story 数だけでなく、必須 UI、最低構造、状態衝突、panel theme を検査する。
- Modal の別ネイティブ画面（native window）は、親表示領域（display bounds）内の同一 display 配置と前面表示を KUC core model で計画し、未対応を fallback で隠さない。
- Storybook の回帰条件は marker だけにせず、操作後 state 反映、重ね表示（overlay）描画、別 window 実描画まで含める。

旧 01〜24 の archive を復帰させる場合も、旧 task の checkbox は引き継がない。
`docs/architecture/ui-separation/owned-ui-task-map.md` の対応表を入口にして、KUC 独自 UI task として再作成する。
