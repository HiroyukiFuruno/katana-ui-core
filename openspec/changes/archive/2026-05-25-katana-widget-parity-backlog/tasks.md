# Tasks — katana-widget-parity-backlog

> Superseded: 01〜24 と追加 UI の要件は `openspec/changes/establish-kuc-atoms-molecules-catalog/` へ移管する。このファイルの `[x]` は履歴であり、現在の KUC atoms / molecules 完了の根拠にしない。

## 1. 横断調査

- [x] 1.1 katana で複数回利用されている UI を洗い出し、widget 化対象を一覧化する。
- [x] 1.2 katana-chat-ui で複数回利用されている UI を洗い出し、widget 化対象を一覧化する。
- [x] 1.3 既存 01〜22 の change に含まれているもの、まだ定義されていないもの、再設計が必要なものに分類する。
- [x] 1.4 Tabs、Breadcrumb、SideMenu、ProgressBar、SelectionList、SlideControl、DynamicArrayEditor、AlignCenterWrapper を実装順に並べる。
- [x] 1.5 既存 widget の再実装 gate を通す。対象は TextInput、SearchBox、Tooltip、Card、Accordion、Modal、Popover、ColorPicker とする。
- [x] 1.6 各 widget の Storybook は、実操作、操作結果、callback log、light / dark の確認を同じ画面内で行えるようにする。
- [x] 1.7 追加 widget（TreeView、ComboBox、MenuButton、CommandPalette、StatusBar、Toolbar、LoadingDots、NotificationToast）を既存 widget との依存関係を考慮して実装順に並べる。

### 1.5 / 1.6 物理整理（2026-05-11）

| 対象 | 1.5 再実装 gate 完了可否 | 1.6 Storybook 同一画面確認 完了可否 | 根拠 |
|---|---|---|---|
| TextInput | yes | yes | `openspec/changes/12-text-input/tasks.md` は実装、テスト、Storybook、完了確認が完了済み。light / dark はStorybookのglobal theme切替で一元確認する。 |
| SearchBox | yes | yes | `openspec/changes/13-search-box/tasks.md` は実装、テスト、Storybook、完了確認が完了済み。submit / option log とglobal theme切替で確認する。 |
| Tooltip | yes | yes | `openspec/changes/14-tooltip/tasks.md` は hover / focus / delay / close の実 event 対応まで完了済み。表示差分はglobal theme切替で確認する。 |
| Card | yes | yes | `openspec/changes/17-card/tasks.md` の完了確認まで実施し、`KATANA_UI_WIDGET_STORYBOOK_PAGE=card` のウィンドウ単体スクリーンショットで確認した。 |
| Accordion | yes | yes | `openspec/changes/18-accordion/tasks.md` の完了確認まで実施し、`KATANA_UI_WIDGET_STORYBOOK_PAGE=accordion` のウィンドウ単体スクリーンショットで確認した。 |
| Modal | yes | yes | 2026-05-13 再修正: Storybook の主導線を `Modal` native window に戻し、同一ウィンドウ内の重ね表示は `OverlayDialog` 比較に分離した。`modal` / 全ページ `storybook-smoke` 通過。 |
| Popover | yes | yes | `openspec/changes/21-popover/tasks.md` の完了確認まで実施し、`KATANA_UI_WIDGET_STORYBOOK_PAGE=popover` のウィンドウ単体スクリーンショットで確認した。 |
| ColorPicker | yes | yes | `openspec/changes/22-rgba-color-picker/tasks.md` は完了確認 `4.1`〜`4.3` と再実装 gate `5.1` が完了済み。preview / callback とglobal theme切替で確認する。 |

結論:

- 1.5 は yes。対象 widget の個別 change に残っていた未完了項目を完了した。
- 1.6 は yes。light / dark は画面ごとの重複表示ではなく、Storybook のglobal theme切替で一元確認する。

### 1.4 / 1.7 Codex-5.3-Spark 実装順

1. AlignCenterWrapper、LoadingDots、ProgressBar
2. Toolbar、StatusBar、NotificationToast
3. SelectionList、SlideControl、DynamicArrayEditor
4. Tabs、Breadcrumb、TreeView
5. MenuButton、SideMenu
6. ComboBox
7. CommandPalette

### 0/1 採用判定

| 判定 | 対象 | 扱い |
|---:|---|---|
| 1 | ProgressBar | 実装対象 |
| 1 | Tabs | 実装対象 |
| 1 | Breadcrumb | 実装対象 |
| 1 | SideMenu | 実装対象 |
| 1 | SelectionList | 実装対象 |
| 1 | SlideControl | 実装対象 |
| 1 | DynamicArrayEditor | 実装対象 |
| 1 | AlignCenterWrapper | 実装対象 |
| 1 | TreeView | 実装対象 |
| 1 | ComboBox | 実装対象 |
| 1 | MenuButton | 実装対象 |
| 1 | CommandPalette | 実装対象 |
| 1 | StatusBar | 実装対象 |
| 1 | Toolbar | 実装対象 |
| 1 | LoadingDots | 実装対象 |
| 1 | NotificationToast | 実装対象 |
| 0 | DiffViewer | 実装しない |
| 1 | CodeDiff | `24-code-diff` で実装対象 |
| 0 | ProblemsPanel | 実装しない |
| 0 | Dashboard | 実装しない |
| 0 | Splash | 実装しない |
| 0 | Chat output cards | 実装しない |
| 0 | Composer | 実装しない |
| 0 | Thread | 実装しない |
| 0 | History | 実装しない |
| 0 | AdapterControls | 実装しない |
| 0 | ProviderIconSelector | 実装しない |

### 1.3 分類結果（2026-05-11 横断調査）

**katana 由来（`katana-ui/src/`）:**

