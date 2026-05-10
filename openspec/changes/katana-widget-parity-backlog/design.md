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
katana の tab 実装を移植対象として確認し、閉じる、並び替え、overflow、未保存状態の表現が必要かを実装前に分類する。

### Breadcrumb

階層 path を crumb 配列として受け取り、各 crumb は label、任意 icon、disabled、on_click を持つ。
省略表示と separator の変更を扱う。

### SideMenu

左右配置、幅（width）指定、幅 0、hover 展開、固定展開を扱う。
SVG icon 配列を受け取り、icon ごとに click callback と pop content を設定する。
pop は modal 風、popover 風、領域拡張型を選べるようにする。
SideMenu 自体が popover / modal / overlay に依存しすぎないよう、pop 表示方式は enum と content slot で切り替える。

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

## Open Questions

- katana-chat-ui の対象 repo path は横断調査 task で確定する。
- Modal の別ウィンドウ化と同一ウィンドウ overlay の分離は、既存 `20-modal-overlay` の design で扱う。
