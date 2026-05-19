# Atoms contract

作成日: 2026-05-18
対象: `widget::atoms`

## 結論

atoms は、利用側が UI を組み立てる最小部品として公開する。
各 atom は option、action、event、state、preset、test、Storybook page を持つ。
表示専用で action / event がない場合も、`none` と明記して完了条件から逃がさない。
Material UI は、見た目、押せる範囲、状態表現、option 分割の初期基準として使う。
ただし React / Material UI 互換ではなく、KUC の Rust 型付き DTO、preset、partial override、internal state、action-event-state 自動テストを正本にする。

## 共通契約

| 項目 | 契約 |
| --- | --- |
| option | builder API または typed props で渡す。 |
| action | `UiAction` で表現する。表示専用は `none`。 |
| event | core event / component event として記録する。表示専用は `none`。 |
| state | `UiStateId` を持つか、state 不要である理由を契約に書く。 |
| preset | Storybook の KUC Tabs で切り替える。 |
| preview | KUC panel 内で実描画する。文字列だけの placeholder は不可。 |
| settings | option を画面上で変更し、preview に反映する。 |
| test | option 解決、action、event、state 遷移を自動テストにする。 |
| rendering contract | 主要 preset の非空 render command、bounds、theme 差分、配置、状態差分を数値化して検査する。画像回帰は DoD にしない。 |

全 UI は `CommonWidgetPropsDto` 相当の共通 props を持つ。
共通 props は width、height、disabled、visible、tab-index、z-index、border、focusable を型付きで表し、文字列の後解釈にしない。
preset はこの DTO の初期値を生成するだけであり、利用側は preset + partial override と DTO 完全指定の両方を使える。
partial override は未指定項目だけ preset 値を残し、完全指定は preset 値を参照しない。

クリックできる atom は、Button 専用 event ではなく汎用 click event を使う。
汎用 click event は target `UiStateId`、pointer / keyboard の発火元、before / after state summary、component event への変換結果を持つ。

## 4.1 Text

| 項目 | 契約 |
| --- | --- |
| option | content、text role、font role、color token、accessibility label、align |
| action | style replacement、align replacement |
| event | none |
| state | resolved typography、resolved color、line metrics、`UiStateId` |
| preset | heading、body、caption、code、muted、日本語、英日混在、絵文字 |
| test | role 解決、font role 解決、色差し替え、上下中央揃え、align node 解決 |
| Storybook page | role grid、theme 切替、文字サンプル、line metrics、align 表示 |

Text は Storybook 専用の描画文字列ではなく、core の構成部品として扱う。
`AlignNode` 相当の配置 node は、中央寄せ、左右寄せ、上下中央揃えを text と他 atom に共通適用できる契約にする。

## 4.2 Icon

Icon は SVG icon atom として扱う。
`SvgIcon` 相当の typed props を持ち、画像ファイルの貼り付けではなく、SVG source、viewBox、path summary、stroke / fill policy、theme token、accessibility label を core model に保持する。

| 項目 | 契約 |
| --- | --- |
| option | icon source、viewBox、path summary、stroke / fill policy、size、color token、custom SVG、accessibility label |
| action | style replacement |
| event | none |
| state | resolved size、resolved color、parsed icon summary、`UiStateId` |
| preset | preset SVG、custom SVG、small / medium / large、accent / muted |
| test | SVG parse、viewBox 解決、size token、color token、theme 追従 |
| Storybook page | icon grid、custom SVG 入力、theme 切替、resolved props 表示 |

## 4.3 Buttons

対象: `Button`、`SvgButton`、`TextButton`、`IconTextButton`

Button 系は Material UI のボタン状態を初期基準にするが、KUC では別型として扱う。
`Button` は面と枠を持つ通常ボタン、`TextButton` はテキスト主体のボタン、`SvgButton` は見た目上アイコンのみのボタンである。
`SvgButton` は accessibility label を必須にし、表示 label を持たない。
`IconTextButton` は icon と label を同じ押下対象にまとめる。

Button 幅は `ButtonWidthDto` で表す。
`auto`、`px`、`percent`、`fill` を持ち、preset は初期値生成だけに使う。
利用側は preset から一部上書き、または DTO 直接指定で幅、height、padding、min size、border、radius、label align、icon gap、focusable を完全上書きできる。