| ソース | 分類 | 備考 |
|---|---|---|
| `widgets/accordion/` | 既存 (18) | 実装済み |
| `widgets/align_center/` | 1 | AlignCenterWrapper |
| `widgets/color_picker/` | 既存 (22) | 再実装 gate 対象 |
| `widgets/combo_box/` | 1 | ComboBox として widget 化 |
| `widgets/key_cap.rs` | 既存 (16) | 実装済み |
| `widgets/menu_button/` | 1 | MenuButton として widget 化 |
| `widgets/modal/` | 既存 (20) | 再実装 gate 対象 |
| `widgets/search_bar/` | 既存 (13) | 再実装 gate 対象 |
| `widgets/segmented_toggle.rs` | 既存 (09) | 実装済み |
| `widgets/shortcut.rs` | 既存 (16) | KeyCap に含まれる |
| `widgets/toggle/` | 既存 (08) | 実装済み |
| `views/top_bar/tab_bar/` | 1 | Tabs に統合 |
| `views/top_bar/status_bar.rs` | 1 | StatusBar として widget 化 |
| `views/app_frame/breadcrumbs.rs` | 1 | Breadcrumb |
| `views/modals/command_palette.rs` | 1 | CommandPalette として widget 化 |
| `views/panels/explorer/` | 1 | TreeView の出典 |
| `views/panels/toc/` | 1 | TreeView の出典（TOC 構造） |
| `views/panels/tree/` | 1 | TreeView の出典（ツリー構造） |
| `settings/settings_tree.rs` | 1 | TreeView の出典（設定ツリー） |
| `views/diff_viewer/` | 0 | ファイルパス（file path）、承認、拒否、複数ファイル移動を含むドメイン固有 UI のため、そのままは実装しない。2つのコード文字列だけを見比べる汎用 `CodeDiff` は `24-code-diff` で扱う。 |
| `views/panels/problems/` | 0 | ドメイン固有（lint 表示）のため実装しない |
| `views/panels/dashboard/` | 0 | ドメイン固有のため実装しない |
| `views/splash.rs` | 0 | ドメイン固有のため実装しない |

**katana-chat-ui 由来（`katana-chat-ui-adapter/src/widget/`）:**

| ソース | 分類 | 備考 |
|---|---|---|
| `widget/toolbar.rs` | 1 | Toolbar として widget 化 |
| `widget/thinking_indicator.rs` | 1 | LoadingDots として widget 化 |
| `widget/output_cards.rs` | 0 | ドメイン固有（chat output）のため実装しない |
| `widget/composer.rs` | 0 | ドメイン固有（chat input）のため実装しない |
| `widget/thread.rs` | 0 | ドメイン固有（message thread）のため実装しない |
| `widget/history.rs` | 0 | ドメイン固有（session history）のため実装しない |
| `widget/adapter_controls.rs` | 0 | ドメイン固有（AI adapter 制御）のため実装しない |
| `widget/provider_icon_selector.rs` | 0 | ドメイン固有（adapter 選択）のため実装しない |
| `widget/action_button.rs` | 既存 (05-07) | SVG/Text/IconText Button に含まれる |

## 2. ProgressBar

- [x] 2.1 確定進捗、未確定進捗、label、percent 表示、size、tone を API として定義する。
- [x] 2.2 Storybook に通常進捗、未確定進捗、低 / 中 / 高 progress、dark / light を置く。

## 3. Tabs

- [x] 3.1 tab item は label、任意 icon、selected、disabled、on_select を持つ。
- [x] 3.2 content ありの場合は、選択 tab に紐づく任意 node を表示する。
- [x] 3.3 content なしの場合は callback で外部 UI と連動できるようにする。
- [x] 3.4 Storybook に content あり / なし、閉じられる tab、disabled、overflow を置く。

## 4. Breadcrumb

- [x] 4.1 crumb 配列、separator、任意 icon、disabled、on_click を API として定義する。
- [x] 4.2 長い path の省略表示と、最終 crumb の click 可否を指定できるようにする。
- [x] 4.3 Storybook に file path、settings path、長い path を置く。

## 5. SideMenu

- [x] 5.1 左右配置、幅（width）指定、幅 0、hover 展開、固定展開を API として定義する。
- [x] 5.2 SVG icon 配列を受け取り、icon ごとの callback を実行できるようにする。
- [x] 5.3 icon からさらに pop 表示できるようにし、content は上位から node として渡す。
- [x] 5.4 pop 方式は modal 風、popover 風、領域拡張型から選べるようにする。
- [x] 5.5 Storybook に左 menu、右 menu、hover 展開、icon pop を置く。
- [/] 5.6 KatanA のアクティビティバー（activity rail）/ プレビュー側パネル（preview side panel）を基準に、細い暗色アイコンバー（icon rail）、上寄せグループ（group）、下寄せグループ、アクティブ背景（active background）を実装する。
- [/] 5.7 左配置では内容パネル（panel）が右へ、右配置では内容パネルが左へ伸びることを実装・検証する。
- [/] 5.8 クリック（click）では内容パネルを固定表示し、同じアイコン（icon）の再クリックで閉じる。
- [/] 5.9 ホバー（hover）ではアイコン上で遅延表示し、ポインター（pointer）がアイコンバーと内容パネル外へ出たら閉じる。
- [/] 5.10 クリック直後はホバー表示へ即時復帰しないよう冷却時間を持つ。
- [/] 5.11 Storybook の SideMenu は白い大枠や巨大な空白パネルではなく、KatanA 風の左右バーのサンプル（sample）と処理ログ（callback log）を確認できる画面にする。

## 6. SelectionList

- [x] 6.1 画像2枚目のように、section label、色付き marker、選択 highlight、もっと表示を表現できるようにする。
- [x] 6.2 item は label、marker color、selected、disabled、on_select、optional content を持つ。
- [x] 6.3 Storybook に dark / light theme preset list と「もっと表示」を置く。

## 7. SlideControl

- [x] 7.1 整数 / 小数、最小値、最大値、step、単位、表示 format、現在値を API として定義する。
- [x] 7.2 値の適用先は callback と node 更新の両方を扱えるようにする。
- [x] 7.3 Storybook に UIコントラスト補正のような slider + 数値入力を置く。

