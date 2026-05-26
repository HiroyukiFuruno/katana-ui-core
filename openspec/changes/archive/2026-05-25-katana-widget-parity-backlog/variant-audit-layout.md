# layout / navigation Storybook variant audit（23.1〜23.10）

## 対象

- feedback: `Tooltip`, `Badge`, `KeyCap`, `ProgressBar`, `StatusBar`, `NotificationToast`
- layout: `Card`, `Accordion`, `SideMenu`, `CommandPalette`, `SplitPane`, `Modal`, `Popover`, `AlignCenter`, `Toolbar`
- navigation: `Breadcrumb`, `Tabs`, `TreeView`
- data/editing: `SelectionList`, `DynamicArrayEditor`, `CodeDiff`

## 画面で確認できる項目

### Tooltip

- placement、delay、focus、close、edge flip を確認できる。
- Tooltip を開いても `floem::view_state` のクラッシュが発生しないことを `storybook-smoke` で確認済み。

### Card

- 複雑な子 node、form control、button、accordion 相当の nested content、padding、outlined / elevated / interactive を確認できる。
- 右ペインの縦スクロールは Storybook root 側で右端に寄せる。

### Accordion

- default open / closed、trigger area、nested、tree line、expand / collapse を確認できる。
- Storybook sidebar からも TreeView と組み合わせた実利用を確認する。

### Modal

- 主導線は native window で開く `Modal`。
- 同一ウィンドウ内 overlay は `OverlayDialog` として比較表示に分離済み。
- close、Esc、focus return、parent 操作抑制を確認できる。

### Popover / MenuButton / ComboBox / CommandPalette

- open / close、placement、outside click 相当、Esc 相当、callback log を画面上で確認できる。
- 以前発生していた初期表示クラッシュは `storybook-smoke` で回帰確認済み。

### SideMenu

- left / right、width、width 0、hover expand、fixed、modal-like / popover-like / expand pop を確認できる。

### AlignCenter / Toolbar / StatusBar

- width、height、padding、gap、alignment、leading / trailing、action log、severity を確認できる。

### Breadcrumb

- crumb 配列、separator、icon、disabled、click、long path 省略を確認できる。
- BG / border は option で、default は false。
- JSON 相当の children から hover tree を再帰表示できる。
- `Icon` を使い、icon と文字を上下中央に揃える。

### Tabs

- content あり、content なし、外部 UI 連携、閉じられる tab、disabled、overflow を確認できる。

### TreeView

- nested JSON 相当の children、parent / leaf icon、trigger mode、全開 / 全閉、horizontal line option、active、hover、context log を確認できる。
- 通常表示と virtualized mode は別例で確認できる。

### SelectionList / DynamicArrayEditor / CodeDiff

- SelectionList は section、marker、selected、disabled、もっと表示を確認できる。
- DynamicArrayEditor は add / delete / edit / reorder / max / empty を確認できる。
- CodeDiff は split / inline、long line、theme を確認できる。

## theme 観点

- light / dark は Storybook の global theme 切替で確認する。
- 各 page は theme token 経由で background、text、border、icon、hover、active 色を描画する。

## reference 観点

- KatanA 本家スクリーンショットとの差分が大きかった Storybook sidebar と TreeView は、箱型 Accordion 表示を廃止して TreeView 行表示へ戻した。
- Breadcrumb、ColorPicker、TreeView、Sidebar など、ユーザーフィードバックが出た reference 乖離は個別タスク化し、修正済みのものだけ完了扱いにした。

## 未確認

なし。

## 検証

- `cargo fmt --all`: pass
- `just storybook-check`: pass
- `just ast-lint`: pass
- `just storybook-smoke`: pass

`storybook-smoke` は初期表示クラッシュ確認だけに使い、variant 完了根拠にはしない。