| 項目 | 契約 |
| --- | --- |
| option | common props dto、label、icon、icon position、variant、tone、size、width dto、disabled、loading、accessibility label |
| action | press、keyboard activation、focus、hover、active、loading suppress |
| event | generic click、keyboard activation、focus event、command event |
| state | hover、active、focused、disabled、loading、callback log、`UiStateId` |
| preset | primary、secondary、ghost、link、danger、success、icon only、icon leading、icon trailing、disabled、loading、auto width、px width、percent width、fill width |
| test | disabled 抑止、loading 抑止、keyboard activation、generic click event、focus ring、icon / label spacing、common props dto 解決、width dto 解決、preset + partial override、DTO 完全上書き |
| Storybook page | button matrix、settings 変更、callback log、action / event / state 履歴 |

## 4.4 Input / TextInput

`TextInput` は `Input` atom の公開名または alias として扱う。

| 項目 | 契約 |
| --- | --- |
| option | value、placeholder、font role、leading slot、trailing slot、size、disabled、readonly、invalid、clear action |
| action | type、clear、focus、blur、submit with Enter、IME commit、emoji commit |
| event | key input、text input、IME composition、IME commit、emoji text、focus event、change event、submit event |
| state | value、focused、disabled、readonly、invalid、cursor / selection summary、`UiStateId` |
| preset | default、leading icon、trailing action、readonly、disabled、invalid、日本語、絵文字 |
| test | disabled / readonly 抑止、clear action、IME commit、emoji input、上下中央揃え |
| Storybook page | live input、settings for flags、state log、event log、action log |

## 4.5 Checkbox / Radio / Switch / Toggle

Checkbox、Radio、Switch は Material UI の見た目と操作を初期基準にする。
Switch は label + switch の行コンポーネントを持ち、つまみ部分だけでなく行全体クリックで on / off が切り替わる。
Toggle は旧要件の名称として残す場合も、KUC v0.1.0 の設計基準では Switch を主名にする。

| 項目 | 契約 |
| --- | --- |
| option | common props dto、checked / selected、label、label position、size、disabled、row click enabled、accessibility label |
| action | toggle、select、row toggle、keyboard activation、focus |
| event | change event、row click event、focus event、keyboard event |
| state | checked、selected、focused、disabled、callback log、`UiStateId` |
| preset | on、off、disabled、large、compact、switch row、radio group sample |
| test | disabled blocks change、keyboard activation、radio selection、switch row click、state uniqueness、common props dto 解決 |
| Storybook page | live control、label + switch row、group comparison、callback log、state / event / action 履歴 |

## 4.6 Badge / Divider / Spacer / KeyCap

| UI | option | action | event | state | preset | test | Storybook page |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Badge | label、tone、variant、size、leading icon、dismiss action | dismiss if configured | dismiss event if configured | resolved tone、shape、dismiss config、`UiStateId` | neutral、accent、danger、warning、success | tone matrix、dismiss action、theme 追従 | dense status grid、settings、state summary |
| Divider | direction、tone、spacing、label | none | none | resolved line metrics、`UiStateId` | horizontal、vertical、with label | size / spacing、theme 追従 | divider examples、bounds overlay |
| Spacer | width、height、flex policy | none | none | resolved size、`UiStateId` | fixed、fill、compact gap | layout size、gap effect | spacing playground、layout report |
| KeyCap | key label、modifier combo、size、tone、platform display | none | none | resolved platform label、font role、`UiStateId` | single key、combo、macOS、non-macOS | platform display、monospace role | shortcut samples、platform setting |

## 4.7 Loading / Progress / Color

| UI | option | action | event | state | preset | test | Storybook page |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Spinner | size、color、speed、reduced motion、label | animation tick、reduced motion toggle | animation frame event | phase、running / paused、`UiStateId` | small、medium、large、reduced motion | reduced motion fixed frame、theme color | loading group、animation state |
| LoadingDots | dot count、speed、color、label、reduced motion | animation tick、reduced motion toggle | animation frame event | phase、running / paused、`UiStateId` | 3 dots、5 dots、fast、slow | dot count、phase serialization | loading group、phase inspector |
| ProgressBar | determinate、percent、label、tone、size | set progress、reset、switch mode | progress change event | percent、mode、label、`UiStateId` | indeterminate、0、50、100、success、danger | clamp、mode switch、a11y label | progress playground、value log |
| ColorSwatch | color token、custom color、palette、size、disabled | select color、focus | color change、focus event | selected color、focused swatch、disabled、`UiStateId` | token palette、custom palette、disabled | selected ring、disabled blocks change、theme token | palette grid、selection preview |