## 8. DynamicArrayEditor

- [x] 8.1 画像3枚目のように、配列 item の追加、削除、編集、並び替えを扱う。
- [x] 8.2 item 表示は文字列固定ではなく、上位から node として渡せるようにする。
- [x] 8.3 empty state、最大件数、削除不可 item、disabled を API と Storybook に置く。

## 9. AlignCenterWrapper

- [x] 9.1 katana の AlignCenter 相当として、子要素を縦横中央に置く wrapper を追加する。
- [x] 9.2 幅、高さ、padding、gap、disabled 時の見え方を指定できるようにする。
- [x] 9.3 Storybook に button、color picker、icon を中央配置する例を置く。

## 10. TreeView（横断調査 追加）

- [x] 10.1 item 構造（label、icon、indent、expanded、active、disabled）を API として定義する。
- [x] 10.2 leaf / parent item を再帰的に描画する view を実装する。katana explorer の dir_entry / file_entry パターンを参考にする。
- [x] 10.3 expand / collapse / select / active の state 管理を ops に実装する。
- [x] 10.4 hover highlight、vertical indent line、force open を扱う。
- [/] 10.5 TreeView は通常スクロールを内包しない composable view とし、親 scroll container に置いても二重スクロールで壊れないようにする。
- [/] 10.6 nested JSON 相当の `children` 構造を入力として受け取り、階層深さを TreeView 側で再帰的に算出する。
- [/] 10.7 parent / leaf の冒頭に任意 SVG icon を指定できる API を維持する。
- [/] 10.8 parent item の開閉領域を icon only / label only / icon + label / disabled で制御できるようにする。
- [/] 10.9 全開 / 全閉じ control を左上に表示できるようにし、default は false とする。
- [/] 10.10 展開中の水平線表示を option 化し、default false、線種、太さ、RGBa 色を指定できるようにする。
- [x] 10.11 virtualized mode は利用側が明示した場合だけ有効にし、通常 TreeView と同じ行表示・選択・開閉挙動を維持する。
  - 2026-05-12: `TreeView::virtualized(true)` を追加し、既定値は false のまま維持。固定行高の仮想リストで通常行と同じ `TreeViewRowRenderer` を使う構成にし、`tree-view` と全ページ `storybook-smoke` が通過。
- [/] 10.12 Storybook にファイルツリー、TOC、設定ツリーの例を置き、開閉アイコン、任意 SVG、active background、hover background、垂直線、親 scroll 内配置を確認する。
  - 2026-05-12: TreeView Storybook を少量・意味のある JSON 相当の入れ子データに作り直し、ファイルツリー、TOC、設定ツリー、任意 SVG、選択ログ、context ログ、親 scroll 内配置を確認できるようにした。`STORYBOOK_SMOKE_PAGES="tree-view" just storybook-smoke` と全ページ `just storybook-smoke` 通過。

## 11. ComboBox（横断調査 追加）

- [x] 11.1 TextInput (12) + Popover (21) の組み合わせで API を定義する。
- [x] 11.2 入力によるフィルタリング、strict mode（自由入力不可）を扱う。
- [x] 11.3 選択肢は label + value ペア、on_select / on_input_change callback を持つ。
- [x] 11.4 Storybook にフォント選択、ファイル名入力（選択肢あり）、自由入力の例を置く。

## 12. MenuButton（横断調査 追加）

- [x] 12.1 trigger（label / icon / node）+ content（Popover slot）の API を定義する。
- [x] 12.2 framed / unframed variant を実装する。
- [x] 12.3 on_open / on_close callback を持つ。
- [x] 12.4 Storybook に framed button menu、unframed text menu、icon menu の例を置く。

## 13. CommandPalette（横断調査 追加）

- [x] 13.1 検索入力 + 結果リスト + キーボードナビゲーションの汎用構造を定義する。
- [x] 13.2 result item は label、icon、shortcut、score、payload を持つ。
- [x] 13.3 provider trait / callback でドメインロジックを注入できるようにする。
- [x] 13.4 ↑↓ 選択移動、Enter 実行、Escape 閉じるを実装する。
- [x] 13.5 Storybook にファイル検索風、コマンド検索風の例を置く。

## 14. StatusBar（横断調査 追加）

- [x] 14.1 leading slot（severity icon + message）、trailing slot（任意 node）の API を定義する。
- [x] 14.2 severity enum（Error / Warning / Success / Info）で配色とアイコンを制御する。
- [x] 14.3 on_action callback を持つ。
- [x] 14.4 Storybook に各 severity、action button 付き、spinner 付きの例を置く。

## 15. Toolbar（横断調査 追加）

- [x] 15.1 leading slot + trailing slot を任意 node として受け取る API を定義する。
- [x] 15.2 gap、alignment（center / top / bottom）を指定できるようにする。
- [x] 15.3 Storybook に icon toolbar、text + icon toolbar、identity + actions toolbar の例を置く。

## 16. LoadingDots（横断調査 追加）

- [x] 16.1 dot 数、サイズ（idle / active）、アニメーション速度を API として定義する。
- [x] 16.2 label あり / なしの variant を実装する。
- [x] 16.3 tone（配色）を theme token に追従させる。
- [x] 16.4 Storybook に label 付き、label なし、速度差、dark / light の例を置く。

## 17. NotificationToast（横断調査 追加）

- [x] 17.1 severity、message、optional action button、duration の API を定義する。
- [x] 17.2 自動消去 + 手動 dismiss を実装する。
- [x] 17.3 複数トーストのスタック表示と position 指定を扱う。
- [x] 17.4 Storybook に各 severity、自動消去、手動 dismiss、スタック表示の例を置く。

## 18. 完了確認

