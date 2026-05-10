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

### Modified Capabilities

- なし。

## Impact

- `crates/katana-ui-widget/src/composite` と `crates/katana-ui-widget/src/layout` に新規 widget が追加される。
- `storybook/src/pages` に各 widget の live sample が追加される。
- katana / katana-chat-ui で個別実装している UI の移植候補が整理される。
- OpenSpec の完了判定は、`tasks.md` の checkbox だけでなく、Storybook 上の実操作と widget API の再利用性を確認してから行う。
