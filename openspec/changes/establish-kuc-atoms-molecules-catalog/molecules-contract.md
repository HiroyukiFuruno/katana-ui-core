# Molecules contract

作成日: 2026-05-18
対象: `widget::molecules`

## 結論

molecules は複数の atoms と model state を組み合わせる部品として公開する。
親 molecule は子 atom の state を潰さず、親 state と子 state を Storybook と自動テストで別々に追跡できる必要がある。

## 共通契約

| 項目 | 契約 |
| --- | --- |
| option | typed props または builder API で渡す。 |
| action | open / close / select / input / drag / dismiss / mode switch を `UiAction` として扱う。 |
| event | action の結果として component event と core event を記録する。 |
| state | parent `UiStateId` と child `UiStateId` を分離する。 |
| preset | KUC Tabs で複数 preset を切り替える。 |
| preview | KUC panel 内で実描画し、placeholder 表示だけにしない。 |
| settings | option を画面上で変更し、preview と state summary に反映する。 |
| test | 親 state、子 state、event routing、disabled / readonly 抑止を自動テストにする。 |
| image regression | 主要 preset、開閉後、選択後、theme 差分、配置を検査する。 |

## 5.1 Selection controls

対象: `SelectBox`、`ComboBox`、`MenuButton`、`SegmentedToggle`

| UI | option | action | event | state | preset | test | Storybook page |
| --- | --- | --- | --- | --- | --- | --- | --- |
| SelectBox | selected value、options、placeholder、disabled、long list、placement | open、close、select、outside close、keyboard navigation | open / close、selection change、focus | open、selected value、highlighted option、callback log | short、long、placeholder、disabled | select closes panel、disabled trigger、long list bounds | trigger + panel、settings、logs |
| ComboBox | input value、options、free input、filter policy、disabled | type、filter、open、select、clear | input event、filter event、selection change | input value、filtered results、selected option | strict、free input、empty result、disabled | filter result、IME input、selection | live input + list、event log |
| MenuButton | label、items、icon、placement、disabled | open、close、select item、keyboard navigation | open / close、item command | open、highlighted item、selected command | text menu、icon menu、disabled item | command dispatch、disabled item | menu operation page |
| SegmentedToggle | selected value、options、icons、disabled segment、size | select segment、keyboard navigation | selection change、focus | selected value、focused segment、callback log | text、icon、mixed、disabled option | selected marker、empty options、keyboard | segmented control preview |

## 5.2 Form and editing molecules

対象: `SearchBox`、`FormField`、`SlideControl`、`DynamicArrayEditor`

| UI | option | action | event | state | preset | test | Storybook page |
| --- | --- | --- | --- | --- | --- | --- | --- |
| SearchBox | value、placeholder、search options、leading icon、clear button、disabled | input、clear、submit、toggle option、Esc clear | change、submit、clear、option change、key | value、focused、selected options、callback log | simple、all options、disabled、regex / word / case | Enter submit、Esc clear、IME / emoji、option toggle | search preview、option settings |
| FormField | label、description、error、required、child control、layout | focus child、set invalid、submit child | child focus、validation event | child state ids、invalid、required | normal、required、error、with help | label association、error layout、child state | field wrapper page |
| SlideControl | min、max、step、value、disabled、orientation | drag、keyboard increment、keyboard decrement、set value | value change、drag start / end | value、dragging、focused、callback log | horizontal、vertical、disabled、stepped | clamp、step、keyboard、drag | slider preview、value log |
| DynamicArrayEditor | items、min / max length、item schema、reorder enabled | add、delete、reorder、edit item | add / delete / reorder / edit | item states、order、active item、validation | empty、filled、max reached、reorder | child state uniqueness、order update | array editor playground |

## 5.3 Overlay and transient molecules

対象: `Tooltip`、`Popover`、`ModalOverlay`、`NotificationToast`

| UI | option | action | event | state | preset | test | Storybook page |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Tooltip | label、placement、delay、max width、hover / focus trigger | hover open、focus open、close on leave / blur | pointer enter / leave、focus、open / close | visible、anchor、placement、timer summary | top、bottom、start、end、edge flip | delay、focus open、placement flip | hover / focus target、logs |
| Popover | open、anchor、placement、offset、width、outside click、Esc | open、close、outside click、Esc、select content | open / close、outside click、key | open、anchor、placement、focus summary | placements、auto flip、offset、fixed width | placement、outside click、Esc、focus | popover preview、settings |
| ModalOverlay | open、title、size、dismiss policy、Esc、backdrop、focus trap、footer、native mode | open、close、Esc、backdrop click、footer action、focus return | open / close、dismiss reason、focus return | open、focused element、modal stack、callback log | small、medium、custom、with footer、native | focus trap、focus return、Esc / backdrop policy | modal operation page |
| NotificationToast | message、severity、duration、dismiss action、position | show、dismiss、auto timeout、action click | show、dismiss、timeout、action | visible、severity、timer、callback log | success、warning、error、action、stacked | duration、manual dismiss、stack order | toast stack page |