- [x] 18.1 `just fmt-check`
- [x] 18.2 `just storybook-check`
- [x] 18.3 `RUSTFLAGS="-D warnings" cargo test -p katana-ui-widget`
- [x] 18.4 `just ast-lint`
- [x] 18.5 Storybook 上で各 widget の実操作と callback log を確認する。
- [x] 18.6 OpenSpec の checkbox は、上記確認が終わるまで完了扱いにしない。
  - 2026-05-11 整理結果: yes。18.5、1.5、1.6 の未完了項目は解消済み。

## 19. Storybook ナビゲーション / 横幅フィードバック

- [/] 19.1 左メニューの縦スクロールバーがボタンに重ならないよう、メニュー内側に余白を確保する。
- [/] 19.2 現在開いているページが左メニュー上で判別できる表示を追加する。
- [/] 19.3 TextInput / SearchBox / Tooltip / ColorPickerRgba のページスクロール領域が右側いっぱいに伸びるようにする。
- [/] 19.4 Storybook 左メニューをカテゴリ別の TreeView 表示に作り直し、選択状態の確認を兼ねる。
- [/] 19.5 Storybook 左メニューで Accordion も併用し、カテゴリ開閉、ネスト展開、選択状態、アクティブ表示の検証を兼ねる。
  - 2026-05-12: Storybook 左メニューのカテゴリを Accordion、各カテゴリ配下を TreeView に変更し、カテゴリ開閉とページ選択状態を Storybook 自身で常時検証できる構成にした。
  - 2026-05-13: Accordion body の固定高による巨大な空白を修正し、左メニュー上でカテゴリ高さが内容に追従するようにした。
- [/] 19.6 19.5 の前提として、TreeView / Accordion のネスト展開、選択中表示、展開アニメーション、インデント線が実利用に耐えるかを再確認し、不備があれば widget 側を修正する。
  - 2026-05-12: Sidebar / TreeView / Accordion / Breadcrumb の対象ページを `storybook-smoke` で確認し、`just storybook-check` と `just ast-lint` を通過。
  - 2026-05-13: TreeView の開閉アイコンを文字記号から内部 `Icon` の SVG に変更し、行中央揃えを維持。`tree-view` / 全ページ `storybook-smoke` 通過。

## 20. Runtime crash / layout feedback（2026-05-12）

- [/] 20.1 Tooltip を開いたときに `adapter::view_state` の `index out of bounds` で落ちないようにする。
  - 2026-05-12: overlay 削除と focus 復帰を次 tick に遅延する共通 lifecycle を追加し、Tooltip の overlay close path に適用。`STORYBOOK_SMOKE_PAGES="tooltip" just storybook-smoke` と全ページ `just storybook-smoke` 通過。
- [/] 20.2 Toolbar ページを開いたときに `adapter::view_state` の `index out of bounds` で落ちないようにする。
  - 2026-05-12: `STORYBOOK_SMOKE_PAGES="toolbar" just storybook-smoke` と全ページ `just storybook-smoke` 通過。
- [/] 20.3 CommandPalette ページを開いたときに `adapter::view_state` の `index out of bounds` で落ちないようにする。
  - 2026-05-12: `STORYBOOK_SMOKE_PAGES="command-palette" just storybook-smoke` と全ページ `just storybook-smoke` 通過。
- [/] 20.4 Card / AlignCenter の Storybook ページで、右側コンテンツ領域の縦スクロールバーを画面右端に表示する。
  - 2026-05-12: Storybook root / content / Card / AlignCenter / Accordion の scroll container を `width_full` + `height_full` に揃え、右ペインが画面幅まで伸びるようにした。
- [/] 20.5 Tabs は content あり / なし、外部 UI 連携、閉じられるタブ、overflow を実用水準の UI と API に再設計する。
  - 2026-05-12: Tabs は content node を持つタブと callback 連携タブを分け、閉じる操作と overflow サンプルを Storybook で確認できる構成にした。
- [/] 20.6 Breadcrumb は BG / border を option 化し、default は false とする。
  - 2026-05-12: `BreadcrumbProps` の background / border option を維持し、default false のまま Storybook で比較できる状態にした。
- [/] 20.7 Breadcrumb は `Icon` を使い、アイコンと文字の上下中央を揃える。
  - 2026-05-12: Breadcrumb segment は内部 `Icon` を使い、行内の icon / label / separator の中央揃えを維持。
- [/] 20.8 Breadcrumb は階層を JSON 相当の入れ子構造で受け取り、各階層のホバーで子階層 TreeView を再帰表示できるようにする。
  - 2026-05-12: crumb の children から hover TreeView overlay を開く実装を追加し、階層候補を再帰的に表示できるようにした。
- [/] 20.9 Breadcrumb の Storybook はファイル階層、設定階層、長いパス省略、クリック結果を、見た目と操作が分かる状態で確認できるようにする。
  - 2026-05-12: `STORYBOOK_SMOKE_PAGES="breadcrumb" just storybook-smoke` と全ページ `just storybook-smoke` 通過。
- [/] 20.10 PopBar（現行 Storybook 上は Popover 相当）を開いたときに `adapter::view_state` の `index out of bounds` で落ちないようにする。
  - 2026-05-12: overlay 削除と focus 復帰を次 tick に遅延する共通 lifecycle を Popover / MenuButton / ComboBox / ColorPicker に適用。`STORYBOOK_SMOKE_PAGES="popover" just storybook-smoke` と全ページ `just storybook-smoke` 通過。
- [/] 20.11 Storybook の全ページを `KATANA_UI_WIDGET_STORYBOOK_PAGE` で 1 ページずつ初期表示し、起動時クラッシュがないことを確認する回帰ゲートを作る。
  - 2026-05-12: `justfile` に `storybook-smoke` を追加し、全ページ `just storybook-smoke` 通過。

## 21. Modal 再評価フィードバック（2026-05-13）

- [/] 21.1 `Modal` を Storybook の主導線で別ネイティブウィンドウとして開く構成に修正する。同一ウィンドウ内の重ね表示は `OverlayDialog` として補助比較に下げる。
  - 2026-05-13: `Modal Samples` の先頭を「別ウィンドウでModalを開く」操作に変更し、`OverlayDialog` は下段比較に移動した。
