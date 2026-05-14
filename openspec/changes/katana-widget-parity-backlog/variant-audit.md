# Storybook 設定網羅表

## 判定ルール

- `yes`: Storybook 画面上で、見た目または操作結果を確認できる。
- `n/a`: widget の性質上、該当しない。
- `storybook-smoke`: 初期表示クラッシュ確認だけに使う。variant の完了根拠にはしない。

## 共通観点

| 観点 | 確認内容 |
|---|---|
| appearance | size、tone、variant、border、background、icon、placement、spacing、disabled / readonly の見た目差分 |
| behavior | click、hover、focus、keyboard、drag、open / close、dismiss、expand / collapse、select、input、submit |
| state | default、active、selected、disabled、readonly、loading、error、empty、overflow、long text |
| callback | 操作結果が画面内の log、selected value、preview、count、status に反映される |
| theme | global light / dark 切替で SVG icon、border、background、text、hover、active が追従する |
| reference | KatanA / katana-astro のスクリーンショットと見比べ、乖離が大きければ修正対象に戻す |

## Widget 別 gate（2026-05-13）

| widget | appearance | behavior | state | callback | theme | reference | 根拠 |
|---|---|---|---|---|---|---|---|
| Text | yes | n/a | yes | n/a | yes | n/a | role、color、`max_lines`、`align` を表示比較する。 |
| Icon | yes | n/a | yes | n/a | yes | yes | size、tone、theme 追従を表示する。 |
| Spinner | yes | n/a | yes | n/a | yes | n/a | size、tone、active、`speed_rps` を表示比較する。 |
| LoadingDots | yes | n/a | yes | n/a | yes | yes | label、active、dot 数、gap、速度、色 override を表示する。 |
| SvgButton | yes | yes | yes | yes | yes | yes | variant、tone、size、disabled、loading、callback log を表示する。 |
| TextButton | yes | yes | yes | yes | yes | yes | size、tone、disabled、callback log を表示する。 |
| IconTextButton | yes | yes | yes | yes | yes | yes | icon 位置、label、disabled、callback log を表示する。 |
| Toggle | yes | yes | yes | yes | yes | yes | readonly、disabled、size、callback log を表示する。 |
| SegmentedToggle | yes | yes | yes | yes | yes | yes | 2/3/5項目、disabled、callback log を表示する。 |
| SelectBox | yes | yes | yes | yes | yes | yes | placeholder、selected、long list、open / close、size を表示する。 |
| ComboBox | yes | yes | yes | yes | yes | yes | strict、free input、filter、disabled、open、placement、callback log を表示する。 |
| ColorSwatch | yes | yes | yes | yes | yes | yes | circle / square、selected preview、callback log を表示する。 |
| ColorPickerRgba | yes | yes | yes | yes | yes | yes | RGB/RGBA、alpha drag、size、border option、dark を表示する。 |
| TextInput | yes | yes | yes | yes | yes | yes | icon 内包、Hidden / Reserved / Visible、clear、loading、readonly、disabled、size を表示する。 |
| SearchBox | yes | yes | yes | yes | yes | yes | search icon 内包、preset、icon/control mode、regex / word / case、submit / clear、dark SVG を表示する。 |
| Tooltip | yes | yes | yes | yes | yes | yes | placement、delay、focus、close、edge flip を表示する。 |
| Badge | yes | n/a | yes | n/a | yes | n/a | tone、size、long text を表示する。 |
| KeyCap | yes | n/a | yes | n/a | yes | n/a | shortcut、size、disabled を表示する。 |
| ProgressBar | yes | n/a | yes | n/a | yes | n/a | determinate、indeterminate、label、percent、tone を表示する。 |
| StatusBar | yes | yes | yes | yes | yes | yes | severity、trailing、action、spinner、height / padding / gap を表示する。 |
| NotificationToast | yes | yes | yes | yes | yes | yes | severity、auto dismiss、manual dismiss、stack、position を表示する。 |
| Card | yes | yes | yes | yes | yes | yes | complex child node、slots、interactive child、padding、variants を表示する。 |
| Accordion | yes | yes | yes | yes | yes | yes | trigger area、default open、animation、nested、tree line を表示する。 |
| MenuButton | yes | yes | yes | yes | yes | yes | framed / unframed、icon、placement、open / close callbacks を表示する。 |
| SideMenu | yes | yes | yes | yes | yes | yes | left / right、width 0、hover、click 固定、pop 方式を表示する。 |
| CommandPalette | yes | yes | yes | yes | yes | yes | input、keyboard、Enter、Esc、provider callback、disabled / close log を表示する。 |
| SplitPane | yes | yes | yes | yes | yes | yes | horizontal / vertical、drag、reset、size を表示する。 |
| Modal | yes | yes | yes | yes | yes | yes | native window、close、Esc、focus return、parent 操作抑制を表示する。 |
| Popover | yes | yes | yes | yes | yes | yes | placement、outside click、Esc、content slot、edge flip を表示する。 |
| AlignCenter | yes | n/a | yes | n/a | yes | yes | width、height、padding、gap、disabled を表示する。 |
| Toolbar | yes | yes | yes | yes | yes | yes | leading / trailing、gap、alignment、action log を表示する。 |
| Breadcrumb | yes | yes | yes | yes | yes | yes | hover tree、JSON children、icon align、BG / border option、click log を表示する。 |
| DynamicArrayEditor | yes | yes | yes | yes | yes | yes | add、delete、edit、reorder、max、empty を表示する。 |
| Tabs | yes | yes | yes | yes | yes | yes | content あり / なし、外部 UI 連携、close、disabled、overflow を表示する。 |
| TreeView | yes | yes | yes | yes | yes | yes | nested JSON、icon align、trigger mode、expand controls、lines、context log を表示する。 |
| CodeDiff | yes | n/a | yes | n/a | yes | yes | split / inline、long line、theme を表示する。 |

## 追加対応（2026-05-13）

- `Text`: `max_lines` と `align` を Storybook に追加し、widget 側の表示にも反映した。
- `Spinner`: `speed_rps` の slow / normal / fast を追加した。
- `LoadingDots`: `dot_count`、`dot_gap`、`animation_speed_ms`、`color` を追加した。
- `SvgButton`: `size` と `loading` の実 widget 表示と callback log を追加した。
- `SelectBox`: `SelectSize` の sm / md / lg を追加した。
- `ComboBox`: `disabled`、`open`、`placement` を追加した。
- `TextInput`: `leading_icon_mode` の Hidden / Reserved / Visible を追加した。
- `SearchBox`: `icon_preset`、`icon_mode`、`control_mode` を追加した。
- `SlideControl`: `custom_format`、`disabled`、`readonly` を追加した。

## 検証結果（2026-05-13）

- `cargo fmt --all`: pass
- `just storybook-check`: pass
- `just ast-lint`: pass
- `just storybook-smoke`: pass

`storybook-smoke` はクラッシュ回帰の根拠としてのみ扱う。variant gate は上記の Storybook 画面上の表示・操作ログを根拠とする。
