# Atoms contract

作成日: 2026-05-18
対象: `widget::atoms`

## 結論

atoms は、利用側が UI を組み立てる最小部品として公開する。
各 atom は option、action、event、state、preset、test、Storybook page を持つ。
表示専用で action / event がない場合も、`none` と明記して完了条件から逃がさない。

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
| image regression | 主要 preset の非空描画、theme 差分、配置を検査する。 |

## 4.1 Text

| 項目 | 契約 |
| --- | --- |
| option | content、text role、font role、color token、accessibility label |
| action | style replacement |
| event | none |
| state | resolved typography、resolved color、line metrics、`UiStateId` |
| preset | heading、body、caption、code、muted、日本語、英日混在、絵文字 |
| test | role 解決、font role 解決、色差し替え、上下中央揃え |
| Storybook page | role grid、theme 切替、文字サンプル、line metrics 表示 |

## 4.2 Icon

| 項目 | 契約 |
| --- | --- |
| option | icon source、size、color token、custom SVG、accessibility label |
| action | style replacement |
| event | none |
| state | resolved size、resolved color、parsed icon summary、`UiStateId` |
| preset | preset SVG、custom SVG、small / medium / large、accent / muted |
| test | SVG parse、size token、color token、theme 追従 |
| Storybook page | icon grid、custom SVG 入力、theme 切替、resolved props 表示 |

## 4.3 Buttons

対象: `Button`、`SvgButton`、`TextButton`、`IconTextButton`

| 項目 | 契約 |
| --- | --- |
| option | label、icon、icon position、variant、tone、size、disabled、loading、accessibility label |
| action | press、keyboard activation、focus、hover、active、loading suppress |
| event | pointer click、keyboard activation、focus event、command event |
| state | hover、active、focused、disabled、loading、callback log、`UiStateId` |
| preset | primary、secondary、ghost、link、danger、success、icon only、icon leading、icon trailing、disabled、loading |
| test | disabled 抑止、loading 抑止、keyboard activation、focus ring、icon / label spacing |
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

## 4.5 Checkbox / Radio / Toggle

| 項目 | 契約 |
| --- | --- |
| option | checked / selected、label、size、disabled、accessibility label |
| action | toggle、select、keyboard activation、focus |
| event | change event、focus event、keyboard event |
| state | checked、selected、focused、disabled、callback log、`UiStateId` |
| preset | on、off、disabled、large、compact、radio group sample |
| test | disabled blocks change、keyboard activation、radio selection、state uniqueness |
| Storybook page | live control、group comparison、callback log、state / event / action 履歴 |

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