- [/] 21.2 Storybook のページ名、見出し、説明を `ModalOverlay` 中心から `Modal` 中心へ改め、画面上で「別ウィンドウが開く」ことを最初に操作できるようにする。
  - 2026-05-13: サイドバー表記を `Modal` に変更し、`modal` route を追加。既存 `modal-overlay` route は互換 alias として残した。
- [/] 21.3 `Modal::view` / `Modal::open_window` の API を再評価し、`open=true` のときだけ native window が開き、同一ウィンドウ内表示に戻らないことを確認する。
  - 2026-05-13: `Modal::open_window` は `open=false` で no-op、`open=true` で native window を開く実装を維持。Storybook 側は `OverlayDialog::view` をModal主導線から外した。
- [/] 21.4 `OverlayDialog` は同一ウィンドウ内 overlay として名前・説明・Storybook 導線を分離し、Modal と混同しないようにする。
  - 2026-05-13: Storybook 上で `OverlayDialog 比較: 同じウィンドウ内に重ねる表示です。Modalではありません。` と明示した。
- [/] 21.5 別ウィンドウでの close、Esc、focus return、親ウィンドウとの相互作用抑制を Storybook から確認できるようにする。
  - 2026-05-13: close / Esc / focus return log と親ウィンドウ操作 policy の選択を `Modal` ページに配置した。
- [/] 21.6 `20-modal-overlay` archive の完了扱いは再評価対象として扱い、修正完了まで Modal の再実装 gate を no のまま維持する。
  - 2026-05-13: 再評価 gate を no に戻したうえで修正し、`just check`、`just storybook-check`、`just ast-lint`、`just storybook-smoke` 通過後に yes へ戻した。
- [/] 21.7 `Modal::open_window` は Storybook の button action から直接 window 生成しない。次 tick に native window 生成と focus 要求を寄せ、runtime gate で `native-window-created` を確認する。

## 22. TreeView / Storybook Sidebar 再評価フィードバック（2026-05-13）

- [/] 22.1 Storybook 左メニューの Accordion body が固定高で巨大な空白を作らないようにし、開いたカテゴリの高さは実コンテンツに追従させる。
  - 2026-05-13: Accordion の開ききった body は固定 height を外し、max-height のみで制御するよう修正した。
  - 2026-05-13: スクリーンショット再確認で固定高由来の空白が残っていたため、Sidebar 側でカテゴリ item 数から body height を算出する方式へ修正した。
- [/] 22.2 Storybook 左メニューの `ModalOverlay` 表記を `Modal` に改め、カテゴリ内の現在ページ表示を維持する。
  - 2026-05-13: Sidebar の該当ラベルを `Modal` に変更し、Page enum も `Modal` に改名した。
- [/] 22.3 TreeView の開閉アイコンは文字記号ではなく内部 `Icon` による SVG 表示にし、ラベルと上下中央を揃える。
  - 2026-05-13: 開閉アイコンと空スロットを `row_chrome` に分離し、SVG icon と中央揃え slot を使うようにした。
- [/] 22.4 TreeView / Sidebar の smoke を再実行し、初期表示で崩れないことを確認する。
  - 2026-05-13: `STORYBOOK_SMOKE_PAGES="modal tree-view" just storybook-smoke` と全ページ `just storybook-smoke` が通過。
  - 2026-05-13: 追加でウィンドウ単体スクリーンショットを確認し、`STORYBOOK_SMOKE_PAGES="overview tree-view" just storybook-smoke`、`just storybook-check`、`just ast-lint` が通過。
- [/] 22.5 Storybook 左メニューは KatanA 本家のファイルツリー（file tree）に寄せ、Accordion の箱型見出しではなく TreeView の親子行として表示する。
  - 2026-05-13: Sidebar のカテゴリを `Accordion` から `TreeViewItem` の親ノードに変更し、カテゴリ、ページ、選択状態を単一の TreeView で表現するよう修正した。
- [/] 22.6 TreeView の親ノードには内部 `Icon` のフォルダ SVG を使い、行内の開閉アイコン、フォルダアイコン、文字の上下中央を揃える。
  - 2026-05-13: item icon を専用 slot に包み、行高に対して上下中央へ配置するよう修正した。
- [/] 22.7 KatanA 本家スクリーンショットとの差分確認を Storybook スクリーンショットで実施し、左メニューと TreeView ページの視覚乖離を再評価する。
  - 2026-05-13: `/tmp/katana-ui-widget-ss/sidebar-overview-treeview.png` と `/tmp/katana-ui-widget-ss/tree-view-alignment.png` を取得し、左メニューの箱型 Accordion 表示が消えて TreeView 行になったこと、TreeView の icon / label が同じ行内で中央寄せになったことを確認した。
  - 2026-05-13: `just storybook-check`、`just ast-lint`、`STORYBOOK_SMOKE_PAGES="overview tree-view" just storybook-smoke`、全ページ `just storybook-smoke` が通過。

## 23. Storybook 設定網羅 gate（2026-05-13）

- [x] 23.1 各 widget の公開 API のうち、見た目または挙動が変わる設定値を洗い出し、Storybook に表示すべき variant 一覧を作る。
  - 2026-05-13: `variant-audit.md`、`variant-audit-core.md`、`variant-audit-layout.md` に一覧化した。
- [x] 23.2 variant 一覧は widget ごとに `appearance`、`behavior`、`state`、`callback`、`theme` に分類する。
  - 2026-05-13: widget 別 gate 表で分類し、各 page の表示根拠を記録した。
- [x] 23.3 `appearance` は size、tone、variant、border、background、icon、placement、spacing、disabled / readonly の見た目差分を対象にする。
  - 2026-05-13: 不足していた Text、Spinner、LoadingDots、SvgButton、SelectBox、ComboBox、TextInput、SearchBox、SlideControl の表示差分を追加した。
