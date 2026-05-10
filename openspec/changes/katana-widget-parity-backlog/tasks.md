# Tasks — katana-widget-parity-backlog

## 1. 横断調査

- [x] 1.1 katana で複数回利用されている UI を洗い出し、widget 化候補を一覧化する。
- [x] 1.2 katana-chat-ui で複数回利用されている UI を洗い出し、widget 化候補を一覧化する。
- [x] 1.3 既存 01〜22 の change に含まれているもの、未定義のもの、再設計が必要なものに分類する。
- [ ] 1.4 Tabs、Breadcrumb、SideMenu、ProgressBar、SelectionList、SlideControl、DynamicArrayEditor、AlignCenterWrapper を実装順に並べる。
- [ ] 1.5 既存 widget の再実装 gate を通す。対象は TextInput、SearchBox、Tooltip、Card、Accordion、Modal、Popover、ColorPicker とする。
- [ ] 1.6 各 widget の Storybook は、実操作、操作結果、callback log、light / dark の確認を同じ画面内で行えるようにする。
- [ ] 1.7 追加 widget（TreeView、ComboBox、MenuButton、CommandPalette、StatusBar、Toolbar、LoadingDots、NotificationToast）を既存 widget との依存関係を考慮して実装順に並べる。

### 1.3 分類結果（2026-05-11 横断調査）

**katana 由来（`katana-ui/src/`）:**

| ソース | 分類 | 備考 |
|---|---|---|
| `widgets/accordion/` | 既存 (18) | 実装済み |
| `widgets/align_center/` | 計画済み (parity) | AlignCenterWrapper |
| `widgets/color_picker/` | 既存 (22) | 再実装 gate 対象 |
| `widgets/combo_box/` | **新規追加** | ComboBox として widget 化 |
| `widgets/key_cap.rs` | 既存 (16) | 実装済み |
| `widgets/menu_button/` | **新規追加** | MenuButton として widget 化 |
| `widgets/modal/` | 既存 (20) | 再実装 gate 対象 |
| `widgets/search_bar/` | 既存 (13) | 再実装 gate 対象 |
| `widgets/segmented_toggle.rs` | 既存 (09) | 実装済み |
| `widgets/shortcut.rs` | 既存 (16) | KeyCap に含まれる |
| `widgets/toggle/` | 既存 (08) | 実装済み |
| `views/top_bar/tab_bar/` | 計画済み (parity) | Tabs に統合 |
| `views/top_bar/status_bar.rs` | **新規追加** | StatusBar として widget 化 |
| `views/app_frame/breadcrumbs.rs` | 計画済み (parity) | Breadcrumb |
| `views/modals/command_palette.rs` | **新規追加** | CommandPalette として widget 化 |
| `views/panels/explorer/` | **新規追加** | TreeView の出典 |
| `views/panels/toc/` | **新規追加** | TreeView の出典（TOC 構造） |
| `views/panels/tree/` | **新規追加** | TreeView の出典（ツリー構造） |
| `settings/settings_tree.rs` | **新規追加** | TreeView の出典（設定ツリー） |
| `views/diff_viewer/` | 除外 | ドメイン固有（diff 表示） |
| `views/panels/problems/` | 除外 | ドメイン固有（lint 表示） |
| `views/panels/dashboard/` | 除外 | ドメイン固有 |
| `views/splash.rs` | 除外 | ドメイン固有 |

**katana-chat-ui 由来（`katana-chat-ui-floem/src/widget/`）:**

| ソース | 分類 | 備考 |
|---|---|---|
| `widget/toolbar.rs` | **新規追加** | Toolbar として widget 化 |
| `widget/thinking_indicator.rs` | **新規追加** | LoadingDots として widget 化 |
| `widget/output_cards.rs` | 除外 | ドメイン固有（chat output） |
| `widget/composer.rs` | 除外 | ドメイン固有（chat input） |
| `widget/thread.rs` | 除外 | ドメイン固有（message thread） |
| `widget/history.rs` | 除外 | ドメイン固有（session history） |
| `widget/vendor_controls.rs` | 除外 | ドメイン固有（AI vendor 制御） |
| `widget/provider_icon_selector.rs` | 除外 | ドメイン固有（vendor 選択） |
| `widget/action_button.rs` | 既存 (05-07) | SVG/Text/IconText Button に含まれる |

## 2. ProgressBar

- [ ] 2.1 確定進捗、未確定進捗、label、percent 表示、size、tone を API として定義する。
- [ ] 2.2 Storybook に通常進捗、未確定進捗、低 / 中 / 高 progress、dark / light を置く。

## 3. Tabs

- [ ] 3.1 tab item は label、任意 icon、selected、disabled、on_select を持つ。
- [ ] 3.2 content ありの場合は、選択 tab に紐づく任意 node を表示する。
- [ ] 3.3 content なしの場合は callback で外部 UI と連動できるようにする。
- [ ] 3.4 Storybook に content あり / なし、閉じられる tab、disabled、overflow を置く。

## 4. Breadcrumb

- [ ] 4.1 crumb 配列、separator、任意 icon、disabled、on_click を API として定義する。
- [ ] 4.2 長い path の省略表示と、最終 crumb の click 可否を指定できるようにする。
- [ ] 4.3 Storybook に file path、settings path、長い path を置く。

## 5. SideMenu

- [ ] 5.1 左右配置、幅（width）指定、幅 0、hover 展開、固定展開を API として定義する。
- [ ] 5.2 SVG icon 配列を受け取り、icon ごとの callback を実行できるようにする。
- [ ] 5.3 icon からさらに pop 表示できるようにし、content は上位から node として渡す。
- [ ] 5.4 pop 方式は modal 風、popover 風、領域拡張型から選べるようにする。
- [ ] 5.5 Storybook に左 menu、右 menu、hover 展開、icon pop を置く。

