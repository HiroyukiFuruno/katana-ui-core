# 01〜24 UI DoD 正本

## 判定方針

この文書は、旧 01〜24 を KUC の atoms / molecules として実装する時の完了条件である。
Storybook に同じ骨格のカードと契約表が出ているだけでは完了ではない。
Storybook は静的見本帳ではなく、左 TreeView で選んだ UI の layout、option、action、event、state、rendering、panel 独立 scroll を実画面で触ってフィードバックするための場である。
正しさの確認と完了判定は Storybook やユーザー操作に任せず、自動テスト、数値化された layout / rendering contract、入力回帰、静的検査で行う。

各 UI は、少なくとも次を満たすまで未完了として扱う。

- 画面でその UI だと一目で分かる専用 preview を持つ。
- settings で主要 option を変更でき、preview / state / action / event に反映される。
- preset tab で正常系、操作後、端境界、theme 差分を切り替えられる。
- action と event が受動 UI 以外で実際に発火し、component state が変わる。
- 自動テストで option / action / event / state / layout / visual を固定する。
- 数値化された rendering contract で、他 UI と同じ汎用 fallback になっていないことを検出する。

## 01〜24 DoD

| # | UI | 画面で実現すること | 操作 | Storybook DoD | 自動検査 DoD |
| --- | --- | --- | --- | --- | --- |
| 01 | Theme / Panel theme | Katana accent の light / dark / panel theme が navigation、preview、inspector に反映される。 | theme 切替、style sheet 差し替え。 | theme control、各領域の theme id、色 token 表を表示する。 | facade、theme diff、panel theme、snapshot 差分。 |
| 02 | Text | heading / body / caption / code / muted と日本語・英日混在・絵文字が読める。 | 受動 UI。style 差し替えのみ。 | role grid と mixed text sample を専用 preview に出す。 | font role、line metrics、上下中央、emoji fallback。 |
| 03 | Icon | SVG icon、size、color token、a11y label が見える。 | 受動 UI。 | icon grid、custom SVG、accent / muted を出す。 | typed SVG props、size、color、theme 追従。 |
| 04 | Spinner / LoadingDots | spinner と dots が別形状で、speed / reduced motion / label が分かる。 | animation tick、reduced motion toggle。 | motion preset と state summary を表示する。 | phase、speed、reduced motion、非空 visual。 |
| 05 | SvgButton | icon only button として押せる。disabled / loading が見た目で分かる。 | click、focus、hover、active。 | icon button grid と callback log を表示する。 | disabled suppress、loading suppress、focus ring、theme color。 |
| 06 | TextButton | primary / secondary / ghost / link / danger / success が区別できる。 | click、keyboard activation。 | variant / tone / size matrix と log を表示する。 | matrix、disabled、loading、callback。 |
| 07 | IconTextButton | leading / trailing icon と text の spacing が分かる。 | click、keyboard activation。 | icon position comparison と log を表示する。 | icon spacing、disabled、loading、theme。 |
| 08 | Toggle | on / off の switch が明確で、disabled が効く。 | toggle、keyboard activation。 | live toggle、state reflection、log を表示する。 | checked state、disabled block、a11y label。 |
| 09 | SegmentedToggle | segment 選択、selected marker、disabled segment が分かる。 | segment select、keyboard navigation。 | text / icon segments と selected marker を表示する。 | selected marker、empty options、disabled segment。 |
| 10 | SelectBox | trigger と options panel が分かれ、placeholder / long list / placement が確認できる。 | open、close、select、outside close、keyboard navigation。 | trigger + floating options、selection log を表示する。 | open/close、select closes、disabled trigger、long list bounds。 |
| 11 | ColorSwatch | palette grid、selected ring、disabled が見える。 | color select。 | palette、selection preview、callback log を表示する。 | selected ring、disabled block、token resolution。 |
| 12 | Input / TextInput | value、placeholder、leading / trailing slot、invalid / readonly / disabled が分かる。 | type、clear、focus、blur、submit、IME commit、emoji input。 | text input preview、settings、cursor / selection、event log を表示する。 | readonly / disabled、clear、IME、emoji、vertical center。 |
| 13 | SearchBox | search icon、clear button、regex / word / case option が枠内にある。 | input、clear、submit、option toggle、Esc clear。 | Material UI 風の枠内 icon と option controls を表示する。 | submit、Esc clear、option toggle、SVG theme color。 |
| 14 | Tooltip | anchor と floating tooltip が placement ごとに見える。 | hover open、focus open、leave / blur close。 | hover / focus target、overlay、operation log を表示する。 | delay、focus open、placement flip、close lifecycle。 |
| 15 | Badge | tone / variant / size / leading icon が密な badge grid で分かる。 | 受動 UI。 | neutral / accent / danger / warning / success を表示する。 | tone / variant / size matrix、theme 追従。 |
| 16 | KeyCap | single key、modifier combo、macOS / non-macOS 表示が分かる。 | 受動 UI。 | shortcut samples と platform variants を表示する。 | modifier display、size / tone resolution。 |
| 17 | Card | header / body / footer / actions slot と interactive state が分かる。 | card click、child click、focus。 | TextInput / Button / Badge / Accordion 入り card と log を表示する。 | hover / active、child isolation、shadow / border。 |
| 18 | Accordion | header と body、indicator、trigger area、tree mode が分かる。 | toggle、group expand / collapse。 | full-row header、single / multiple、tree mode、reduced motion を表示する。 | disabled block、controlled / uncontrolled、trigger area、group behavior。 |
| 19 | SplitPane | horizontal / vertical、handle、ratio、min clamp が見える。 | drag handle、double-click reset、keyboard resize。 | draggable split preview、ratio log を表示する。 | clamp、reset、direction layout、handle hover。 |
| 20 | ModalOverlay / Modal | overlay dialog と native modal の違い、focus trap、footer が分かる。 | open、close、Esc、backdrop、footer action、focus return。 | open / close controls、native modal log を表示する。 | Esc、backdrop suppression、focus trap、focus return、native callback。 |
| 21 | Popover | anchor、placement、offset、outside close が分かる。 | open、close、outside click、Esc、select content。 | 4 placements、auto flip、close condition log を表示する。 | placement、auto flip、outside / Esc、focus handling。 |
| 22 | ColorPicker | RGB / RGBA panel、color plane、hue、alpha、preview が使える。 | open、close、plane drag、hue drag、alpha drag、numeric input。 | live panel、preview reflection、value label を表示する。 | readonly / disabled、all controls update、theme 追従。 |
| 23 | ColorPicker parity | floating panel、color-only trigger、trigger size、border、seamless hue bar が分かる。 | open overlay、close、drag color / alpha、trigger size change。 | opened panel、RGBA trigger preview、size presets を表示する。 | continuous update、no fake eyedropper、trigger sizing、hue bar continuity。 |
| 24 | CodeDiff | split left-right、split top-bottom、inline、added / removed / unchanged / collapsed が読める。 | mode switch、direction switch、expand / collapse、scroll sync。 | mode controls、collapse controls、long line / whitespace / Japanese diff を表示する。 | LCS、multi-byte highlight、line count mismatch、trailing newline、empty lines、scroll sync。 |

## Storybook 完了条件

Storybook は各 UI で次を満たす。

- 左の TreeView で UI を選ぶと、preview の形が UI ごとに変わる。
- 中央本文は全件カード一覧ではなく、選択中 UI の preview / settings 相当 / state / event / action / preset / quality を深く確認する。
- settings は実際の typed option を変更する。
- 操作できる UI は action / event / state が同時に変わる。
- 操作や settings 変更後は、chip やログ文言だけでなく preview 本体の rendering が変わる。
- Navigation / Preview / Details の縦スクロールは panel ごとに独立している。
- preset は UI ごとの意味を持ち、全 UI で同じ dummy 表示にしない。
- preview は generic fallback ではなく、UI 専用 renderer で描画する。
- 01〜24 の各 UI で、default と interactive の pixel 差分だけでなく、UI 固有領域の pixel marker を検査する。