- [x] 23.4 `behavior` は click、hover、focus、keyboard、drag、open / close、dismiss、expand / collapse、select、input、submit を対象にする。
  - 2026-05-13: open / close、placement、select、input、submit、disabled / readonly / loading 抑制の操作確認を Storybook 表示または log に反映した。
- [x] 23.5 `state` は default、active、selected、disabled、readonly、loading、error、empty、overflow、long text を対象にする。
  - 2026-05-13: disabled、readonly、loading、open、long text、overflow などの未露出状態を Storybook に追加または既存表示の根拠を明文化した。
- [x] 23.6 `callback` はユーザー操作で値が変わるものすべてについて、Storybook 画面上に結果ログまたは反映先を置く。
  - 2026-05-13: 値変化系は log / selected value / preview / count / status に反映し、button 系も callback log で確認する。
- [x] 23.7 `theme` は global light / dark 切替で、SVG icon、border、background、text、hover、active の配色差分を確認する。
  - 2026-05-13: theme は Storybook の global 切替で一元確認し、各 widget は theme token 経由の描画を維持する。
- [x] 23.8 KatanA 本家または katana-astro のスクリーンショットがある widget は、Storybook 画面を撮影して乖離が大きいものを修正対象に戻す。
  - 2026-05-13: 既に乖離が大きかった Storybook sidebar / TreeView / Breadcrumb / ColorPicker は個別 feedback task に戻して修正済み。新たな未対応乖離は `variant-audit.md` 上に残していない。
- [x] 23.9 すべての widget について、公開 API にあるが Storybook に出ていない設定を「未確認」として扱い、完了扱いにしない。
  - 2026-05-13: subagent 監査で残った未露出 API を追加し、`未確認なし` として audit を更新した。
- [x] 23.10 `storybook-smoke` は初期表示確認に限定し、variant 網羅の完了根拠にはしない。variant gate は目視または操作ログ付きの画面確認を別途必要とする。
  - 2026-05-13: `storybook-smoke` はクラッシュ回帰確認としてのみ記録し、variant 完了根拠は Storybook 画面上の表示・操作ログ・audit 表に分離した。

## 24. Storybook interaction regression gate（2026-05-13）

- [/] 24.1 既存 `storybook-smoke` は「2秒起動して落ちない」だけで、click / open / pop 表示を検証していない問題を明文化する。
- [/] 24.2 Popover ページは open / dismiss state を dyn_container の再描画キーに含め、Open 操作で実際に pop 表示へ遷移するようにする。
- [/] 24.3 overlay / pop 系ページ（Popover / ComboBox / MenuButton / Tooltip）を `KATANA_UI_WIDGET_STORYBOOK_INTERACTION=open` で起動する `storybook-interaction-smoke` を追加する。
- [/] 24.4 `storybook-smoke` と `storybook-interaction-smoke` を両方通し、初期表示だけでなく開いた状態のクラッシュも検知する。
- [/] 24.5 公開前に `storybook-check` / `ast-lint` / `storybook-smoke` / `storybook-interaction-smoke` をまとめて実行できる `storybook-regression` を追加する。
- [/] 24.6 `storybook-interaction-smoke` を専用スクリプト化し、対象ページがinteraction markerを出さない場合は失敗させる。
- [/] 24.7 Popover は `replay-open` で「閉じた状態から開く」状態変化を起動後に発火し、open後の再描画 marker まで検証する。
- [/] 24.8 SelectBox / ComboBox / ColorPickerRgba も `open` interaction marker を出し、候補・パネル表示の起動クラッシュを検知対象に含める。
- [/] 24.9 `storybook-regression` に `cargo test --workspace --all-targets` を含め、UI起動確認だけでなく商用コードの単体回帰も同時に検証する。
- [/] 24.10 `storybook-requirement-gate` を追加し、起動ではなく「要求された状態変化・callback log・open/render marker」まで検証対象にする。
- [/] 24.11 Toggle / SegmentedToggle / Spinner / ColorSwatch / TextInput / SearchBox / Tabs / DynamicArrayEditor / TreeView / Breadcrumb / CommandPalette / Toolbar を requirement scenario として固定し、marker 未実装なら失敗させる。
- [/] 24.12 SearchBox / DynamicArrayEditor / TreeView / Breadcrumb / CommandPalette は商用コード側の契約テスト（contract tests）も追加し、Storybook 表示だけではなく公開 API の要件保持を検証する。
- [/] 24.13 `storybook-regression` に `storybook-requirement-gate` を追加し、公開前ゲートで要件未充足を検知できるようにする。
- [/] 24.14 pop 系 UI の `add_overlay` を `create_effect` 内で即時実行しない。overlay 追加・削除・focus 復帰は次 tick に寄せ、Adapter の View ツリー更新中に overlay 状態を競合させない。
- [/] 24.15 `app_state.rs` の `PoisonError` は二次被害として扱い、一次原因である `adapter::view_state` の `index out of bounds` を pop 系 state / overlay lifecycle の回帰として検知する。
- [/] 24.16 `storybook-requirement-gate` は対応表だけでなく実 Storybook を起動し、操作再生後の marker と終了コードを確認する。marker 前後のクラッシュは失敗として扱う。
- [/] 24.17 `add_overlay` / `remove_overlay` の直接利用を `OverlayLifecycle` に限定する静的ゲートを追加し、同じ不具合型を再導入したら `storybook-regression` と `check` で失敗させる。
- [/] 24.18 SideMenu の pop 表示も runtime requirement gate に含め、初期 pop 表示で overlay lifecycle が壊れないことを検知する。
- [/] 24.19 Storybook の暗色表示切り替えは event handler 内で即時に全体表示ツリーを作り替えない。`theme-toggle` runtime scenario を追加し、切り替え時の `view_state` crash を検知する。
- [/] 24.20 TreeView の開閉/選択/全展開/全折りたたみは event handler 内で即時に自分自身の行ツリーを作り替えない。次 tick に寄せて、Sidebar や TreeView ページの操作で `view_state` crash を起こさない。
- [/] 24.21 Modal の別 native window 起動も runtime requirement gate に含め、Storybook 上の「開いた扱い」だけでなく、別ウィンドウの view 生成 callback まで到達したことを検知する。