## 6. SelectionList

- [ ] 6.1 画像2枚目のように、section label、色付き marker、選択 highlight、もっと表示を表現できるようにする。
- [ ] 6.2 item は label、marker color、selected、disabled、on_select、optional content を持つ。
- [ ] 6.3 Storybook に dark / light theme preset list と「もっと表示」を置く。

## 7. SlideControl

- [ ] 7.1 整数 / 小数、最小値、最大値、step、単位、表示 format、現在値を API として定義する。
- [ ] 7.2 値の適用先は callback と node 更新の両方を扱えるようにする。
- [ ] 7.3 Storybook に UIコントラスト補正のような slider + 数値入力を置く。

## 8. DynamicArrayEditor

- [ ] 8.1 画像3枚目のように、配列 item の追加、削除、編集、並び替えを扱う。
- [ ] 8.2 item 表示は文字列固定ではなく、上位から node として渡せるようにする。
- [ ] 8.3 empty state、最大件数、削除不可 item、disabled を API と Storybook に置く。

## 9. AlignCenterWrapper

- [ ] 9.1 katana の AlignCenter 相当として、子要素を縦横中央に置く wrapper を追加する。
- [ ] 9.2 幅、高さ、padding、gap、disabled 時の見え方を指定できるようにする。
- [ ] 9.3 Storybook に button、color picker、icon を中央配置する例を置く。

## 10. TreeView（横断調査 追加）

- [ ] 10.1 item 構造（label、icon、indent、expanded、active、disabled）を API として定義する。
- [ ] 10.2 leaf / parent item を再帰的に描画する view を実装する。katana explorer の dir_entry / file_entry パターンを参考にする。
- [ ] 10.3 expand / collapse / select / active の state 管理を ops に実装する。
- [ ] 10.4 hover highlight、vertical indent line、force open を扱う。
- [ ] 10.5 virtual scroll 対応で大量 item のパフォーマンスを維持する。
- [ ] 10.6 Storybook にファイルツリー、TOC、設定ツリーの例を置く。

## 11. ComboBox（横断調査 追加）

- [ ] 11.1 TextInput (12) + Popover (21) の組み合わせで API を定義する。
- [ ] 11.2 入力によるフィルタリング、strict mode（自由入力不可）を扱う。
- [ ] 11.3 選択候補は label + value ペア、on_select / on_input_change callback を持つ。
- [ ] 11.4 Storybook にフォント選択、ファイル名入力（候補あり）、自由入力の例を置く。

## 12. MenuButton（横断調査 追加）

- [ ] 12.1 trigger（label / icon / node）+ content（Popover slot）の API を定義する。
- [ ] 12.2 framed / unframed variant を実装する。
- [ ] 12.3 on_open / on_close callback を持つ。
- [ ] 12.4 Storybook に framed button menu、unframed text menu、icon menu の例を置く。

## 13. CommandPalette（横断調査 追加）

- [ ] 13.1 検索入力 + 結果リスト + キーボードナビゲーションの汎用構造を定義する。
- [ ] 13.2 result item は label、icon、shortcut、score、payload を持つ。
- [ ] 13.3 provider trait / callback でドメインロジックを注入できるようにする。
- [ ] 13.4 ↑↓ 選択移動、Enter 実行、Escape 閉じるを実装する。
- [ ] 13.5 Storybook にファイル検索風、コマンド検索風の例を置く。

## 14. StatusBar（横断調査 追加）

- [ ] 14.1 leading slot（severity icon + message）、trailing slot（任意 node）の API を定義する。
- [ ] 14.2 severity enum（Error / Warning / Success / Info）で配色とアイコンを制御する。
- [ ] 14.3 on_action callback を持つ。
- [ ] 14.4 Storybook に各 severity、action button 付き、spinner 付きの例を置く。

## 15. Toolbar（横断調査 追加）

- [ ] 15.1 leading slot + trailing slot を任意 node として受け取る API を定義する。
- [ ] 15.2 gap、alignment（center / top / bottom）を指定できるようにする。
- [ ] 15.3 Storybook に icon toolbar、text + icon toolbar、identity + actions toolbar の例を置く。

## 16. LoadingDots（横断調査 追加）

- [ ] 16.1 dot 数、サイズ（idle / active）、アニメーション速度を API として定義する。
- [ ] 16.2 label あり / なしの variant を実装する。
- [ ] 16.3 tone（配色）を theme token に追従させる。
- [ ] 16.4 Storybook に label 付き、label なし、速度差、dark / light の例を置く。

## 17. NotificationToast（横断調査 追加）

- [ ] 17.1 severity、message、optional action button、duration の API を定義する。
- [ ] 17.2 自動消去 + 手動 dismiss を実装する。
- [ ] 17.3 複数トーストのスタック表示と position 指定を扱う。
- [ ] 17.4 Storybook に各 severity、自動消去、手動 dismiss、スタック表示の例を置く。

## 18. 完了確認

- [ ] 18.1 `just fmt-check`
- [ ] 18.2 `just storybook-check`
- [ ] 18.3 `RUSTFLAGS="-D warnings" cargo test -p katana-ui-widget`
- [ ] 18.4 `just ast-lint`
- [ ] 18.5 Storybook 上で各 widget の実操作と callback log を確認する。
- [ ] 18.6 OpenSpec の checkbox は、上記確認が終わるまで完了扱いにしない。
