# Tasks — katana-widget-parity-backlog

## 1. 横断調査

- [ ] 1.1 katana で複数回利用されている UI を洗い出し、widget 化候補を一覧化する。
- [ ] 1.2 katana-chat-ui で複数回利用されている UI を洗い出し、widget 化候補を一覧化する。
- [ ] 1.3 既存 01〜22 の change に含まれているもの、未定義のもの、再設計が必要なものに分類する。
- [ ] 1.4 Tabs、Breadcrumb、SideMenu、ProgressBar、SelectionList、SlideControl、DynamicArrayEditor、AlignCenterWrapper を実装順に並べる。
- [ ] 1.5 既存 widget の再実装 gate を通す。対象は TextInput、SearchBox、Tooltip、Card、Accordion、Modal、Popover、ColorPicker とする。
- [ ] 1.6 各 widget の Storybook は、実操作、操作結果、callback log、light / dark の確認を同じ画面内で行えるようにする。

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

## 10. 完了確認

- [ ] 10.1 `just fmt-check`
- [ ] 10.2 `just storybook-check`
- [ ] 10.3 `RUSTFLAGS="-D warnings" cargo test -p katana-ui-widget`
- [ ] 10.4 `just ast-lint`
- [ ] 10.5 Storybook 上で各 widget の実操作と callback log を確認する。
- [ ] 10.6 OpenSpec の checkbox は、上記確認が終わるまで完了扱いにしない。
