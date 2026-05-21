# Panel scroll layout redesign

## 目的

Panel は、KUC の Storybook だけでなく app 側の editor / viewer / side pane でも使う基盤である。
そのため、スクロールバーを後から座標で重ねる実装ではなく、Panel 自身が「外枠」「描画可能な内側」「スクロールバー予約領域」「入力判定領域」を一貫して決める。

## 画面上の見え方

- 左の Navigation、中央の Preview、右の Inspector はそれぞれ独立した Panel として見える。
- 各 Panel に必要なときだけ縦または横のスクロールバーが出る。
- スクロールバーが出ても、部品本体や設定欄へ重ならない。
- 内容が Panel 内に収まる場合、スクロールバーも余白スクロールも発生しない。
- つまみを最後まで動かすと、縦は下端、横は右端へ余白なく到達する。

## 詳細設計

### PanelFrame

`PanelFrame` は Panel の外枠である。
背景、枠線、Panel 全体の hit target はこの矩形から決める。

### ContentViewport

`ContentViewport` は子要素を描画してよい領域である。
スクロールバーを予約表示する場合、縦スクロールバー分の右 gutter と横スクロールバー分の下 gutter を除いた矩形にする。

### ScrollbarGutter

`ScrollbarGutter` はスクロールバー専用の予約領域である。
overlay 表示を明示した Panel 以外では、gutter は content と重ならない。

### ClipRect

`ClipRect` は描画を切り詰める矩形である。
Panel 内の title、preview、settings、nested panel は `ContentViewport` で clip される。
Inspector の settings 行や Tooltip がこの矩形を破って別 Panel へ食い込む状態を禁止する。

### HitRect

クリック、ホイール、drag の判定は `PanelFrame` と `ContentViewport` から導出する。
表示用の座標と入力用の座標を別定数で持たない。

## 必須テスト

- Navigation / Preview / Inspector の `PanelFrame` が互いに重ならない。
- Inspector の settings controls は Inspector の `ContentViewport` 内に収まる。
- 各 Panel の scrollbar track と thumb は自身の `PanelFrame` 内に収まる。
- `content <= viewport` の場合、scrollbar は表示されず、scroll offset は増えない。
- `content > viewport` の場合、scroll offset の最大値で thumb bottom / right が track bottom / right と一致する。
- Preview の横 scroll は、横 overflow がある page だけで有効になる。
- Panel の描画は `ClipRect` で切られ、子 Panel や Inspector が隣の Panel へめり込まない。

## 実装方針

- Storybook 側に散っている `INSPECTOR_X`、`PREVIEW_SCROLL_X`、`*_VIEWPORT_WIDTH` のような定数を、`PanelRegionLayout` から導出する。
- `panel_scroll_state` は overflow 量だけを返し、描画位置は `panel_layout` が返す。
- `panel_scrollbars` は track / thumb の算出だけを担当し、Panel 外の座標を直接持たない。
- `navigation`、`preview`、`inspector` は `ContentViewport` を受け取り、描画と入力の基準を揃える。
- 既存の screenshot や Storybook 操作を完了根拠にせず、上記を contract test で固定する。