## 5.4 Surface and navigation molecules

対象: `Card`、`Accordion`、`SplitPane`、`Tabs`、`Breadcrumb`、`SideMenu`、`Toolbar`、`StatusBar`

| UI | option | action | event | state | preset | test | Storybook page |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Card | variant、padding、interactive、header、body、footer、actions | click、child click、focus | card click、focus、child event | hover、active、focused、child state ids | plain、outlined、interactive、form card | child operation isolation、theme | card with child controls |
| Accordion | header、expanded、disabled、controlled、indicator、multiple、tree mode、shared disclosure foundation | toggle、group expand / collapse | toggle、focus、callback | expanded、selected、depth、callback、shared disclosure state | closed、open、disabled、multiple、tree | controlled / uncontrolled、disabled、tree mode、TreeView 共有開閉 state | live accordion page |
| SplitPane | direction、ratio、min sizes、handle width、reset | drag handle、double-click reset、keyboard ratio | ratio change、drag lifecycle | ratio、dragging、hover handle | horizontal、vertical、min clamp、nested | clamp、reset、orientation | draggable split page |
| Tabs | selected tab、tabs、disabled tab、orientation、closeable、pinned、group id、dirty marker、icon | select tab、close tab、pin / unpin、move within group、keyboard navigation | tab change、tab close、pin change、group change、dirty marker event、focus | selected index、focused tab、closed tab ids、pinned ids、group state、dirty ids、child page state | horizontal、vertical、disabled tab、closeable、pinned、grouped、dirty、icon tabs | keyboard、state persistence、close / pin / group / dirty / icon marker | preset tabs sample |
| Breadcrumb | items、active item、separator、overflow | select crumb、open overflow | navigation event、overflow open | active item、overflow state | simple、long、overflow、disabled | item command、overflow | hierarchy navigation page |
| SideMenu | sections、selected item、collapsed、icon mode | select item、collapse、expand、hover | selection、collapse change | selected item、collapsed、hover expansion | expanded、collapsed、nested | selection persistence、keyboard | side navigation page |
| Toolbar | actions、groups、disabled action、overflow | press action、open overflow、keyboard | command、overflow open | active action、overflow state、child state ids | dense、grouped、overflow、disabled | command routing、child state | action rail page |
| StatusBar | severity、message、actions、dismiss | dismiss、press action | dismiss、command | severity、visible、callback log | info、warning、error、with action | severity style、dismiss | status examples page |

Accordion と TreeView は disclosure foundation を共有する。
ただし表示上の indent、line、folder/file icon、trigger area は部品ごとの option として分離し、開閉 state と event routing だけを共通化する。

Tabs は単なる preset 切替だけではなく、アプリ UI の tab bar として必要な close、pin、group、dirty、icon を core 契約に含める。
Storybook preset tabs はこの契約の利用例であり、アプリ側で同じ部品を再利用できる必要がある。

## 5.5 Structured navigation and command molecules

対象: `TreeView`、`SelectionList`、`CommandPalette`

| UI | option | action | event | state | preset | test | Storybook page |
| --- | --- | --- | --- | --- | --- | --- | --- |
| TreeView | nodes、expanded ids、selected id、line display、disabled nodes | expand、collapse、select、keyboard navigation | expand / collapse、selection、focus | expanded ids、selected id、active node、node state ids | flat、nested、line display、disabled node | child state uniqueness、keyboard、large tree | Storybook left nav equivalent |
| SelectionList | items、sections、selected id、marker、more row | select、keyboard move、load more | selection、keyboard、more row | selected id、focused row、section summary | simple、sectioned、marker、more row | selected marker、empty state | list selection page |
| CommandPalette | query、items、providers、empty state、shortcut labels | type query、select command、keyboard navigation、clear | query change、command selected、close | query、filtered actions、active id | default、empty、grouped、shortcut | filtering、keyboard、IME query | command page with log |

## 5.6 Color and code molecules

対象: `ColorPicker`、`CodeDiff`

| UI | option | action | event | state | preset | test | Storybook page |
| --- | --- | --- | --- | --- | --- | --- | --- |
| ColorPicker | RGB / RGBA value、mode、readonly、disabled、title、trigger size、alpha、overlay | open、close、change color plane、change hue、change alpha、numeric input | color change、open / close、alpha update | selected color、panel open、mode、readonly、disabled | RGB、RGBA、readonly、disabled、all trigger sizes | readonly / disabled、continuous update、trigger sizing | live panel、value proof |
| CodeDiff | before / after、first line、mode、direction、collapsed blocks、highlight ranges、theme | switch mode、switch direction、expand / collapse、scroll sync | mode change、expand / collapse、scroll sync | diff model、collapsed blocks、line pairing、summary counts | split、inline、vertical、no diff、Japanese text | LCS、multi-byte ranges、line count、trailing newline | diff controls、sync preview |