## 25. Storybook ページ構造 / ast-lint フィードバック（2026-05-14）

- [/] 25.1 Storybook の各標準ページは、先頭に Title を表示し、その下に live widget / callback log / state display を置く。
- [/] 25.2 `SvgButton` / `TextButton` / `IconTextButton` などで、Title より前に live widget が表示される構造を廃止する。
- [/] 25.3 左メニューの TreeView 選択は、ページ本体の差し替えを同一イベント中に行わず、次 tick に遅延して `view_state` crash を避ける。
- [/] 25.4 Storybook 用 ast-lint として、標準ページが `page_content` の前に独自 `v_stack` を置く構造を検出する静的ゲートを追加する。
- [/] 25.5 `just ast-lint` に Storybook ページ構造 lint を接続し、今後追加される標準ページも同じ構造を要求する。
- [/] 25.6 widget 実装ディレクトリを tree 的に列挙し、Storybook menu の数・Page・label が一致しない場合に lint で失敗させる。
- [/] 25.7 Storybook 起動後にページ選択相当の state change を発火し、左メニュー選択後の `view_state` crash を requirement gate で検知する。
- [/] 25.8 SideMenu の cleanup は widget 内の開閉 state を更新しない。破棄中は global overlay の削除だけに限定し、削除済み view の style 再評価による `view_state` crash を防ぐ。
- [/] 25.9 Modal Storybook は同一ウィンドウ内の `OverlayDialog` 比較を削除し、`Modal = 別 native window を開く widget` として確認できる構成に統一する。
- [/] 25.10 Modal の別ウィンドウはクリック時に直接 `new_window` を発行しない。次 tick で native window を生成し、`on_open` callback で実際の window view 生成を Storybook から確認する。

## 26. Modal 設定ボタン / 自動テスト強化フィードバック（2026-05-14）

- [/] 26.1 Modal の設定ボタンはプリセット一括変更ではなく、size / Esc / parent interaction / footer を独立して変更する。
- [/] 26.2 Modal の設定ボタンは現在選択中の値に `✓` を表示し、画面上で反映状態が分かるようにする。
- [/] 26.3 Modal の size 設定は `Sm` と `Custom` が同じ実効幅にならないよう、Custom 幅を明確に分ける。
- [/] 26.4 Modal 設定の単体テストを追加し、size 変更が Esc / parent / footer を壊さないこと、Esc 変更が size / parent / footer を壊さないこと、footer 変更が size / Esc を壊さないことを検知する。
- [/] 26.5 Storybook requirement gate に Modal 設定ボタン別 scenario を追加し、各設定適用後に別 native window view 生成 callback まで到達することを検知する。
- [/] 26.6 Modal footer の scenario は、別 native window が開いた事実だけでなく、選択中 footer と実際に `Modal` へ渡す footer 本文が一致する場合だけ成功させる。
  - 2026-05-14: `footer-form` / `footer-detail` は、選択中 footer と `Modal::footer(...)` に渡した本文が一致した場合だけ `storybook-requirement-gate` の marker を出すようにした。
- [/] 26.7 multi display 環境では、`Modal` が親ウィンドウと同じ display に出るよう、親 `WindowId` を指定できる配置 API を追加し、Storybook では main window を親として渡す。
  - 2026-05-14: `Modal::same_display_as(parent_window_id)` を追加し、Storybook の `main window` を親として渡す。macOS の monitor / screen 列挙 API は `NSEnumerator` crash を起こすため使わず、親ウィンドウの画面上の矩形から別 native window の初期位置を計算する。
  - 2026-05-14: `just storybook-regression` 通過。unit test / Storybook compile / ast-lint / headless smoke / interaction smoke / requirement gate をまとめて確認した。
- [/] 26.8 Modal の別 native window は、親 UI の背面に回らず、生成後に前面へ表示されるよう明示的に表示順を上げる。
  - 2026-05-14: `WindowLevel` だけに依存せず、Modal window の view 生成後に対象 window を再表示して focus を要求する処理を追加した。
- [/] 26.9 Modal の別 native window は、親 Storybook を縮小していても、親ウィンドウの見えている位置から出る。
  - 2026-05-14: 親より Modal window が大きい場合は、無理に中央寄せせず親ウィンドウの開始位置へ寄せる regression test を追加した。
- [/] 26.10 Modal の位置計算は、macOS で crash する `screen_layout` / monitor 列挙へ依存しない。
  - 2026-05-14: `parent.screen_layout()` 経由で `NSEnumerator` crash が再発したため削除し、親ウィンドウ矩形のみで位置を決める実装に戻した。
- [/] 26.11 Modal の `open_window()` は「予約しただけ」を成功扱いしない。事前に検知できる失敗は `Result` で返し、Storybook は error log として表示する。
  - 2026-05-14: `bool` 返却を `Result<bool, ModalOpenError>` に変更し、無効な座標と未完成の same-display placement を失敗として扱う。
- [/] 26.12 Storybook の Modal 主導線は未完成の `same_display_as` を使わず、まず別 native window を確実に開く最小構成へ戻す。
  - 2026-05-14: Storybook の Modal open から親 window placement 指定を外し、失敗時は `native log` に error を表示する。

## 27. Overlay / Storybook state 設計見直しフィードバック（2026-05-14）

- [/] 27.1 `Tooltip` や `Popover` など、吹き出し表示（overlay）が開く部品で `view_state.rs` の `index out of bounds` が出る問題を、その場しのぎではなく所有期間の設計から直す。
  - 2026-05-14: `OverlayLifetime` を導入し、遅延された overlay 追加・削除・focus 要求が、所有元の view 破棄後に古い view へ触れないようにした。
- [/] 27.2 Storybook 左メニューは、ページ選択のたびに TreeView 全体を作り直さない。ページ切り替え中の再構築競合を避ける。
  - 2026-05-14: `sidebar_tree` の再構築キーから `current_page` を外し、ページ切り替えでは sidebar TreeView を再生成しないようにした。
- [/] 27.3 `Tooltip` の遅延 hover 表示は、対象 view が破棄された後に visible state を更新しない。
  - 2026-05-14: mounted flag を追加し、遅延 hover callback が破棄後に `visible.set(true)` を実行しないようにした。
- [/] 27.4 `Breadcrumb` の hover tree は、閉じる遅延処理が破棄後に open state を更新しない。
  - 2026-05-14: mounted flag と `OverlayLifetime` を組み合わせ、hover close の遅延処理と overlay lifecycle を所有元 view に紐づけた。
- [/] 27.5 MenuButton / ComboBox / ColorPicker / SideMenu / Tooltip / Breadcrumb の overlay lifecycle は、共通の所有期間 guard を通す。
  - 2026-05-14: 各 widget の `OverlayLifecycle` 呼び出しに `OverlayLifetime` を渡し、破棄済み owner への遅延追加・focus を止める構成にした。
- [/] 27.6 Storybook requirement gate は、marker だけでなく、ページ切り替えを連続実行して `view_state` crash を検知する。
  - 2026-05-14: `overview:select-page-cycle:all-pages-stable` scenario を追加し、Tooltip / Popover / MenuButton / ComboBox / ColorPicker / SideMenu / CommandPalette / Toolbar / TreeView / Overview を連続で表示切り替えする。
- [x] 27.7 Modal の同一 display 配置は未完成扱いに戻す。未対応を fallback で隠さず、別 native window が確実に前面かつ親と同じ display に出る実装で完了にする。
  - 2026-05-17: `ModalWindowPlacement::same_display(...)` を KUC core model として追加し、親表示領域（display bounds）外や大きすぎる modal を成功扱いにしない。Storybook は `--open-modal-window` で別 native window を実際に開き、`same_display=true` と `frontmost=true` を gate にした。
- [x] 27.8 Storybook の「起動した」「marker が出た」だけを品質根拠にしない。UI操作後に落ちないこと、状態が画面に反映されること、別 window / overlay が実際に出ることを回帰条件にする。
  - 2026-05-17: 静的HTML export は完了根拠にしない。KUC の中核（core）UIと表示枠（panel）で動く可視 Storybook、操作後 state 反映、overlay / window 実描画を回帰条件にする。
  - 2026-05-17: `--runtime-regression` で `state_reflected=true`、`overlay_rendered=true`、`modal_plan_same_display=true`、`modal_plan_frontmost=true` を確認し、`storybook-requirement-gate` で別 window 実描画まで実行する。

## 28. KUC 独自 UI parity reset（2026-05-17）

- [x] 28.1 archive 済み 01〜24 を KUC 独自 UI task へ読み替える対応表を `docs/architecture/ui-separation/owned-ui-task-map.md` に作成する。
- [x] 28.2 Storybook は `katana-ui-core::panel::Panel` で root、navigation、preview を表現し、Adapter 経由では描画しない。
- [x] 28.3 表示枠（panel）は `ThemeSnapshot` を必ず受け取り、`storybook-requirement-gate` で `panel_theme_configured=true` を必須にする。
- [x] 28.4 UI ごとの状態（state）を component 内部に閉じ、重複 UI の `UiStateId` 衝突を gate で検知する。
- [x] 28.5 旧 01 `Theme / Panel theme`: theme token、`ThemeSnapshot`、panel theme id、light / dark 差分を KUC core model と Storybook panel 上で再確認する。
- [x] 28.6 旧 02〜04 `Text` / `Icon` / `Spinner`: atom として中立 `UiNode` 化し、Storybook panel 上で最低構造と theme 適用を確認する。
- [x] 28.7 旧 05〜07 `SvgButton` / `TextButton` / `IconTextButton`: button 系を KUC atom / molecule として整理し、同一 label 複数配置時の state 一意性を確認する。
- [x] 28.8 旧 08〜12 `Toggle` / `SegmentedToggle` / `SelectBox` / `ColorSwatch` / `TextInput`: 選択・入力 state を内部 state として持たせ、Storybook panel 上で反映先を確認する。
- [x] 28.9 旧 13〜17 `SearchBox` / `Tooltip` / `Badge` / `KeyCap` / `Card`: 表示だけでなく、入力、hover、補助情報、配置構造を KUC core model で確認する。
- [x] 28.10 旧 18〜21 `Accordion` / `SplitPane` / `Modal` / `Popover`: 開閉、分割、重ね表示、別窓相当の状態を KUC model と adapter 境界に分けて再定義する。
- [x] 28.11 旧 22〜24 `ColorPicker` / `ColorPicker parity` / `CodeDiff`: 色選択と差分表示を KUC 独自 UI として再確認し、旧 Adapter 実装の完了扱いを引き継がない。
- [x] 28.12 追加 UI `Tabs` / `Breadcrumb` / `SideMenu` / `SelectionList` / `SlideControl` / `DynamicArrayEditor` / `TreeView` / `ComboBox` / `MenuButton` / `CommandPalette` / `StatusBar` / `Toolbar` / `LoadingDots` / `NotificationToast` を、KUC core model、内部 state、Storybook panel、theme gate の 4 条件で再判定する。
- [x] 28.13 旧 Adapter Storybook の目視完了記録は参考情報に限定し、KUC Storybook panel 上で確認できない UI は未完了として扱う。
- [x] 28.14 `docs/architecture/ui-separation/ui-core-parity-gap.md` を UI ごとの未完了 / 完了証跡表として更新し、`just storybook-regression` の marker だけを品質根拠にしない。
